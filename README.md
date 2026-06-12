# Kasir API

Backend REST API untuk sistem Point-of-Sale (Kasir) UMKM. Dibangun dengan Rust, Axum, SQLx, dan MySQL. Mendukung autentikasi JWT, manajemen produk (termasuk gambar & stok), order dengan validasi stok, dashboard penjualan, dan pemrosesan order via suara menggunakan fuzzy matching.

## Tech Stack

| Lapisan | Teknologi |
|---|---|
| Bahasa | Rust (2021 edition) |
| Web Framework | Axum 0.8 |
| Database | MySQL (via SQLx 0.8) |
| Autentikasi | JWT (jsonwebtoken 9) |
| Hashing Password | bcrypt |
| Validasi | validator 0.18 |
| Konfigurasi | dotenvy |
| Fuzzy Matching | strsim 0.11 (Jaro-Winkler) |
| AI / Voice | Reqwest → Gemini API |

---

## Persiapan

### 1. Prasyarat

- Rust (stable)
- MySQL 8.0+

### 2. Konfigurasi

Salin `.env.example` ke `.env` dan isi nilainya:

```env
APP_HOST=127.0.0.1
APP_PORT=8000
DATABASE_URL=mysql://root:password@localhost:3306/kasir
JWT_SECRET=ganti-dengan-secret-yang-aman
AI_API_KEY=api-key-gemini-anda
AI_API_URL=https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent
```

### 3. Migrasi Database

Jalankan migration secara berurutan:

```bash
mysql -u root -p kasir < migrations/001_create_users_table.sql
mysql -u root -p kasir < migrations/002_create_products_table.sql
mysql -u root -p kasir < migrations/003_create_orders_tables.sql
mysql -u root -p kasir < migrations/004_create_feedback_table.sql
mysql -u root -p kasir < migrations/005_add_product_image_stock.sql
```

### 4. Jalankan Server

```bash
cargo run
```

Server berjalan di `http://127.0.0.1:8000`.

---

## Arsitektur

```
src/
├── config.rs
├── state.rs                  # AppState (db pool + config)
├── main.rs
├── database/
├── models/                   # Struct SQLx FromRow
│   ├── user.rs
│   ├── product.rs            # + image_url, stock
│   ├── order.rs
│   └── feedback.rs
├── dto/                      # DTO Request/Response
│   ├── auth/
│   ├── product.rs            # + image_url, stock
│   ├── order.rs
│   ├── feedback.rs
│   ├── dashboard.rs          # (BARU)
│   └── ai.rs                 # + ParseOrderRequest, MatchedOrderItem
├── repositories/             # Layer akses database
│   ├── user_repository.rs
│   ├── product_repository.rs # + find_all_active_for_user, decrement_stock
│   ├── order_repository.rs
│   ├── feedback_repository.rs
│   └── dashboard_repository.rs  # (BARU) - SQL agregat
├── services/                 # Layer business logic
│   ├── auth/
│   ├── product_service.rs    # + image_url, stock
│   ├── order_service.rs      # + validasi & kurangi stok
│   ├── feedback_services.rs
│   ├── dashboard_service.rs  # (BARU)
│   └── ai_service.rs         # + parse_order + fuzzy matching
├── handlers/                 # HTTP handlers
│   ├── auth/
│   ├── products.rs
│   ├── orders.rs
│   ├── feedback.rs
│   ├── dashboard.rs          # (BARU)
│   └── ai.rs                 # + parse_order
├── routes/                   # Registrasi route
│   ├── auth.rs
│   ├── product.rs
│   ├── order.rs
│   ├── feedback.rs
│   ├── dashboard.rs          # (BARU)
│   └── ai.rs                 # + /parse-order
├── middleware/
│   └── jwt.rs
└── errors/
    └── app_error.rs
```

---

## Format Response

Semua endpoint menggunakan format JSON yang konsisten.

**Sukses (satu item):**
```json
{
  "success": true,
  "message": "...",
  "data": { ... }
}
```

**Sukses (daftar paginasi):**
```json
{
  "success": true,
  "message": "...",
  "data": [ ... ],
  "total": 100,
  "page": 1,
  "limit": 10
}
```

**Error:**
```json
{
  "success": false,
  "message": "Deskripsi error",
  "data": null
}
```

---

## Autentikasi

Endpoint yang dilindungi membutuhkan header `Authorization`:

```
Authorization: Bearer <jwt_token>
```

Token JWT berlaku selama **7 hari**.

---

## Endpoint API

### Auth — `/api/auth`

