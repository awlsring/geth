$version: "2.0"

namespace awlsring.geth.control

use smithy.framework#ValidationException

use awlsring.geth.common#ResourceNotFoundException

@readonly
@http(method: "GET", uri: "/machine/{id}", code: 200)
operation DescribeMachine {
    input: DescribeMachineInput,
    output: DescribeMachineOutput,
    errors: [
        ResourceNotFoundException,
        ValidationException
    ]
}

@input
structure DescribeMachineInput {
    @httpLabel
    @required
    id: MachineId,
}

@output
structure DescribeMachineOutput {
    @required
    summary: MachineSummary
}
