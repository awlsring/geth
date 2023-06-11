use std::{sync::Arc, str::FromStr};

use aws_smithy_http_server::Extension;
use geth_agent_server::{output::GetDiskOutput, output::ListDisksOutput, model::{DiskSummary, DiskType}, input::GetDiskInput, input::ListDisksInput, error};
use sysinfo::DiskKind;

use crate::{server::server::State, stats::disk::Disk};


pub async fn get_disk(input: GetDiskInput, state: Extension<Arc<State>>) -> Result<GetDiskOutput, error::GetDiskError> {
    let ctl = state.controller.lock().await;
    let disks = ctl.storage();

    let disk = disks.get_disk(input.name());

    match disk {
        Some(d) => {
            let sum = disk_to_summary(d);
            let output = GetDiskOutput { summary: sum };
            Ok(output)
        }
        None => Err(error::GetDiskError::ResourceNotFoundException(error::ResourceNotFoundException { message: format!("Disk {} not found", input.name()) }))
    }
}

pub async fn list_disks(_input: ListDisksInput, state: Extension<Arc<State>>) -> Result<ListDisksOutput, error::ListDisksError> {
    let ctl = state.controller.lock().await;
    let disks = ctl.storage();
    let sums = disks_to_summaries(disks.disks());
    let output = ListDisksOutput { summaries: sums };
    Ok(output)
}

pub fn disks_to_summaries(disks: Vec<&Disk>) -> Vec<DiskSummary> {
    let mut summaries = Vec::new();
    for disk in disks {
        let sum = disk_to_summary(disk);
        summaries.push(sum);
    }

    summaries
}

pub fn disk_to_summary(disk: &Disk) -> DiskSummary {
    let name = disk.name().to_owned();
    let mount_point = disk.mount_point().to_owned();
    let file_system = disk.file_system().to_owned();
    let total = *disk.total_space() as i64;
    let available = *disk.available_space() as i64;
    let removeable = *disk.is_removable();
    let t = match disk.disk_type() {
        DiskKind::HDD => "HDD",
        DiskKind::SSD => "SSD",
        DiskKind::Unknown(_) => "HDD",
    };
    let kind = DiskType::from_str(t);

    DiskSummary {
        name,
        mount_point,
        file_system,
        total_space: total,
        available_space: available,
        removeable,
        r#type: kind.unwrap(),
    }
}