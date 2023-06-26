$version: "2.0"

namespace awlsring.geth.control

use smithy.framework#ValidationException

use awlsring.geth.common#ResourceNotFoundException
use awlsring.geth.common#InvalidInputException

@http(method: "POST", uri: "/machine", code: 200)
operation RegisterMachine {
    input: RegisterMachineInput,
    output: RegisterMachineOutput,
    errors: [
        ResourceNotFoundException,
        InvalidInputException
        ValidationException
    ]
}

@input
structure RegisterMachineInput {
    @required
    address: String

    @required
    groupId: String
}

@output
structure RegisterMachineOutput {
    @required
    summary: MachineSummary
}
