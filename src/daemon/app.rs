use std::time::Duration;

use crate::ui;
use crate::{Cli, Event};
use gtk::Application;
use gtk::CssProvider;
use gtk::glib;
use gtk::prelude::*;
use gtkls::{Edge, Layer, LayerShell};

use super::APP_ID;

pub struct GlimpsOSD {
    pub app: Application,
}

/// This is the fallback in case anything with the cli args
/// or configuration goes wrong if not used with
/// --no-fallback
impl Default for GlimpsOSD {
    fn default() -> Self {
        Self {
            app: gtk::Application::builder().application_id(APP_ID).build(),
        }
    }
}

impl GlimpsOSD {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn run(&self, cli: Cli, event: Event) {
        self.app.connect_activate(move |app| {
            let style = Cli::_get_style_from_cli(&cli);
            let config = Cli::_get_config_from_cli(&cli);
            let provider = CssProvider::new();
            provider.load_from_data(style.as_str());
            gtk::style_context_add_provider_for_display(
                &gtk::gdk::Display::default().expect("Could not get default display"),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
            let window = Self::osd_window(app);
            match &event {
                Event::PowerProfile { new_profile } => {
                    window.set_child(Some(&ui::osd_power_profile(
                        event.to_css_classes(),
                        config
                            .osd
                            .power_profile
                            .get_based_on_new_profile(new_profile),
                    )));
                }
                Event::Battery {
                    is_present: _,
                    state: _,
                    percentage,
                } if percentage.is_some() => window.set_child(Some(&ui::osd_battery(
                    event.to_css_classes(),
                    config.osd.battery.get_based_on_new_battery_status(&event),
                    percentage.unwrap() / 100_f64,
                ))),
                Event::Battery {
                    is_present: _,
                    state: _,
                    percentage: _,
                } => window.set_child(Some(&ui::osd_battery_without_level(
                    event.to_css_classes(),
                    config.osd.battery.get_based_on_new_battery_status(&event),
                ))),
                Event::Brightness {
                    device: _,
                    percent: _,
                } => todo!("We need brightness widget"),
            }
            window.init_layer_shell();
            window.set_layer(Layer::Overlay);
            let edge = Edge::from(config.positioning.anchor);
            window.set_anchor(edge, true);
            if let Some(left_margin) = config.positioning.margin {
                window.set_margin(edge, left_margin);
            }
            window.present();

            let window_weak = window.downgrade();
            glib::timeout_add_local(Duration::from_millis(config.duration), move || {
                if let Some(window) = window_weak.upgrade() {
                    window.close();
                }
                glib::ControlFlow::Break
            });
        });
        let args: Vec<String> = vec![];
        self.app.run_with_args(&args);
    }
}
