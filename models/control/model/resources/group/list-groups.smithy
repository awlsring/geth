$version: "2.0"

namespace awlsring.geth.control

use smithy.framework#ValidationException

use awlsring.geth.common#ResourceNotFoundException

@readonly
@http(method: "GET", uri: "/group", code: 200)
operation ListGroups {
    input: ListGroupsInput,
    output: ListGroupsOutput,
    errors: [
        ResourceNotFoundException,
        ValidationException
    ]
}

@input
structure ListGroupsInput {}

@output
structure ListGroupsOutput {
    @required
    summary: GroupSummaries
}
