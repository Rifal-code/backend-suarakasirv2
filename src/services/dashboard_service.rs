use chrono::{Duration, Utc};
use rust_decimal::Decimal;

use crate::{
    dto::dashboard::{
        DashboardOverviewResponse, DashboardRangeQuery, DashboardSalesChartResponse,
        DashboardTopProductResponse, DashboardTrendResponse, TrendDirection,
    },
    errors::AppError,
    repositories::DashboardRepository,
};

pub struct DashboardService {
    repo: DashboardRepository,
}

impl DashboardService {
    pub fn new(repo: DashboardRepository) -> Self {
        Self { repo }
    }

    pub async fn overview(&self, user_id: &str) -> Result<DashboardOverviewResponse, AppError> {
        let now = Utc::now();
        let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap_or_default();
        let today_start_utc = chrono::DateTime::<Utc>::from_naive_utc_and_offset(today_start, Utc);

        let week_start_date = now.date_naive()
            .checked_sub_days(chrono::Days::new(7))
            .unwrap_or(now.date_naive());
        let week_start = week_start_date
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();

        let month_start_date = now.date_naive()
            .checked_sub_days(chrono::Days::new(30))
            .unwrap_or(now.date_naive());
        let month_start = month_start_date
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();

        let (total_sales_today, total_orders_today) = self
            .repo
            .sales_and_orders_since(user_id, today_start_utc)
            .await?;

        let (total_sales_week, total_orders_week) = self
            .repo
            .sales_and_orders_since(user_id, week_start)
            .await?;

        let (total_sales_month, total_orders_month) = self
            .repo
            .sales_and_orders_since(user_id, month_start)
            .await?;

        let best_selling_product = self.repo.best_selling_product(user_id).await?;
        let recent_orders_count = self.repo.recent_orders_count(user_id).await?;

        Ok(DashboardOverviewResponse {
            total_sales_today,
            total_sales_week,
            total_sales_month,
            total_orders_today,
            total_orders_week,
            total_orders_month,
            best_selling_product,
            recent_orders_count,
        })
    }

    pub async fn sales_chart(
        &self,
        user_id: &str,
        query: &DashboardRangeQuery,
    ) -> Result<DashboardSalesChartResponse, AppError> {
        let range = query.range.as_deref().unwrap_or("7d");
        let (label, data) = match range {
            "30d" => {
                let data = self.repo.sales_chart(user_id, 30).await?;
                ("30d".to_string(), data)
            }
            "1y" => {
                let data = self.repo.sales_chart_monthly(user_id).await?;
                ("1y".to_string(), data)
            }
            _ => {
                let data = self.repo.sales_chart(user_id, 7).await?;
                ("7d".to_string(), data)
            }
        };

        Ok(DashboardSalesChartResponse { range: label, data })
    }

    pub async fn top_products(
        &self,
        user_id: &str,
        query: &DashboardRangeQuery,
    ) -> Result<Vec<DashboardTopProductResponse>, AppError> {
        let days = range_to_days(query.range.as_deref());
        self.repo.top_products(user_id, days, 10).await
    }

    pub async fn trends(
        &self,
        user_id: &str,
        query: &DashboardRangeQuery,
    ) -> Result<DashboardTrendResponse, AppError> {
        let range = query.range.as_deref().unwrap_or("7d");
        let days = range_to_days(Some(range)) as i64;

        let now = Utc::now();
        let current_start = now - Duration::days(days);
        let previous_start = current_start - Duration::days(days);

        let (current_sales, current_orders) = self
            .repo
            .period_summary(user_id, current_start, now)
            .await?;

        let (previous_sales, previous_orders) = self
            .repo
            .period_summary(user_id, previous_start, current_start)
            .await?;

        let sales_growth_pct = growth_pct(previous_sales, current_sales);
        let order_growth_pct = growth_pct_i64(previous_orders, current_orders);

        Ok(DashboardTrendResponse {
            range: range.to_string(),
            current_sales,
            previous_sales,
            sales_growth_pct,
            current_orders,
            previous_orders,
            order_growth_pct,
            sales_trend: trend_direction(sales_growth_pct),
            order_trend: trend_direction(order_growth_pct),
        })
    }
}

// ─────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────

fn range_to_days(range: Option<&str>) -> u32 {
    match range {
        Some("30d") => 30,
        Some("1y") => 365,
        _ => 7,
    }
}

fn growth_pct(previous: Decimal, current: Decimal) -> f64 {
    if previous.is_zero() {
        if current.is_zero() {
            return 0.0;
        }
        return 100.0;
    }
    let prev_f: f64 = previous.try_into().unwrap_or(0.0);
    let curr_f: f64 = current.try_into().unwrap_or(0.0);
    ((curr_f - prev_f) / prev_f * 100.0 * 100.0).round() / 100.0
}

fn growth_pct_i64(previous: i64, current: i64) -> f64 {
    if previous == 0 {
        if current == 0 {
            return 0.0;
        }
        return 100.0;
    }
    ((current - previous) as f64 / previous as f64 * 100.0 * 100.0).round() / 100.0
}

fn trend_direction(pct: f64) -> TrendDirection {
    if pct > 1.0 {
        TrendDirection::Up
    } else if pct < -1.0 {
        TrendDirection::Down
    } else {
        TrendDirection::Flat
    }
}
