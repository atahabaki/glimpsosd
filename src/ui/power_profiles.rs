use crate::GlimpsOSD;

impl GlimpsOSD {
    pub(crate) fn osd_power_profile() -> gtk::Label {
        gtk::Label::builder()
            .css_classes(vec!["power", "power-saver"])
            .label("  Power-Saver")
            // .label("  Balanced")
            // .label(" Performance")
            .build()
    }
}
