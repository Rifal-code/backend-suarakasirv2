pub mod login;
pub mod logout;
pub mod register;

pub use login::{LoginRequest, LoginResponse, UserInfo};
pub use register::{RegisterRequest, RegisterResponse};

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 2, max = 100, message = "Name must be between 2 and 100 characters"))]
    pub name: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: Option<String>,

    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub description: Option<String>,
}
