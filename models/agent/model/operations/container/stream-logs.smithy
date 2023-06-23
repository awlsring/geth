$version: "2.0"

namespace awlsring.geth.agent
use smithy.framework#ValidationException
use awlsring.geth.common#ResourceNotFoundException

// binary data stream
// @readonly
// @http(method: "GET", uri: "/container/{id}/logs", code: 200)
// operation StreamContainerLogs {
//     input: StreamContainerLogsInput,
//     output: StreamContainerLogsOutput,
//     errors: [
//         ResourceNotFoundException,
//         ValidationException
//     ]
// }

// @input
// structure StreamContainerLogsInput {
//     @httpLabel
//     @required
//     id: ContainerId,
// }

// @output
// structure StreamContainerLogsOutput {
//     @required
//     @httpPayload
//     data: StreamingBlob
// }

// @streaming
// blob StreamingBlob

@readonly
@http(method: "GET", uri: "/container/{id}/logs", code: 200)
operation StreamContainerLogs {
    input: StreamContainerLogsInput,
    output: StreamContainerLogsOutput,
    errors: [
        ValidationException,
        ResourceNotFoundException,
    ]
}

@input
structure StreamContainerLogsInput {
    @httpLabel
    @required
    id: ContainerId,

    @httpQuery("limit")
    limit: Integer,

    @httpQuery("follow")
    follow: Boolean,
}

@output
structure StreamContainerLogsOutput {
    @required
    @httpPayload
    logs: Logs
}

@streaming
union Logs {
    line: LogLine
}

structure LogLine {
    message: String

    timestamp: Long
}