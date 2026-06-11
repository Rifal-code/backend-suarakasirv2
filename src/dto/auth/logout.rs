use serde::Serialize;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct LogoutResponse {
    pub message: String,
}
