use std::time::Duration;

use crate::{Cli, Event, model::config::Configuration};
use gtk::Application;
use gtk::CssProvider;
use gtk::glib;
use gtk::prelude::*;
use gtkls::{Edge, Layer, LayerShell};

use super::{APP_ID, OSD_CSS};

pub(crate) struct GlimpsOSD {
    pub _style: String,
    pub _config: Configuration,
    pub _app: Application,
}

/// This is the fallback in case anything with the cli args
/// or configuration goes wrong if not used with
/// --no-fallback
impl Default for GlimpsOSD {
    fn default() -> Self {
        GlimpsOSD {
            _app: gtk::Application::builder().application_id(APP_ID).build(),
            _style: OSD_CSS.to_owned(),
            _config: Configuration::default(),
        }
    }
}

impl GlimpsOSD {
    pub(crate) fn from_cli(cli: Cli) -> Self {
        let _style = Cli::_get_style_from_cli(&cli);
        let _config = Cli::_get_config_from_cli(&cli);

        GlimpsOSD {
            _app: gtk::Application::builder().application_id(APP_ID).build(),
            _style,
            _config,
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

    pub(crate) fn run(&self, event: Event) {
        self._app.connect_activate(move |app| {
            let provider = CssProvider::new();
            provider.load_from_path("examples/style.css");
            gtk::style_context_add_provider_for_display(
                &gtk::gdk::Display::default().expect("Could not get default display"),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
            let window = Self::osd_window(app);
            let child = match &event {
                Event::Power { new_profile } => {
                    GlimpsOSD::osd_power_profile(new_profile.to_owned())
                }
                Event::Brightness { device, percent } => todo!("We need brightness widget"),
            };
            window.set_child(Some(&child));
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
        self._app.run();
    }
}
