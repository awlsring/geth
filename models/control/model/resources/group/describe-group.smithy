$version: "2.0"

namespace awlsring.geth.control

use smithy.framework#ValidationException

use awlsring.geth.common#ResourceNotFoundException

@readonly
@http(method: "GET", uri: "/group/{id}", code: 200)
operation DescribeGroup {
    input: DescribeGroupInput,
    output: DescribeGroupOutput,
    errors: [
        ResourceNotFoundException,
        ValidationException
    ]
}

@input
structure DescribeGroupInput {
    @httpLabel
    @required
    id: GroupId,
}

@output
structure DescribeGroupOutput {
    @required
    summary: GroupSummary
}
