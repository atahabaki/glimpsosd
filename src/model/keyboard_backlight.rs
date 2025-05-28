use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.UPower.KbdBacklight",
    default_service = "org.freedesktop.UPower.KbdBacklight",
    default_path = "/org/freedesktop/UPower/KbdBacklight"
)]
pub(crate) trait KbdBacklight {
    #[zbus(signal)]
    fn brightness_changed(&self) -> Result<i32>;
}
