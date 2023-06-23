$version: "2.0"

namespace awlsring.geth.agent
use smithy.framework#ValidationException
use awlsring.geth.common#ResourceNotFoundException

resource Disk {
    identifiers: { name: DiskName },
    read: GetDisk,
    list: ListDisks,
}

string DiskName

structure DiskSummary {
    @required
    device: String

    @required
    model: String

    @required
    vendor: String

    @required
    interface: DiskInterface

    @required
    serial: String

    @required
    type: DiskType

    @required
    sectorSize: Integer

    @required
    sizeRaw: Long

    @required
    sizeActual: Long
}

list DiskSummaries {
    member: DiskSummary
}

enum DiskType {
    HDD = "HDD",
    SSD = "SSD",
    NVME = "NVME",
    UNKNOWN = "Unknown",
}

enum DiskInterface {
    SATA = "SATA",
    SCSI = "SCSI",
    PCI_E = "PCIe",
    UNKNOWN = "Unknown",
}

@readonly
@http(method: "GET", uri: "/disk/{name}", code: 200)
operation GetDisk {
    input: GetDiskInput,
    output: GetDiskOutput,
    errors: [
        ResourceNotFoundException,
        ValidationException
    ]
}

@input
structure GetDiskInput {
    @httpLabel
    @required
    name: DiskName,
}

@output
structure GetDiskOutput {
    @required
    summary: DiskSummary
}

@readonly
@http(method: "GET", uri: "/disk", code: 200)
operation ListDisks {
    input: ListDisksInput,
    output: ListDisksOutput,
    errors: [ValidationException]
}

@input 
structure ListDisksInput {}

@output
structure ListDisksOutput {
    @required
    summaries: DiskSummaries
}
