use geth_control_client::types::{
    AddressSummary, AddressVersion, CpuSummary, MachineArchitecture, MachineClass, MachineSummary,
    MemorySummary, OperatingSystemSummary, StorageSummary,
};

pub fn format_name(name: Option<&str>, id: Option<&str>) -> String {
    match (name, id) {
        (Some(name), Some(_)) => name.to_string(),
        (Some(name), None) => name.to_string(),
        (None, Some(id)) => id.to_string(),
        _ => "".to_string(),
    }
}

pub fn format_os(os: Option<&OperatingSystemSummary>) -> String {
    match os {
        Some(os) => match (os.name(), os.version()) {
            (Some(os), Some(version)) => format!("{} {}", os, version),
            (Some(os), None) => os.to_string(),
            _ => "Unknown".to_string(),
        },
        _ => "Unknown".to_string(),
    }
}

pub fn format_cpu(cpu: Option<&CpuSummary>) -> String {
    match cpu {
        Some(cpu) => match cpu.cores() {
            Some(cores) => format!("CPU: {}", cores),
            _ => "CPU: ?".to_string(),
        },
        _ => "CPU: ?".to_string(),
    }
}

pub fn format_bytes(bytes: i64) -> String {
    let units: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < units.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    let rounded_size = size.round() as i64;

    format!("{}{}", rounded_size, units[unit_idx])
}

pub fn format_mem(mem: Option<&MemorySummary>) -> String {
    match mem {
        Some(mem) => match mem.total() {
            Some(t) => {
                let b = format_bytes(t);
                format!("Mem: {}", b)
            }
            _ => "".to_string(),
        },
        _ => "".to_string(),
    }
}

pub fn format_disk(storage: Option<&StorageSummary>) -> String {
    match storage {
        Some(storage) => match storage.total() {
            Some(t) => {
                let b = format_bytes(t);
                format!("Disk: {}", b)
            }
            _ => "".to_string(),
        },
        _ => "".to_string(),
    }
}

pub fn format_arch(cpu: Option<&CpuSummary>) -> String {
    match cpu {
        Some(cpu) => match cpu.architecture() {
            Some(arch) => match arch {
                MachineArchitecture::X86 => "x86".to_string(),
                MachineArchitecture::Arm => "arm".to_string(),
                _ => "".to_string(),
            },
            _ => "".to_string(),
        },
        _ => "".to_string(),
    }
}

pub fn format_class(class: Option<&MachineClass>) -> String {
    match class {
        Some(class) => match class {
            MachineClass::VirtualMachine => "vm".to_string(),
            MachineClass::Hypervisor => "hv".to_string(),
            MachineClass::BareMetal => "metal".to_string(),
            _ => "".to_string(),
        },
        _ => "".to_string(),
    }
}

pub fn format_group(group: Option<&str>) -> String {
    match group {
        Some(group) => group.to_string(),
        _ => "".to_string(),
    }
}
