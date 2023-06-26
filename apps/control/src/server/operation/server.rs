

// fn agent_to_control_system_summary(summary: geth_agent_client::types::SystemSummary) -> geth_control_server::model::SystemSummary {
//     geth_control_server::model::SystemSummary {
//         hostname: summary.hostname().unwrap_or_default().to_string(),
//         machine_id: summary.machine_id().unwrap_or_default().to_string(),
//         family: summary.family().unwrap_or_default().to_string(),
//         kernel_version: summary.kernel_version().unwrap_or_default().to_string(),
//         os: summary.os().unwrap_or_default().to_string(),
//         os_version: summary.os_version().unwrap_or_default().to_string(),
//         os_pretty: summary.os_pretty().unwrap_or_default().to_string(),
//         boot_time: summary.boot_time().unwrap_or_default(),
//         up_time: summary.up_time().unwrap_or_default(),
//     }
// }

// fn agent_to_control_cpu_summary(summary: &geth_agent_client::types::CpuSummary) -> geth_control_server::model::CpuSummary {
//     let mut core_utils = Vec::new();
//     match summary.utilization() {
//         Some(util) => {
//             for core in util {
//                 core_utils.push(agent_to_control_core_utilization(core));
//             }
//         },
//         None => ()
//     }
    
//     geth_control_server::model::CpuSummary {
//         model: summary.model().unwrap_or_default().to_string(),
//         vendor: summary.vendor().unwrap_or_default().to_string(),
//         cores: summary.cores().unwrap_or_default(), 
//         architecture: summary.architecture().unwrap_or_default().to_string(),
//         utilization: core_utils
//     }
// }

// fn agent_to_control_memory_summary(summary: &geth_agent_client::types::MemorySummary) -> geth_control_server::model::MemorySummary {
//    let mem_sum = match summary.memory() {
//         Some(mem) => {
//             agent_to_control_memory_object(mem)
//         },
//         None => geth_control_server::model::MemoryTypeSummary {
//             total: 0,
//             available: 0,
//             used: 0,
//         }
//    };

//    let swap_sum = match summary.swap() {
//         Some(swap) => {
//             agent_to_control_memory_object(swap)
//         },
//         None => geth_control_server::model::MemoryTypeSummary {
//             total: 0,
//             available: 0,
//             used: 0,
//         }
//     };
    
//     geth_control_server::model::MemorySummary {
//         memory: mem_sum,
//         swap: swap_sum,
//     }
// }

// fn agent_to_control_memory_object(obj: &geth_agent_client::types::MemoryTypeSummary) -> geth_control_server::model::MemoryTypeSummary {
//     geth_control_server::model::MemoryTypeSummary {
//         total: obj.total().unwrap_or_default(),
//         available: obj.available().unwrap_or_default(),
//         used: obj.used().unwrap_or_default(),
//     }
// }

// fn agent_to_control_disk_summaries(d: Vec<&geth_agent_client::types::DiskSummary>) -> Vec<geth_control_server::model::DiskSummary> {
//     let mut disks = Vec::new();
//     for disk in d {
//         disks.push(agent_to_control_disk_object(disk));
//     }
//     disks
// }

// fn agent_to_control_disk_object(obj: &geth_agent_client::types::DiskSummary) -> geth_control_server::model::DiskSummary {
//     geth_control_server::model::DiskSummary {
//         device: obj.device().unwrap_or_default().to_string(),
//         model: obj.model().unwrap_or_default().to_string(),
//         vendor: obj.vendor().unwrap_or_default().to_string(),
//         interface: match obj.interface() {
//             Some(i) => {
//                 match i {
//                     geth_agent_client::types::DiskInterface::Sata => DiskInterface::Sata,
//                     geth_agent_client::types::DiskInterface::Scsi => DiskInterface::Scsi,
//                     geth_agent_client::types::DiskInterface::PciE => DiskInterface::PciE,
//                     _ => DiskInterface::Unknown,
//                 }
//             }
//             None => DiskInterface::Unknown
//         },
//         serial: obj.serial().unwrap_or_default().to_string(),
//         r#type: match obj.r#type() {
//             Some(t) => {
//                 match t {
//                     geth_agent_client::types::DiskType::Hdd => DiskType::Hdd,
//                     geth_agent_client::types::DiskType::Ssd => DiskType::Ssd,
//                     geth_agent_client::types::DiskType::Nvme => DiskType::Nvme,
//                     _ => DiskType::Unknown,
//                 }
//             }
//             None => DiskType::Unknown
//         },
//         sector_size: obj.sector_size().unwrap_or_default(),
//         size_raw: obj.size_raw().unwrap_or_default(),
//         size_actual: obj.size_actual().unwrap_or_default(),
//     }
// }

