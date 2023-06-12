
#[derive(Clone, Debug)]
pub struct AuthController {}

impl AuthController {
    pub fn new() -> Self {
        AuthController {}
    }

    pub async fn auth(&self, operation: &str, key: &str) -> bool {
        if operation == "Health" {
            return true;
        }

        // load in with key list
        if key == "toes" {
            return true;
        }

        false
    }
}