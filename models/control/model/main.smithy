$version: "2.0"

namespace awlsring.geth.control

use aws.protocols#restJson1
use smithy.framework#ValidationException
use awlsring.geth.common#Health
use awlsring.geth.common#UnauthorizedException

@title("Geth Control REST API")
@restJson1
@httpBearerAuth
@httpApiKeyAuth(scheme: "ApiKey", name: "Authorization", in: "header")
@paginated(
    inputToken: "nextToken",
    outputToken: "nextToken",
    pageSize: "pageSize"
)
service GethControl {
    version: "2023-06-23",
    resources: [ Machine, Group ],
    operations: [ Health ],
    errors: [ UnauthorizedException ]
}