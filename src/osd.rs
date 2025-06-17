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
