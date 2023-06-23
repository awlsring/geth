$version: "2.0"

namespace awlsring.geth.agent
use smithy.framework#ValidationException

@readonly
@http(method: "GET", uri: "/container/{id}/statistics", code: 200)
operation StreamContainerStatistics {
    input: StreamContainerStatisticsInput,
    output: StreamContainerStatisticsOutput,
    errors: [
        ResourceNotFoundException,
        ValidationException
    ]
}

@input
structure StreamContainerStatisticsInput {
    @httpLabel
    @required
    id: ContainerId,
}

@output
structure StreamContainerStatisticsOutput {
    @required
    @httpPayload
    stream: StatisticsStream
}

@streaming
union StatisticsStream {
    statistics: ContainerStatistics,
}