use std::time::Duration;

use crate::tocss::ToCSSClasses;
use crate::ui;
use crate::{Cli, Event};
use gtk::Application;
use gtk::CssProvider;
use gtk::glib;
use gtk::prelude::*;
use gtkls::{Edge, Layer, LayerShell};

use super::APP_ID;

pub(crate) struct GlimpsOSD {
    pub _app: Application,
}

/// This is the fallback in case anything with the cli args
/// or configuration goes wrong if not used with
/// --no-fallback
impl Default for GlimpsOSD {
    fn default() -> Self {
        GlimpsOSD {
            _app: gtk::Application::builder().application_id(APP_ID).build(),
        }
    }
}

impl GlimpsOSD {
    pub(crate) fn new() -> Self {
        GlimpsOSD {
            _app: gtk::Application::builder().application_id(APP_ID).build(),
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

    pub(crate) fn run(&self, cli: Cli, event: Event) {
        self._app.connect_activate(move |app| {
            let _style = Cli::_get_style_from_cli(&cli);
            let _config = Cli::_get_config_from_cli(&cli);
            let provider = CssProvider::new();
            provider.load_from_path("examples/style.css");
            gtk::style_context_add_provider_for_display(
                &gtk::gdk::Display::default().expect("Could not get default display"),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
            let window = Self::osd_window(app);
            let child = match &event {
                Event::PowerProfile { new_profile } => ui::osd_power_profile(
                    event.to_css_classes(),
                    _config
                        .osdtext
                        .power_profile_text
                        ._get_based_on_new_profile_text(new_profile),
                ),
                Event::PowerDevice { state } => todo!(),
                Event::Brightness { device, percent } => todo!("We need brightness widget"),
            };
            window.set_child(Some(&child));
            window.init_layer_shell();
            window.set_layer(Layer::Overlay);
            let edge = Edge::from(_config.positioning.anchor);
            window.set_anchor(Edge::from(edge), true);
            if let Some(left_margin) = _config.positioning.margin {
                window.set_margin(edge, left_margin);
            }
            window.present();

            let window_weak = window.downgrade();
            glib::timeout_add_local(Duration::from_millis(_config.duration), move || {
                if let Some(window) = window_weak.upgrade() {
                    window.close();
                }
                glib::ControlFlow::Break
            });
        });
        let args: Vec<String> = vec![];
        self._app.run_with_args(&args);
    }
}
