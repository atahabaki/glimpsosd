pub(crate) fn osd_power_device(css_classes: Vec<String>, display_text: String) -> gtk::Label {
    gtk::Label::builder()
        .css_classes(css_classes)
        .label(display_text)
        .build()
}
