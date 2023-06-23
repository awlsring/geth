$version: "2.0"

namespace awlsring.geth.agent

use smithy.framework#ValidationException
use aws.protocols#restJson1

@title("Geth Agent REST API")
@restJson1
@httpBearerAuth
@httpApiKeyAuth(scheme: "ApiKey", name: "Authorization", in: "header")
@paginated(
    inputToken: "nextToken",
    outputToken: "nextToken",
    pageSize: "pageSize"
)
service GethAgent {
    version: "2023-06-07",
    resources: [
        Container,
        NetworkInterface,
        Overview
        System,
        Memory,
        Swap,
        Disk,
        Volume,
        Cpu,
    ],
    operations: [ Health ],
    errors: [ UnauthorizedException ]
}