use std::collections::HashMap;

use sysinfo::{DiskExt, DiskKind, System, SystemExt};
use sysinfo::Disk as DiskSys;

use super::util::{u8_as_string, handle_optional_str};

pub struct Disk {
    name: String,
    mount_point: String,
    available_space: u64,
    total_space: u64,
    file_system: String,
    is_removable: bool,
    disk_type: DiskKind,
}

impl Disk {
    pub fn new(disk: &DiskSys) -> Disk {
        let name = handle_optional_str(disk.name().to_str());
        let mount_point = handle_optional_str(disk.mount_point().to_str());
        let available_space = disk.available_space();
        let total_space = disk.total_space();
        let file_system = u8_as_string(disk.file_system());
        let is_removable = disk.is_removable();
        let disk_type = disk.kind();

        Disk {
            name,
            mount_point,
            available_space,
            total_space,
            file_system,
            is_removable,
            disk_type,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn mount_point(&self) -> &String {
        &self.mount_point
    }

    pub fn available_space(&self) -> &u64 {
        &self.available_space
    }

    pub fn total_space(&self) -> &u64 {
        &self.total_space
    }

    pub fn file_system(&self) -> &String {
        &self.file_system
    }

    pub fn is_removable(&self) -> &bool {
        &self.is_removable
    }

    pub fn disk_type(&self) -> &DiskKind {
        &self.disk_type
    }

    pub fn update(&mut self, disk: &DiskSys) {
        self.available_space = disk.available_space();
        self.total_space = disk.total_space();
    }

}


pub struct Storage {
    disks: HashMap::<String, Disk>,
}

impl Storage {
    pub fn new(system: &System) -> Storage {
        let mut disks = HashMap::<String, Disk>::new();

        for d in system.disks() {
            let disk = Disk::new(d);
            disks.insert(disk.name().clone(), disk);
        }

        Storage {
            disks,
        }
    }

    pub fn get_disk(&self, name: &str) -> Option<&Disk> {
        self.disks.get(name)
    }
        

    pub fn disks(&self) -> Vec<&Disk> {
        let mut disks = Vec::new();

        for d in self.disks.values() {
            disks.push(d);
        }

        disks
    }

    pub fn update(&mut self, system: &System) {
        for d in system.disks() {
            let disk = self.disks.get_mut(d.name().to_str().unwrap()).unwrap();
            disk.update(d);
        }
    }
}