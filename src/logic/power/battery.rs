use futures_lite::StreamExt;
use tokio::sync::mpsc::Sender;
use zbus::Connection;

use crate::{Event, PowerDeviceProxy};

pub async fn is_battery() -> bool {
    let connection = Connection::system().await.unwrap();
    let proxy = PowerDeviceProxy::new(&connection).await.unwrap();
    let device_type = proxy.type_().await.unwrap();
    let power_supply = proxy.power_supply().await.unwrap();
    // According to
    // https://upower.freedesktop.org/docs/Device.html
    // device should met these conditions to be a Battery
    power_supply && device_type == 2
}

pub fn battery_state_thread(tx: Sender<Event>) {
    tokio::spawn(async move {
        let connection = Connection::system().await.unwrap();
        let proxy = PowerDeviceProxy::new(&connection).await.unwrap();
        let mut changes = proxy.receive_state_changed().await;
        while let Some(changed) = changes.next().await {
            if let Ok(state) = changed.get().await {
                let is_present = proxy.is_present().await.unwrap();
                let percentage = proxy.percentage().await.ok();
                tx.send(Event::Battery {
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

pub fn battery_present_thread(tx: Sender<Event>) {
    tokio::spawn(async move {
        let connection = Connection::system().await.unwrap();
        let proxy = PowerDeviceProxy::new(&connection).await.unwrap();
        let mut changes = proxy.receive_is_present_changed().await;
        while let Some(changed) = changes.next().await {
            if let Ok(is_present) = changed.get().await {
                let state = proxy.state().await.unwrap();
                let percentage = proxy.percentage().await.ok();
                tx.send(Event::Battery {
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

pub fn battery_percent_thread(tx: Sender<Event>) {
    tokio::spawn(async move {
        let connection = Connection::system().await.unwrap();
        let proxy = PowerDeviceProxy::new(&connection).await.unwrap();
        let mut changes = proxy.receive_percentage_changed().await;
        while let Some(changed) = changes.next().await {
            if let Ok(percentage) = changed.get().await {
                let is_present = proxy.is_present().await.unwrap();
                let state = proxy.state().await.unwrap();
                tx.send(Event::Battery {
                    is_present,
                    state,
                    percentage: Some(percentage),
                })
                .await
                .unwrap();
            }
        }
    });
}
