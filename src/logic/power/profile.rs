use futures_lite::StreamExt;
use tokio::sync::mpsc::Sender;
use zbus::Connection;

use crate::{Event, PowerProfilesProxy};

pub(crate) fn power_profile_thread(tx: Sender<Event>) {
    tokio::spawn(async move {
        let connection = Connection::system().await.unwrap();
        let proxy = PowerProfilesProxy::new(&connection).await.unwrap();
        let mut changes = proxy.receive_active_profile_changed().await;
        while let Some(changed) = changes.next().await {
            if let Ok(new_profile) = changed.get().await {
                tx.send(Event::PowerProfile { new_profile }).await.unwrap();
            }
        }
    });
}
