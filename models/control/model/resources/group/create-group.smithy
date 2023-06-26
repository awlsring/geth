$version: "2.0"

namespace awlsring.geth.control

use smithy.framework#ValidationException

use awlsring.geth.common#ResourceNotFoundException
use awlsring.geth.common#InvalidInputException

@http(method: "POST", uri: "/group", code: 200)
operation CreateGroup {
    input: CreateGroupInput,
    output: CreateGroupOutput,
    errors: [
        ResourceNotFoundException,
        InvalidInputException
        ValidationException
    ]
}

@input
structure CreateGroupInput {
    @required
    name: String

    platform: String

    location: String
}

@output
structure CreateGroupOutput {
    @required
    summary: GroupSummary
}
