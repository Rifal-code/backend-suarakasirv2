use chrono::Utc;
use sqlx::MySqlPool;

use crate::{errors::AppError, models::User};

pub struct UserRepository {
    pool: MySqlPool,
}

impl UserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, password, description, address, contact, \
             created_at, updated_at, deleted_at \
             FROM users WHERE email = ? AND deleted_at IS NULL",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, password, description, address, contact, \
             created_at, updated_at, deleted_at \
             FROM users WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn create(
        &self,
        id: &str,
        name: &str,
        email: &str,
        password_hash: &str,
        description: Option<&str>,
    ) -> Result<User, AppError> {
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO users (id, name, email, password, description, created_at, updated_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .bind(description)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let user = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::InternalServerError("Failed to retrieve created user".to_string()))?;

        Ok(user)
    }

    pub async fn update(
        &self,
        id: &str,
        name: Option<&str>,
        email: Option<&str>,
        password: Option<&str>,
        description: Option<Option<&str>>,
        address: Option<Option<&str>>,
        contact: Option<Option<&str>>,
    ) -> Result<User, AppError> {
        let now = Utc::now();

        if let Some(n) = name {
            sqlx::query("UPDATE users SET name = ?, updated_at = ? WHERE id = ?")
                .bind(n)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(e) = email {
            sqlx::query("UPDATE users SET email = ?, updated_at = ? WHERE id = ?")
                .bind(e)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(p) = password {
            sqlx::query("UPDATE users SET password = ?, updated_at = ? WHERE id = ?")
                .bind(p)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(desc) = description {
            sqlx::query("UPDATE users SET description = ?, updated_at = ? WHERE id = ?")
                .bind(desc)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(addr) = address {
            sqlx::query("UPDATE users SET address = ?, updated_at = ? WHERE id = ?")
                .bind(addr)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(ct) = contact {
            sqlx::query("UPDATE users SET contact = ?, updated_at = ? WHERE id = ?")
                .bind(ct)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        let user = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(user)
    }

    pub async fn email_exists_excluding(
        &self,
        email: &str,
        exclude_id: &str,
    ) -> Result<bool, AppError> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE email = ? AND id != ? AND deleted_at IS NULL",
        )
        .bind(email)
        .bind(exclude_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }
}
