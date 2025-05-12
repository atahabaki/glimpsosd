use gtk::prelude::*;

const APP_ID: &str = "dev.atahabaki.glimpsosd";

fn on_activate(app: &gtk::Application) {
    let button = gtk::Label::new(Some("glimpsosd: Fresh new OSD"));
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("glimpsosd")
        .decorated(false)
        .resizable(false)
        .focusable(false)
        .child(&button)
        .build();
    window.present();
}

fn main() {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(on_activate);
    app.run();
}
