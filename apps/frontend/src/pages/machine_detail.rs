use aws_smithy_client::{
    erase::{DynConnector, DynMiddleware},
    SdkError,
};
use geth_control_client::{
    operation::describe_machine::{
        builders::DescribeMachineOutputBuilder, DescribeMachineError, DescribeMachineOutput,
    },
    primitives::DateTime,
    types::{
        builders::{
            AddressSummaryBuilder, CpuSummaryBuilder, MachineStatusSummaryBuilder,
            MachineSummaryBuilder, MemorySummaryBuilder, OperatingSystemSummaryBuilder,
            StorageSummaryBuilder, TagBuilder,
        },
        AddressVersion, MachineArchitecture, MachineClass, MachineSummary,
    },
    Client,
};
use leptos::*;
use leptos_router::{use_params_map, Outlet, A};

use crate::components::machine::details::{header::DetailHeader, view::MachineDetailView};

#[component]
pub fn MachineDetail(cx: Scope) -> impl IntoView {
    log!("Loading machine detail");
    let params = use_params_map(cx);
    let identifier = params.with(|params| params.get("id").cloned().unwrap_or_default());
    let (id, _) = create_signal(cx, identifier);
    let machine_data = create_local_resource_with_initial_value(
        cx,
        id,
        |v| async move { fetch_machine(v).await },
        None,
    );

    view! { cx,
        <div class="container text-center">
            <DetailHeader id=id.get()/>
            {move || match machine_data.read(cx) {
                None => view! { cx, <p>"Loading..."</p> }.into_view(cx),
                Some(data) => match data {
                    Ok(summary) => view! { cx, <MachineDetailView summary=summary /> }.into_view(cx),
                    Err(e) => view! { cx, <p>{format!("Error: {}", e)}</p> }.into_view(cx),
                }
            }}
        </div>
    }
}

// fn get_client() -> Client<DynConnector, DynMiddleware<DynConnector>> {
//     todo!()
// }

//tmp machine list for ui dev
async fn fetch_machine(identifier: String) -> Result<MachineSummary, String> {
    // let c = get_client();
    // let r = c.describe_machine().identifier(identifier).send().await;

    let machine = MachineSummaryBuilder::default()
        .identifier(identifier)
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

    Ok(machine)
}
