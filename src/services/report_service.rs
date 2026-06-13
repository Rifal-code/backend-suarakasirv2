use chrono::{Duration, Utc};
use genpdf::{
    elements::{Break, LinearLayout, Paragraph, TableLayout},
    fonts,
    style::{self, Style},
    Alignment, Document, SimplePageDecorator,
};
use rust_decimal::Decimal;
use sqlx::MySqlPool;

use crate::{
    dto::report::{
        ReportData, ReportRangeQuery, ReportTopProduct, ReportTransactionItem,
        ReportTransactionRow,
    },
    errors::AppError,
    models::User,
    services::AiInsightService,
};

pub struct ReportService {
    pool: MySqlPool,
    ai_insight: AiInsightService,
}

impl ReportService {
    pub fn new(pool: MySqlPool, ai_api_key: String, ai_api_url: String) -> Self {
        Self {
            pool,
            ai_insight: AiInsightService::new(ai_api_key, ai_api_url),
        }
    }

    // ─────────────────────────────────────────────
    // Public entry point
    // ─────────────────────────────────────────────

    pub async fn generate_pdf(
        &self,
        user: &User,
        query: &ReportRangeQuery,
        font_dir: &str,
    ) -> Result<Vec<u8>, AppError> {
        let data = self.build_report_data(user, query).await?;
        let pdf_bytes = self.render_pdf(&data, font_dir)?;
        Ok(pdf_bytes)
    }

    // ─────────────────────────────────────────────
    // Data aggregation
    // ─────────────────────────────────────────────

    async fn build_report_data(
        &self,
        user: &User,
        query: &ReportRangeQuery,
    ) -> Result<ReportData, AppError> {
        let range_str = query.range.as_deref().unwrap_or("7d");
        let days: i64 = match range_str {
            "30d" => 30,
            "1y" => 365,
            _ => 7,
        };
        let tipe_laporan = match range_str {
            "30d" => "30 HARI",
            "1y" => "TAHUNAN",
            _ => "7 HARI",
        };

        let now = Utc::now();
        let since = now - Duration::days(days);

        let periode_mulai = since.format("%d/%m/%Y").to_string();
        let periode_selesai = now.format("%d/%m/%Y").to_string();

        // ── Transaction detail rows ──────────────────
        let tx_rows = sqlx::query_as::<_, ReportTransactionRow>(
            "SELECT \
               o.id AS order_id, \
               DATE_FORMAT(o.created_at, '%d/%m/%Y') AS order_date, \
               p.name AS product_name, \
               oi.quantity, \
               oi.unit_price, \
               oi.subtotal \
             FROM order_items oi \
             JOIN orders o ON o.id = oi.order_id \
             JOIN products p ON p.id = oi.product_id \
             WHERE o.user_id = ? AND o.deleted_at IS NULL AND o.created_at >= ? \
             ORDER BY o.created_at ASC, p.name ASC",
        )
        .bind(&user.id)
        .bind(since)
        .fetch_all(&self.pool)
        .await?;

        // ── Summary stats ────────────────────────────
        let total_omzet: Decimal = tx_rows.iter().map(|r| r.subtotal).sum();
        let total_qty: i64 = tx_rows.iter().map(|r| r.quantity as i64).sum();

        let jumlah_transaksi: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM orders \
             WHERE user_id = ? AND deleted_at IS NULL AND created_at >= ?",
        )
        .bind(&user.id)
        .bind(since)
        .fetch_one(&self.pool)
        .await?;

        // ── Top products ─────────────────────────────
        let top_products = sqlx::query_as::<_, ReportTopProduct>(
            "SELECT p.name AS product_name, \
                    CAST(SUM(oi.quantity) AS UNSIGNED) AS total_quantity, \
                    COALESCE(SUM(oi.subtotal), 0) AS total_revenue \
             FROM order_items oi \
             JOIN orders o ON o.id = oi.order_id \
             JOIN products p ON p.id = oi.product_id \
             WHERE o.user_id = ? AND o.deleted_at IS NULL AND o.created_at >= ? \
             GROUP BY p.name \
             ORDER BY total_quantity DESC \
             LIMIT 10",
        )
        .bind(&user.id)
        .bind(since)
        .fetch_all(&self.pool)
        .await?;

        let produk_terlaris = top_products
            .first()
            .map(|p| p.product_name.clone())
            .unwrap_or_else(|| "-".to_string());

        let top_revenue = top_products
            .first()
            .map(|p| p.total_revenue)
            .unwrap_or(Decimal::ZERO);

