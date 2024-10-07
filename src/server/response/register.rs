use serde::Serialize;

#[derive(Serialize)]
pub struct IsRegisterResponse {
    is_register: bool,
}

impl IsRegisterResponse {
    pub fn new(is_register_response: bool) -> IsRegisterResponse {
        IsRegisterResponse {
            is_register: is_register_response,
        }
    }
}