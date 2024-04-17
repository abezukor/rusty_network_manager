//! # D-Bus interface proxy for: `org.freedesktop.NetworkManager.WiMax.Nsp`
//!
//! This code was generated by `zbus-xmlgen` `4.0.1` from D-Bus introspection data.
//! Source: `org.freedesktop.NetworkManager.WiMax.Nsp.xml`.
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

impl NspProxy<'_> {
    pub async fn new_from_path(
        device_path: zbus::zvariant::OwnedObjectPath,
        connection: &Connection,
    ) -> Result<NspProxy<'_>> {
        NspProxy::builder(connection)
            .path(device_path)?
            .build()
            .await
    }
}

#[proxy(
    default_path = "/org/freedesktop/NetworkManager/WiMax/Nsp",
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.WiMax.Nsp",
    assume_defaults = true
)]
trait Nsp {
    /// Name property
    #[zbus(property)]
    fn name(&self) -> zbus::Result<String>;

    /// NetworkType property
    #[zbus(property)]
    fn network_type(&self) -> zbus::Result<u32>;

    /// SignalQuality property
    #[zbus(property)]
    fn signal_quality(&self) -> zbus::Result<u32>;
}
