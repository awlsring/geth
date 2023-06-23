$version: "2.0"

namespace awlsring.geth.agent
use smithy.framework#ValidationException

@readonly
@http(method: "GET", uri: "/container", code: 200)
operation ListContainers {
    input: ListContainersInput,
    output: ListContainersOutput,
    errors: [
        ValidationException
    ]
}

@input
structure ListContainersInput {}

@output
structure ListContainersOutput {
    @required
    summaries: ContainerSummaries
}
