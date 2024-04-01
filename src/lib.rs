mod network_manager;

pub use zbus::{Connection, Result, proxy};
pub use network_manager::NetworkManagerProxy;

#[cfg(feature = "access_point")]
pub use network_manager::access_point::AccessPointProxy;
#[cfg(feature = "access_point")]
pub use network_manager::access_point_flags::NM80211ApFlags;
#[cfg(feature = "access_point")]
pub use network_manager::access_point_security_flags::NM80211ApSecurityFlags;
#[cfg(feature = "active")]
pub use network_manager::active::ActiveProxy;
#[cfg(feature = "adsl")]
pub use network_manager::adsl::AdslProxy;
#[cfg(feature = "agent_manager")]
pub use network_manager::agent_manager::AgentManagerProxy;
#[cfg(feature = "bluetooth")]
pub use network_manager::bluetooth::BluetoothProxy;
#[cfg(feature = "bond")]
pub use network_manager::bond::BondProxy;
#[cfg(feature = "bridge")]
pub use network_manager::bridge::BridgeProxy;
#[cfg(feature = "access_point")]
pub use network_manager::channel::Channel;
#[cfg(feature = "checkpoint")]
pub use network_manager::checkpoint::CheckpointProxy;
#[cfg(feature = "connection")]
pub use network_manager::connection::ConnectionProxy;
#[cfg(feature = "device")]
pub use network_manager::device::DeviceProxy;
#[cfg(feature = "dhcp4config")]
pub use network_manager::dhcp4config::DHCP4ConfigProxy;
#[cfg(feature = "dhcp6config")]
pub use network_manager::dhcp6config::DHCP6ConfigProxy;
#[cfg(feature = "dummy")]
pub use network_manager::dummy::DummyProxy;
#[cfg(feature = "generic")]
pub use network_manager::generic::GenericProxy;
#[cfg(feature = "hsr")]
pub use network_manager::hsr::HsrProxy;
#[cfg(feature = "infiniband")]
pub use network_manager::infiniband::InfinibandProxy;
#[cfg(feature = "ip4config")]
pub use network_manager::ip4config::IP4ConfigProxy;
#[cfg(feature = "ip6config")]
pub use network_manager::ip6config::IP6ConfigProxy;
#[cfg(feature = "iptunnel")]
pub use network_manager::iptunnel::IPTunnelProxy;
#[cfg(feature = "loopback")]
pub use network_manager::loopback::LoopbackProxy;
#[cfg(feature = "lowpan")]
pub use network_manager::lowpan::LowpanProxy;
#[cfg(feature = "macsec")]
pub use network_manager::macsec::MacsecProxy;
#[cfg(feature = "macvlan")]
pub use network_manager::macvlan::MacvlanProxy;
#[cfg(feature = "modem")]
pub use network_manager::modem::ModemProxy;
#[cfg(feature = "nsp")]
pub use network_manager::nsp::NspProxy;
#[cfg(feature = "olpc_mesh")]
pub use network_manager::olpc_mesh::OlpcMeshProxy;
#[cfg(feature = "ovs_bridge")]
pub use network_manager::ovs_bridge::OvsBridgeProxy;
#[cfg(feature = "ovs_interface")]
pub use network_manager::ovs_interface::OvsInterfaceProxy;
#[cfg(feature = "ovs_port")]
pub use network_manager::ovs_port::OvsPortProxy;
//#[cfg(feature = "wireless")]
//pub use network_manager::plugin::
#[cfg(feature = "ppp")]
pub use network_manager::ppp::PPPProxy;
#[cfg(feature = "secret_agent")]
pub use network_manager::secret_agent::SecretAgentProxy;
#[cfg(feature = "settings")]
pub use network_manager::settings::SettingsProxy;
#[cfg(feature = "settings")]
pub use network_manager::settings::SettingsConnectionProxy;
#[cfg(feature = "statistics")]
pub use network_manager::statistics::StatisticsProxy;
#[cfg(feature = "team")]
pub use network_manager::team::TeamProxy;
#[cfg(feature = "tum")]
pub use network_manager::tun::TunProxy;
#[cfg(feature = "veth")]
pub use network_manager::veth::VethProxy;
#[cfg(feature = "vlan")]
pub use network_manager::vlan::VlanProxy;
#[cfg(feature = "vrf")]
pub use network_manager::vrf::VrfProxy;
#[cfg(feature = "vxlan")]
pub use network_manager::vxlan::VxlanProxy;
#[cfg(feature = "wi_max")]
pub use network_manager::wi_max::WiMaxProxy;
#[cfg(feature = "wifi_p2p")]
pub use network_manager::wifi_p2p::WifiP2PProxy;
#[cfg(feature = "wifi_p2ppeer")]
pub use network_manager::wifi_p2ppeer::WifiP2PPeerProxy;
#[cfg(feature = "wire_guard")]
pub use network_manager::wire_guard::WireGuardProxy;
#[cfg(feature = "wired")]
pub use network_manager::wired::WiredProxy;
#[cfg(feature = "wireless")]
pub use network_manager::wireless::WirelessProxy;
#[cfg(feature = "wpan")]
pub use network_manager::wpan::WpanProxy;

pub use network_manager::device_type::DeviceType;
pub use network_manager::state::State;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
