#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserResponse {
    status_code: u16,
    body: String,
}

impl UserResponse {
    pub fn new(status_code: u16, body: String) -> Self {
        Self { status_code, body }
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    pub fn body(self) -> String {
        self.body
    }
}

// TODO: test new
