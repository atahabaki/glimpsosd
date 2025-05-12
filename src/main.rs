use gtk::CssProvider;
use gtk::glib;
use gtk::prelude::*;
use std::time::Duration;

const APP_ID: &str = "dev.atahabaki.glimpsosd";

fn on_activate(app: &gtk::Application) {
    let provider = CssProvider::new();
    provider.load_from_path("examples/style.css");
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not get default display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    let progressbar = gtk::ProgressBar::builder()
        .css_classes(vec!["volume", "max"])
        .text("  @DEFAULT_SINK@: .75")
        .show_text(true)
        .fraction(0.75)
        .build();
    let window = osd_window(app);
    window.present();

    let window_weak = window.downgrade();
    glib::timeout_add_local(Duration::from_millis(500), move || {
        if let Some(window) = window_weak.upgrade() {
            window.close();
        }
        glib::ControlFlow::Break
    });
}

fn osd_window(app: &gtk::Application) -> gtk::ApplicationWindow {
    gtk::ApplicationWindow::builder()
        .application(app)
        .title("glimpsosd")
        .css_name("glimpsosd")
        .decorated(false)
        .resizable(false)
        .focusable(false)
        .build()
}

fn main() {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(on_activate);
    app.run();
}
