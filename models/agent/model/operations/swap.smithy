$version: "2.0"

namespace awlsring.geth.agent
use smithy.framework#ValidationException

resource Swap {
    read: GetSwap,
}

@readonly
@http(method: "GET", uri: "/swap", code: 200)
operation GetSwap {
    input: GetSwapInput,
    output: GetSwapOutput,
    errors: [ValidationException]
}

@input 
structure GetSwapInput {}

@output
structure GetSwapOutput {
    @required
    summary: SwapSummary
}

structure SwapSummary {
    @required
    total: Long

    @required
    available: Long

    @required
    used: Long
}