use clap::Parser;
use daemon::{app::GlimpsOSD, cli::Cli};
use futures_lite::StreamExt;
use model::{event::Event, power_device::PowerDeviceProxy, power_profiles::PowerProfilesProxy};
use zbus::Connection;

pub(crate) mod daemon;
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
    let tx_power_profile = tx.clone();

    tokio::spawn(async move {
        let connection = Connection::system().await.unwrap();
        let proxy = PowerProfilesProxy::new(&connection).await.unwrap();
        let mut changes = proxy.receive_active_profile_changed().await;
        while let Some(changed) = changes.next().await {
            if let Ok(new_profile) = changed.get().await {
                tx_power_profile
                    .send(Event::PowerProfile { new_profile })
                    .await
                    .unwrap();
            }
        }
    });

    let connection = Connection::system().await.unwrap();
    let proxy = PowerDeviceProxy::new(&connection).await.unwrap();
    let device_type = proxy.type_().await.unwrap();
    let power_supply = proxy.power_supply().await.unwrap();
    // According to
    // https://upower.freedesktop.org/docs/Device.html
    // device should met these conditions to be a Battery
    let is_battery = power_supply && device_type == 2;
    if is_battery {
        let tx_power_state = tx.clone();
        tokio::spawn(async move {
            let mut changes = proxy.receive_state_changed().await;
            while let Some(changed) = changes.next().await {
                if let Ok(state) = changed.get().await {
                    let is_present = proxy.is_present().await.unwrap();
                    let percentage = proxy.percentage().await.unwrap();
                    tx_power_state
                        .send(Event::Battery {
                            is_present,
                            state,
                            percentage,
                        })
                        .await
                        .unwrap();
                }
            }
        });
        let tx_power_is_present = tx.clone();
        tokio::spawn(async move {
            let connection = Connection::system().await.unwrap();
            let proxy = PowerDeviceProxy::new(&connection).await.unwrap();
            let mut changes = proxy.receive_is_present_changed().await;
            while let Some(changed) = changes.next().await {
                if let Ok(is_present) = changed.get().await {
                    let state = proxy.state().await.unwrap();
                    let percentage = proxy.percentage().await.unwrap();
                    tx_power_is_present
                        .send(Event::Battery {
                            is_present,
                            state,
                            percentage,
                        })
                        .await
                        .unwrap();
                }
            }
        });
        let tx_power_percentage = tx.clone();
        tokio::spawn(async move {
            let connection = Connection::system().await.unwrap();
            let proxy = PowerDeviceProxy::new(&connection).await.unwrap();
            let mut changes = proxy.receive_percentage_changed().await;
            while let Some(changed) = changes.next().await {
                if let Ok(percentage) = changed.get().await {
                    let is_present = proxy.is_present().await.unwrap();
                    let state = proxy.state().await.unwrap();
                    tx_power_percentage
                        .send(Event::Battery {
                            is_present,
                            state,
                            percentage,
                        })
                        .await
                        .unwrap();
                }
            }
        });
    }

    while let Some(event) = rx.recv().await {
        GlimpsOSD::new().run(Cli::parse(), event);
    }
}
