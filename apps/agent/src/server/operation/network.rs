use std::{sync::Arc, net::IpAddr};

use aws_smithy_http_server::Extension;
use geth_agent_server::{output::{GetNetworkInterfaceOutput, ListNetworkInterfacesOutput}, model::{NetworkInterfaceSummary, AddressSummary, AddressVersion, NetworkInterfaceTrafficSummary}, input::{GetNetworkInterfaceInput, ListNetworkInterfacesInput}, error};

use crate::{server::http::State, stats::network::{NetworkInterface, AddressKind}};


pub async fn get_network_interface(input: GetNetworkInterfaceInput, state: Extension<Arc<State>>) -> Result<GetNetworkInterfaceOutput, error::GetNetworkInterfaceError> {
    let ctl = state.controller.lock().await;
    let network = ctl.network();

    let net = network.get_network_interface(input.name());

    match net {
        Some(n) => {
            let sum = network_interface_to_summary(n);
            let output = GetNetworkInterfaceOutput { summary: sum };
            Ok(output)
        }
        None => Err(error::GetNetworkInterfaceError::ResourceNotFoundException(error::ResourceNotFoundException { message: format!("Network Interface {} not found", input.name()) }))
    }
}

pub async fn list_network_interfaces(_input: ListNetworkInterfacesInput, state: Extension<Arc<State>>) -> Result<ListNetworkInterfacesOutput, error::ListNetworkInterfacesError> {
    let ctl = state.controller.lock().await;
    let network = ctl.network();
    let sums = network_interfaces_to_summaries(network.network_interfaces());
    let output = ListNetworkInterfacesOutput { summaries: sums };

    Ok(output)
}

pub fn network_interfaces_to_summaries(nics: Vec<&NetworkInterface>) -> Vec<NetworkInterfaceSummary> {
    let mut summaries = Vec::new();
    for nic in nics {
        let sum = network_interface_to_summary(nic);
        summaries.push(sum);
    }

    summaries
}

pub fn network_interface_to_summary(iface: &NetworkInterface) -> NetworkInterfaceSummary {

    let name = iface.name().to_string();
    let mac = iface.mac().to_string();

    let mut addresses = Vec::new();
    for addr in iface.addresses() {
        let version = addr.version();
        let address = addr.address().to_string();
        let netmask = handle_optional_ip(addr.netmask());
        let broadcast = handle_optional_ip(addr.broadcast());

        let addr = AddressSummary {
            version: address_kind_to_smithy(version),
            address,
            netmask,
            broadcast,
        };

        addresses.push(addr);
    }
    
    NetworkInterfaceSummary { 
        name,
        addresses,
        mac_address: mac,
        bytes_traffic: NetworkInterfaceTrafficSummary {
            transmitted: *iface.bytes().transmitted() as i64,
            recieved: *iface.bytes().recieved() as i64,
        },
        packet_traffic: NetworkInterfaceTrafficSummary {
            transmitted: *iface.packets().transmitted() as i64,
            recieved: *iface.packets().recieved() as i64,
        },
    }
}

fn address_kind_to_smithy(kind: &AddressKind) -> AddressVersion {
    match kind {
        AddressKind::V4 => AddressVersion::V4,
        AddressKind::V6 => AddressVersion::V6,
        AddressKind::V6Local => AddressVersion::V6Local,
    }
}

fn handle_optional_ip(ip: &Option<IpAddr>) -> Option<String> {
    ip.as_ref().map(|ip| ip.to_string())
}