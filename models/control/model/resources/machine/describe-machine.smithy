$version: "2.0"

namespace awlsring.geth.control

use smithy.framework#ValidationException

use awlsring.geth.common#ResourceNotFoundException

@readonly
@http(method: "GET", uri: "/machine/{identifier}", code: 200)
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
    identifier: MachineId,
}

@output
structure DescribeMachineOutput {
    @required
    summary: MachineSummary
}