        // ── Growth vs previous period ────────────────
        let prev_since = since - Duration::days(days);
        let (prev_sales, prev_orders): (Decimal, i64) = {
            #[derive(sqlx::FromRow)]
            struct PrevRow { total: Option<Decimal>, cnt: i64 }
            let row = sqlx::query_as::<_, PrevRow>(
                "SELECT COALESCE(SUM(total_amount), 0) AS total, COUNT(*) AS cnt \
                 FROM orders \
                 WHERE user_id = ? AND deleted_at IS NULL \
                   AND created_at >= ? AND created_at < ?",
            )
            .bind(&user.id)
            .bind(prev_since)
            .bind(since)
            .fetch_one(&self.pool)
            .await?;
            (row.total.unwrap_or(Decimal::ZERO), row.cnt)
        };

        let sales_growth = growth_pct_decimal(prev_sales, total_omzet);
        let order_growth = growth_pct_i64(prev_orders, jumlah_transaksi);

        // ── AI insight ───────────────────────────────
        let ai_insight = self
            .ai_insight
            .generate(total_omzet, jumlah_transaksi, &produk_terlaris, top_revenue, sales_growth, order_growth)
            .await
            .unwrap_or_else(|_| "Insight AI tidak tersedia.".to_string());

        // ── Format detail rows ───────────────────────
        let detail_penjualan: Vec<ReportTransactionItem> = tx_rows
            .into_iter()
            .map(|r| ReportTransactionItem {
                tanggal: r.order_date,
                id_transaksi: shorten_id(&r.order_id),
                nama_produk: r.product_name,
                qty: r.quantity,
                harga_satuan: format_rupiah(r.unit_price),
                subtotal: format_rupiah(r.subtotal),
            })
            .collect();

        // ── UMKM info from user profile ──────────────
        let nama_umkm = user.name.clone();
        let alamat_umkm = user.address.clone().unwrap_or_else(|| "-".to_string());
        let kontak_umkm = user.contact.clone().unwrap_or_else(|| "-".to_string());

