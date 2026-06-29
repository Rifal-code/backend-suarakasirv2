use chrono::{DateTime, Datelike, Utc};
use rust_decimal::Decimal;
use sqlx::MySqlPool;

use crate::{
    dto::dashboard::{
        BestSellingProduct, DashboardTopProductResponse, SalesDataPoint,
    },
    errors::AppError,
};

fn start_of_day_n_days_ago(days: u64) -> DateTime<Utc> {
    let today = Utc::now().date_naive();
    let target = today
        .checked_sub_days(chrono::Days::new(days))
        .unwrap_or(today);
    target.and_hms_opt(0, 0, 0).unwrap().and_utc()
}

#[derive(sqlx::FromRow)]
struct SumRow {
    total: Option<Decimal>,
    cnt: i64,
}

#[derive(sqlx::FromRow)]
struct BestProductRow {
    product_id: String,
    product_name: String,
    total_quantity: u64,
    total_revenue: Option<Decimal>,
}

pub struct DashboardRepository {
    pool: MySqlPool,
}

impl DashboardRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    // ─────────────────────────────────────────────
    // Overview helpers
    // ─────────────────────────────────────────────

    pub async fn sales_and_orders_since(
        &self,
        user_id: &str,
        since: DateTime<Utc>,
    ) -> Result<(Decimal, i64), AppError> {
        let row = sqlx::query_as::<_, SumRow>(
            "SELECT COALESCE(SUM(total_amount), 0) AS total, COUNT(*) AS cnt \
             FROM orders \
             WHERE user_id = ? AND deleted_at IS NULL AND created_at >= ?",
        )
        .bind(user_id)
        .bind(since)
        .fetch_one(&self.pool)
        .await?;

        Ok((row.total.unwrap_or(Decimal::ZERO), row.cnt))
    }

    pub async fn best_selling_product(
        &self,
        user_id: &str,
    ) -> Result<Option<BestSellingProduct>, AppError> {
        let row = sqlx::query_as::<_, BestProductRow>(
            "SELECT oi.product_id, p.name AS product_name, \
                    CAST(SUM(oi.quantity) AS UNSIGNED) AS total_quantity, \
                    SUM(oi.subtotal) AS total_revenue \
             FROM order_items oi \
             JOIN orders o ON o.id = oi.order_id \
             JOIN products p ON p.id = oi.product_id \
             WHERE o.user_id = ? AND o.deleted_at IS NULL \
             GROUP BY oi.product_id, p.name \
             ORDER BY total_quantity DESC \
             LIMIT 1",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| BestSellingProduct {
            product_id: r.product_id,
            product_name: r.product_name,
            total_quantity: r.total_quantity,
            total_revenue: r.total_revenue.unwrap_or(Decimal::ZERO),
        }))
    }

    pub async fn recent_orders_count(&self, user_id: &str) -> Result<i64, AppError> {
        let since = start_of_day_n_days_ago(30);
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM orders \
             WHERE user_id = ? AND deleted_at IS NULL AND created_at >= ?",
        )
        .bind(user_id)
        .bind(since)
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    // ─────────────────────────────────────────────
    // Sales chart — daily aggregation
    // ─────────────────────────────────────────────

    pub async fn sales_chart(
        &self,
        user_id: &str,
        days: u32,
    ) -> Result<Vec<SalesDataPoint>, AppError> {
        // Generate a series of date labels and join with aggregate
        // MySQL doesn't have generate_series so we query the range and group by DATE
        let since = start_of_day_n_days_ago(days as u64);

        #[derive(sqlx::FromRow)]
        struct DayRow {
            day_label: String,
            total_sales: Option<Decimal>,
            total_orders: i64,
        }

        let rows = sqlx::query_as::<_, DayRow>(
            "SELECT DATE_FORMAT(created_at, '%Y-%m-%d') AS day_label, \
                    COALESCE(SUM(total_amount), 0) AS total_sales, \
                    COUNT(*) AS total_orders \
             FROM orders \
             WHERE user_id = ? AND deleted_at IS NULL AND created_at >= ? \
             GROUP BY day_label \
             ORDER BY day_label ASC",
        )
        .bind(user_id)
        .bind(since)
        .fetch_all(&self.pool)
        .await?;

        // Fill in missing days with zero
        let mut result: Vec<SalesDataPoint> = Vec::new();
        let start_date = since.date_naive();
        let today = Utc::now().date_naive();

        let mut cursor = start_date;
        while cursor <= today {
            let label = cursor.format("%Y-%m-%d").to_string();
            if let Some(row) = rows.iter().find(|r| r.day_label == label) {
                result.push(SalesDataPoint {
                    label: label.clone(),
                    total_sales: row.total_sales.unwrap_or(Decimal::ZERO),
                    total_orders: row.total_orders as u64,
                });
            } else {
                result.push(SalesDataPoint {
                    label,
                    total_sales: Decimal::ZERO,
                    total_orders: 0,
                });
            }
            cursor = cursor
                .checked_add_days(chrono::Days::new(1))
                .unwrap_or(today);
        }

        Ok(result)
    }

    /// Monthly aggregation for "1y" range
    pub async fn sales_chart_monthly(
        &self,
        user_id: &str,
    ) -> Result<Vec<SalesDataPoint>, AppError> {
        let since = start_of_day_n_days_ago(365);

        #[derive(sqlx::FromRow)]
        struct MonthRow {
            month_label: String,
            total_sales: Option<Decimal>,
            total_orders: i64,
        }

        let rows = sqlx::query_as::<_, MonthRow>(
            "SELECT DATE_FORMAT(created_at, '%Y-%m') AS month_label, \
                    COALESCE(SUM(total_amount), 0) AS total_sales, \
                    COUNT(*) AS total_orders \
             FROM orders \
             WHERE user_id = ? AND deleted_at IS NULL AND created_at >= ? \
             GROUP BY month_label \
             ORDER BY month_label ASC",
        )
        .bind(user_id)
        .bind(since)
        .fetch_all(&self.pool)
        .await?;

        // Fill missing months
        let mut result: Vec<SalesDataPoint> = Vec::new();
        let now = Utc::now();
        let mut cursor_year = now.year();
        let mut cursor_month = now.month();

        // Go back 11 months
        let mut months: Vec<String> = Vec::new();
        for _ in 0..12 {
            months.push(format!("{}-{:02}", cursor_year, cursor_month));
            if cursor_month == 1 {
                cursor_month = 12;
                cursor_year -= 1;
            } else {
                cursor_month -= 1;
            }
        }
        months.reverse();

        for label in months {
            if let Some(row) = rows.iter().find(|r| r.month_label == label) {
                result.push(SalesDataPoint {
                    label: label.clone(),
                    total_sales: row.total_sales.unwrap_or(Decimal::ZERO),
                    total_orders: row.total_orders as u64,
                });
            } else {
                result.push(SalesDataPoint {
                    label,
                    total_sales: Decimal::ZERO,
                    total_orders: 0,
                });
            }
        }

        Ok(result)
    }

    // ─────────────────────────────────────────────
    // Top products
    // ─────────────────────────────────────────────

    pub async fn top_products(
        &self,
        user_id: &str,
        days: u32,
        limit: u32,
    ) -> Result<Vec<DashboardTopProductResponse>, AppError> {
        let since = start_of_day_n_days_ago(days as u64);

        let rows = sqlx::query_as::<_, DashboardTopProductResponse>(
            "SELECT oi.product_id, p.name AS product_name, \
                    CAST(SUM(oi.quantity) AS UNSIGNED) AS total_quantity, \
                    COALESCE(SUM(oi.subtotal), 0) AS total_revenue \
             FROM order_items oi \
             JOIN orders o ON o.id = oi.order_id \
             JOIN products p ON p.id = oi.product_id \
             WHERE o.user_id = ? AND o.deleted_at IS NULL AND o.created_at >= ? \
             GROUP BY oi.product_id, p.name \
             ORDER BY total_quantity DESC \
             LIMIT ?",
        )
        .bind(user_id)
        .bind(since)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    // ─────────────────────────────────────────────
    // Trends comparison
    // ─────────────────────────────────────────────

    pub async fn period_summary(
        &self,
        user_id: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<(Decimal, i64), AppError> {
        let row = sqlx::query_as::<_, SumRow>(
            "SELECT COALESCE(SUM(total_amount), 0) AS total, COUNT(*) AS cnt \
             FROM orders \
             WHERE user_id = ? AND deleted_at IS NULL \
               AND created_at >= ? AND created_at < ?",
        )
        .bind(user_id)
        .bind(from)
        .bind(to)
        .fetch_one(&self.pool)
        .await?;

        Ok((row.total.unwrap_or(Decimal::ZERO), row.cnt))
    }
}
