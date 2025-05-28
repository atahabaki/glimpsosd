use clap::Parser;
use clap::command;
use futures_lite::StreamExt;
use gtk::CssProvider;
use gtk::glib;
use gtk::prelude::*;
use gtkls::{Edge, Layer, LayerShell};
use model::power_profiles::PowerProfilesProxy;
use std::path::PathBuf;
use std::time::Duration;
use zbus::Connection;

pub(crate) mod model;
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

    fn run(&self, event: GlimpsOSDEvent) {
        self.app.connect_activate(move |app| {
            let provider = CssProvider::new();
            provider.load_from_path("examples/style.css");
            gtk::style_context_add_provider_for_display(
                &gtk::gdk::Display::default().expect("Could not get default display"),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
            let window = Self::osd_window(app);
            match &event {
                GlimpsOSDEvent::Power(new_power_profile) => {
                    window.set_child(Some(&Self::osd_power_profile(
                        new_power_profile.to_string(),
                    )));
                }
            }
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
        });
        self.app.run();
    }
}

#[derive(Debug)]
enum GlimpsOSDEvent {
    Power(String),
}

#[derive(Parser)]
#[command(about, version)]
struct Cli {
    /// Use this style.css file instead.
    /// By default, glimpsosd uses XDG_CONFIG_HOME/glimpsosd/style.css
    #[arg(short, long)]
    style: Option<PathBuf>,
    /// Use this config.ron file instead.
    /// By default, glimpsosd uses XDG_CONFIG_HOME/glimpsosd/config.ron
    #[arg(short, long)]
    config: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    let tx_power = tx.clone();

    tokio::spawn(async move {
        let connection = Connection::system().await.unwrap();
        let proxy = PowerProfilesProxy::new(&connection).await.unwrap();
        let mut changes = proxy.receive_active_profile_changed().await;
        while let Some(changed) = changes.next().await {
            if let Ok(new_profile) = changed.get().await {
                tx_power
                    .send(GlimpsOSDEvent::Power(new_profile))
                    .await
                    .unwrap();
            }
        }
    });

    while let Some(event) = rx.recv().await {
        GlimpsOSD::new().run(event);
    }
}
