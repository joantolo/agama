//! Representation of the network configuration
//!
//! * This module contains the types that represent the network concepts. They are supposed to be
//! agnostic from the real network service (e.g., NetworkManager).
use uuid::Uuid;

use crate::error::NetworkStateError;
use agama_lib::network::types::SSID;
use std::{fmt, net::Ipv4Addr, str};

#[derive(Default)]
pub struct NetworkState {
    pub devices: Vec<Device>,
    pub connections: Vec<Connection>,
}

impl NetworkState {
    /// Returns a NetworkState struct with the given devices and connections.
    ///
    /// * `devices`: devices to include in the state.
    /// * `connections`: connections to include in the state.
    pub fn new(devices: Vec<Device>, connections: Vec<Connection>) -> Self {
        Self {
            devices,
            connections,
        }
    }

    /// Get device by name
    ///
    /// * `name`: device name
    pub fn get_device(&self, name: &str) -> Option<&Device> {
        self.devices.iter().find(|d| d.name == name)
    }

    /// Get connection by UUID
    ///
    /// * `uuid`: connection UUID
    pub fn get_connection(&self, uuid: Uuid) -> Option<&Connection> {
        self.connections.iter().find(|c| c.uuid() == uuid)
    }

    /// Get connection by UUID as mutable
    ///
    /// * `uuid`: connection UUID
    pub fn get_connection_mut(&mut self, uuid: Uuid) -> Option<&mut Connection> {
        self.connections.iter_mut().find(|c| c.uuid() == uuid)
    }

    /// Adds a new connection.
    ///
    /// It uses the `id` to decide whether the connection already exists.
    pub fn add_connection(&mut self, conn: Connection) -> Result<(), NetworkStateError> {
        if let Some(_) = self.get_connection(conn.uuid()) {
            return Err(NetworkStateError::ConnectionExists(conn.uuid()));
        }

        self.connections.push(conn);
        Ok(())
    }

    /// Updates a connection with a new one.
    ///
    /// It uses the `id` to decide which connection to update.
    ///
    /// Additionally, it registers the connection to be removed when the changes are applied.
    pub fn update_connection(&mut self, conn: Connection) -> Result<(), NetworkStateError> {
        let Some(old_conn) = self.get_connection_mut(conn.uuid()) else {
            return Err(NetworkStateError::UnknownConnection(conn.uuid()));
        };

        *old_conn = conn;
        Ok(())
    }

