use reqwest::Client;
use rust_decimal::Decimal;
use serde_json::json;

use crate::errors::AppError;

/// Generates a short Indonesian business insight using the configured Gemini API.
/// Insight text only — no PDF, no HTML, no layout.
pub struct AiInsightService {
    client: Client,
    api_key: String,
    api_url: String,
}

impl AiInsightService {
    pub fn new(api_key: String, api_url: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            api_url,
        }
    }

    /// Generate a short business insight paragraph in Indonesian.
    ///
    /// # Arguments
    /// * `total_sales`   – Omzet periode ini (Rupiah)
    /// * `total_orders`  – Jumlah transaksi periode ini
    /// * `top_product`   – Nama produk terlaris
    /// * `top_revenue`   – Revenue produk terlaris (Rupiah)
    /// * `sales_growth`  – Persentase pertumbuhan penjualan vs periode sebelumnya
    /// * `order_growth`  – Persentase pertumbuhan transaksi vs periode sebelumnya
    pub async fn generate(
        &self,
        total_sales: Decimal,
        total_orders: i64,
        top_product: &str,
        top_revenue: Decimal,
        sales_growth: f64,
        order_growth: f64,
    ) -> Result<String, AppError> {
        // If AI is not configured, return a safe static fallback
        if self.api_key.is_empty() || self.api_url.is_empty() {
            return Ok(self.fallback_insight(total_sales, total_orders, top_product, sales_growth));
        }

        let prompt = format!(
            "Kamu adalah asisten bisnis UMKM. \
             Berikan satu paragraf ringkas (2-3 kalimat) dalam bahasa Indonesia tentang kinerja penjualan berikut. \
             Jangan gunakan bullet point. Jangan tambahkan judul. Hanya teks insight saja.\n\n\
             Data:\n\
             - Total omzet: Rp {}\n\
             - Jumlah transaksi: {}\n\
             - Produk terlaris: {} (kontribusi Rp {})\n\
             - Pertumbuhan penjualan: {:.1}%\n\
             - Pertumbuhan transaksi: {:.1}%",
            format_rupiah(total_sales),
            total_orders,
            top_product,
            format_rupiah(top_revenue),
            sales_growth,
            order_growth,
        );

        let payload = json!({
            "contents": [{"parts": [{"text": prompt}]}]
        });

        let url = format!("{}?key={}", self.api_url, self.api_key);

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(format!("AI insight request failed: {}", e)))?;

        if !response.status().is_success() {
            // Non-fatal: fall back to static insight
            return Ok(self.fallback_insight(total_sales, total_orders, top_product, sales_growth));
        }

        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to parse AI response: {}", e)))?;

        let text = body["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("")
            .trim()
            .to_string();

        if text.is_empty() {
            return Ok(self.fallback_insight(total_sales, total_orders, top_product, sales_growth));
        }

        Ok(text)
    }

    /// Static fallback when AI is unavailable or not configured.
    fn fallback_insight(
        &self,
        total_sales: Decimal,
        total_orders: i64,
        top_product: &str,
        sales_growth: f64,
    ) -> String {
        let trend = if sales_growth > 0.0 {
            format!("meningkat {:.1}%", sales_growth)
        } else if sales_growth < 0.0 {
            format!("turun {:.1}%", sales_growth.abs())
        } else {
            "stabil".to_string()
        };

        format!(
            "Penjualan periode ini {} dibanding periode sebelumnya dengan total omzet Rp {} \
             dari {} transaksi. Produk terlaris adalah {}. \
             Pertahankan ketersediaan stok produk unggulan untuk menjaga momentum penjualan.",
            trend,
            format_rupiah(total_sales),
            total_orders,
            top_product,
        )
    }
}

fn format_rupiah(amount: Decimal) -> String {
    // Simple thousands separator
    let s = amount.to_string();
    let parts: Vec<&str> = s.split('.').collect();
    let integer_part = parts[0];
    let chars: Vec<char> = integer_part.chars().rev().collect();
    let grouped: String = chars
        .chunks(3)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(".")
        .chars()
        .rev()
        .collect();
    grouped
}
