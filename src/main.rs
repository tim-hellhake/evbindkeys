/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::config::{Binding, Config, Device};
use evdev_rs::enums::*;
use evdev_rs::{DeviceWrapper, GrabMode, InputEvent, ReadFlag, ReadStatus, UninitDevice};
use std::fs::read_to_string;
use std::fs::File;
use std::thread;
use std::thread::JoinHandle;
use subprocess::{Exec, ExitStatus};

mod config;

fn main() {
    let mut args = std::env::args();

    if args.len() != 2 {
        println!("Usage: evbindkeys [config-file]");
        std::process::exit(1);
    }

    let example = read_to_string(&args.nth(1).unwrap()).unwrap();
    let config: Config = toml::from_str(&example).unwrap();
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for device in config.devices {
        let handle = start_loop(device);
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn start_loop(
    Device {
        file: path,
        exclusive,
        bindings,
    }: Device,
) -> JoinHandle<()> {
    println!("Opening input device {}", &path);
    let file = File::open(&path).unwrap();
    let un_init_device = UninitDevice::new().unwrap();
    let mut device = un_init_device.set_file(file).unwrap();

    if exclusive {
        println!("Requesting exclusive access to input device {}", &path);
        match device.grab(GrabMode::Grab) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Could not get exclusive access to device: {}", err)
            }
        }
    }

    println!(
        "Starting to poll device '{}' (0x{:x}/0x{:x}) at {} ({})",
        device.name().unwrap_or(""),
        device.vendor_id(),
        device.product_id(),
        &path,
        device.phys().unwrap_or("")
    );

    thread::spawn(move || loop {
        match device.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING) {
            Ok((status, event)) => match status {
                ReadStatus::Success => handle_event(&bindings, &event),
                _ => {}
            },
            Err(err) => {
                eprintln!("Failed to get next event from {}: {}", path, err)
            }
        };
    })
}

fn handle_event(
    bindings: &Vec<Binding>,
    InputEvent {
        time: _,
        event_code,
        value,
    }: &InputEvent,
) {
    match event_code {
        EventCode::EV_KEY(_) => {
            for binding in bindings {
                if binding.key == event_code.to_string() {
                    match value {
                        0 => match &binding.on_key_down {
                            Some(cmd) => {
                                exec(cmd);
                            }
                            None => {}
                        },
                        1 => match &binding.on_key_up {
                            Some(cmd) => {
                                exec(cmd);
                            }
                            None => {}
                        },
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }
}

fn exec(cmd: &str) {
    println!("Executing '{}'", cmd);

    match Exec::shell(cmd).join() {
        Ok(status) => match status {
            ExitStatus::Exited(0) => {}
            _ => {
                eprintln!("Command '{}' exited with {:?}", cmd, status)
            }
        },
        Err(err) => {
            eprintln!("Failed to execute '{}': {}", cmd, err)
        }
    };
}
