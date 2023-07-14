use std::sync::Arc;

use chrono::{DateTime, Utc};
use geth_agent_client::types::{
    AddressVersion as AgentAddressVersion, DiskInterface as AgentDiskInterface,
    DiskSummary as AgentDiskSummary, DiskType as AgentDiskType, MemoryTypeSummary,
    NetworkInterfaceSummary as AgentNetworkInterfaceSummary, OverviewSummary,
    VolumeSummary as AgentVolumeSummary,
};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum MachineType {
    BareMetal,
    VirtualMachine,
    Hypervisor,
}

#[derive(Clone, Debug)]
pub enum MachineState {
    Running,
    Stopped,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct Tag {
    pub(crate) key: Arc<str>,
    pub(crate) value: Arc<str>,
}

#[derive(Clone, Debug)]
pub struct MachineStatusSummary {
    pub(crate) state: MachineState,
    pub(crate) last_checked: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct SystemSummary {
    pub(crate) machine_id: Arc<str>,
    pub(crate) family: Arc<str>,
    pub(crate) kernel_version: Arc<str>,
    pub(crate) os_version: Arc<str>,
    pub(crate) os: Arc<str>,
    pub(crate) os_pretty: Arc<str>,
    pub(crate) hostname: Arc<str>,
}

#[derive(Clone, Debug)]
pub struct MemorySummary {
    pub(crate) memory: u64,
    pub(crate) swap: u64,
}

#[derive(Clone, Debug)]
pub struct CpuSummary {
    pub(crate) cores: u64,
    pub(crate) architecture: Arc<str>,
    pub(crate) model: Option<Arc<str>>,
    pub(crate) vendor: Option<Arc<str>>,
}

#[derive(Clone, Debug)]
pub enum DiskType {
    HDD,
    SSD,
    NVME,
    Unknown,
}

#[derive(Clone, Debug)]
pub enum DiskInterface {
    SATA,
    SCSI,
    PCIE,
    UNKNOWN,
}

#[derive(Clone, Debug)]
pub struct DiskSummary {
    pub(crate) device: Arc<str>,
    pub(crate) r#type: DiskType,
    pub(crate) size_actual: u64,
    pub(crate) vendor: Option<Arc<str>>,
    pub(crate) model: Option<Arc<str>>,
    pub(crate) interface: DiskInterface,
    pub(crate) serial: Option<Arc<str>>,
    pub(crate) sector_size: Option<u64>,
    pub(crate) size_raw: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct VolumeSummary {
    pub(crate) name: Arc<str>,
    pub(crate) mount_point: Arc<str>,
    pub(crate) total_space: u64,
    pub(crate) file_system: Option<Arc<str>>,
}

#[derive(Clone, Debug)]
pub struct NetworkInterfaceSummary {
    pub(crate) name: Arc<str>,
    pub(crate) addresses: Arc<[Arc<str>]>,
    pub(crate) r#virtual: bool,
    pub(crate) mac: Option<Arc<str>>,
    pub(crate) speed: Option<u64>,
    pub(crate) mtu: Option<u64>,
    pub(crate) duplex: Option<Arc<str>>,
    pub(crate) vendor: Option<Arc<str>>,
}

#[derive(Clone, Debug)]
pub enum AddressVersion {
    V4,
    V6,
    V6Local,
}

#[derive(Clone, Debug)]
pub struct AddressSummary {
    pub(crate) version: AddressVersion,
    pub(crate) address: Arc<str>,
    pub(crate) netmask: Option<Arc<str>>,
    pub(crate) broadcast: Option<Arc<str>>,
}

#[derive(Clone, Debug)]
pub struct ContainerSummary {
    pub(crate) container_id: Arc<str>,
    pub(crate) name: Arc<str>,
    pub(crate) image: Arc<str>,
    pub(crate) created: DateTime<Utc>,
    pub(crate) state: Arc<str>,
}

#[derive(Clone, Debug)]
pub struct Machine {
    pub(crate) id: Arc<str>,
    pub(crate) group: Arc<str>,
    pub(crate) status: MachineStatusSummary,
    pub(crate) added: DateTime<Utc>,
    pub(crate) updated: Option<DateTime<Utc>>,
    pub(crate) machine_type: MachineType,
    pub(crate) tags: Arc<[Tag]>,
    pub(crate) system: Option<SystemSummary>,
    pub(crate) memory: Option<MemorySummary>,
    pub(crate) cpu: Option<CpuSummary>,
    pub(crate) disks: Option<Arc<[DiskSummary]>>,
    pub(crate) volumes: Option<Arc<[VolumeSummary]>>,
    pub(crate) network_interfaces: Option<Arc<[NetworkInterfaceSummary]>>,
    pub(crate) addresses: Option<Arc<[AddressSummary]>>,
    pub(crate) containers: Option<Arc<[ContainerSummary]>>,
}

impl Machine {
    fn make_id() -> Arc<str> {
        let uuid = Uuid::new_v4();
        let short_uuid = uuid.simple().to_string();

        let id = format!("m-{}", short_uuid);

        id.into()
    }

