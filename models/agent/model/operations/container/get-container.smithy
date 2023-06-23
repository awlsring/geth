$version: "2.0"

namespace awlsring.geth.agent
use smithy.framework#ValidationException
use awlsring.geth.common#ResourceNotFoundException

@readonly
@http(method: "GET", uri: "/container/{id}", code: 200)
operation GetContainer {
    input: GetContainerInput,
    output: GetContainerOutput,
    errors: [
        ValidationException,
        ResourceNotFoundException,
    ]
}

@input
structure GetContainerInput {
    @httpLabel
    @required
    id: ContainerId,
}

@output
structure GetContainerOutput {
    @required
    summary: ContainerSummary
}
