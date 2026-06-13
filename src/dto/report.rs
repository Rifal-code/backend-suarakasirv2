use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────
// Query params
// ─────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ReportRangeQuery {
    /// "7d" | "30d" | "1y"
    pub range: Option<String>,
}

// ─────────────────────────────────────────────
// Report data aggregated from DB
// ─────────────────────────────────────────────

/// A single order_items row joined with order and product data.
#[derive(Debug, sqlx::FromRow)]
pub struct ReportTransactionRow {
    pub order_id: String,
    pub order_date: String,
    pub product_name: String,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub subtotal: Decimal,
}

/// Aggregated stats per product for the top-products section.
#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct ReportTopProduct {
    pub product_name: String,
    pub total_quantity: u64,
    pub total_revenue: Decimal,
}

/// Complete data bundle used by ReportService and PDF builder.
#[derive(Debug, Serialize)]
pub struct ReportData {
    // UMKM / owner info
    pub nama_umkm: String,
    pub alamat_umkm: String,
    pub kontak_umkm: String,
    pub nama_pemilik: String,
    pub kota_terbit: String,

    // Period
    pub range: String,
    pub periode_mulai: String,
    pub periode_selesai: String,
    pub tipe_laporan: String,

    // Summary stats
    pub total_omzet: Decimal,
    pub jumlah_transaksi: i64,
    pub total_qty: i64,
    pub produk_terlaris: String,

    // AI insight
    pub ai_insight: String,

    // Detail rows
    pub detail_penjualan: Vec<ReportTransactionItem>,

    // Top products table
    pub top_products: Vec<ReportTopProduct>,
}

/// A formatted detail row for PDF / response.
#[derive(Debug, Serialize)]
pub struct ReportTransactionItem {
    pub tanggal: String,
    pub id_transaksi: String,
    pub nama_produk: String,
    pub qty: i32,
    pub harga_satuan: String,
    pub subtotal: String,
}
