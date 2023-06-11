use sysinfo::{System, SystemExt};

struct MemoryObject {
    total: u64,
    used: u64,
    available: u64,
}

impl MemoryObject {
    pub fn total(&self) -> &u64 {
        &self.total
    }

    pub fn used(&self) -> &u64 {
        &self.used
    }

    pub fn available(&self) -> &u64 {
        &self.available
    }
}

pub struct Memory {
    parent: MemoryObject,
}

impl Memory {
    pub fn new(system: &System) -> Memory {
        Memory {
            parent: MemoryObject {
                total: system.total_memory(),
                used: system.used_memory(),
                available: system.free_memory(),
            },
        }
    }

    pub fn total(&self) -> &u64 {
        self.parent.total()
    }

    pub fn used(&self) -> &u64 {
        self.parent.used()
    }

    pub fn available(&self) -> &u64 {
        self.parent.available()
    }

    pub fn update(&mut self, system: &System) {
        self.parent.used = system.used_memory();
        self.parent.available = system.available_memory();
    }
}

pub struct Swap {
    parent: MemoryObject,
}


impl Swap {
    pub fn new(system: &System) -> Swap {
        Swap {
            parent: MemoryObject {
                total: system.total_swap(),
                used: system.used_swap(),
                available: system.free_swap(),
            },
        }
    }

    pub fn total(&self) -> &u64 {
        self.parent.total()
    }

    pub fn used(&self) -> &u64 {
        self.parent.used()
    }

    pub fn available(&self) -> &u64 {
        self.parent.available()
    }

    pub fn update(&mut self, system: &System) {
        self.parent.used = system.used_swap();
        self.parent.available = system.free_swap();
    }
}