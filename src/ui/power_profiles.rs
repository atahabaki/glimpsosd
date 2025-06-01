pub(crate) fn osd_power_profile(state: String) -> gtk::Label {
    let label = match &state[..] {
        "power-saver" => "  Power-Saver",
        "balanced" => "  Balanced",
        "performance" => " Performance",
        _ => "",
    };
    gtk::Label::builder()
        .css_classes(vec!["power", state.clone().as_str()])
        .label(label)
        .build()
}
