use std::collections::HashMap;

use zbus::{Result, proxy};

#[proxy(
    interface = "org.freedesktop.UPower.PowerProfiles",
    default_service = "org.freedesktop.UPower.PowerProfiles",
    default_path = "/org/freedesktop/UPower/PowerProfiles"
)]
pub(crate) trait PowerProfiles {
    #[zbus(property)]
    fn active_profile(&self) -> Result<String>;

    #[zbus(property)]
    fn set_active_profile(&self, value: String) -> Result<()>;

    #[zbus(property)]
    fn profiles(&self) -> Result<Vec<HashMap<String, String>>>;
}
