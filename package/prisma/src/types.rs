use crate::{machine_summary, PrismaClient};

machine_summary::include!(machine_full_summary {
    status
    tags
    system
    memory
    cpu
    disks
    volumes
    network_interfaces
    addresses
    containers
});
pub type MachineSummaryFull = machine_full_summary::Data;
