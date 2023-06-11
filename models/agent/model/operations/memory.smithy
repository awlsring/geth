$version: "2.0"

namespace awlsring.geth.agent
use smithy.framework#ValidationException

resource Memory {
    read: GetMemory,
}

@readonly
@http(method: "GET", uri: "/memory", code: 200)
operation GetMemory {
    input: GetMemoryInput,
    output: GetMemoryOutput,
    errors: [ValidationException]
}

@input
structure GetMemoryInput {}

@output
structure GetMemoryOutput {
    @required
    summary: MemorySummary
}

structure MemorySummary {
    @required
    total: Long

    @required
    available: Long

    @required
    used: Long
}