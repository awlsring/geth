$version: "2.0"

namespace awlsring.geth.control

use smithy.framework#ValidationException

use awlsring.geth.common#ResourceNotFoundException

@idempotent
@http(method: "DELETE", uri: "/group/{id}", code: 200)
operation DeleteGroup {
    input: DeleteGroupInput,
    output: DeleteGroupOutput,
    errors: [
        ResourceNotFoundException,
        ValidationException,
    ]
}

@input
structure DeleteGroupInput {
    @httpLabel
    @required
    id: GroupId,
}

@output
structure DeleteGroupOutput {
    @required
    success: Boolean
}