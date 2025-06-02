use clap::Parser;
use daemon::{app::GlimpsOSD, cli::Cli};
use futures_lite::StreamExt;
use model::{event::Event, power_profiles::PowerProfilesProxy};
use zbus::Connection;

pub(crate) mod daemon;
pub(crate) mod model;
pub(crate) mod ui;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    let tx_power = tx.clone();

    tokio::spawn(async move {
        let connection = Connection::system().await.unwrap();
        let proxy = PowerProfilesProxy::new(&connection).await.unwrap();
        let mut changes = proxy.receive_active_profile_changed().await;
        while let Some(changed) = changes.next().await {
            if let Ok(new_profile) = changed.get().await {
                tx_power
                    .send(Event::PowerProfile { new_profile })
                    .await
                    .unwrap();
            }
        }
    });

    while let Some(event) = rx.recv().await {
        GlimpsOSD::new().run(Cli::parse(), event);
    }
}