    /// Removes a connection from the state.
    ///
    /// Additionally, it registers the connection to be removed when the changes are applied.
    pub fn remove_connection(&mut self, uuid: Uuid) -> Result<(), NetworkStateError> {
        let Some(conn) = self.get_connection_mut(uuid) else {
            return Err(NetworkStateError::UnknownConnection(uuid));
        };

        conn.remove();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::{BaseConnection, Connection, EthernetConnection, NetworkState};
    use crate::error::NetworkStateError;

    #[test]
    fn test_add_connection() {
        let mut state = NetworkState::default();
        let uuid = Uuid::new_v4();
        let base = BaseConnection {
            uuid,
            ..Default::default()
        };
        let conn0 = Connection::Ethernet(EthernetConnection { base });
        state.add_connection(conn0).unwrap();
        let found = state.get_connection(uuid).unwrap();
        assert_eq!(found.uuid(), uuid);
    }

    #[test]
    fn test_add_duplicated_connection() {
        let mut state = NetworkState::default();
        let uuid = Uuid::new_v4();
        let base = BaseConnection {
            uuid,
            ..Default::default()
        };
        let conn0 = Connection::Ethernet(EthernetConnection { base });
        state.add_connection(conn0.clone()).unwrap();
        let error = state.add_connection(conn0).unwrap_err();
        assert!(matches!(error, NetworkStateError::ConnectionExists(_)));
    }

    #[test]
    fn test_update_connection() {
        let mut state = NetworkState::default();
        let uuid = Uuid::new_v4();
        let base0 = BaseConnection {
            uuid,
            ..Default::default()
        };
        let conn0 = Connection::Ethernet(EthernetConnection { base: base0 });
        state.add_connection(conn0).unwrap();

        let base1 = BaseConnection {
            uuid,
            ..Default::default()
        };
        let conn2 = Connection::Ethernet(EthernetConnection { base: base1 });
        state.update_connection(conn2).unwrap();
        let found = state.get_connection(uuid).unwrap();
        assert_eq!(found.uuid(), uuid);
    }

    #[test]
    fn test_update_unknown_connection() {
        let mut state = NetworkState::default();
        let uuid = Uuid::new_v4();
        let base = BaseConnection {
            uuid,
            ..Default::default()
        };
        let conn0 = Connection::Ethernet(EthernetConnection { base });
        let error = state.update_connection(conn0).unwrap_err();
        assert!(matches!(error, NetworkStateError::UnknownConnection(_)));
    }

    #[test]
    fn test_remove_connection() {
        let mut state = NetworkState::default();
        let uuid = Uuid::new_v4();
        let base0 = BaseConnection {
            uuid,
            ..Default::default()
        };
        let conn0 = Connection::Ethernet(EthernetConnection { base: base0 });
        state.add_connection(conn0).unwrap();
        state.remove_connection(uuid).unwrap();
        let found = state.get_connection(uuid).unwrap();
        assert!(found.is_removed());
    }

    #[test]
    fn test_remove_unknown_connection() {
        let mut state = NetworkState::default();
        let error = state.remove_connection(Uuid::new_v4()).unwrap_err();
        assert!(matches!(error, NetworkStateError::UnknownConnection(_)));
    }
}

/// Network device
#[derive(Debug, Clone)]
pub struct Device {
    pub name: String,
    pub type_: DeviceType,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DeviceType {
    Loopback = 0,
    Ethernet = 1,
    Wireless = 2,
}

impl TryFrom<u8> for DeviceType {
    type Error = NetworkStateError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DeviceType::Loopback),
            1 => Ok(DeviceType::Ethernet),
            2 => Ok(DeviceType::Wireless),
            _ => Err(NetworkStateError::InvalidDeviceType(value)),
        }
    }
}

/// Represents an available network connection
#[derive(Debug, PartialEq, Clone)]
pub enum Connection {
    Ethernet(EthernetConnection),
    Wireless(WirelessConnection),
    Loopback(LoopbackConnection),
}

impl Connection {
    pub fn new(id: String, device_type: DeviceType) -> Self {
        let base = BaseConnection {
            id: id.to_string(),
            ..Default::default()
        };
        match device_type {
            DeviceType::Wireless => Connection::Wireless(WirelessConnection {
                base,
                ..Default::default()
            }),
            DeviceType::Loopback => Connection::Loopback(LoopbackConnection { base }),
            DeviceType::Ethernet => Connection::Ethernet(EthernetConnection { base }),
        }
    }

    /// TODO: implement a macro to reduce the amount of repetitive code. The same applies to
    /// the base_mut function.
    pub fn base(&self) -> &BaseConnection {
        match &self {
            Connection::Ethernet(conn) => &conn.base,
            Connection::Wireless(conn) => &conn.base,
            Connection::Loopback(conn) => &conn.base,
        }
    }

    pub fn base_mut(&mut self) -> &mut BaseConnection {
        match self {
            Connection::Ethernet(conn) => &mut conn.base,
            Connection::Wireless(conn) => &mut conn.base,
            Connection::Loopback(conn) => &mut conn.base,
        }
    }

    pub fn id(&self) -> &str {
        self.base().id.as_str()
    }

    pub fn uuid(&self) -> Uuid {
        self.base().uuid
    }

    pub fn ipv4(&self) -> &Ipv4Config {
        &self.base().ipv4
    }

    pub fn ipv4_mut(&mut self) -> &mut Ipv4Config {
        &mut self.base_mut().ipv4
    }

    pub fn remove(&mut self) {
        self.base_mut().status = Status::Removed;
    }

    pub fn is_removed(&self) -> bool {
        self.base().status == Status::Removed
    }
}

#[derive(Debug, Default, Clone)]
pub struct BaseConnection {
    pub id: String,
    pub uuid: Uuid,
    pub ipv4: Ipv4Config,
    pub status: Status,
}

