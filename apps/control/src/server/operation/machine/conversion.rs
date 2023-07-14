use geth_control_server::model::{
    CpuSummary, DiskSummary, MachineStatus, MachineSummary, MemorySummary, MemoryTypeSummary,
    NetworkInterfaceSummary, SystemSummary, Tag, TagString,
};

use crate::model::machine::{AddressVersion, DiskInterface, DiskType, Machine, MachineState};

pub fn machine_to_summary(machine: Machine) -> MachineSummary {
    MachineSummary {
        id: machine.id.to_string(),
        group: machine.group.to_string(),
        status: match machine.status.state {
            MachineState::Running => MachineStatus::Running,
            MachineState::Stopped => MachineStatus::Stopped,
            MachineState::Unknown => MachineStatus::Unknown,
        },
        added: machine.added.timestamp(),
        tags: machine
            .tags
            .iter()
            .map(|t| Tag {
                key: TagString::try_from(t.key.to_string())
                    .unwrap_or(TagString::try_from("error".to_string()).unwrap()),
                value: TagString::try_from(t.value.to_string())
                    .unwrap_or(TagString::try_from("error".to_string()).unwrap()),
            })
            .collect(),
        updated: machine.updated.map(|u| u.timestamp()),
        system: match machine.system {
            Some(s) => Some(SystemSummary {
                machine_id: s.machine_id.to_string(),
                family: s.family.to_string(),
                kernel_version: s.kernel_version.to_string(),
                os_version: s.os_version.to_string(),
                os_pretty: s.os_pretty.to_string(),
                os: s.os.to_string(),
                hostname: s.hostname.to_string(),
            }),
            None => None,
        },
        memory: machine.memory.map(|m| MemorySummary {
            memory: MemoryTypeSummary {
                total: m.memory as i64,
            },
            swap: MemoryTypeSummary {
                total: m.swap as i64,
            },
        }),
        cpu: match machine.cpu {
            Some(c) => Some(CpuSummary {
                cores: c.cores as i32,
                architecture: c.architecture.to_string(),
                model: c.model.map(|m| m.to_string()),
                vendor: c.vendor.map(|m| m.to_string()),
            }),
            None => None,
        },
        network: match machine.network_interfaces {
            Some(n) => {
                let mut interfaces = Vec::new();
                for interface in n.iter() {
                    interfaces.push(NetworkInterfaceSummary {
                        name: interface.name.to_string(),
                        addresses: interface.addresses.iter().map(|a| a.to_string()).collect(),
                        mac_address: interface.mac.clone().map(|m| m.to_string()),
                        r#virtual: interface.r#virtual,
                        vendor: interface.vendor.clone().map(|v| v.to_string()),
                        mtu: interface.mtu.map(|m| m as i32),
                        duplex: interface.duplex.clone().map(|d| d.to_string()),
                        speed: interface.speed.map(|s| s as i32),
                    });
                }
                Some(interfaces)
            }
            None => None,
        },
        disks: match machine.disks {
            Some(d) => {
                let mut disks = Vec::new();
                for disk in d.iter() {
                    disks.push(DiskSummary {
                        device: disk.device.to_string(),
                        r#type: match disk.r#type {
                            DiskType::HDD => geth_control_server::model::DiskType::Hdd,
                            DiskType::SSD => geth_control_server::model::DiskType::Ssd,
                            DiskType::NVME => geth_control_server::model::DiskType::Nvme,
                            DiskType::Unknown => geth_control_server::model::DiskType::Unknown,
                        },
                        size_actual: disk.size_actual as i64,
                        size_raw: disk.size_raw.map(|s| s as i64),
                        sector_size: disk.sector_size.map(|s| s as i32),
                        interface: match disk.interface {
                            DiskInterface::SCSI => {
                                Some(geth_control_server::model::DiskInterface::Scsi)
                            }
                            DiskInterface::SATA => {
                                Some(geth_control_server::model::DiskInterface::Sata)
                            }
                            DiskInterface::PCIE => {
                                Some(geth_control_server::model::DiskInterface::PciE)
                            }
                            _ => Some(geth_control_server::model::DiskInterface::Unknown),
                        },
                        model: disk.model.clone().map(|m| m.to_string()),
                        vendor: disk.vendor.clone().map(|v| v.to_string()),
                        serial: disk.serial.clone().map(|s| s.to_string()),
                    });
                }
                Some(disks)
            }
            None => None,
        },
        volumes: match machine.volumes {
            Some(v) => {
                let mut volumes = Vec::new();
                for volume in v.iter() {
                    volumes.push(geth_control_server::model::VolumeSummary {
                        name: volume.name.to_string(),
                        mount_point: volume.mount_point.to_string(),
                        total_space: volume.total_space as i64,
                        file_system: volume.file_system.clone().map(|f| f.to_string()),
                    });
                }
                Some(volumes)
            }
            None => None,
        },
        addresses: match machine.addresses {
            Some(a) => {
                let mut addresses = Vec::new();
                for address in a.iter() {
                    addresses.push(geth_control_server::model::AddressSummary {
                        address: address.address.to_string(),
                        netmask: address.netmask.clone().map(|n| n.to_string()),
                        broadcast: address.broadcast.clone().map(|b| b.to_string()),
                        version: match address.version {
                            AddressVersion::V4 => geth_control_server::model::AddressVersion::V4,
                            AddressVersion::V6 => geth_control_server::model::AddressVersion::V6,
                            AddressVersion::V6Local => {
                                geth_control_server::model::AddressVersion::V6Local
                            }
                        },
                    });
                }
                Some(addresses)
            }
            None => None,
        },
        containers: None,
    }
}
