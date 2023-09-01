use geth_control_client::types::{AddressSummary, AddressVersion, MachineSummary};
use leptos::*;
use leptos_icons::*;

use crate::components::machine::{
    status_icon::StatusIcon,
    summary_format::{
        format_arch, format_class, format_cpu, format_disk, format_group, format_mem, format_name,
        format_os,
    },
};

#[component]
pub fn MachineList(cx: Scope, machines: ReadSignal<Vec<MachineSummary>>) -> impl IntoView {
    view! { cx,
        <div class="text-gray-700">
            {
                machines.get().into_iter()
                    .map(|n| view! { cx, <MachineListRows machine=n.clone() />})
                    .collect::<Vec<_>>()
            }
        </div>
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
                        <div class="pr-2">
                            <StatusIcon status={ machine.status() } />
                        </div>
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
                                <div class="pr-1">{ format_cpu(machine.cpu()) }</div>
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
