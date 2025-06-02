use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Event {
    PowerProfile {
        new_profile: String,
    },
    Battery {
        is_present: bool,
        state: u32,
        percentage: f64,
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
