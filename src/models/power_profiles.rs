use zbus::{Result, proxy};

#[proxy(
    interface = "org.freedesktop.UPower.PowerProfiles",
    default_service = "org.freedesktop.UPower.PowerProfiles",
    default_path = "/org/freedesktop/UPower/PowerProfiles"
)]
pub(crate) trait PowerProfiles {
    #[zbus(property)]
    fn active_profile(&self) -> Result<String>;
}
