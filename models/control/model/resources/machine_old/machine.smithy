// $version: "2.0"

// namespace awlsring.geth.control

// use smithy.framework#ValidationException

// use awlsring.geth.common#Tags
// use awlsring.geth.common#StringList

// resource Machine {
//     identifiers: { id: MachineId },
//     read: DescribeMachine,
//     list: ListMachines,
//     create: RegisterMachine,
//     delete: RemoveMachine,
//     operations: [ DescribeMachineUtilization ]
// }

// string MachineId

// enum MachineType {
//     BARE_METAL = "BareMetal",
//     VIRTUAL_MACHINE = "VirtualMachine",
//     HYPERVISOR = "Hypervisor",
// }

// enum MachineStatus {
//     RUNNING = "Running",
//     STOPPED = "Stopped",
//     UNKNOWN = "Unknown"
// }

// structure MachineStatusSummary {
//     @required
//     status: MachineStatus

//     @required
//     lastChecked: Long
// }

// structure MemorySummary {
//     @required
//     memory: MemoryTypeSummary
//     @required
//     swap: MemoryTypeSummary
// }

// structure MemoryTypeSummary {
//     @required
//     total: Long
// }

// structure SystemSummary {
//     @required
//     machineId: String
    
//     @required
//     family: String

//     @required
//     kernelVersion: String

//     @required
//     os: String

//     @required
//     osVersion: String

//     @required
//     osPretty: String

//     @required
//     hostname: String
// }

// structure CpuSummary {
//     @required
//     cores: Integer

//     @required
//     architecture: String

//     model: String

//     vendor: String
// }

// list DiskSummaries {
//     member: DiskSummary
// }

// enum DiskType {
//     HDD = "HDD",
//     SSD = "SSD",
//     NVME = "NVME",
//     UNKNOWN = "Unknown",
// }

// enum DiskInterface {
//     SATA = "SATA",
//     SCSI = "SCSI",
//     PCI_E = "PCIe",
//     UNKNOWN = "Unknown",
// }

// structure DiskSummary {
//     @required
//     device: String

//     @required
//     type: DiskType

//     @required
//     sizeActual: Long

//     model: String

//     vendor: String

//     interface: DiskInterface

//     serial: String

//     sectorSize: Integer

//     sizeRaw: Long
// }

// list VolumeSummaries {
//     member: VolumeSummary
// }

// structure VolumeSummary {
//     @required
//     name: String

//     @required
//     mountPoint: String

//     @required
//     totalSpace: Long

//     fileSystem: String
// }

// structure NetworkInterfaceSummary {
//     @required
//     name: String

//     @required
//     addresses: StringList

//     @required
//     virtual: Boolean

//     macAddress: String

//     vendor: String

//     mtu: Integer

//     duplex: String

//     speed: Integer
// }

// list NetworkInterfaceSummaries {
//     member: NetworkInterfaceSummary
// }

// structure AddressSummary {
//     @required
//     version: AddressVersion

//     @required
//     address: String

//     netmask: String

//     broadcast: String
// }

// list AddressSummaries {
//     member: AddressSummary
// }

// enum AddressVersion {
//     V4 = "V4",
//     V6 = "V6",
//     V6_LOCAL = "V6Local",
// }

// structure MachineSummary {
//     @required
//     id: MachineId

//     @required
//     status: MachineStatus

//     @required
//     group: GroupId

//     @required
//     added: Long

//     @required
//     tags: Tags,

//     @required
//     type: MachineType

//     updated: Long

//     system: SystemSummary

//     memory: MemorySummary

//     cpu: CpuSummary

//     disks: DiskSummaries

//     volumes: VolumeSummaries

//     networkInterfaces: NetworkInterfaceSummaries

//     addresses: AddressSummaries

//     containers: StringList
// }

// list MachineSummaries {
//     member: MachineSummary
// }