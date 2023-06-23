$version: "2.0"

namespace awlsring.geth.control
use smithy.framework#ValidationException
use awlsring.geth.common#ResourceNotFoundException
use awlsring.geth.agent#OverviewSummary

string ServerId

resource Server {
    identifiers: { id: ServerId },
    read: GetServer,
}

@readonly
@http(method: "GET", uri: "/server/{id}", code: 200)
operation GetServer {
    input: GetServerInput,
    output: GetServerOutput,
    errors: [
        ResourceNotFoundException,
        ValidationException
    ]
}

@input
structure GetServerInput {
    @httpLabel
    @required
    id: ServerId,
}

@output
structure GetServerOutput {
    @required
    summary: OverviewSummary
}