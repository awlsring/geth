$version: "2.0"

namespace awlsring.geth.control

use smithy.framework#ValidationException

use awlsring.geth.common#ResourceNotFoundException

@readonly
@http(method: "GET", uri: "/machine/{id}/utilization", code: 200)
operation DescribeMachineUtilization {
    input: DescribeMachineUtilizationInput,
    output: DescribeMachineUtilizationOutput,
    errors: [
        ResourceNotFoundException,
        ValidationException
    ]
}

@input
structure DescribeMachineUtilizationInput {
    @httpLabel
    @required
    id: MachineId,
}

@output
structure DescribeMachineUtilizationOutput {
    @required
    summary: MachineUtilizationSummary
}

structure MemoryUtilizationSummary {
    memory: MemoryTypeUtilizationSummary

    swap: MemoryTypeUtilizationSummary
}

structure MemoryTypeUtilizationSummary {
    @required
    total: Long

    available: Long

    used: Long
}

structure CoreUtilizationSummary {
    @required
    name: String

    usage: Float

    frequency: Float
}

list CpuUtilizationSummary {
    member: CoreUtilizationSummary
}

structure VolumeUtilizationSummary {
    @required
    name: String

    @required
    totalSpace: Long

    availableSpace: Long

    usedSpace: Long
}

list VolumeUtilizationSummaries {
    member: VolumeUtilizationSummary
}

structure NetworkInterfaceTrafficSummary {
    transmitted: Long

    recieved: Long
}

structure NetworkInterfaceUtilizationSummary {
    @required
    name: String

    bytesTraffic: NetworkInterfaceTrafficSummary

    packetTraffic: NetworkInterfaceTrafficSummary
}

list NetworkInterfaceUtilizationSummaries {
    member: NetworkInterfaceUtilizationSummary
}

structure MachineUtilizationSummary {
    @required
    id: MachineId

    bootTime: Long

    upTime: Long

    memory: MemoryUtilizationSummary

    cpu: CpuUtilizationSummary

    volumes: VolumeUtilizationSummaries

    networkInterfaces: NetworkInterfaceUtilizationSummaries
}