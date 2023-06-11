$version: "2.0"

namespace awlsring.geth.agent
use smithy.framework#ValidationException

resource Overview {
    read: GetOverview,
}

@readonly
@http(method: "GET", uri: "/overview", code: 200)
operation GetOverview {
    input: GetOverviewInput,
    output: GetOverviewOutput,
    errors: [ValidationException]
}

@input
structure GetOverviewInput {}

@output
structure GetOverviewOutput {
    @required
    summary: OverviewSummary
}

structure OverviewSummary {
    @required
    system: SystemSummary

    @required
    memory: MemorySummary

    @required
    swap: SwapSummary

    @required
    cpu: CpuSummary

    @required
    disk: DiskSummaries

    @required
    network: NetworkInterfaceSummaries
}