impl PartialEq for BaseConnection {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.uuid == other.uuid && self.ipv4 == other.ipv4
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Status {
    #[default]
    Present,
    Removed,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ipv4Config {
    pub method: IpMethod,
    pub addresses: Vec<(Ipv4Addr, u32)>,
    pub nameservers: Vec<Ipv4Addr>,
    pub gateway: Option<Ipv4Addr>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum IpMethod {
    #[default]
    Disabled = 0,
    Auto = 1,
    Manual = 2,
    LinkLocal = 3,
}
impl fmt::Display for IpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match &self {
            IpMethod::Disabled => "disabled",
            IpMethod::Auto => "auto",
            IpMethod::Manual => "manual",
            IpMethod::LinkLocal => "link-local",
        };
        write!(f, "{}", name)
    }
}

// NOTE: we could use num-derive.
impl TryFrom<u8> for IpMethod {
    type Error = NetworkStateError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(IpMethod::Disabled),
            1 => Ok(IpMethod::Auto),
            2 => Ok(IpMethod::Manual),
            3 => Ok(IpMethod::LinkLocal),
            _ => Err(NetworkStateError::InvalidIpMethod(value)),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct EthernetConnection {
    pub base: BaseConnection,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct WirelessConnection {
    pub base: BaseConnection,
    pub wireless: WirelessConfig,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LoopbackConnection {
    pub base: BaseConnection,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct WirelessConfig {
    pub mode: WirelessMode,
    pub ssid: SSID,
    pub password: Option<String>,
    pub security: SecurityProtocol,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum WirelessMode {
    Unknown = 0,
    AdHoc = 1,
    #[default]
    Infra = 2,
    AP = 3,
    Mesh = 4,
}

impl TryFrom<&str> for WirelessMode {
    type Error = NetworkStateError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "unknown" => Ok(WirelessMode::Unknown),
            "adhoc" => Ok(WirelessMode::AdHoc),
            "infrastructure" => Ok(WirelessMode::Infra),
            "ap" => Ok(WirelessMode::AP),
            "mesh" => Ok(WirelessMode::Mesh),
            _ => Err(NetworkStateError::InvalidWirelessMode(value.to_string())),
        }
    }
}

impl fmt::Display for WirelessMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match &self {
            WirelessMode::Unknown => "unknown",
            WirelessMode::AdHoc => "adhoc",
            WirelessMode::Infra => "infrastructure",
            WirelessMode::AP => "ap",
            WirelessMode::Mesh => "mesh",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SecurityProtocol {
    #[default]
    WEP, // No encryption or WEP ("none")
    OWE,            // Opportunistic Wireless Encryption ("owe")
    DynamicWEP,     // Dynamic WEP ("ieee8021x")
    WPA2,           // WPA2 + WPA3 personal ("wpa-psk")
    WPA3Personal,   // WPA3 personal only ("sae")
    WPA2Enterprise, // WPA2 + WPA3 Enterprise ("wpa-eap")
    WPA3Only,       // WPA3 only ("wpa-eap-suite-b192")
}

impl fmt::Display for SecurityProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match &self {
            SecurityProtocol::WEP => "none",
            SecurityProtocol::OWE => "owe",
            SecurityProtocol::DynamicWEP => "ieee8021x",
            SecurityProtocol::WPA2 => "wpa-psk",
            SecurityProtocol::WPA3Personal => "sae",
            SecurityProtocol::WPA2Enterprise => "wpa-eap",
            SecurityProtocol::WPA3Only => "wpa-eap-suite-b192",
        };
        write!(f, "{}", value)
    }
}

impl TryFrom<&str> for SecurityProtocol {
    type Error = NetworkStateError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "none" => Ok(SecurityProtocol::WEP),
            "owe" => Ok(SecurityProtocol::OWE),
            "ieee8021x" => Ok(SecurityProtocol::DynamicWEP),
            "wpa-psk" => Ok(SecurityProtocol::WPA2),
            "sae" => Ok(SecurityProtocol::WPA3Personal),
            "wpa-eap" => Ok(SecurityProtocol::WPA2Enterprise),
            "wpa-eap-suite-b192" => Ok(SecurityProtocol::WPA3Only),
            _ => Err(NetworkStateError::InvalidSecurityProtocol(
                value.to_string(),
            )),
        }
    }
}
