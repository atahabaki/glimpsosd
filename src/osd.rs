use clap::Parser;
use daemon::{app::GlimpsOSD, cli::Cli};
use logic::power::{
    battery::{battery_percent_thread, battery_present_thread, battery_state_thread, is_battery},
    profile::power_profile_thread,
};
use model::{event::Event, power_device::PowerDeviceProxy, power_profiles::PowerProfilesProxy};

pub(crate) mod daemon;
pub(crate) mod logic;
pub(crate) mod model;
pub(crate) mod ui;

mod tocss {
    pub(crate) trait ToCSSClasses {
        fn to_css_classes(&self) -> Vec<String>;
    }
}

impl tocss::ToCSSClasses for Event {
    fn to_css_classes(&self) -> Vec<String> {
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
                        model::event::BacklightDevice::Keyboard => "keyboard",
                        model::event::BacklightDevice::Display => "display",
                    }
                    .into(),
                );
            }
        }
        vec
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    power_profile_thread(tx.clone());

    if is_battery().await {
        battery_state_thread(tx.clone());
        battery_percent_thread(tx.clone());
        battery_present_thread(tx.clone());
    }

    while let Some(event) = rx.recv().await {
        GlimpsOSD::new().run(Cli::parse(), event);
    }
}
