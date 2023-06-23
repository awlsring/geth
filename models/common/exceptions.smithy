$version: "2.0"

namespace awlsring.geth.common

@error("client")
@httpError(400)
structure InvalidInputException {
    @required
    message: String
}

@error("client")
@httpError(404)
structure ResourceNotFoundException {
    @required
    message: String
}

@error("client")
@httpError(401)
structure UnauthorizedException {
    @required
    message: String
}

@error("server")
@httpError(500)
structure InternalServerException {
    @required
    message: String
}