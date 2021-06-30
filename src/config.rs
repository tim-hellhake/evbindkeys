/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub devices: Vec<Device>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Device {
    pub file: String,
    pub exclusive: bool,
    pub bindings: Vec<Binding>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Binding {
    pub key: String,
    pub on_key_down: Option<String>,
    pub on_key_up: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_parse_example() {
        let example = read_to_string("example.toml").unwrap();
        let config: Config = toml::from_str(&example).unwrap();
        assert_eq!(
            config,
            Config {
                devices: vec![Device {
                    file: String::from("/dev/input/by-id/example-event-kbd"),
                    exclusive: true,
                    bindings: vec![
                        Binding {
                            key: String::from("KEY_3"),
                            on_key_down: Some(String::from("pamixer -i 10")),
                            on_key_up: None,
                        },
                        Binding {
                            key: String::from("KEY_6"),
                            on_key_down: Some(String::from("pamixer -d 10")),
                            on_key_up: None,
                        }
                    ],
                }]
            }
        );
    }
}
