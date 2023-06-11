use sysinfo::SystemExt;
use sysinfo::System as Sys;

use super::cpu::CPU;
use super::disk::Storage;
use super::memory::{Memory, Swap};
use super::network::Network;
use super::system::System;

pub struct SystemController {
    _sys: Sys,
    system: System,
    memory: Memory,
    swap: Swap,
    cpu: CPU,
    network: Network,
    storage: Storage,
}

impl SystemController {
    pub fn new() -> SystemController {
        let mut sys = Sys::new_all();
        sys.refresh_all();

        let system = System::new(&sys);
        let memory = Memory::new(&sys);
        let swap = Swap::new(&sys);
        let cpu = CPU::new(&sys);
        let network = Network::new(&sys);
        let storage = Storage::new(&sys);

        

        SystemController {
            _sys: sys,
            system,
            memory,
            swap,
            cpu,
            network,
            storage,
        }
    }

    pub fn system(&self) -> &System {
        &self.system
    }

    pub fn cpu(&self) -> &CPU {
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

    pub async fn refresh(&mut self) {
        self._sys.refresh_all();
        self.refresh_system();
        self.refresh_memory();
        self.refresh_cpu();
        self.refresh_network();
        self.refresh_storage();
    }

    fn refresh_system(&mut self) {
        self.system.update_up_time(&self._sys)
    }

    fn refresh_memory(&mut self) {
        self.memory.update(&self._sys);
        self.swap.update(&self._sys);
    }

    fn refresh_cpu(&mut self) {
        self.cpu.update(&self._sys);
    }

    fn refresh_network(&mut self) {
        self.network.update(&self._sys);
    }

    fn refresh_storage(&mut self) {
        self.storage.update(&self._sys);
    }

}