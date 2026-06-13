# Kasir API

Backend REST API untuk sistem Point-of-Sale (Kasir) UMKM. Dibangun dengan Rust, Axum, SQLx, dan MySQL. Mendukung autentikasi JWT, manajemen produk (termasuk gambar & stok), order dengan validasi stok, dashboard penjualan, pemrosesan order via suara menggunakan fuzzy matching, dan laporan PDF penjualan bertenaga AI.

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
| PDF Generation | genpdf 0.2 |

---

## Persiapan

### 1. Prasyarat

- Rust (stable)
- MySQL 8.0+
- Font LiberationSans di direktori `./fonts/` (sudah disertakan di repo)

### 2. Konfigurasi

Salin `.env.example` ke `.env` dan isi nilainya:

```env
APP_HOST=127.0.0.1
APP_PORT=8000
DATABASE_URL=mysql://root:password@localhost:3306/kasir
JWT_SECRET=ganti-dengan-secret-yang-aman
AI_API_KEY=api-key-gemini-anda
AI_API_URL=https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent
FONT_DIR=./fonts
```

### 3. Migrasi Database

Jalankan migration secara berurutan:

```bash
mysql -u root -p kasir < migrations/001_create_users_table.sql
mysql -u root -p kasir < migrations/002_create_products_table.sql
mysql -u root -p kasir < migrations/003_create_orders_tables.sql
mysql -u root -p kasir < migrations/004_create_feedback_table.sql
mysql -u root -p kasir < migrations/005_add_product_image_stock.sql
mysql -u root -p kasir < migrations/006_add_user_address_contact.sql
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
├── state.rs                       # AppState (db pool + config)
├── main.rs
├── database/
├── models/                        # Struct SQLx FromRow
│   ├── user.rs                    # + address, contact
│   ├── product.rs                 # + image_url, stock
│   ├── order.rs
│   └── feedback.rs
├── dto/                           # DTO Request/Response
│   ├── auth/                      # + address, contact di Profile
│   ├── product.rs                 # + image_url, stock
│   ├── order.rs
│   ├── feedback.rs
│   ├── dashboard.rs
│   ├── report.rs                  # (BARU) Report & PDF
│   └── ai.rs                      # + ParseOrderRequest, MatchedOrderItem
├── repositories/                  # Layer akses database
│   ├── user_repository.rs         # + address, contact
│   ├── product_repository.rs
│   ├── order_repository.rs
│   ├── feedback_repository.rs
│   └── dashboard_repository.rs
├── services/                      # Layer business logic
│   ├── auth/                      # + address, contact
│   ├── product_service.rs
│   ├── order_service.rs
│   ├── feedback_services.rs
│   ├── dashboard_service.rs
│   ├── ai_service.rs              # + parse_order + fuzzy matching
│   ├── ai_insight_service.rs      # (BARU) Insight teks dari Gemini
│   └── report_service.rs          # (BARU) Agregasi data + PDF genpdf
├── handlers/                      # HTTP handlers
│   ├── auth/
│   ├── products.rs
│   ├── orders.rs
│   ├── feedback.rs
│   ├── dashboard.rs
│   ├── ai.rs
│   └── report_handler.rs          # (BARU) GET /reports/sales/pdf
├── routes/                        # Registrasi route
│   ├── auth.rs
│   ├── product.rs
│   ├── order.rs
│   ├── feedback.rs
│   ├── dashboard.rs
│   ├── ai.rs
│   └── report.rs                  # (BARU) /api/reports
├── middleware/
│   └── jwt.rs
└── errors/
    └── app_error.rs               # + From<genpdf::error::Error>
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

#### `GET /api/auth/me`

Response sekarang menyertakan `address` dan `contact`:

```json
{
  "success": true,
  "message": "Profile fetched successfully",
  "data": {
    "id": "uuid",
    "name": "Budi Santoso",
    "email": "budi@example.com",
    "description": "Pemilik warung makan",
    "address": "Jl. Merdeka No. 10, Jakarta",
    "contact": "+6281234567890"
  }
}
```

#### `PUT /api/auth/me`

Dapat mengupdate field berikut:

| Field | Tipe | Wajib | Keterangan |
|-------|------|-------|------------|
| `name` | string | Tidak | 2–100 karakter |
| `email` | string | Tidak | Harus email valid, tidak duplikat |
| `password` | string | Tidak | Min. 6 karakter |
| `description` | string | Tidak | Bebas |
| `address` | string | Tidak | Maks. 255 karakter — digunakan di laporan PDF |
| `contact` | string | Tidak | Maks. 100 karakter — digunakan di laporan PDF |

> ⚠️ `id`, `created_at`, dan `updated_at` **tidak dapat diubah**.

**Request:**
```json
{
  "name": "Budi Santoso",
  "address": "Jl. Merdeka No. 10, Jakarta",
  "contact": "+6281234567890"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Profile updated successfully",
  "data": {
    "id": "uuid",
    "name": "Budi Santoso",
    "email": "budi@example.com",
    "description": "...",
    "address": "Jl. Merdeka No. 10, Jakarta",
    "contact": "+6281234567890"
  }
}
```

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

**Aturan Fuzzy Matching:**
- Algoritma: **Jaro-Winkler** (dari crate `strsim`)
- `confidence`: nilai 0.0–1.0 (4 desimal)
- `needs_confirmation: true` jika confidence < 0.80
- Daftar produk **tidak pernah dikirim ke Gemini**

---

### Laporan PDF — `/api/reports` 🔒

#### `GET /api/reports/sales/pdf`

Menghasilkan laporan penjualan dalam format PDF. Data diambil dari database berdasarkan user yang login.

**Autentikasi:** Bearer JWT

**Query Parameters:**

| Parameter | Nilai | Default | Keterangan |
|-----------|-------|---------|------------|
| `range` | `7d` | ✓ | Laporan 7 hari terakhir |
| `range` | `30d` | — | Laporan 30 hari terakhir |
| `range` | `1y` | — | Laporan 1 tahun terakhir |

**Contoh Request:**

```http
GET /api/reports/sales/pdf?range=30d
Authorization: Bearer <jwt_token>
```

**Response:** `application/pdf` (file PDF langsung diunduh)

```
Content-Type: application/pdf
Content-Disposition: attachment; filename="laporan-penjualan-30d.pdf"
```

**Isi PDF (mengikuti struktur `templates/reports/sales_report.html`):**

1. **Header UMKM** — nama, alamat, dan kontak dari profil user
2. **Informasi Laporan** — periode mulai & selesai
3. **Ringkasan** — total omzet, jumlah transaksi, total item, produk terlaris
4. **Insight AI** — analisis bisnis singkat dalam bahasa Indonesia yang dihasilkan Gemini
5. **Produk Terlaris** — tabel top 10 produk berdasarkan qty terjual
6. **Detail Transaksi** — tabel semua order items dalam periode dengan total footer
7. **Tanda Tangan** — nama pemilik dan tanggal cetak

> 💡 **Tips:** Lengkapi dulu `address` dan `contact` di profil (`PUT /api/auth/me`) agar laporan PDF menampilkan informasi UMKM yang lengkap.

> 💡 **Insight AI:** Jika `AI_API_KEY` tidak dikonfigurasi, sistem akan menggunakan insight statis yang dihitung dari data penjualan aktual.

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
| `500` | `InternalServerError` | Error database, server, atau PDF generation |

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
  address VARCHAR(255),          -- BARU: alamat UMKM untuk laporan
  contact VARCHAR(100),          -- BARU: nomor telepon/WA untuk laporan
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

---

## Catatan Deployment

1. **Jalankan migration 006** sebelum start server:
   ```bash
   mysql -u root -p kasir < migrations/006_add_user_address_contact.sql
   ```

2. **Pastikan direktori `fonts/` tersedia** di working directory server:
   ```
   fonts/
   ├── LiberationSans-Regular.ttf
   ├── LiberationSans-Bold.ttf
   ├── LiberationSans-Italic.ttf
   └── LiberationSans-BoldItalic.ttf
   ```
   Atau set `FONT_DIR=/path/to/fonts` di `.env`.

3. **AI Insight bersifat opsional** — jika `AI_API_KEY` tidak di-set, laporan tetap bisa dibuat dengan insight statis berdasarkan data DB.