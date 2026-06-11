use chrono::Utc;
use sqlx::MySqlPool;

use crate::{errors::AppError, models::Feedback};

pub struct FeedbackRepository {
    pool: MySqlPool,
}

#[derive(sqlx::FromRow)]
struct FeedbackWithUser {
    pub id: String,
    pub user_id: String,
    pub user_name: String,
    pub message: String,
    pub is_public: i8,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl FeedbackRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn find_all_public(
        &self,
        page: u32,
        limit: u32,
    ) -> Result<(Vec<(Feedback, String)>, i64), AppError> {
        let offset = (page.saturating_sub(1)) * limit;

        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM feedback WHERE is_public = 1 AND deleted_at IS NULL",
        )
        .fetch_one(&self.pool)
        .await?;

        let rows = sqlx::query_as::<_, FeedbackWithUser>(
            "SELECT f.id, f.user_id, u.name as user_name, f.message, \
             f.is_public, f.created_at, f.updated_at, f.deleted_at \
             FROM feedback f \
             JOIN users u ON u.id = f.user_id \
             WHERE f.is_public = 1 AND f.deleted_at IS NULL \
             ORDER BY f.created_at DESC LIMIT ? OFFSET ?",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let result = rows
            .into_iter()
            .map(|r| {
                let user_name = r.user_name.clone();
                let feedback = Feedback {
                    id: r.id,
                    user_id: r.user_id,
                    message: r.message,
                    is_public: r.is_public != 0,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                    deleted_at: r.deleted_at,
                };
                (feedback, user_name)
            })
            .collect();

        Ok((result, total))
    }

    pub async fn find_public_by_id(&self, id: &str) -> Result<Option<(Feedback, String)>, AppError> {
        let row = sqlx::query_as::<_, FeedbackWithUser>(
            "SELECT f.id, f.user_id, u.name as user_name, f.message, \
             f.is_public, f.created_at, f.updated_at, f.deleted_at \
             FROM feedback f \
             JOIN users u ON u.id = f.user_id \
             WHERE f.id = ? AND f.is_public = 1 AND f.deleted_at IS NULL",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            let user_name = r.user_name.clone();
            let feedback = Feedback {
                id: r.id,
                user_id: r.user_id,
                message: r.message,
                is_public: r.is_public != 0,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            };
            (feedback, user_name)
        }))
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Feedback>, AppError> {
        let feedback = sqlx::query_as::<_, Feedback>(
            "SELECT id, user_id, message, is_public, created_at, updated_at, deleted_at \
             FROM feedback WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(feedback)
    }

    pub async fn create(
        &self,
        id: &str,
        user_id: &str,
        message: &str,
        is_public: bool,
    ) -> Result<Feedback, AppError> {
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO feedback (id, user_id, message, is_public, created_at, updated_at) \
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(user_id)
        .bind(message)
        .bind(is_public)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let feedback = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::InternalServerError("Failed to retrieve created feedback".to_string()))?;

        Ok(feedback)
    }

    pub async fn update(
        &self,
        id: &str,
        message: Option<&str>,
        is_public: Option<bool>,
    ) -> Result<Feedback, AppError> {
        let now = Utc::now();

        if let Some(m) = message {
            sqlx::query("UPDATE feedback SET message = ?, updated_at = ? WHERE id = ?")
                .bind(m)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(p) = is_public {
            sqlx::query("UPDATE feedback SET is_public = ?, updated_at = ? WHERE id = ?")
                .bind(p)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        let feedback = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Feedback not found".to_string()))?;

        Ok(feedback)
    }

    pub async fn soft_delete(&self, id: &str) -> Result<(), AppError> {
        let now = Utc::now();

        sqlx::query("UPDATE feedback SET deleted_at = ?, updated_at = ? WHERE id = ?")
            .bind(now)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
