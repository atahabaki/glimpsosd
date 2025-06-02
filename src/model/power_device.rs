#[zbus::proxy(
    interface = "org.freedesktop.UPower.Device",
    default_service = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower/Device"
)]
pub(crate) trait PowerDevice {
    #[zbus(property)]
    fn state(&self) -> zbus::Result<u32>;
}
