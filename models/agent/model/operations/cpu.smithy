$version: "2.0"

namespace awlsring.geth.agent
use smithy.framework#ValidationException

resource Cpu {
    read: GetCpu,
}

@readonly
@http(method: "GET", uri: "/cpu", code: 200)
operation GetCpu {
    input: GetCpuInput,
    output: GetCpuOutput,
    errors: [ValidationException]
}

@input
structure GetCpuInput {}

@output
structure GetCpuOutput {
    @required
    summary: CpuSummary
}

structure CpuSummary {
    @required
    cores: Integer

    @required
    architecture: String

    @required
    model: String

    @required
    vendor: String
    
    @required
    utilization: CpuUtilization
}

structure CoreUtilization {
    @required
    name: String

    @required
    usage: Long

    @required
    frequency: Float
}

list CpuUtilization {
    member: CoreUtilization
}