        Ok(ReportData {
            nama_umkm: nama_umkm.clone(),
            alamat_umkm,
            kontak_umkm,
            nama_pemilik: nama_umkm,
            kota_terbit: "-".to_string(),
            range: range_str.to_string(),
            periode_mulai,
            periode_selesai,
            tipe_laporan: tipe_laporan.to_string(),
            total_omzet,
            jumlah_transaksi,
            total_qty,
            produk_terlaris,
            ai_insight,
            detail_penjualan,
            top_products,
        })
    }

    // ─────────────────────────────────────────────
    // PDF rendering — follows sales_report.html layout
    // ─────────────────────────────────────────────

    fn render_pdf(&self, data: &ReportData, font_dir: &str) -> Result<Vec<u8>, AppError> {
        // Load font family
        let font_family = fonts::from_files(font_dir, "LiberationSans", None)
            .map_err(|e| AppError::InternalServerError(format!("Failed to load fonts: {}", e)))?;

        let mut doc = Document::new(font_family);
        doc.set_title(format!("Laporan Penjualan - {}", data.nama_umkm));

        let mut decorator = SimplePageDecorator::new();
        decorator.set_margins(12);
        doc.set_page_decorator(decorator);

        // ── Styles ────────────────────────────────────
        let style_title = Style::new().bold().with_font_size(16);
        let style_subtitle = Style::new().bold().with_font_size(11);
        let style_meta = Style::new().with_font_size(9);
        let style_section = Style::new().bold().with_font_size(10);
        let style_bold = Style::new().bold().with_font_size(9);
        let style_normal = Style::new().with_font_size(9);
        let style_header = Style::new().bold().with_font_size(8);
        let style_small = Style::new().with_font_size(8);

        // ── Header Section ────────────────────────────
        // Follows: <div class="header">
        doc.push(
            Paragraph::new(style::StyledString::new(
                data.nama_umkm.to_uppercase(),
                style_title,
            ))
            .aligned(Alignment::Center),
        );

        doc.push(
            Paragraph::new(style::StyledString::new(
                format!("{} | {}", data.alamat_umkm, data.kontak_umkm),
                style_meta,
            ))
            .aligned(Alignment::Center),
        );

        doc.push(
            Paragraph::new(style::StyledString::new(
                format!("LAPORAN PENJUALAN {}", data.tipe_laporan),
                style_subtitle,
            ))
            .aligned(Alignment::Center),
        );

        doc.push(
            Paragraph::new(style::StyledString::new(
                format!("Periode: {} s/d {}", data.periode_mulai, data.periode_selesai),
                style_meta,
            ))
            .aligned(Alignment::Center),
        );

        doc.push(Break::new(1));

        // ── Summary Section ───────────────────────────
        // Follows: <table class="summary">
        doc.push(
            Paragraph::new(style::StyledString::new(
                "RINGKASAN PENJUALAN",
                style_section,
            ))
        );

        let mut summary_table = TableLayout::new(vec![1, 1, 1, 1]);
        summary_table.set_cell_decorator(genpdf::elements::FrameCellDecorator::new(true, true, false));

        let summary_row = summary_table.row();
        {
            let mut r = summary_row;
            // Column 1
            let mut col1 = LinearLayout::vertical();
            col1.push(Paragraph::new(style::StyledString::new("TOTAL OMZET", style_header)).aligned(Alignment::Center));
            col1.push(Paragraph::new(style::StyledString::new(format!("Rp {}", format_rupiah(data.total_omzet)), style_bold)).aligned(Alignment::Center));
            r.push_element(col1);

            // Column 2
            let mut col2 = LinearLayout::vertical();
            col2.push(Paragraph::new(style::StyledString::new("JUMLAH TRANSAKSI", style_header)).aligned(Alignment::Center));
            col2.push(Paragraph::new(style::StyledString::new(data.jumlah_transaksi.to_string(), style_bold)).aligned(Alignment::Center));
            r.push_element(col2);

            // Column 3
            let mut col3 = LinearLayout::vertical();
            col3.push(Paragraph::new(style::StyledString::new("TOTAL ITEM", style_header)).aligned(Alignment::Center));
            col3.push(Paragraph::new(style::StyledString::new(data.total_qty.to_string(), style_bold)).aligned(Alignment::Center));
            r.push_element(col3);

            // Column 4
            let mut col4 = LinearLayout::vertical();
            col4.push(Paragraph::new(style::StyledString::new("PRODUK TERLARIS", style_header)).aligned(Alignment::Center));
            col4.push(Paragraph::new(style::StyledString::new(&data.produk_terlaris, style_bold)).aligned(Alignment::Center));
            r.push_element(col4);

            r.push()?;
        }

        doc.push(summary_table);
        doc.push(Break::new(1));

        // ── AI Insight Section ────────────────────────
        // Follows: <div class="insight">
        doc.push(Paragraph::new(style::StyledString::new("INSIGHT AI", style_section)));

        doc.push(
            Paragraph::new(style::StyledString::new(
                data.ai_insight.as_str(),
                style_normal,
            ))
        );

        doc.push(Break::new(1));

        // ── Top Products Section ──────────────────────
        if !data.top_products.is_empty() {
            doc.push(Paragraph::new(style::StyledString::new("PRODUK TERLARIS", style_section)));

            let mut top_table = TableLayout::new(vec![1, 1, 1]);
            top_table.set_cell_decorator(genpdf::elements::FrameCellDecorator::new(true, true, false));

            // Header row
            let top_header_row = top_table.row();
            {
                let mut r = top_header_row;
                r.push_element(Paragraph::new(style::StyledString::new("Nama Produk", style_bold)));
                r.push_element(Paragraph::new(style::StyledString::new("Total Qty", style_bold)).aligned(Alignment::Center));
                r.push_element(Paragraph::new(style::StyledString::new("Revenue", style_bold)).aligned(Alignment::Right));
                r.push()?;
            }

            for product in &data.top_products {
                let row = top_table.row();
                let mut r = row;
                r.push_element(Paragraph::new(style::StyledString::new(&product.product_name, style_normal)));
                r.push_element(Paragraph::new(style::StyledString::new(product.total_quantity.to_string(), style_normal)).aligned(Alignment::Center));
                r.push_element(Paragraph::new(style::StyledString::new(format!("Rp {}", format_rupiah(product.total_revenue)), style_normal)).aligned(Alignment::Right));
                r.push()?;
            }

            doc.push(top_table);
            doc.push(Break::new(1));
        }

        // ── Transaction Detail Section ────────────────
        // Follows: <div class="section-title">Detail Transaksi Penjualan</div>
        //          and <table class="table">
        doc.push(Paragraph::new(style::StyledString::new(
            "DETAIL TRANSAKSI PENJUALAN",
            style_section,
        )));

        let mut tx_table = TableLayout::new(vec![1, 3, 4, 5, 2, 4, 4]);
        tx_table.set_cell_decorator(genpdf::elements::FrameCellDecorator::new(true, true, false));

        // Table header row
        let tx_header_row = tx_table.row();
        {
            let mut r = tx_header_row;
            r.push_element(Paragraph::new(style::StyledString::new("No", style_bold)).aligned(Alignment::Center));
            r.push_element(Paragraph::new(style::StyledString::new("Tanggal", style_bold)));
            r.push_element(Paragraph::new(style::StyledString::new("ID Transaksi", style_bold)));
            r.push_element(Paragraph::new(style::StyledString::new("Nama Produk", style_bold)));
            r.push_element(Paragraph::new(style::StyledString::new("Qty", style_bold)).aligned(Alignment::Center));
            r.push_element(Paragraph::new(style::StyledString::new("Harga Satuan", style_bold)).aligned(Alignment::Right));
            r.push_element(Paragraph::new(style::StyledString::new("Subtotal", style_bold)).aligned(Alignment::Right));
            r.push()?;
        }

        for (idx, item) in data.detail_penjualan.iter().enumerate() {
            let row = tx_table.row();
            let mut r = row;
            r.push_element(Paragraph::new(style::StyledString::new((idx + 1).to_string(), style_small)).aligned(Alignment::Center));
            r.push_element(Paragraph::new(style::StyledString::new(&item.tanggal, style_small)));
            r.push_element(Paragraph::new(style::StyledString::new(&item.id_transaksi, style_small)));
            r.push_element(Paragraph::new(style::StyledString::new(&item.nama_produk, style_small)));
            r.push_element(Paragraph::new(style::StyledString::new(item.qty.to_string(), style_small)).aligned(Alignment::Center));
            r.push_element(Paragraph::new(style::StyledString::new(format!("Rp {}", item.harga_satuan), style_small)).aligned(Alignment::Right));
            r.push_element(Paragraph::new(style::StyledString::new(format!("Rp {}", item.subtotal), style_bold)).aligned(Alignment::Right));
            r.push()?;
        }

        // Footer total row
        {
            let footer_row = tx_table.row();
            let mut r = footer_row;
            r.push_element(Paragraph::new(style::StyledString::new("", style_bold)));
            r.push_element(Paragraph::new(style::StyledString::new("", style_bold)));
            r.push_element(Paragraph::new(style::StyledString::new("", style_bold)));
            r.push_element(Paragraph::new(style::StyledString::new("TOTAL", style_bold)).aligned(Alignment::Right));
            r.push_element(Paragraph::new(style::StyledString::new(data.total_qty.to_string(), style_bold)).aligned(Alignment::Center));
            r.push_element(Paragraph::new(style::StyledString::new("-", style_bold)).aligned(Alignment::Right));
            r.push_element(Paragraph::new(style::StyledString::new(format!("Rp {}", format_rupiah(data.total_omzet)), style_bold)).aligned(Alignment::Right));
            r.push()?;
        }

        doc.push(tx_table);

        // ── Signature Section ─────────────────────────
        // Follows: <div class="footer">
        doc.push(Break::new(3));
        doc.push(
            Paragraph::new(style::StyledString::new(
                format!("{}, {}", data.kota_terbit, data.tanggal_cetak()),
                style_normal,
            ))
            .aligned(Alignment::Right),
        );
        doc.push(Paragraph::new(style::StyledString::new("Mengetahui,", style_normal)).aligned(Alignment::Right));
        doc.push(Break::new(3));
        doc.push(Paragraph::new(style::StyledString::new(&data.nama_pemilik, style_bold)).aligned(Alignment::Right));
        doc.push(Paragraph::new(style::StyledString::new("Pemilik UMKM", style_small)).aligned(Alignment::Right));

        // ── Render to Vec<u8> ─────────────────────────
        let mut buf: Vec<u8> = Vec::new();
        doc.render(&mut buf)
            .map_err(|e| AppError::InternalServerError(format!("PDF render failed: {}", e)))?;

        Ok(buf)
    }
}

// ─────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────

fn format_rupiah(amount: Decimal) -> String {
    let s = amount.round().to_string();
    let parts: Vec<&str> = s.split('.').collect();
    let integer = parts[0];
    let chars: Vec<char> = integer.chars().rev().collect();
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

fn shorten_id(id: &str) -> String {
    if id.len() > 8 {
        format!("...{}", &id[id.len() - 8..])
    } else {
        id.to_string()
    }
}

fn growth_pct_decimal(prev: Decimal, current: Decimal) -> f64 {
    if prev.is_zero() {
        if current.is_zero() { return 0.0; }
        return 100.0;
    }
    let p: f64 = prev.try_into().unwrap_or(0.0);
    let c: f64 = current.try_into().unwrap_or(0.0);
    ((c - p) / p * 100.0 * 100.0).round() / 100.0
}

fn growth_pct_i64(prev: i64, current: i64) -> f64 {
    if prev == 0 {
        if current == 0 { return 0.0; }
        return 100.0;
    }
    ((current - prev) as f64 / prev as f64 * 100.0 * 100.0).round() / 100.0
}

impl ReportData {
    pub fn tanggal_cetak(&self) -> String {
        Utc::now().format("%d %B %Y").to_string()
    }
}
