$version: "2.0"

namespace awlsring.geth.agent
use smithy.framework#ValidationException

resource NetworkInterface {
    identifiers: { name: NetworkInterfaceName },
    read: GetNetworkInterface,
    list: ListNetworkInterfaces,
}

string NetworkInterfaceName

structure NetworkInterfaceSummary {
    @required
    name: String

    @required
    addresses: AddressSummaries

    @required
    macAddress: String

    // maybe make util stuff a seprate call to do lazy refreshes
    @required
    bytesTraffic: NetworkInterfaceTrafficSummary

    @required
    packetTraffic: NetworkInterfaceTrafficSummary
}

list NetworkInterfaceSummaries {
    member: NetworkInterfaceSummary
}

structure NetworkInterfaceTrafficSummary {
    @required
    transmitted: Long

    @required
    recieved: Long
}

structure AddressSummary {
    @required
    version: AddressVersion

    @required
    address: String

    netmask: String

    broadcast: String
}

list AddressSummaries {
    member: AddressSummary
}

enum AddressVersion {
    V4 = "V4",
    V6 = "V6",
    V6_LOCAL = "V6Local",
}

@readonly
@http(method: "GET", uri: "/network/{name}", code: 200)
operation GetNetworkInterface {
    input: GetNetworkInterfaceInput,
    output: GetNetworkInterfaceOutput,
    errors: [
        ResourceNotFoundException,
        ValidationException
    ]
}

@input 
structure GetNetworkInterfaceInput {
    @httpLabel
    @required
    name: NetworkInterfaceName,
}

@output
structure GetNetworkInterfaceOutput {
    @required
    summary: NetworkInterfaceSummary
}

@readonly
@http(method: "GET", uri: "/network", code: 200)
operation ListNetworkInterfaces {
    input: ListNetworkInterfacesInput,
    output: ListNetworkInterfacesOutput,
    errors: [ValidationException]
}

@input 
structure ListNetworkInterfacesInput {}

@output
structure ListNetworkInterfacesOutput {
    @required
    summaries: NetworkInterfaceSummaries
}
