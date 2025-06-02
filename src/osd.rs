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
            Event::PowerDevice { state } => {
                vec.push("power_device".into());
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
            }
            Event::Brightness { device, percent } => {
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

    let tx_power_device = tx.clone();
    tokio::spawn(async move {
        let connection = Connection::system().await.unwrap();
        let proxy = PowerDeviceProxy::new(&connection).await.unwrap();
        let mut changes = proxy.receive_state_changed().await;
        while let Some(changed) = changes.next().await {
            if let Ok(state) = changed.get().await {
                tx_power_device
                    .send(Event::PowerDevice { state })
                    .await
                    .unwrap();
            }
        }
    });

    while let Some(event) = rx.recv().await {
        GlimpsOSD::new().run(Cli::parse(), event);
    }
}
