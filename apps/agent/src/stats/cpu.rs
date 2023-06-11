use sysinfo::{System, SystemExt, CpuExt};
use std::{env::consts::ARCH, collections::HashMap};

use super::util;

pub struct Core {
    name: String,
    frequency: u64,
    usage: f32,
}

impl Core {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn frequency(&self) -> &u64 {
        &self.frequency
    }

    pub fn usage(&self) -> &f32 {
        &self.usage
    }
}

pub struct CPU {
    cores: HashMap<String, Core>,
    core_count: usize,
    architecture: String,
    vendor: String,
    brand: String,
}

impl CPU {
    pub fn new(system: &System) -> CPU {
        let arch = String::from(ARCH);
        let vendor;
        let brand;

        let cpu = &system.cpus()[0];

        vendor = cpu.vendor_id().to_string();
        brand = cpu.brand().to_string();

        let mut cores = HashMap::new();

        for cpu in system.cpus() {
            let core = Core {
                name: cpu.name().to_string(),
                frequency: cpu.frequency(),
                usage: cpu.cpu_usage(),
            };
            cores.insert(cpu.name().to_string(), core);
        }

        let core_count = util::handle_optional_usize(system.physical_core_count());

        CPU {
            core_count: core_count,
            cores,
            architecture: arch,
            vendor: vendor,
            brand: brand,
        }
    }

    pub fn print_debug(&self) {
        println!("CPU Debug:");
        println!("  Architecture: {}", self.architecture);
        println!("  Vendor: {}", self.vendor);
        println!("  Brand: {}", self.brand);
        println!("  Core Count: {}", self.core_count);
        println!("  Cores:");
        for (_, core) in &self.cores {
            println!("    Core: {}", core.name);
            println!("      Frequency: {}", core.frequency);
            println!("      Usage: {}", core.usage);
        }
    }

    pub fn get_core(&self, name: &str) -> Option<&Core> {
        self.cores.get(name)
    }

    pub fn update(&mut self, system: &System) {
        for cpu in system.cpus() {
            let core = Core {
                name: cpu.name().to_string(),
                frequency: cpu.frequency(),
                usage: cpu.cpu_usage(),
            };
            self.cores.insert(cpu.name().to_string(), core);
        }
    }

    pub fn cores(&self) -> Vec<&Core> {
        let mut cores = Vec::new();

        for (_, core) in &self.cores {
            cores.push(core);
        }

        cores
    }

    pub fn core_count(&self) -> usize {
        self.core_count
    }

    pub fn architecture(&self) -> &String {
        &self.architecture
    }

    pub fn vendor(&self) -> &String {
        &self.vendor
    }

    pub fn brand(&self) -> &String {
        &self.brand
    }
}



