$version: "2.0"

namespace awlsring.geth.agent
use smithy.framework#ValidationException

resource Disk {
    identifiers: { name: DiskName },
    read: GetDisk,
    list: ListDisks,
}

string DiskName

structure DiskSummary {
    @required
    name: String

    @required
    mountPoint: String

    @required
    availableSpace: Long

    @required
    totalSpace: Long

    @required
    fileSystem: String

    @required
    removeable: Boolean

    @required
    type: DiskType
}

list DiskSummaries {
    member: DiskSummary
}

enum DiskType {
    HDD = "HDD",
    SDD = "SSD",
    NVME = "NVME",
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
    input: ListDiskInput,
    output: ListDisksOutput,
    errors: [ValidationException]
}

@input 
structure ListDiskInput {}

@output
structure ListDisksOutput {
    @required
    summaries: DiskSummaries
}
