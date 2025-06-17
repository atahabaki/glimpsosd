pub fn osd_battery_without_level(css_classes: Vec<String>, display_text: String) -> gtk::Label {
    gtk::Label::builder()
        .css_classes(css_classes)
        .label(display_text)
        .build()
}

pub fn osd_battery(
    css_classes: Vec<String>,
    display_text: String,
    fraction: f64,
) -> gtk::ProgressBar {
    gtk::ProgressBar::builder()
        .css_classes(css_classes)
        .text(display_text)
        .show_text(true)
        .fraction(fraction)
        .build()
}
