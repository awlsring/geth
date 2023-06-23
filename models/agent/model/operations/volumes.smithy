$version: "2.0"

namespace awlsring.geth.agent
use smithy.framework#ValidationException

resource Volume {
    identifiers: { name: VolumeName },
    read: GetVolume,
    list: ListVolumes,
}

string VolumeName

structure VolumeSummary {
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
    type: VolumeType
}

list VolumeSummaries {
    member: VolumeSummary
}

enum VolumeType {
    HDD = "HDD",
    SDD = "SSD",
}

@readonly
@http(method: "GET", uri: "/volume/{name}", code: 200)
operation GetVolume {
    input: GetVolumeInput,
    output: GetVolumeOutput,
    errors: [
        ResourceNotFoundException,
        ValidationException
    ]
}

@input
structure GetVolumeInput {
    @httpLabel
    @required
    name: VolumeName,
}

@output
structure GetVolumeOutput {
    @required
    summary: VolumeSummary
}

@readonly
@http(method: "GET", uri: "/volume", code: 200)
operation ListVolumes {
    input: ListVolumesInput,
    output: ListVolumesOutput,
    errors: [ValidationException]
}

@input 
structure ListVolumesInput {}

@output
structure ListVolumesOutput {
    @required
    summaries: VolumeSummaries
}
