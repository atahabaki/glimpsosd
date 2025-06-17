use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Event {
    PowerProfile {
        new_profile: String,
    },
    Battery {
        is_present: bool,
        state: u32,
        percentage: f64,
    },
    Brightness {
        device: BacklightDevice,
        percent: f64,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum BacklightDevice {
    Keyboard,
    Display,
}

impl Event {
    pub fn _to_css_classes(&self) -> Vec<String> {
        let mut vec = Vec::<String>::new();
        match self {
            Event::PowerProfile { new_profile } => {
                vec.push("power_profile".into());
                match new_profile.as_str() {
                    "power-saver" | "balanced" | "performance" => vec.push(new_profile.clone()),
                    _ => vec.push("unknown".to_owned()),
                }
            }
            Event::Battery {
                is_present,
                state,
                percentage,
            } => {
                vec.push("battery".into());
                vec.push(if *is_present { "present" } else { "removed" }.into());
                vec.push(
                    match state {
                        1 => "charging",
                        2 => "discharging",
                        3 => "empty",
                        4 => "fully-charged",
                        5 => "pending-charge",
                        6 => "pending-discharge",
                        _ => "unknown",
                    }
                    .into(),
                );
                vec.push(
                    match percentage {
                        0_f64..10_f64 => "one",
                        10_f64..20_f64 => "two",
                        20_f64..30_f64 => "three",
                        30_f64..40_f64 => "four",
                        40_f64..50_f64 => "five",
                        50_f64..60_f64 => "six",
                        60_f64..70_f64 => "seven",
                        70_f64..80_f64 => "eight",
                        80_f64..90_f64 => "nine",
                        90_f64..100_f64 => "ten",
                        _ => "unknown",
                    }
                    .into(),
                );
            }
            Event::Brightness { device, percent: _ } => {
                vec.push("brightness".into());
                vec.push(
                    match device {
                        BacklightDevice::Keyboard => "keyboard",
                        BacklightDevice::Display => "display",
                    }
                    .into(),
                );
            }
        }
        vec
    }
}
