use futures_lite::StreamExt;
use gtk::CssProvider;
use gtk::glib;
use gtk::prelude::*;
use gtkls::{Edge, Layer, LayerShell};
use models::power_profiles::PowerProfilesProxy;
use std::time::Duration;
use zbus::Connection;

pub(crate) mod models;
pub(crate) mod ui;

const APP_ID: &str = "dev.atahabaki.glimpsosd";

struct GlimpsOSD {
    app: gtk::Application,
}

impl GlimpsOSD {
    fn new() -> Self {
        Self {
            app: gtk::Application::builder().application_id(APP_ID).build(),
        }
    }

    fn osd_window(app: &gtk::Application) -> gtk::ApplicationWindow {
        gtk::ApplicationWindow::builder()
            .application(app)
            .title("GlimpsOSD")
            .css_name("glimpsosd")
            .decorated(false)
            .resizable(false)
            .focusable(false)
            .build()
    }

    fn connect_activate(&self) {
        self.app.connect_activate(|app| {
            GlimpsOSD::on_activate(app);
        });
    }

    fn on_activate(app: &gtk::Application) {
        let provider = CssProvider::new();
        provider.load_from_path("examples/style.css");
        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().expect("Could not get default display"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        let window = Self::osd_window(app);
        window.init_layer_shell();
        window.set_layer(Layer::Overlay);
        window.set_anchor(Edge::Bottom, true);
        window.set_margin(Edge::Bottom, 50);
        window.present();

        let window_weak = window.downgrade();
        glib::timeout_add_local(Duration::from_millis(500), move || {
            if let Some(window) = window_weak.upgrade() {
                window.close();
            }
            glib::ControlFlow::Break
        });
    }

    fn run(&self) {
        self.connect_activate();
        self.app.run();
    }
}

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let connection = Connection::system().await?;
    let proxy = PowerProfilesProxy::new(&connection).await?;
    let mut changes = proxy.receive_active_profile_changed().await;
    while let Some(changed) = changes.next().await {
        if let Ok(new_profile) = changed.get().await {
            GlimpsOSD::new().run();
        }
    }
    Ok(())
}
