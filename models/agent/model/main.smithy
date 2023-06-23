$version: "2.0"

namespace awlsring.geth.agent

use aws.protocols#restJson1
use smithy.framework#ValidationException
use awlsring.geth.common#UnauthorizedException

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
        Disk,
        Volume,
        Cpu,
    ],
    operations: [ Health ],
    errors: [ UnauthorizedException ]
}