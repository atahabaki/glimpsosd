use zbus::{Result, proxy};

#[proxy(
    interface = "org.freedesktop.UPower.Device",
    default_service = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower/devices/DisplayDevice"
)]
pub(crate) trait PowerDevice {
    #[zbus(property)]
    fn state(&self) -> Result<u32>;
    #[zbus(property)]
    fn percentage(&self) -> Result<f64>;
    #[zbus(property)]
    fn is_present(&self) -> Result<bool>;
    #[zbus(property)]
    fn type_(&self) -> Result<u32>;
    #[zbus(property)]
    fn power_supply(&self) -> Result<bool>;
}
