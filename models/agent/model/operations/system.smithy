$version: "2.0"

namespace awlsring.geth.agent
use smithy.framework#ValidationException

resource System {
    read: GetSystem,
}

structure SystemSummary {
    @required
    machineId: String
    
    @required
    family: String

    @required
    kernelVersion: String

    @required
    os: String

    @required
    osVersion: String

    @required
    osPretty: String

    @required
    hostname: String

    @required
    bootTime: Long

    @required
    upTime: Long
}

@readonly
@http(method: "GET", uri: "/system", code: 200)
operation GetSystem {
    input: GetSystemInput,
    output: GetSystemOutput,
    errors: [ValidationException]
}

@input 
structure GetSystemInput {}

@output
structure GetSystemOutput {
    @required
    summary: SystemSummary
}