    fn get_memory_value(memory: Option<&MemoryTypeSummary>) -> u64 {
        if memory.is_none() {
            return 0;
        }

        let memory = memory.unwrap();

        memory.total().unwrap_or(0) as u64
    }

    fn get_disks_from_summary(disks: Option<&[AgentDiskSummary]>) -> Option<Arc<[DiskSummary]>> {
        disks?;

        let disks = disks.unwrap();

        if disks.is_empty() {
            return None;
        }

        let mut summaries = Vec::new();

        for disk in disks {
            summaries.push(DiskSummary {
                device: Arc::from(disk.device.to_owned().unwrap_or("".into())),
                r#type: match disk.r#type() {
                    Some(t) => match t {
                        AgentDiskType::Hdd => DiskType::HDD,
                        AgentDiskType::Ssd => DiskType::SSD,
                        AgentDiskType::Nvme => DiskType::NVME,
                        AgentDiskType::UnknownValue => DiskType::Unknown,
                        AgentDiskType::Unknown(_) => DiskType::Unknown,
                        _ => DiskType::Unknown,
                    },
                    None => DiskType::Unknown,
                },
                size_actual: disk.size_actual.unwrap_or(0) as u64,
                vendor: disk.vendor().map(Arc::from),
                model: disk.model().map(Arc::from),
                interface: match disk.interface() {
                    Some(i) => match i {
                        AgentDiskInterface::Sata => DiskInterface::SATA,
                        AgentDiskInterface::Scsi => DiskInterface::SCSI,
                        AgentDiskInterface::PciE => DiskInterface::PCIE,
                        AgentDiskInterface::UnknownValue => DiskInterface::UNKNOWN,
                        AgentDiskInterface::Unknown(_) => DiskInterface::UNKNOWN,
                        _ => DiskInterface::UNKNOWN,
                    },
                    None => DiskInterface::UNKNOWN,
                },
                serial: disk.serial().map(Arc::from),
                sector_size: disk.sector_size().map(|s| s as u64),
                size_raw: disk.size_raw().map(|s| s as u64),
            });
        }

        Some(summaries.into())
    }

    fn get_volumes_from_summary(
        volumes: Option<&[AgentVolumeSummary]>,
    ) -> Option<Arc<[VolumeSummary]>> {
        volumes?;

        let volumes = volumes.unwrap();

        if volumes.is_empty() {
            return None;
        }

        let mut summaries = Vec::new();

        for volume in volumes {
            summaries.push(VolumeSummary {
                name: Arc::from(volume.name.to_owned().unwrap_or("".into())),
                mount_point: Arc::from(volume.mount_point.to_owned().unwrap_or("".into())),
                total_space: volume.total_space.unwrap_or(0) as u64,
                file_system: volume.file_system().map(Arc::from),
            });
        }

        Some(summaries.into())
    }

