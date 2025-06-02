use zbus::Result;
use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.UPower.KbdBacklight",
    default_service = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower/KbdBacklight"
)]
pub(crate) trait KeyboardBacklight {
    fn get_max_brightness(&self) -> Result<i32>;
    fn get_brightness(&self) -> Result<i32>;
    fn set_brightness(&self, brightness: i32) -> Result<()>;
    /// This dbus signal only works if your keyboard backlight is
    /// handled by this dbus service.
    /// Otherwise it is handled mostly by the kernel.
    #[zbus(signal)]
    fn brightness_changed(&self) -> Result<i32>;
}
