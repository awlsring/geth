$version: "2.0"

namespace awlsring.geth.control

use smithy.framework#ValidationException

use awlsring.geth.common#Tags
use awlsring.geth.common#StringList

resource Machine {
    identifiers: { identifier: MachineId },
    read: DescribeMachine,
    list: ListMachines,
}

string MachineId

enum MachineClass {
    BARE_METAL = "BareMetal",
    VIRTUAL_MACHINE = "VirtualMachine",
    HYPERVISOR = "Hypervisor",
}

enum MachineStatus {
    RUNNING = "Running",
    STARTING = "Starting",
    STOPPING = "Stopping",
    STOPPED = "Stopped",
    UNKNOWN = "Unknown"
}

enum MachineArchitecture {
    X_86 = "x86",
    ARM = "arm",
}

structure MachineSummary {
    @documentation("The identifier of the machine.")
    @required
    identifier: MachineId

    @documentation("The name of the machine.")
    name: String

    @documentation("The provider assigned identifier for the machine.")
    providerId: String

    @documentation("The type of the machine categorized by the provider. This is like the instanceType on AWS.")
    providerType: String

    @documentation("The last known status of the machine and time last checked.")
    @required
    status: MachineStatusSummary

    @documentation("The group the machine belongs to.")
    @required
    group: GroupId

    @documentation("The time the machine was added.")
    @required
    added: Timestamp

    @documentation("The tags assigned to the machine.")
    @required
    tags: Tags,

    @documentation("The class of the machine.")
    @required
    class: MachineClass

    @documentation("The location of the machine. This is represented as the location assigned by the provider. This is like region on AWS.")
    @required
    location: String

    @documentation("The time the machine was last updated.")
    updated: Timestamp

    @documentation("The summary of the CPU")
    @required
    cpu: CpuSummary

    @documentation("The summary of the memory")
    @required
    memory: MemorySummary

    @documentation("The summary of the storage on the machine")
    @required
    storage: StorageSummary

    @documentation("List of network interfaces on the machine")
    networkInterfaces: NetworkInterfaceSummaries

    @documentation("List of addresses used by the machine.")
    @required
    addresses: AddressSummaries

    @documentation("The operating system running on the machine.")
    os: OperatingSystemSummary
}

structure CpuSummary {
    @required
    cores: Integer

    @required
    architecture: MachineArchitecture

    model: String

    vendor: String
}

structure MemorySummary {
    @required
    total: Long
}

structure StorageSummary {
    @required
    total: Long

    @required
    disks: DiskSummaries
}

enum AddressVersion {
    V4 = "V4",
    V6 = "V6",
    V6_LOCAL = "V6Local",
}

structure AddressSummary {
    @required
    version: AddressVersion

    @required
    address: String
}

list AddressSummaries {
    member: AddressSummary
}

structure OperatingSystemSummary {
    name: String
    version: String
    kernel: String
}

structure MachineStatusSummary {
    @required
    status: MachineStatus

    @required
    lastChecked: Timestamp
}

enum DiskType {
    HDD = "HDD",
    SSD = "SSD",
    NVME = "NVME",
    UNKNOWN = "Unknown",
}

structure DiskSummary {
    @required
    identifier: String

    type: DiskType

    @required
    size: Long
}

list DiskSummaries {
    member: DiskSummary
}

structure NetworkInterfaceSummary {
    @required
    name: String

    @required
    addresses: StringList

    macAddress: String

    vendor: String

    mtu: Integer

    duplex: String

    speed: Integer
}

list NetworkInterfaceSummaries {
    member: NetworkInterfaceSummary
}

list MachineSummaries {
    member: MachineSummary
}