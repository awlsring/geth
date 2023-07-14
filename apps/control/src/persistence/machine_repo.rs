use std::sync::Arc;

use super::repository::Repository;
use async_trait::async_trait;
use prisma::{
    machine_summary,
    types::{machine_full_summary, MachineSummaryFull},
    PrismaClient,
};

use chrono::{DateTime, FixedOffset, Local, Utc};

use crate::model::machine::{
    AddressSummary, AddressVersion, ContainerSummary, CpuSummary, DiskInterface, DiskSummary,
    DiskType, Machine, MachineState, MachineStatusSummary, MachineType, MemorySummary,
    NetworkInterfaceSummary, SystemSummary, Tag, VolumeSummary,
};

pub struct MachinePrismaRepository {
    conn: PrismaClient,
}

impl MachinePrismaRepository {
    pub fn new(conn: PrismaClient) -> Self {
        MachinePrismaRepository { conn }
    }

    fn db_to_model(machine: MachineSummaryFull) -> Machine {
        Machine {
            id: machine.id.clone().into(),
            group: machine.group.clone().into(),
            status: convert_status_summary(machine.status),
            added: machine.added.into(),
            updated: handle_optional_datetime(machine.updated),
            machine_type: convert_machine_type(machine.r#type),
            tags: convert_tags(machine.tags),
            system: convert_system_summary(machine.system),
            memory: convert_memory_summary(machine.memory),
            cpu: convert_cpu_summary(machine.cpu),
            disks: convert_disks_summaries(machine.disks),
            volumes: convert_volume_summaries(machine.volumes),
            network_interfaces: convert_network_interface_summaries(machine.network_interfaces),
            addresses: convert_address_summaries(machine.addresses),
            containers: convert_container_summaries(machine.containers),
        }
    }
}

#[async_trait]
impl Repository<Machine, String> for MachinePrismaRepository {
    async fn find_by_id(&self, id: String) -> Option<Machine> {
        let machine = self
            .conn
            .machine_summary()
            .find_unique(machine_summary::id::equals(id))
            .include(machine_full_summary::include())
            .exec()
            .await
            .unwrap();

        match machine {
            None => return None,
            Some(machine) => return Some(MachinePrismaRepository::db_to_model(machine)),
        }
    }

    async fn find_all(&self) -> Arc<[Machine]> {
        let results = self
            .conn
            .machine_summary()
            .find_many(vec![])
            .include(machine_full_summary::include())
            .exec()
            .await
            .unwrap();

        let mut machines: Vec<Machine> = Vec::new();
        for machine in results {
            machines.push(MachinePrismaRepository::db_to_model(machine));
        }
        machines.into()
    }

    async fn modify(&mut self, item: Machine) -> Result<(), String> {
        todo!()
    }

