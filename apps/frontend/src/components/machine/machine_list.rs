use geth_control_client::types::{
    AddressSummary, AddressVersion, CpuSummary, MachineArchitecture, MachineClass, MachineStatus,
    MachineStatusSummary, MachineSummary, MemorySummary, OperatingSystemSummary, StorageSummary,
};
use leptos::*;
use leptos_icons::*;

#[component]
pub fn MachineList(cx: Scope, machines: ReadSignal<Vec<MachineSummary>>) -> impl IntoView {
    view! { cx,
        <div class="py-4 text-gray-700">
            {
                machines.get().into_iter()
                    .map(|n| view! { cx, <MachineListRows machine=n.clone() />})
                    .collect::<Vec<_>>()
            }
        </div>
    }
}

fn status_icon(cx: Scope, status: Option<&MachineStatusSummary>) -> impl IntoView {
    match status {
        Some(status) => match status.status() {
            Some(MachineStatus::Running) => view! { cx,
                <Icon icon=icon!(BsCircleFill) width="1.2em" height="1.2em" class="text-green-500 mt-3" />
            },
            Some(MachineStatus::Stopped) => view! { cx,
                <Icon icon=icon!(BsCircleFill) width="1.2em" height="1.2em" class="text-red-500 mt-3" />
            },
            _ => view! { cx,
                <Icon icon=icon!(BsCircleFill) width="1.2em" height="1.2em" class="text-gray-500 mt-3" />
            },
        },
        _ => view! { cx,
            <Icon icon=icon!(BsCircleFill) width="1.2em" height="1.2em" class="text-gray-500 mt-3" />
        },
    }
}

fn format_name(name: Option<&str>, id: Option<&str>) -> String {
    match (name, id) {
        (Some(name), Some(_)) => name.to_string(),
        (Some(name), None) => name.to_string(),
        (None, Some(id)) => id.to_string(),
        _ => "".to_string(),
    }
}

fn format_os(os: Option<&OperatingSystemSummary>) -> String {
    match os {
        Some(os) => match (os.name(), os.version()) {
            (Some(os), Some(version)) => format!("{} {}", os, version),
            (Some(os), None) => os.to_string(),
            _ => "Unknown".to_string(),
        },
        _ => "Unknown".to_string(),
    }
}

fn format_cpus(cpu: Option<&CpuSummary>) -> String {
    match cpu {
        Some(cpu) => match cpu.cores() {
            Some(cores) => format!("CPU: {}", cores),
            _ => "".to_string(),
        },
        _ => "".to_string(),
    }
}

fn format_bytes(bytes: i64) -> String {
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

fn format_mem(mem: Option<&MemorySummary>) -> String {
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

fn format_disk(storage: Option<&StorageSummary>) -> String {
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

fn format_arch(cpu: Option<&CpuSummary>) -> String {
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

fn format_class(class: Option<&MachineClass>) -> String {
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

fn format_group(group: Option<&str>) -> String {
    match group {
        Some(group) => group.to_string(),
        _ => "".to_string(),
    }
}

fn addresses_field(cx: Scope, addresses: Option<&[AddressSummary]>) -> impl IntoView {
    //#TODO: refactor to create a list of v4 and v6, then display the first and leave a note of +(x-1) more
    match addresses {
        Some(addresses) => view! { cx,
            <div class="flex flex-col text-left">
                { "Addresses" }
                <div class="flex flex-col text-xs text-gray-500">
                    {
                        addresses.iter()
                            .map(|a| view! { cx, <div>{ format_address(a.clone()) }</div> })
                            .collect::<Vec<_>>()
                    }
                </div>
            </div>
        },
        _ => view! { cx,
            <div class="flex flex-col text-left">
                { "Addresses" }
                <div class="flex flex-col text-xs text-gray-500">
                    { "None" }
                </div>
            </div>
        },
    }
}

fn format_address_version(version: &AddressVersion) -> String {
    match version {
        AddressVersion::V4 => "v4".to_string(),
        AddressVersion::V6 => "v6".to_string(),
        _ => "v?".to_string(),
    }
}

fn format_address(address: AddressSummary) -> String {
    match (address.version(), address.address()) {
        (Some(version), Some(address)) => {
            format!("{}: {} (+1 more)", format_address_version(version), address)
        }
        _ => "".to_string(),
    }
}

#[component]
pub fn MachineListRows(cx: Scope, machine: MachineSummary) -> impl IntoView {
    view! { cx,
        <a href={ format!("/machines/{}", machine.identifier().unwrap_or("?")) }>
            <div class="rounded-lg items-center justify-between border border-gray-200 bg-gray-100 my-2">
                <div class="text-left text-xs text-gray-500 mt-2 px-4">{ machine.identifier().unwrap_or("?").to_string() }</div>
                <div class="grid grid-cols-12 gap-4 text-sm">
                    <div class="col-span-3 p-4 flex">
                        <div class="pr-2">{ status_icon(cx, machine.status()) }</div>
                        <div class="flex flex-col text-left">
                            { format_name(machine.name(), machine.provider_id()) }
                            <div class="flex text-xs text-gray-500">
                                <p>{ format_arch(machine.cpu()) }</p>
                                <p class="px-1">{ "|" }</p>
                                <p>{ format_class(machine.class()) }</p>
                                <p class="px-1">{ "|" }</p>
                                <p>{ format_group(machine.group()) }</p>
                            </div>
                        </div>
                    </div>
                    <div class="col-span-4 mb-2 py-2 px-4 flex">
                        <div class="flex flex-col text-left">
                            { "System" }
                            <div class="flex text-left text-xs text-gray-500">
                                <div class="pr-1">{ format!("OS: {}", format_os(machine.os())) }</div>
                            </div>
                            <div class="flex text-left text-xs text-gray-500">
                                <div class="pr-1">{ format_cpus(machine.cpu()) }</div>
                                <div class="pr-1">{ format_mem(machine.memory()) }</div>
                                <div class="pr-1">{ format_disk(machine.storage()) }</div>
                            </div>
                        </div>
                    </div>
                    <div class="col-span-3 mb-2 py-2 px-4 flex">
                        { addresses_field(cx, machine.addresses()) }
                    </div>
                    <div class="col-span-2 p-4 flex">
                        <div class="mt-3 ml-5">
                            <button>
                                <Icon icon=icon!(BsThreeDots) width="1.2em" height="1.2em" />
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </a>
    }
}