| Method | Path | Auth | Deskripsi |
|--------|------|------|-----------|
| `POST` | `/api/auth/register` | Publik | Daftar akun baru |
| `POST` | `/api/auth/login` | Publik | Login, terima JWT |
| `POST` | `/api/auth/logout` | 🔒 JWT | Logout (hapus token di sisi klien) |
| `GET`  | `/api/auth/me` | 🔒 JWT | Profil pengguna yang login |
| `PUT`  | `/api/auth/me` | 🔒 JWT | Update profil |

---

### Produk — `/api/products` 🔒

Semua endpoint produk memerlukan JWT. User hanya bisa akses produk miliknya sendiri.

| Method | Path | Deskripsi |
|--------|------|-----------|
| `GET` | `/api/products` | Daftar produk (paginasi, cari nama) |
| `GET` | `/api/products/:id` | Detail produk |
| `POST` | `/api/products` | Buat produk baru |
| `PUT` | `/api/products/:id` | Update produk |
| `DELETE` | `/api/products/:id` | Hapus produk (soft delete) |

#### `POST /api/products`

```json
{
  "name": "Kopi Susu",
  "price": "15000.00",
  "description": "Kopi susu segar",
  "image_url": "https://example.com/kopi.jpg",
  "stock": 50
}
```

**Aturan:**
- `name`: 2–255 karakter, unik per user
- `price`: harus > 0
- `image_url`: opsional, harus URL valid
- `stock`: opsional, default 0

---

### Order — `/api/orders` 🔒

> ⚠️ **Harga dihitung server-side.** Harga dari klien diabaikan.
> ⚠️ **Stok divalidasi** sebelum order disimpan, dan **dikurangi otomatis** setelah berhasil.

| Method | Path | Deskripsi |
|--------|------|-----------|
| `GET` | `/api/orders` | Daftar order (paginasi, filter status/tanggal) |
| `GET` | `/api/orders/:id` | Detail order beserta item |
| `POST` | `/api/orders` | Buat order baru |
| `PUT` | `/api/orders/:id` | Update order |
| `DELETE` | `/api/orders/:id` | Hapus order (soft delete) |

#### `POST /api/orders`

```json
{
  "items": [
    { "product_id": "uuid", "quantity": 2 },
    { "product_id": "uuid", "quantity": 1 }
  ]
}
```

**Alur:**
1. Validasi stok tiap produk
2. Hitung `unit_price` dari database (bukan dari klien)
3. Hitung `subtotal = quantity × unit_price`
4. Hitung `total_amount = Σ subtotals`
5. Simpan order & order_items
6. Kurangi stok tiap produk

**Error stok tidak cukup (422):**
```json
{
  "success": false,
  "message": "Insufficient stock for 'Kopi Susu'. Available: 3, requested: 5",
  "data": null
}
```

---

### Feedback — `/api/feedback`

| Method | Path | Auth | Deskripsi |
|--------|------|------|-----------|
| `GET` | `/api/feedback` | Publik | Daftar feedback publik |
| `GET` | `/api/feedback/:id` | Publik | Detail feedback publik |
| `POST` | `/api/feedback` | 🔒 JWT | Kirim feedback |
| `PUT` | `/api/feedback/:id` | 🔒 JWT | Update feedback sendiri |
| `DELETE` | `/api/feedback/:id` | 🔒 JWT | Hapus feedback sendiri |

---

### Dashboard — `/api/dashboard` 🔒

Semua endpoint dashboard mengembalikan data milik user yang login. Data diambil via SQL agregasi — tidak ada load semua data ke memory.

| Method | Path | Query Params | Deskripsi |
|--------|------|--------------|-----------|
| `GET` | `/api/dashboard` | — | Ringkasan penjualan |
| `GET` | `/api/dashboard/sales` | `range=7d\|30d\|1y` | Data grafik penjualan |
| `GET` | `/api/dashboard/top-products` | `range=7d\|30d\|1y` | Produk terlaris |
| `GET` | `/api/dashboard/trends` | `range=7d\|30d\|1y` | Perbandingan pertumbuhan |

#### `GET /api/dashboard`

```json
{
  "success": true,
  "message": "Dashboard overview",
  "data": {
    "total_sales_today": "125000.00",
    "total_sales_week": "870000.00",
    "total_sales_month": "3500000.00",
    "total_orders_today": 8,
    "total_orders_week": 54,
    "total_orders_month": 210,
    "best_selling_product": {
      "product_id": "uuid",
      "product_name": "Kopi Susu",
      "total_quantity": 120,
      "total_revenue": "1800000.00"
    },
    "recent_orders_count": 210
  }
}
```

#### `GET /api/dashboard/sales?range=7d`

```json
{
  "success": true,
  "message": "Sales chart data",
  "data": {
    "range": "7d",
    "data": [
      { "label": "2026-06-06", "total_sales": "150000.00", "total_orders": 10 },
      { "label": "2026-06-07", "total_sales": "0.00", "total_orders": 0 }
    ]
  }
}
```

