use crate::model::config::Configuration;

use super::OSD_CSS;

pub(crate) struct GlimpsOSD {
    pub style: String,
    pub config: Configuration,
}

/// This is the fallback in case anything with the cli args
/// or configuration goes wrong if not used with
/// --no-fallback
impl Default for GlimpsOSD {
    fn default() -> Self {
        GlimpsOSD {
            style: OSD_CSS.to_owned(),
            config: Configuration::default(),
        }
    }
}
