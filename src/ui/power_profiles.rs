pub(crate) fn osd_power_profile(css_class: String, display_text: String) -> gtk::Label {
    gtk::Label::builder()
        .css_classes(vec!["power", css_class.clone().as_str()])
        .label(display_text)
        .build()
}