// fn agent_to_control_network_object(obj: &geth_agent_client::types::NetworkInterfaceSummary) -> geth_control_server::model::NetworkInterfaceSummary {
//     geth_control_server::model::NetworkInterfaceSummary {
//         name: obj.name().unwrap_or_default().to_string(),
//         addresses: match obj.addresses() {
//             Some(a) => {
//                 let mut addresses = Vec::new();
//                 for ad in a.iter() {
//                     addresses.push(agent_to_control_address_object(ad));
//                 }
//                 addresses
//             },
//             None => Vec::new()
//         },
//         bytes_traffic: agent_to_control_network_traffic_object(obj.bytes_traffic()),
//         packet_traffic: agent_to_control_network_traffic_object(obj.packet_traffic()),
//         r#virtual: obj.r#virtual().unwrap_or_default(),
//         mac_address: handle_optional_string_conv(obj.mac_address()),
//         vendor: handle_optional_string_conv(obj.vendor()),
//         mtu: obj.mtu(),
//         duplex: handle_optional_string_conv(obj.duplex()),
//         speed: obj.speed(),
//     }
// }

// fn agent_to_control_address_summaries(a: Option<Vec<&geth_agent_client::types::AddressSummary>>) -> Vec<geth_control_server::model::AddressSummary> {
//     match a {
//         Some(addresses) => {
//             let mut a = Vec::new();
//             for address in addresses {
//                 a.push(agent_to_control_address_object(address));
//             }
//             a
//         },
//         None => Vec::new(),
//     }
// }

// fn agent_to_control_address_object(obj: &geth_agent_client::types::AddressSummary) -> geth_control_server::model::AddressSummary {
//     geth_control_server::model::AddressSummary {
//         version: match obj.version() {
//             Some(v) => {
//                 match v {
//                     geth_agent_client::types::AddressVersion::V4 => AddressVersion::V4,
//                     geth_agent_client::types::AddressVersion::V6 => AddressVersion::V6,
//                     geth_agent_client::types::AddressVersion::V6Local => AddressVersion::V6Local,
//                     _ => AddressVersion::V4,
//                 }
//             }
//             None => AddressVersion::V4,
//         },
//         address: obj.address().unwrap_or_default().to_string(),
//         netmask: handle_optional_string_conv(obj.netmask()),
//         broadcast: handle_optional_string_conv(obj.broadcast()),
//     }
// }

// fn agent_to_control_network_traffic_object(n: Option<&geth_agent_client::types::NetworkInterfaceTrafficSummary>) -> geth_control_server::model::NetworkInterfaceTrafficSummary {
//     match n {
//         Some(n) => {
//             geth_control_server::model::NetworkInterfaceTrafficSummary {
//                 transmitted: n.transmitted().unwrap_or_default(),
//                 recieved: n.recieved().unwrap_or_default(),
//             }
//         },
//         None => geth_control_server::model::NetworkInterfaceTrafficSummary {
//             transmitted: 0,
//             recieved: 0,
//         }
//     }
// }

// fn agent_to_control_core_utilization(summary: &geth_agent_client::types::CoreUtilization) -> geth_control_server::model::CoreUtilization {
//     geth_control_server::model::CoreUtilization {
//         name: summary.name().unwrap_or_default().to_string(),
//         usage: summary.usage().unwrap_or_default(),
//         frequency: summary.frequency().unwrap_or_default(),
//     }
// }

// fn handle_optional_string_conv(s: Option<&str>) -> Option<String> {
//     match s {
//         Some(s) => Some(s.to_string()),
//         None => None,
//     }
// }


// // fn agent_to_control_server_summary(summary: geth_agent_client::types::OverviewSummary) -> OverviewSummary {
// //     let system = summary.system();
// //     let s = match system {
// //         Some(sum) => {
// //             geth_control_server::model::SystemSummary {
// //                 hostname: sum.hostname().unwrap_or_default().to_string(),
// //                 machine_id: todo!(),
// //                 family: todo!(),
// //                 kernel_version: todo!(),
// //                 os: todo!(),
// //                 os_version: todo!(),
// //                 os_pretty: todo!(),
// //                 boot_time: todo!(),
// //                 up_time: todo!(),
// //             }
// //         },
// //         None => {
// //             geth_control_server::model::SystemSummary {
// //                 hostname: "".to_string(),
// //                 machine_id: "".to_string(),
// //                 family: "".to_string(),
// //                 kernel_version: "".to_string(),
// //                 os: "".to_string(),
// //                 os_version: "".to_string(),
// //                 os_pretty: "".to_string(),
// //                 boot_time: 0,
// //                 up_time: 0,
// //             }
// //         }
// //     }

    
// // }