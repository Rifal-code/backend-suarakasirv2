use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::MySqlPool;

use crate::{
    dto::OrderItemResponse,
    errors::AppError,
    models::Order,
};

pub struct OrderRepository {
    pool: MySqlPool,
}

#[derive(sqlx::FromRow)]
struct OrderItemRow {
    id: String,
    #[allow(dead_code)]
    order_id: String,
    product_id: String,
    product_name: String,
    quantity: i32,
    unit_price: Decimal,
    subtotal: Decimal,
}

impl OrderRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn find_all_by_user(
        &self,
        user_id: &str,
        page: u32,
        limit: u32,
        status: Option<i8>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Result<(Vec<Order>, i64), AppError> {
        let offset = (page.saturating_sub(1)) * limit;

        // Build dynamic WHERE clause
        let mut conditions = vec!["user_id = ?", "deleted_at IS NULL"];
        if status.is_some() {
            conditions.push("status = ?");
        }
        if start_date.is_some() {
            conditions.push("created_at >= ?");
        }
        if end_date.is_some() {
            conditions.push("created_at <= ?");
        }
        let where_clause = conditions.join(" AND ");

        // Count query
        let total: i64 = {
            let sql = format!("SELECT COUNT(*) FROM orders WHERE {}", where_clause);
            let mut q = sqlx::query_scalar::<_, i64>(&sql).bind(user_id);
            if let Some(s) = status {
                q = q.bind(s);
            }
            if let Some(sd) = start_date {
                q = q.bind(sd);
            }
            if let Some(ed) = end_date {
                q = q.bind(ed);
            }
            q.fetch_one(&self.pool).await?
        };

        // Records query
        let orders: Vec<Order> = {
            let sql = format!(
                "SELECT id, user_id, total_amount, status, created_at, updated_at, deleted_at \
                 FROM orders WHERE {} ORDER BY created_at DESC LIMIT ? OFFSET ?",
                where_clause
            );
            let mut q = sqlx::query_as::<_, Order>(&sql).bind(user_id);
            if let Some(s) = status {
                q = q.bind(s);
            }
            if let Some(sd) = start_date {
                q = q.bind(sd);
            }
            if let Some(ed) = end_date {
                q = q.bind(ed);
            }
            q = q.bind(limit).bind(offset);
            q.fetch_all(&self.pool).await?
        };

        Ok((orders, total))
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Order>, AppError> {
        let order = sqlx::query_as::<_, Order>(
            "SELECT id, user_id, total_amount, status, created_at, updated_at, deleted_at \
             FROM orders WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(order)
    }

    pub async fn find_items_by_order(&self, order_id: &str) -> Result<Vec<OrderItemResponse>, AppError> {
        let rows = sqlx::query_as::<_, OrderItemRow>(
            "SELECT oi.id, oi.order_id, oi.product_id, p.name as product_name, \
             oi.quantity, oi.unit_price, oi.subtotal \
             FROM order_items oi \
             JOIN products p ON p.id = oi.product_id \
             WHERE oi.order_id = ?",
        )
        .bind(order_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| OrderItemResponse {
                id: r.id,
                product_id: r.product_id,
                product_name: r.product_name,
                quantity: r.quantity,
                unit_price: r.unit_price,
                subtotal: r.subtotal,
            })
            .collect())
    }

    pub async fn create(
        &self,
        id: &str,
        user_id: &str,
        total_amount: Decimal,
        items: &[(String, String, i32, Decimal, Decimal)],
    ) -> Result<Order, AppError> {
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO orders (id, user_id, total_amount, status, created_at, updated_at) \
             VALUES (?, ?, ?, 0, ?, ?)",
        )
        .bind(id)
        .bind(user_id)
        .bind(total_amount)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        for (item_id, product_id, qty, unit_price, subtotal) in items {
            sqlx::query(
                "INSERT INTO order_items (id, order_id, product_id, quantity, unit_price, subtotal) \
                 VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(item_id)
            .bind(id)
            .bind(product_id)
            .bind(qty)
            .bind(unit_price)
            .bind(subtotal)
            .execute(&self.pool)
            .await?;
        }

        let order = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::InternalServerError("Failed to retrieve created order".to_string()))?;

        Ok(order)
    }

    pub async fn update(
        &self,
        id: &str,
        total_amount: Decimal,
        status: Option<i8>,
        items: &[(String, String, i32, Decimal, Decimal)],
    ) -> Result<Order, AppError> {
        let now = Utc::now();

        if let Some(s) = status {
            sqlx::query(
                "UPDATE orders SET total_amount = ?, status = ?, updated_at = ? WHERE id = ?",
            )
            .bind(total_amount)
            .bind(s)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;
        } else {
            sqlx::query("UPDATE orders SET total_amount = ?, updated_at = ? WHERE id = ?")
                .bind(total_amount)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        // Delete old items and reinsert
        sqlx::query("DELETE FROM order_items WHERE order_id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        for (item_id, product_id, qty, unit_price, subtotal) in items {
            sqlx::query(
                "INSERT INTO order_items (id, order_id, product_id, quantity, unit_price, subtotal) \
                 VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(item_id)
            .bind(id)
            .bind(product_id)
            .bind(qty)
            .bind(unit_price)
            .bind(subtotal)
            .execute(&self.pool)
            .await?;
        }

        let order = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Order not found".to_string()))?;

        Ok(order)
    }

    pub async fn soft_delete(&self, id: &str) -> Result<(), AppError> {
        let now = Utc::now();

        sqlx::query("UPDATE orders SET deleted_at = ?, updated_at = ? WHERE id = ?")
            .bind(now)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
