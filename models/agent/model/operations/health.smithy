$version: "2.0"

namespace awlsring.geth.agent
use smithy.framework#ValidationException

@readonly
@http(method: "GET", uri: "/health", code: 200)
operation Health {
    output: HealthOutput,
    errors: [ValidationException]
}

@output
structure HealthOutput {
    @required
    success: Boolean
}