    fn get_networks_from_summary(
        networks: Option<&[AgentNetworkInterfaceSummary]>,
    ) -> Option<Arc<[NetworkInterfaceSummary]>> {
        networks?;

        let networks = networks.unwrap();

        if networks.is_empty() {
            return None;
        }

        let mut summaries = Vec::new();

        for network in networks {
            let mut addresses = Vec::new();

            if network.addresses().is_some() {
                for addr in network.addresses().unwrap() {
                    addresses.push(Arc::from(addr.address().unwrap_or("")));
                }
            }

            summaries.push(NetworkInterfaceSummary {
                name: Arc::from(network.name.to_owned().unwrap_or("".into())),
                r#virtual: network.r#virtual().unwrap_or(false),
                addresses: Arc::from(addresses),
                mac: network.mac_address().map(Arc::from),
                speed: network.speed().map(|s| s as u64),
                mtu: network.mtu().map(|m| m as u64),
                duplex: network.duplex().map(Arc::from),
                vendor: network.vendor().map(Arc::from),
            });
        }

        Some(summaries.into())
    }

    fn get_addresses_from_summary(
        addresses: Option<&[AgentNetworkInterfaceSummary]>,
    ) -> Option<Arc<[AddressSummary]>> {
        addresses?;

        let addresses = addresses.unwrap();

        if addresses.is_empty() {
            return None;
        }

        let mut summaries = Vec::new();

        for address in addresses {
            if address.addresses().is_none() {
                continue;
            }

            for addr in address.addresses().unwrap() {
                summaries.push(AddressSummary {
                    address: Arc::from(addr.address().unwrap_or("")),
                    netmask: addr.netmask().map(Arc::from),
                    version: match addr.version() {
                        Some(v) => match v {
                            AgentAddressVersion::V4 => AddressVersion::V4,
                            AgentAddressVersion::V6 => AddressVersion::V6,
                            AgentAddressVersion::V6Local => AddressVersion::V6Local,
                            AgentAddressVersion::Unknown(_) => AddressVersion::V4,
                            _ => AddressVersion::V4,
                        },
                        None => AddressVersion::V4,
                    },
                    broadcast: addr.broadcast().map(Arc::from),
                });
            }
        }

        Some(summaries.into())
    }

    pub fn new_from_agent_overview(overview: &OverviewSummary, group: &str) -> Machine {
        let now = Utc::now();

        Machine {
            id: Machine::make_id(),
            group: Arc::from(group),
            status: MachineStatusSummary {
                state: MachineState::Running,
                last_checked: now,
            },
            added: now,
            updated: None,
            machine_type: MachineType::BareMetal,
            tags: Arc::from([]),
            system: overview.system.as_ref().map(|system| SystemSummary {
                machine_id: Arc::from(system.machine_id.to_owned().unwrap_or("".into())),
                family: Arc::from(system.family.to_owned().unwrap_or("".into())),
                kernel_version: Arc::from(system.kernel_version.to_owned().unwrap_or("".into())),
                os_version: Arc::from(system.os_version.to_owned().unwrap_or("".into())),
                os: Arc::from(system.os.to_owned().unwrap_or("".into())),
                os_pretty: Arc::from(system.os_pretty.to_owned().unwrap_or("".into())),
                hostname: Arc::from(system.hostname.to_owned().unwrap_or("".into())),
            }),
            memory: overview.memory().map(|memory| MemorySummary {
                memory: Machine::get_memory_value(memory.memory()),
                swap: Machine::get_memory_value(memory.swap()),
            }),
            cpu: overview.cpu().map(|cpu| CpuSummary {
                cores: cpu.cores().unwrap_or(0) as u64,
                architecture: Arc::from(cpu.architecture().unwrap_or("")),
                model: cpu.model().map(Arc::from),
                vendor: cpu.vendor().map(Arc::from),
            }),
            disks: Machine::get_disks_from_summary(overview.disks()),
            volumes: Machine::get_volumes_from_summary(overview.volumes()),
            network_interfaces: Machine::get_networks_from_summary(overview.network()),
            addresses: Machine::get_addresses_from_summary(overview.network()),
            containers: None,
        }
    }
}