    async fn insert(&mut self, item: Machine) -> Result<(), String> {
        let t = match item.machine_type {
            MachineType::BareMetal => prisma::MachineType::BareMetal,
            MachineType::Hypervisor => prisma::MachineType::Hypervisor,
            MachineType::VirtualMachine => prisma::MachineType::VirtualMachine,
        };

        let result = self
            .conn
            .machine_summary()
            .create(
                item.id.clone().to_string(),
                item.group.clone().to_string(),
                item.added.into(),
                t,
                vec![],
            )
            .exec()
            .await;

        // figure out how to make these with one write
        let machine_status = match item.status.state {
            MachineState::Running => prisma::MachineStatus::Running,
            MachineState::Stopped => prisma::MachineStatus::Stopped,
            MachineState::Unknown => prisma::MachineStatus::Unknown,
        };

        let status_result = self
            .conn
            .machine_status_summary()
            .create(
                machine_status,
                machine_summary::id::equals(item.id.clone().to_string()),
                vec![prisma::machine_status_summary::last_checked::set(
                    item.status.last_checked.into(),
                )],
            )
            .exec()
            .await;

        let tags_result = self
            .conn
            .tag()
            .create_many(
                item.tags
                    .iter()
                    .map(|t| {
                        prisma::tag::create_unchecked(
                            item.id.clone().to_string(),
                            t.key.to_string(),
                            t.value.to_string(),
                            vec![],
                        )
                    })
                    .collect(),
            )
            .exec()
            .await;

        if item.disks.is_some() {
            let disks_result = self
                .conn
                .disk_summary()
                .create_many(
                    item.disks
                        .unwrap()
                        .iter()
                        .map(|d| {
                            let mut optionals: Vec<prisma::disk_summary::SetParam> = Vec::new();

                            optionals.push(prisma::disk_summary::interface::set(Some(
                                convert_disk_interface(&d.interface),
                            )));

                            if d.model.is_some() {
                                optionals.push(prisma::disk_summary::model::set(Some(
                                    d.model.clone().unwrap().to_string(),
                                )));
                            }
                            if d.vendor.is_some() {
                                optionals.push(prisma::disk_summary::vendor::set(Some(
                                    d.vendor.clone().unwrap().to_string(),
                                )));
                            }
                            if d.serial.is_some() {
                                optionals.push(prisma::disk_summary::serial::set(Some(
                                    d.serial.clone().unwrap().to_string(),
                                )));
                            }
                            if d.sector_size.is_some() {
                                optionals.push(prisma::disk_summary::sector_size::set(Some(
                                    d.sector_size.unwrap() as i32,
                                )));
                            }
                            if d.size_raw.is_some() {
                                optionals.push(prisma::disk_summary::size_raw::set(Some(
                                    d.size_raw.unwrap() as i64,
                                )));
                            }

                            prisma::disk_summary::create_unchecked(
                                item.id.clone().to_string(),
                                d.device.to_string(),
                                convert_disk_type(&d.r#type),
                                d.size_actual as i64,
                                optionals,
                            )
                        })
                        .collect(),
                )
                .exec()
                .await;
        }

        if item.volumes.is_some() {
            let volumes_result = self
                .conn
                .volume_summary()
                .create_many(
                    item.volumes
                        .unwrap()
                        .iter()
                        .map(|v| {
                            let mut optionals: Vec<prisma::volume_summary::SetParam> = Vec::new();

                            if v.file_system.is_some() {
                                optionals.push(prisma::volume_summary::file_system::set(Some(
                                    v.file_system.clone().unwrap().to_string(),
                                )));
                            }

                            prisma::volume_summary::create_unchecked(
                                item.id.clone().to_string(),
                                v.name.to_string(),
                                v.mount_point.to_string(),
                                v.total_space as i64,
                                optionals,
                            )
                        })
                        .collect(),
                )
                .exec()
                .await;
        }

        if item.network_interfaces.is_some() {
            let nic_result = self
                .conn
                .network_interface_summary()
                .create_many(
                    item.network_interfaces
                        .unwrap()
                        .iter()
                        .map(|n| {
                            let mut optionals: Vec<prisma::network_interface_summary::SetParam> =
                                Vec::new();

                            if n.mac.is_some() {
                                optionals.push(
                                    prisma::network_interface_summary::mac_address::set(Some(
                                        n.mac.clone().unwrap().to_string(),
                                    )),
                                );
                            }

                            if n.r#virtual {
                                optionals
                                    .push(prisma::network_interface_summary::r#virtual::set(true));
                            }

                            if n.vendor.is_some() {
                                optionals.push(prisma::network_interface_summary::vendor::set(
                                    Some(n.vendor.clone().unwrap().to_string()),
                                ));
                            }

                            if n.mtu.is_some() {
                                optionals.push(prisma::network_interface_summary::mtu::set(Some(
                                    n.mtu.clone().unwrap() as i32,
                                )));
                            }

                            if n.speed.is_some() {
                                optionals.push(prisma::network_interface_summary::speed::set(
                                    Some(n.speed.clone().unwrap() as i32),
                                ));
                            }

                            if n.duplex.is_some() {
                                optionals.push(prisma::network_interface_summary::duplex::set(
                                    Some(n.duplex.clone().unwrap().to_string()),
                                ));
                            }

                            if n.addresses.len() > 0 {
                                let mut adrs = Vec::new();
                                for a in n.addresses.iter() {
                                    adrs.push(a.to_string());
                                }

                                optionals
                                    .push(prisma::network_interface_summary::addresses::set(adrs));
                            }

                            prisma::network_interface_summary::create_unchecked(
                                item.id.clone().to_string(),
                                n.name.to_string(),
                                optionals,
                            )
                        })
                        .collect(),
                )
                .exec()
                .await;
        }

        if item.addresses.is_some() {
            let addresses_result = self
                .conn
                .address_summary()
                .create_many(
                    item.addresses
                        .unwrap()
                        .iter()
                        .map(|a| {
                            let mut optionals: Vec<prisma::address_summary::SetParam> = Vec::new();

                            let version = match a.version {
                                AddressVersion::V4 => prisma::AddressVersion::V4,
                                AddressVersion::V6 => prisma::AddressVersion::V6,
                                AddressVersion::V6Local => prisma::AddressVersion::V6Local,
                            };

                            if a.netmask.is_some() {
                                optionals.push(prisma::address_summary::netmask::set(Some(
                                    a.netmask.clone().unwrap().to_string(),
                                )));
                            }

                            if a.broadcast.is_some() {
                                optionals.push(prisma::address_summary::broadcast::set(Some(
                                    a.broadcast.clone().unwrap().to_string(),
                                )));
                            }

                            prisma::address_summary::create_unchecked(
                                item.id.clone().to_string(),
                                version,
                                a.address.clone().to_string(),
                                optionals,
                            )
                        })
                        .collect(),
                )
                .exec()
                .await;
        }

        if item.containers.is_some() {
            let containers_result = self
                .conn
                .container_summary()
                .create_many(
                    item.containers
                        .unwrap()
                        .iter()
                        .map(|c| {
                            prisma::container_summary::create_unchecked(
                                item.id.clone().to_string(),
                                c.container_id.clone().to_string(),
                                c.name.clone().to_string(),
                                c.image.clone().to_string(),
                                c.created.into(),
                                c.state.clone().to_string(),
                                vec![],
                            )
                        })
                        .collect(),
                )
                .exec()
                .await;
        }

        if item.system.is_some() {
            let system = item.system.unwrap();
            let system_result = self
                .conn
                .system_summary()
                .create(
                    machine_summary::id::equals(item.id.clone().to_string()),
                    system.machine_id.to_string(),
                    system.family.to_string(),
                    system.kernel_version.to_string(),
                    system.os.to_string(),
                    system.os_version.to_string(),
                    system.os_pretty.to_string(),
                    system.hostname.to_string(),
                    vec![],
                )
                .exec()
                .await;
        }

        if item.memory.is_some() {
            let memory = item.memory.unwrap();
            let memory_result = self
                .conn
                .memory_summary()
                .create(
                    machine_summary::id::equals(item.id.clone().to_string()),
                    memory.memory as i64,
                    memory.swap as i64,
                    vec![],
                )
                .exec()
                .await;
        }

        if item.cpu.is_some() {
            let cpu = item.cpu.unwrap();

            let mut optionals: Vec<prisma::cpu_summary::SetParam> = Vec::new();

            if cpu.vendor.is_some() {
                optionals.push(prisma::cpu_summary::vendor::set(Some(
                    cpu.vendor.unwrap().to_string(),
                )));
            }

            if cpu.model.is_some() {
                optionals.push(prisma::cpu_summary::model::set(Some(
                    cpu.model.unwrap().to_string(),
                )));
            }

            let cpu_result = self
                .conn
                .cpu_summary()
                .create(
                    machine_summary::id::equals(item.id.clone().to_string()),
                    cpu.cores as i32,
                    cpu.architecture.to_string(),
                    optionals,
                )
                .exec()
                .await;
        }

        match result {
            Ok(_) => return Ok(()),
            Err(_) => return Err("item".to_string()),
        }
    }

    async fn delete(&mut self, id: String) -> Result<(), String> {
        let result = self
            .conn
            .machine_summary()
            .delete(machine_summary::id::equals(id))
            .exec()
            .await;

        match result {
            Ok(_) => return Ok(()),
            Err(_) => return Err("item".to_string()),
        }
    }
}

fn convert_disk_type(t: &DiskType) -> prisma::DiskType {
    match t {
        DiskType::HDD => prisma::DiskType::Hdd,
        DiskType::SSD => prisma::DiskType::Ssd,
        DiskType::NVME => prisma::DiskType::Nvme,
        DiskType::Unknown => prisma::DiskType::Unknown,
    }
}

fn convert_disk_interface(i: &DiskInterface) -> prisma::DiskInterface {
    match i {
        DiskInterface::SATA => prisma::DiskInterface::Sata,
        DiskInterface::SCSI => prisma::DiskInterface::Scsi,
        DiskInterface::PCIE => prisma::DiskInterface::PciE,
        DiskInterface::UNKNOWN => prisma::DiskInterface::Unknown,
    }
}

fn handle_optional_datetime(t: Option<DateTime<FixedOffset>>) -> Option<DateTime<Utc>> {
    match t {
        None => None,
        Some(t) => Some(t.into()),
    }
}

fn handle_optional_string(s: Option<String>) -> Option<Arc<str>> {
    match s {
        None => None,
        Some(s) => Some(s.into()),
    }
}

fn handle_optional_int(i: Option<i32>) -> Option<u64> {
    match i {
        None => None,
        Some(i) => Some(i as u64),
    }
}

fn handle_optional_big_int(i: Option<i64>) -> Option<u64> {
    match i {
        None => None,
        Some(i) => Some(i as u64),
    }
}

fn convert_memory_summary(m: Option<prisma::memory_summary::Data>) -> Option<MemorySummary> {
    match m {
        None => None,
        Some(m) => Some(MemorySummary {
            memory: m.memory_total as u64,
            swap: m.swap_total as u64,
        }),
    }
}

fn convert_cpu_summary(s: Option<prisma::cpu_summary::Data>) -> Option<CpuSummary> {
    match s {
        None => None,
        Some(s) => Some(CpuSummary {
            cores: s.cores as u64,
            architecture: s.architecture.clone().into(),
            model: handle_optional_string(s.model.clone()),
            vendor: handle_optional_string(s.vendor.clone()),
        }),
    }
}

fn convert_disks_summaries(d: Vec<prisma::disk_summary::Data>) -> Option<Arc<[DiskSummary]>> {
    let mut result = Vec::new();
    for disk in d {
        let t = match disk.r#type {
            prisma::DiskType::Hdd => DiskType::HDD,
            prisma::DiskType::Ssd => DiskType::SSD,
            prisma::DiskType::Nvme => DiskType::NVME,
            prisma::DiskType::Unknown => DiskType::Unknown,
        };

        let i = match disk.interface {
            Some(prisma::DiskInterface::Sata) => DiskInterface::SATA,
            Some(prisma::DiskInterface::Scsi) => DiskInterface::SCSI,
            Some(prisma::DiskInterface::PciE) => DiskInterface::PCIE,
            Some(prisma::DiskInterface::Unknown) => DiskInterface::UNKNOWN,
            None => DiskInterface::UNKNOWN,
        };

        result.push(DiskSummary {
            device: disk.device.clone().into(),
            r#type: t,
            size_actual: disk.size_actual as u64,
            vendor: handle_optional_string(disk.vendor.clone()),
            model: handle_optional_string(disk.model.clone()),
            interface: i,
            serial: handle_optional_string(disk.serial.clone()),
            sector_size: handle_optional_int(disk.sector_size),
            size_raw: handle_optional_big_int(disk.size_raw),
        });
    }
    Some(result.into())
}

fn convert_volume_summaries(v: Vec<prisma::volume_summary::Data>) -> Option<Arc<[VolumeSummary]>> {
    let mut result = Vec::new();
    for volume in v {
        result.push(VolumeSummary {
            name: volume.name.clone().into(),
            mount_point: volume.mount_point.clone().into(),
            total_space: volume.total_space as u64,
            file_system: handle_optional_string(volume.file_system.clone()),
        });
    }
    let it = result.into();
    Some(it)
}

fn convert_network_interface_summaries(
    n: Vec<prisma::network_interface_summary::Data>,
) -> Option<Arc<[NetworkInterfaceSummary]>> {
    let mut result = Vec::new();
    for network_interface in n {
        let mut ads: Vec<Arc<str>> = Vec::new();
        for a in network_interface.addresses.iter() {
            ads.push(a.to_owned().into());
        }

        result.push(NetworkInterfaceSummary {
            name: network_interface.name.clone().into(),
            addresses: ads.into(),
            r#virtual: network_interface.r#virtual,
            mac: handle_optional_string(network_interface.mac_address.clone()),
            speed: handle_optional_int(network_interface.speed),
            mtu: handle_optional_int(network_interface.mtu),
            duplex: handle_optional_string(network_interface.duplex.clone()),
            vendor: handle_optional_string(network_interface.vendor.clone()),
        });
    }
    let it = result.into();
    Some(it)
}

fn convert_address_summaries(
    a: Vec<prisma::address_summary::Data>,
) -> Option<Arc<[AddressSummary]>> {
    let mut result = Vec::new();
    for address in a {
        let version = match address.version {
            prisma::AddressVersion::V4 => AddressVersion::V4,
            prisma::AddressVersion::V6 => AddressVersion::V6,
            prisma::AddressVersion::V6Local => AddressVersion::V6Local,
            _ => AddressVersion::V4,
        };
        result.push(AddressSummary {
            address: address.address.clone().into(),
            netmask: handle_optional_string(address.netmask.clone()),
            broadcast: handle_optional_string(address.broadcast.clone()),
            version: version,
        });
    }
    let it = result.into();
    Some(it)
}

fn convert_container_summaries(
    c: Vec<prisma::container_summary::Data>,
) -> Option<Arc<[ContainerSummary]>> {
    let mut result = Vec::new();
    for container in c {
        result.push(ContainerSummary {
            container_id: container.container_id.clone().into(),
            name: container.name.clone().into(),
            image: container.image.clone().into(),
            created: container.created.into(),
            state: container.state.clone().into(),
        });
    }
    let it = result.into();
    Some(it)
}

fn convert_system_summary(s: Option<prisma::system_summary::Data>) -> Option<SystemSummary> {
    match s {
        None => None,
        Some(s) => Some(SystemSummary {
            hostname: s.hostname.clone().into(),
            os: s.os.clone().into(),
            machine_id: s.machine_id.clone().into(),
            family: s.family.clone().into(),
            kernel_version: s.kernel_version.clone().into(),
            os_version: s.os_version.clone().into(),
            os_pretty: s.os_pretty.clone().into(),
        }),
    }
}

fn convert_tags(tags: std::vec::Vec<prisma::tag::Data>) -> Arc<[Tag]> {
    let mut result = Vec::new();
    for tag in tags {
        result.push(Tag {
            key: tag.key.clone().into(),
            value: tag.value.clone().into(),
        });
    }
    result.into()
}

fn convert_status_summary(s: Option<prisma::machine_status_summary::Data>) -> MachineStatusSummary {
    match s {
        None => MachineStatusSummary {
            state: MachineState::Unknown,
            last_checked: Local::now().into(),
        },
        Some(s) => {
            let state = match s.status {
                prisma::MachineStatus::Running => MachineState::Running,
                prisma::MachineStatus::Stopped => MachineState::Stopped,
                prisma::MachineStatus::Unknown => MachineState::Unknown,
            };
            MachineStatusSummary {
                state,
                last_checked: s.last_checked.into(),
            }
        }
    }
}

fn convert_machine_type(t: prisma::MachineType) -> MachineType {
    match t {
        prisma::MachineType::BareMetal => MachineType::BareMetal,
        prisma::MachineType::VirtualMachine => MachineType::VirtualMachine,
        prisma::MachineType::Hypervisor => MachineType::Hypervisor,
    }
}

fn convert_machine_type_to_prisma(t: MachineType) -> prisma::MachineType {
    match t {
        MachineType::BareMetal => prisma::MachineType::BareMetal,
        MachineType::VirtualMachine => prisma::MachineType::VirtualMachine,
        MachineType::Hypervisor => prisma::MachineType::Hypervisor,
    }
}
