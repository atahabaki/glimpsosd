use crate::{Cli, model::config::Configuration};

use super::OSD_CSS;

pub(crate) struct GlimpsOSD {
    pub _style: String,
    pub _config: Configuration,
}

/// This is the fallback in case anything with the cli args
/// or configuration goes wrong if not used with
/// --no-fallback
impl Default for GlimpsOSD {
    fn default() -> Self {
        GlimpsOSD {
            _style: OSD_CSS.to_owned(),
            _config: Configuration::default(),
        }
    }
}

impl GlimpsOSD {
    pub(crate) fn from_cli(cli: Cli) -> Self {
        let _style = Cli::_get_style_from_cli(&cli);
        let _config = Cli::_get_config_from_cli(&cli);
        GlimpsOSD { _style, _config }
    }
}
