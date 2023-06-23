$version: "2.0"

namespace awlsring.geth.agent

resource Container {
    identifiers: { id: ContainerId },
    read: GetContainer,
    list: ListContainers,
    operations: [
        StreamContainerLogs,
        StreamContainerStatistics,
    ]
}

string ContainerId

enum ContainerState {
    RUNNING = "Running",
    CREATED = "Created",
    RESTARTING = "Restarting",
    REMOVING = "Removing",
    PAUSED = "Paused",
    STOPPED = "Stopped",
    DEAD = "Dead",
    EMPTY = "Empty",
    UNKNOWN = "Unknown",
}

enum ContainerPortProtocol {
    TCP = "tcp",
    UDP = "udp",
    SCTP = "sctp",
    UNKNOWN = "unknown",
}

enum ContainerType {
    DOCKER = "Docker",
    UNKNOWN = "Unknown",
}

structure ContainerPortBinding {
    @required
    hostAddresses: StringList

    @required
    containerPort: Integer

    @required
    protocol: ContainerPortProtocol

    hostPort: Integer
}

list ContainerPortBindings {
    member: ContainerPortBinding
}

structure ContainerVolume {
    source: String

    destination: String

    mode: String
}

list ContainerVolumes {
    member: ContainerVolume
}

structure ContainerNetwork {
    @required
    name: String

    @required
    networkId: String

    @required
    endpointId: String
}

list ContainerNetworks {
    member: ContainerNetwork
}

structure ContainerStatistics {
    cpuUtilization: Float

    memoryUtilization: Float

    memoryUsage: Long

    memoryLimit: Long

    networkRxBytes: Long

    networkTxBytes: Long

    blockReadBytes: Long

    blockWriteBytes: Long
}

structure ContainerSummary {
    @required
    id: ContainerId

    @required
    name: String

    @required
    image: String

    @required
    created: Long

    @required
    state: ContainerState

    ports: ContainerPortBindings

    volumes: ContainerVolumes

    networks: ContainerNetworks

    labels: StringStringMap

    command: String

    environment: StringStringMap

    started: Long

    finished: Long

    @required
    containerType: ContainerType

    statistics: ContainerStatistics
}

list ContainerSummaries {
    member: ContainerSummary
}