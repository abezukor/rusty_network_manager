//! # D-Bus interface proxy for: `org.freedesktop.NetworkManager.Device.Wireless`
//!
//! This code was generated by `zbus-xmlgen` `4.0.1` from D-Bus introspection data.
//! Source: `org.freedesktop.NetworkManager.Device.Wireless.xml`.
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

impl WirelessProxy<'_> {
    pub async fn new_from_path(
        device_path: zbus::zvariant::OwnedObjectPath,
        connection: &Connection,
    ) -> Result<WirelessProxy<'_>> {
        WirelessProxy::builder(connection)
            .path(device_path)
            .expect("Path not found")
            .build()
            .await
    }
}

#[proxy(
    default_path = "/org/freedesktop/NetworkManager/Wireless",
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Wireless",
    assume_defaults = true
)]
trait Wireless {
    /// GetAccessPoints method
    fn get_access_points(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// GetAllAccessPoints method
    fn get_all_access_points(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// RequestScan method
    fn request_scan(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// AccessPointAdded signal
    #[zbus(signal)]
    fn access_point_added(&self, access_point: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// AccessPointRemoved signal
    #[zbus(signal)]
    fn access_point_removed(
        &self,
        access_point: zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// AccessPoints property
    #[zbus(property)]
    fn access_points(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// ActiveAccessPoint property
    #[zbus(property)]
    fn active_access_point(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Bitrate property
    #[zbus(property)]
    fn bitrate(&self) -> zbus::Result<u32>;

    /// HwAddress property
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// LastScan property
    #[zbus(property)]
    fn last_scan(&self) -> zbus::Result<i64>;

    /// Mode property
    #[zbus(property)]
    fn mode(&self) -> zbus::Result<u32>;

    /// PermHwAddress property
    #[zbus(property)]
    fn perm_hw_address(&self) -> zbus::Result<String>;

    /// WirelessCapabilities property
    #[zbus(property)]
    fn wireless_capabilities(&self) -> zbus::Result<u32>;
}
