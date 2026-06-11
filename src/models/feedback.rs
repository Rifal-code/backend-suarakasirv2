use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Feedback {
    pub id: String,
    pub user_id: String,
    pub message: String,
    #[sqlx(rename = "is_public")]
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Feedback {
    pub fn new_id() -> String {
        Uuid::new_v4().to_string()
    }
}