> Range `1y` menggunakan agregasi bulanan (`YYYY-MM`). Range lainnya agregasi harian. Hari tanpa transaksi tetap muncul dengan nilai `0`.

#### `GET /api/dashboard/top-products?range=30d`

```json
{
  "success": true,
  "message": "Top products",
  "data": [
    {
      "product_id": "uuid",
      "product_name": "Kopi Susu",
      "total_quantity": 120,
      "total_revenue": "1800000.00"
    }
  ]
}
```

#### `GET /api/dashboard/trends?range=7d`

```json
{
  "success": true,
  "message": "Sales trends",
  "data": {
    "range": "7d",
    "current_sales": "870000.00",
    "previous_sales": "650000.00",
    "sales_growth_pct": 33.85,
    "current_orders": 54,
    "previous_orders": 40,
    "order_growth_pct": 35.0,
    "sales_trend": "up",
    "order_trend": "up"
  }
}
```

---

### AI & Voice — `/api/ai` 🔒

#### `POST /api/ai/chat`

Chat umum dengan AI assistant.

```json
{ "message": "Rekomendasikan menu untuk siang hari" }
```

#### `POST /api/ai/parse-order`

**Flow Voice Order:**

```
User rekam suara
↓ FE kirim audio ke Gemini
↓ Gemini hasilkan JSON mentah:
  { "items": [{ "n": "baxo", "q": 3 }] }
↓ FE kirim JSON mentah ke endpoint ini
↓ BE fuzzy match ke produk di database
↓ BE kembalikan hasil match ke FE
↓ FE tampilkan form konfirmasi
↓ User validasi lalu POST /api/orders
```

**Request:**
```json
{
  "items": [
    { "n": "baxo", "q": 3 },
    { "n": "es teh mnis", "q": 2 }
  ]
}
```

**Response:**
```json
{
  "success": true,
  "message": "Order parsed from voice input",
  "data": {
    "items": [
      {
        "product_id": "uuid",
        "name": "Bakso",
        "input_name": "baxo",
        "quantity": 3,
        "unit_price": "15000.00",
        "confidence": 0.9333,
        "needs_confirmation": false
      },
      {
        "product_id": "uuid",
        "name": "Es Teh Manis",
        "input_name": "es teh mnis",
        "quantity": 2,
        "unit_price": "5000.00",
        "confidence": 0.9542,
        "needs_confirmation": false
      }
    ]
  }
}
```

**Aturan Fuzzy Matching:**
- Algoritma: **Jaro-Winkler** (dari crate `strsim`)
- Bonus: jika input adalah substring dari nama produk (atau sebaliknya)
- `confidence`: nilai 0.0–1.0 (4 desimal)
- `needs_confirmation: true` jika confidence < 0.80
- Daftar produk **tidak pernah dikirim ke Gemini** — Gemini hanya mengubah suara/teks manusia menjadi JSON mentah

**Setelah dikonfirmasi user, kirim POST /api/orders dengan data yang tervalidasi.**

---

## Referensi Error

| HTTP Status | AppError | Penyebab Umum |
|-------------|----------|---------------|
| `400` | `BadRequest` | Request tidak valid |
| `401` | `Unauthorized` | JWT hilang/invalid, atau password salah |
| `403` | `Forbidden` | Akses resource milik user lain |
| `404` | `NotFound` | Resource tidak ditemukan |
| `409` | `Conflict` | Email/nama produk duplikat |
| `422` | `ValidationError` | Validasi field gagal, stok tidak cukup |
| `500` | `InternalServerError` | Error database atau server |

---

## Keamanan

- Password di-hash dengan **bcrypt**
- Token JWT menggunakan **HS256** dengan secret yang dapat dikonfigurasi
- Semua operasi tulis dibatasi per-user — akses lintas user mengembalikan `403`
- Soft delete menjaga integritas data (record yang dihapus dikecualikan dari semua query)
- Harga order **selalu dihitung server-side** — total dari klien diabaikan
- Stok divalidasi atomik dengan `UPDATE ... WHERE stock >= qty`

---

## Skema Database

```sql
users
  id (PK), name, email (UNIQUE), password, description,
  created_at, updated_at, deleted_at

products
  id (PK), user_id (FK→users), name, price (DECIMAL),
  description, image_url, stock (INT),
  created_at, updated_at, deleted_at

orders
  id (PK), user_id (FK→users), total_amount (DECIMAL),
  status (TINYINT: 0=pending, 1=completed),
  created_at, updated_at, deleted_at

order_items
  id (PK), order_id (FK→orders), product_id (FK→products),
  quantity, unit_price (snapshot harga), subtotal

feedback
  id (PK), user_id (FK→users), message (TEXT),
  is_public (TINYINT), created_at, updated_at, deleted_at
```