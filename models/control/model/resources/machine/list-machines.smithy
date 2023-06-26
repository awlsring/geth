$version: "2.0"

namespace awlsring.geth.control

use smithy.framework#ValidationException

use awlsring.geth.common#ResourceNotFoundException

@readonly
@http(method: "GET", uri: "/machine", code: 200)
operation ListMachines {
    input: ListMachinesInput,
    output: ListMachinesOutput,
    errors: [
        ValidationException
    ]
}

@input
structure ListMachinesInput {}

@output
structure ListMachinesOutput {
    @required
    summary: MachineSummaries
}
