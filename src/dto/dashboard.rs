use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────
// Query params
// ─────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct DashboardRangeQuery {
    /// "7d" | "30d" | "1y"
    pub range: Option<String>,
}

// ─────────────────────────────────────────────
// Overview / ringkasan
// ─────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct DashboardOverviewResponse {
    pub total_sales_today: rust_decimal::Decimal,
    pub total_sales_week: rust_decimal::Decimal,
    pub total_sales_month: rust_decimal::Decimal,
    pub total_orders_today: i64,
    pub total_orders_week: i64,
    pub total_orders_month: i64,
    pub best_selling_product: Option<BestSellingProduct>,
    pub recent_orders_count: i64,
}

#[derive(Debug, Serialize)]
pub struct BestSellingProduct {
    pub product_id: String,
    pub product_name: String,
    pub total_quantity: u64,
    pub total_revenue: rust_decimal::Decimal,
}

// ─────────────────────────────────────────────
// Sales chart
// ─────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct DashboardSalesChartResponse {
    pub range: String,
    pub data: Vec<SalesDataPoint>,
}

#[derive(Debug, Serialize)]
pub struct SalesDataPoint {
    pub label: String,
    pub total_sales: rust_decimal::Decimal,
    pub total_orders: u64,
}

// ─────────────────────────────────────────────
// Top products
// ─────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DashboardTopProductResponse {
    pub product_id: String,
    pub product_name: String,
    pub total_quantity: u64,
    pub total_revenue: rust_decimal::Decimal,
}

// ─────────────────────────────────────────────
// Trends / growth
// ─────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct DashboardTrendResponse {
    pub range: String,
    pub current_sales: rust_decimal::Decimal,
    pub previous_sales: rust_decimal::Decimal,
    pub sales_growth_pct: f64,
    pub current_orders: i64,
    pub previous_orders: i64,
    pub order_growth_pct: f64,
    pub sales_trend: TrendDirection,
    pub order_trend: TrendDirection,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TrendDirection {
    Up,
    Down,
    Flat,
}
