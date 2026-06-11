use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::MySqlPool;

use crate::{errors::AppError, models::Product};

pub struct ProductRepository {
    pool: MySqlPool,
}

impl ProductRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn find_all_by_user(
        &self,
        user_id: &str,
        page: u32,
        limit: u32,
        search: Option<&str>,
    ) -> Result<(Vec<Product>, i64), AppError> {
        let offset = (page.saturating_sub(1)) * limit;

        let (total, products) = if let Some(s) = search {
            let pattern = format!("%{}%", s);
            let total: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM products WHERE user_id = ? AND deleted_at IS NULL AND name LIKE ?",
            )
            .bind(user_id)
            .bind(&pattern)
            .fetch_one(&self.pool)
            .await?;

            let products = sqlx::query_as::<_, Product>(
                "SELECT id, user_id, name, price, description, created_at, updated_at, deleted_at \
                 FROM products WHERE user_id = ? AND deleted_at IS NULL AND name LIKE ? \
                 ORDER BY created_at DESC LIMIT ? OFFSET ?",
            )
            .bind(user_id)
            .bind(&pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

            (total, products)
        } else {
            let total: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM products WHERE user_id = ? AND deleted_at IS NULL",
            )
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

            let products = sqlx::query_as::<_, Product>(
                "SELECT id, user_id, name, price, description, created_at, updated_at, deleted_at \
                 FROM products WHERE user_id = ? AND deleted_at IS NULL \
                 ORDER BY created_at DESC LIMIT ? OFFSET ?",
            )
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

            (total, products)
        };

        Ok((products, total))
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Product>, AppError> {
        let product = sqlx::query_as::<_, Product>(
            "SELECT id, user_id, name, price, description, created_at, updated_at, deleted_at \
             FROM products WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(product)
    }

    pub async fn name_exists_for_user(
        &self,
        name: &str,
        user_id: &str,
        exclude_id: Option<&str>,
    ) -> Result<bool, AppError> {
        let count: i64 = if let Some(eid) = exclude_id {
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM products WHERE name = ? AND user_id = ? AND id != ? AND deleted_at IS NULL",
            )
            .bind(name)
            .bind(user_id)
            .bind(eid)
            .fetch_one(&self.pool)
            .await?
        } else {
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM products WHERE name = ? AND user_id = ? AND deleted_at IS NULL",
            )
            .bind(name)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?
        };

        Ok(count > 0)
    }

    pub async fn create(
        &self,
        id: &str,
        user_id: &str,
        name: &str,
        price: Decimal,
        description: Option<&str>,
    ) -> Result<Product, AppError> {
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO products (id, user_id, name, price, description, created_at, updated_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(user_id)
        .bind(name)
        .bind(price)
        .bind(description)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let product = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::InternalServerError("Failed to retrieve created product".to_string()))?;

        Ok(product)
    }

    pub async fn update(
        &self,
        id: &str,
        name: Option<&str>,
        price: Option<Decimal>,
        description: Option<Option<&str>>,
    ) -> Result<Product, AppError> {
        let now = Utc::now();

        if let Some(n) = name {
            sqlx::query("UPDATE products SET name = ?, updated_at = ? WHERE id = ?")
                .bind(n)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(p) = price {
            sqlx::query("UPDATE products SET price = ?, updated_at = ? WHERE id = ?")
                .bind(p)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(d) = description {
            sqlx::query("UPDATE products SET description = ?, updated_at = ? WHERE id = ?")
                .bind(d)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        let product = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

        Ok(product)
    }

    pub async fn soft_delete(&self, id: &str) -> Result<(), AppError> {
        let now = Utc::now();

        sqlx::query("UPDATE products SET deleted_at = ?, updated_at = ? WHERE id = ?")
            .bind(now)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
