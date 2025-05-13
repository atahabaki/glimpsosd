use gtk::CssProvider;
use gtk::glib;
use gtk::prelude::*;
use std::time::Duration;

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

    fn osd_volume_progressbar() -> gtk::ProgressBar {
        gtk::ProgressBar::builder()
            .css_classes(vec!["volume", "max"])
            .text("  @DEFAULT_SINK@: .75")
            .show_text(true)
            .fraction(0.75)
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
        let child = Self::osd_volume_progressbar();
        let window = Self::osd_window(app);
        window.set_child(Some(&child));
        window.present();

        let window_weak = window.downgrade();
        glib::timeout_add_local(Duration::from_millis(500), move || {
            if let Some(window) = window_weak.upgrade() {
                window.close();
            }
            glib::ControlFlow::Break
        });
    }
}

fn main() {
    let glimpsosd = GlimpsOSD::new();
    glimpsosd.connect_activate();
    glimpsosd.app.run();
}
