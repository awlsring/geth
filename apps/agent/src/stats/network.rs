use std::collections::HashMap;
use std::net::IpAddr;

use sysinfo::{System, SystemExt, NetworkData, NetworkExt};

use network_interface::Addr;
use network_interface::NetworkInterface as NetIface;
use network_interface::NetworkInterfaceConfig;
use network_interface::V6IfAddr;

use super::util::handle_optional_string;

static INTERFACE_ACCEPT_PREFIXES: [&str; 2] = ["eth", "en"];

pub enum AddressKind {
    V4,
    V6,
    V6Local,
}

pub struct Address {
    version: AddressKind,
    address: IpAddr,
    broadcast: Option<IpAddr>,
    netmask: Option<IpAddr>,
}

impl Address {
    pub fn new(addr: &Addr) -> Address {
        let version = match addr {
            Addr::V4(_addr) => AddressKind::V4,
            Addr::V6(addr) => {
                if Address::is_local_ipv6(&addr) {
                    AddressKind::V6Local
                } else {
                    AddressKind::V6
                }
            }
        };
        Address {
            version: version,
            address: addr.ip(),
            broadcast: addr.broadcast(),
            netmask: addr.netmask(),
        }
    }

    fn is_local_ipv6(addr: &V6IfAddr) -> bool {
        addr.ip.to_string().starts_with("fe80")
    }

    pub fn version(&self) -> &AddressKind {
        &self.version
    }

    pub fn address(&self) -> &IpAddr {
        &self.address
    }

    pub fn broadcast(&self) -> &Option<IpAddr> {
        &self.broadcast
    }

    pub fn netmask(&self) -> &Option<IpAddr> {
        &self.netmask
    }
}

pub struct NetworkInterfaceTraffic {
    transmitted: u64,
    recieved: u64
}

impl NetworkInterfaceTraffic {
    pub fn transmitted(&self) -> &u64 {
        &self.transmitted
    }

    pub fn recieved(&self) -> &u64 {
        &self.recieved
    }
}

pub struct NetworkInterface {
    name: String,
    addresses: Vec<Address>,
    mac: String,
    bytes: NetworkInterfaceTraffic,
    packets: NetworkInterfaceTraffic
}

impl NetworkInterface {
    pub fn new(iface: &NetIface) -> NetworkInterface {
        let name = &iface.name;
        let mac = handle_optional_string(iface.mac_addr.to_owned());

        let mut addresses = Vec::new();
        for addr in &iface.addr {
            addresses.push(Address::new(addr));
        }
        
        NetworkInterface {
            name: String::from(name),
            addresses: addresses,
            mac: mac,
            bytes: NetworkInterfaceTraffic {
                transmitted: 0,
                recieved: 0
            },
            packets: NetworkInterfaceTraffic {
                transmitted: 0,
                recieved: 0
            }
        }
    }

    pub fn addresses(&self) -> &Vec<Address> {
        &self.addresses
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn mac(&self) -> &String {
        &self.mac
    }

    pub fn bytes(&self) -> &NetworkInterfaceTraffic {
        &self.bytes
    }

    pub fn packets(&self) -> &NetworkInterfaceTraffic {
        &self.packets
    }

    pub fn update(&mut self, data: &NetworkData) {
        self.bytes.transmitted = data.total_transmitted();
        self.bytes.recieved = data.total_received();
        self.packets.transmitted = data.packets_transmitted();
        self.packets.recieved = data.packets_received();
    }

}

pub struct Network {
    interfaces: HashMap<String, NetworkInterface>
}

impl Network {
    pub fn new(system: &System) -> Network {
        let mut interfaces = HashMap::new();
        let network_interfaces = NetIface::show().unwrap();
        
        for interface in network_interfaces.iter() {
            let mut skip = true;
            for prefix in INTERFACE_ACCEPT_PREFIXES.iter() {
                if interface.name.starts_with(prefix) {
                    skip = false;
                }
            }
            if skip {
                continue;
            }

            if interface.addr.len() == 0 {
                continue;
            }

            let name = &interface.name;

            let network_interface = NetworkInterface::new(interface);
            interfaces.insert(String::from(name), network_interface);
        }        

        let mut net = Network {
            interfaces
        };
        net.update(system);

        net
    }

    pub fn get_network_interface(&self, name: &str) -> Option<&NetworkInterface> {
        self.interfaces.get(name)
    }

    pub fn network_interfaces(&self) -> Vec<&NetworkInterface> {
        let mut nics = Vec::new();

        for (_, nic) in &self.interfaces {
            nics.push(nic);
        }

        nics
    }

    pub fn update(&mut self, system: &System) {
        for (name, iface_data) in system.networks() {

            let iface = self.interfaces.get_mut(name);

            if iface.is_none() {
                continue;
            } else {
                let u = iface.unwrap();
                u.update(iface_data)
            }
        }
    }

}
