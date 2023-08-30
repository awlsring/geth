use geth_control_client::{
    primitives::DateTime,
    types::{
        builders::{
            AddressSummaryBuilder, CpuSummaryBuilder, MachineStatusSummaryBuilder,
            MachineSummaryBuilder, MemorySummaryBuilder, OperatingSystemSummaryBuilder,
            StorageSummaryBuilder, TagBuilder,
        },
        AddressVersion, MachineArchitecture, MachineClass, MachineSummary,
    },
};
use leptos::*;

use crate::components::machine::{list_header::Header, machine_list::MachineList};

pub fn fetch_machines() -> Vec<MachineSummary> {
    let machine = MachineSummaryBuilder::default()
        .identifier("m-12823764234876".to_string())
        .name("test")
        .provider_id("i-12317835122123")
        .provider_type("c5.large")
        .group("aws-dws")
        .added(DateTime::from_secs(0))
        .updated(DateTime::from_secs(0))
        .tags(TagBuilder::default().key("hest").value("input").build())
        .class(MachineClass::VirtualMachine)
        .location("us-west-2")
        .cpu(
            CpuSummaryBuilder::default()
                .cores(4)
                .architecture(MachineArchitecture::X86)
                .build(),
        )
        .memory(
            MemorySummaryBuilder::default()
                .total(1024 * 1024 * 1024 * 4)
                .build(),
        )
        .storage(
            StorageSummaryBuilder::default()
                .total(1024 * 1024 * 1024 * 100)
                .build(),
        )
        .addresses(
            AddressSummaryBuilder::default()
                .version(AddressVersion::V4)
                .address("127.0.0.1")
                .build(),
        )
        .addresses(
            AddressSummaryBuilder::default()
                .version(AddressVersion::V6)
                .address("::1")
                .build(),
        )
        .os(OperatingSystemSummaryBuilder::default()
            .name("Ubuntu")
            .version("20.04")
            .build())
        .status(
            MachineStatusSummaryBuilder::default()
                .status(geth_control_client::types::MachineStatus::Running)
                .last_checked(DateTime::from_secs(0))
                .build(),
        )
        .build();

    let machine2 = MachineSummaryBuilder::default()
        .identifier("m-23423490238422".to_string())
        .name("local")
        .group("dws-sea")
        .added(DateTime::from_secs(0))
        .updated(DateTime::from_secs(0))
        .tags(TagBuilder::default().key("test").value("input").build())
        .class(MachineClass::VirtualMachine)
        .location("awsring-sea")
        .cpu(
            CpuSummaryBuilder::default()
                .cores(4)
                .architecture(MachineArchitecture::X86)
                .build(),
        )
        .memory(
            MemorySummaryBuilder::default()
                .total(1024 * 1024 * 1024 * 4)
                .build(),
        )
        .storage(
            StorageSummaryBuilder::default()
                .total(1024 * 1024 * 1024 * 100)
                .build(),
        )
        .addresses(
            AddressSummaryBuilder::default()
                .version(AddressVersion::V4)
                .address("127.0.0.1")
                .build(),
        )
        .addresses(
            AddressSummaryBuilder::default()
                .version(AddressVersion::V6)
                .address("::1")
                .build(),
        )
        .os(OperatingSystemSummaryBuilder::default()
            .name("Ubuntu")
            .version("20.04")
            .build())
        .status(
            MachineStatusSummaryBuilder::default()
                .status(geth_control_client::types::MachineStatus::Running)
                .last_checked(DateTime::from_secs(0))
                .build(),
        )
        .build();
    vec![machine, machine2]
}

#[component]
pub fn Machines(cx: Scope) -> impl IntoView {
    let (machines, set_machines) = create_signal(cx, Vec::<MachineSummary>::new());
    set_machines(fetch_machines());
    view! { cx,
        <div class="container text-center">
            <Header />
            <MachineList machines=machines />
        </div>
    }
}
