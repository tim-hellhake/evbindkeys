# EVBindKeys
**_Like XBindKeys but without the X and per device._**

EVBindKeys allows mapping arbitrary key events of specific input devices to shell command invocations.

Under the hood, it accesses the evdev kernel interface using libevdev.

# Motivation
I found some cheap mini USB keyboards and decided to use them as shortcut boards.
As every keyboard sends the same keycodes, I needed something that can differentiate between multiple input devices.

Additionally, I wanted to have something that works on a headless device without having to install X server.

# Usage
* Find the input device you want to use (`ls -l /dev/input/by-id`)
* Ensure that you have access to the input device (`sudo chown [user] /dev/input/by-id/example-event-kbd`)
* Lookup the name of the keys you want to use (https://github.com/ndesh26/evdev-rs/blob/0.5.0/src/enums.rs#L304)
* Create a config based on the `example.toml`
* Execute `evbindkeys [config-file]`
