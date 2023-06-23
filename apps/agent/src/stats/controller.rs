use std::collections::HashMap;

use sysinfo::SystemExt;
use hw_info::{Disk, load_disks};
use containers::{Containers, Container};
use sysinfo::System as Sys;

use super::cpu::Cpu;
use super::disk::Storage;
use super::memory::{Memory, Swap};
use super::network::Network;
use super::system::System;

pub struct SystemController {
    system_controller: Sys,
    container_controller: Option<Containers>,
    system: System,
    memory: Memory,
    swap: Swap,
    cpu: Cpu,
    network: Network,
    storage: Storage,
    disks: HashMap<String, Disk>,
    containers: HashMap<String, Container>
}

impl SystemController {
    pub fn new() -> SystemController {
        let mut sys = Sys::new_all();
        sys.refresh_all();
        let container_controller = Containers::new();
        
        let system = System::new(&sys);
        let memory = Memory::new(&sys);
        let swap = Swap::new(&sys);
        let cpu = Cpu::new(&sys);
        let network = Network::new(&sys);
        let storage = Storage::new(&sys);
        let mut disks = HashMap::<String, Disk>::new();
        for disk in load_disks() {
            disks.insert(disk.get_device().to_string(), disk);
        }
        let mut containers = HashMap::<String, Container>::new();

        SystemController {
            system_controller: sys,
            container_controller: container_controller,
            system,
            memory,
            swap,
            cpu,
            network,
            storage,
            disks,
            containers,
        }
    }

    pub fn system(&self) -> &System {
        &self.system
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn swap(&self) -> &Swap {
        &self.swap
    }

    pub fn network(&self) -> &Network {
        &self.network
    }

    pub fn storage(&self) -> &Storage {
        &self.storage
    }

    pub fn disks(&self) -> &HashMap<String, Disk> {
        &self.disks
    }

    pub fn containers(&self) -> &HashMap<String, Container> {
        &self.containers
    }

    pub async fn refresh(&mut self) {
        self.system_controller.refresh_all();
        self.refresh_system();
        self.refresh_memory();
        self.refresh_cpu();
        self.refresh_network();
        self.refresh_storage();
        self.refresh_containers().await;
    }

    fn refresh_system(&mut self) {
        self.system.update_up_time(&self.system_controller)
    }

    fn refresh_memory(&mut self) {
        self.memory.update(&self.system_controller);
        self.swap.update(&self.system_controller);
    }

    fn refresh_cpu(&mut self) {
        self.cpu.update(&self.system_controller);
    }

    fn refresh_network(&mut self) {
        self.network.update(&self.system_controller);
    }

    fn refresh_storage(&mut self) {
        self.storage.update(&self.system_controller);
    }

    async fn refresh_containers(&mut self) {
        if let Some(ref mut container_controller) = self.container_controller {
            let containers = container_controller.list_containers().await;
            for container in containers {
                self.containers.insert(container.id().to_string(), container);
            }
        }
    }

}