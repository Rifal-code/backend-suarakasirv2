use bcrypt::{hash, DEFAULT_COST};

use crate::{
    dto::auth::{
        LoginRequest, LoginResponse, ProfileResponse, RegisterRequest, RegisterResponse,
        UpdateProfileRequest, UserInfo,
    },
    errors::AppError,
    middleware::generate_token,
    models::User,
    repositories::UserRepository,
};

pub struct AuthService {
    user_repo: UserRepository,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(user_repo: UserRepository, jwt_secret: String) -> Self {
        Self { user_repo, jwt_secret }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<RegisterResponse, AppError> {
        if self.user_repo.find_by_email(&req.email).await?.is_some() {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }

        let password_hash = hash(&req.password, DEFAULT_COST).map_err(AppError::from)?;

        let id = User::new_id();
        let user = self
            .user_repo
            .create(&id, &req.name, &req.email, &password_hash, req.description.as_deref())
            .await?;

        Ok(RegisterResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            description: user.description,
        })
    }

    pub async fn login(&self, req: LoginRequest) -> Result<LoginResponse, AppError> {
        let user = self
            .user_repo
            .find_by_email(&req.email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

        let valid = bcrypt::verify(&req.password, &user.password)
            .map_err(|_| AppError::InternalServerError("Password verification failed".to_string()))?;

        if !valid {
            return Err(AppError::Unauthorized("Invalid email or password".to_string()));
        }

        let token = generate_token(&user.id, &self.jwt_secret)?;

        Ok(LoginResponse {
            token,
            user: UserInfo {
                id: user.id,
                name: user.name,
                email: user.email,
                description: user.description,
            },
        })
    }

    pub async fn get_profile(&self, user_id: &str) -> Result<ProfileResponse, AppError> {
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(user_to_profile(user))
    }

    pub async fn update_profile(
        &self,
        user_id: &str,
        req: UpdateProfileRequest,
    ) -> Result<ProfileResponse, AppError> {
        self.user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        if let Some(email) = &req.email {
            if self.user_repo.email_exists_excluding(email, user_id).await? {
                return Err(AppError::Conflict("Email already in use".to_string()));
            }
        }

        let new_password_hash = if let Some(p) = &req.password {
            Some(hash(p, DEFAULT_COST).map_err(AppError::from)?)
        } else {
            None
        };

        let user = self
            .user_repo
            .update(
                user_id,
                req.name.as_deref(),
                req.email.as_deref(),
                new_password_hash.as_deref(),
                req.description.as_ref().map(|d| Some(d.as_str())),
                req.address.as_ref().map(|a| Some(a.as_str())),
                req.contact.as_ref().map(|c| Some(c.as_str())),
            )
            .await?;

        Ok(user_to_profile(user))
    }
}

fn user_to_profile(user: crate::models::User) -> ProfileResponse {
    ProfileResponse {
        id: user.id,
        name: user.name,
        email: user.email,
        description: user.description,
        address: user.address,
        contact: user.contact,
    }
}
