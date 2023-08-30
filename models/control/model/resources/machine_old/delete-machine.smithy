// $version: "2.0"

// namespace awlsring.geth.control

// use smithy.framework#ValidationException

// use awlsring.geth.common#ResourceNotFoundException

// @idempotent
// @http(method: "DELETE", uri: "/machine/{id}", code: 200)
// operation RemoveMachine {
//     input: RemoveMachineInput,
//     output: RemoveMachineOutput,
//     errors: [
//         ResourceNotFoundException,
//         ValidationException,
//     ]
// }

// @input
// structure RemoveMachineInput {
//     @httpLabel
//     @required
//     id: MachineId,
// }

// @output
// structure RemoveMachineOutput {
//     @required
//     success: Boolean
// }