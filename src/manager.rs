// Generated with `zbus-xmlgen system org.freedesktop.network1 /org/freedesktop/network1`, then
// trimmed to the functionality we need.

use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.network1.Manager",
    default_service = "org.freedesktop.network1",
    default_path = "/org/freedesktop/network1",
    gen_async = false,
    gen_blocking = true,
    blocking_name = "ManagerProxy"
)]
pub trait Manager {
    #[zbus(property)]
    fn online_state(&self) -> zbus::Result<String>;
}
