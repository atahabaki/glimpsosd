use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Event {
    PowerProfile {
        new_profile: String,
    },
    Brightness {
        device: BacklightDevice,
        percent: f64,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum BacklightDevice {
    Keyboard,
    Display,
}
