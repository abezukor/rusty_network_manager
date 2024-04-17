//! # D-Bus interface proxy for: `org.freedesktop.NetworkManager.Device.OvsBridge`
//!
//! This code was generated by `zbus-xmlgen` `4.0.1` from D-Bus introspection data.
//! Source: `org.freedesktop.NetworkManager.Device.OvsBridge.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,
use zbus::{proxy, Connection, Result};

impl OvsBridgeProxy<'_> {
    pub async fn new_from_path(
        device_path: zbus::zvariant::OwnedObjectPath,
        connection: &Connection,
    ) -> Result<OvsBridgeProxy<'_>> {
        OvsBridgeProxy::builder(connection)
            .path(device_path)?
            .build()
            .await
    }
}

#[proxy(
    default_path = "/org/freedesktop/NetworkManager/Device/OvsBridge",
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.OvsBridge",
    assume_defaults = true
)]
trait OvsBridge {
    /// Slaves property
    #[zbus(property)]
    fn slaves(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
}
