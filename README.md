<div align="center">
  <h1>🧾 Suara Kasir</h1>
  <p><strong>Backend API Point-of-Sale untuk UMKM Indonesia, bertenaga Rust & Generative AI</strong></p>

  <p>
    <img alt="Rust" src="https://img.shields.io/badge/Rust-2021%20Edition-orange?logo=rust&logoColor=white" />
    <img alt="Axum" src="https://img.shields.io/badge/Axum-0.8-blue?logo=rust" />
    <img alt="MySQL" src="https://img.shields.io/badge/MySQL-8.0+-4479A1?logo=mysql&logoColor=white" />
    <img alt="SQLx" src="https://img.shields.io/badge/SQLx-0.8-lightgrey" />
    <img alt="JWT" src="https://img.shields.io/badge/Auth-JWT-yellow?logo=jsonwebtokens" />
    <img alt="Gemini AI" src="https://img.shields.io/badge/AI-Gemini%20API-8E75B2?logo=google&logoColor=white" />
    <img alt="License" src="https://img.shields.io/badge/License-MIT-green" />
  </p>

  <p>
    <a href="./api-docs.md">📖 Dokumentasi API Lengkap</a>
    ·
    <a href="#-persiapan">🚀 Quick Start</a>
    ·
    <a href="#-arsitektur">🏗️ Arsitektur</a>
    ·
    <a href="#-fitur-unggulan">✨ Fitur</a>
  </p>
</div>

---

<details>
<summary><strong>📑 Daftar Isi (klik untuk buka)</strong></summary>

- [Latar Belakang](#-latar-belakang)
- [Permasalahan](#-permasalahan)
- [Peluang dengan Generative AI](#-peluang-dengan-generative-ai)
- [Tantangan IDCamp Developer Challenge](#-tantangan-idcamp-developer-challenge)
- [Fitur Unggulan](#-fitur-unggulan)
- [Tech Stack](#-tech-stack)
- [Persiapan](#-persiapan)
  - [Prasyarat](#1-prasyarat)
  - [Konfigurasi Environment](#2-konfigurasi-environment)
  - [Migrasi Database](#3-migrasi-database)
  - [Jalankan Server](#4-jalankan-server)
- [Arsitektur](#-arsitektur)
- [Endpoint API](#-endpoint-api)
- [Format Response](#-format-response)
- [Keamanan](#-keamanan)
- [Skema Database](#-skema-database)
- [Catatan Deployment](#-catatan-deployment)

</details>

---

## 🇮🇩 Latar Belakang

Indonesia memiliki lebih dari **64 juta pelaku UMKM** yang secara kolektif menyumbang **60% PDB nasional** dan menjadi penyerap utama tenaga kerja di hampir seluruh wilayah. Angka ini menempatkan UMKM bukan sekadar sektor ekonomi, melainkan fondasi sosial yang menopang kehidupan jutaan keluarga Indonesia.

Namun di balik besarnya skala tersebut, ada kenyataan yang sulit diabaikan: sebagian besar pelaku UMKM masih beroperasi dengan cara-cara yang belum berubah selama bertahun-tahun — pencatatan manual, pemasaran dari mulut ke mulut, dan pengelolaan bisnis yang sepenuhnya bergantung pada intuisi pemilik.

Transformasi digital yang terus digaungkan nyatanya belum menyentuh mayoritas dari mereka. Hanya **12% UMKM** yang benar-benar berhasil mengintegrasikan teknologi ke dalam operasional bisnisnya. Bukan karena tidak mau, tetapi karena solusi yang tersedia seringkali terlalu rumit, terlalu mahal, atau tidak dirancang dengan memahami realita lapangan yang mereka hadapi setiap hari.

---

## ⚠️ Permasalahan

Tantangan utama yang dihadapi pelaku UMKM dalam era digital bukan hanya soal akses terhadap teknologi, melainkan kemampuan untuk memanfaatkannya secara efektif.

- **44%** pelaku UMKM belum memahami cara menggunakan iklan digital
- **60%** mengeluhkan persaingan harga yang tidak seimbang di platform marketplace
- Pencatatan keuangan masih dilakukan secara **manual**
- Ketidakmampuan merespons pelanggan secara cepat melalui kanal digital
- Banyak produk lokal berkualitas **gagal menembus pasar** yang lebih luas karena kendala bahasa dan literasi platform

Akibatnya, potensi besar UMKM Indonesia kerap tidak terealisasi secara optimal, dan transformasi digital yang seharusnya menjadi pendorong pertumbuhan justru terasa jauh dari jangkauan.

---

## 🤖 Peluang dengan Generative AI

Di sinilah **Generative AI** hadir sebagai solusi yang relevan dan demokratis. Teknologi ini mampu mengotomasi berbagai proses yang selama ini membutuhkan keahlian khusus:

- 📝 Pembuatan konten pemasaran otomatis
- 📊 Pencatatan dan pelaporan keuangan via teks atau suara
- 📈 Analisis tren penjualan dengan narasi yang mudah dipahami
- 🛒 Pemrosesan order real-time melalui kanal digital

Tren ini sudah mulai terlihat nyata: **31% UMKM Indonesia** kini sudah mulai menggunakan AI tools untuk kebutuhan konten dan layanan pelanggan. Generative AI membuka peluang bagi siapa saja untuk bersaing secara lebih setara di ekosistem digital — asalkan solusi yang dibangun benar-benar dirancang untuk pengguna non-teknis, tersedia dalam bahasa Indonesia, dan dapat berjalan di perangkat mobile dengan koneksi internet yang terbatas.

---

## 🏆 Tantangan IDCamp Developer Challenge

Dalam kompetisi **IDCamp Developer Challenge**, peserta ditantang untuk membangun solusi berbasis Generative AI yang menjawab *pain point* nyata pelaku UMKM Indonesia. Solusi yang dikembangkan dapat mencakup:

| Area | Deskripsi |
|---|---|
| 🎙️ Voice Order | Input transaksi via suara, dikonversi ke order oleh AI |
| 📊 Dashboard Analitik | Narasi AI yang membantu memahami tren penjualan |
| 📄 Laporan Otomatis | Generate laporan PDF dengan insight bisnis dari Gemini |
| 🤝 Asisten Keuangan | Pencatatan dan rekapitulasi transaksi yang sederhana |

**Kasir UMKM** adalah jawaban atas tantangan tersebut — sebuah backend API yang menggabungkan kekuatan Rust untuk performa tinggi dengan Gemini AI untuk kecerdasan bisnis yang inklusif.

> _Melalui challenge ini diharapkan dapat melahirkan solusi-solusi inovatif yang tidak hanya unggul secara teknis, tetapi juga berakar pada pemahaman mendalam tentang konteks dan kebutuhan nyata pelaku UMKM Indonesia._

---

## ✨ Fitur Unggulan

| Fitur | Deskripsi |
|---|---|
| 🔐 **Autentikasi JWT** | Register, login, dan manajemen profil lengkap dengan `address` & `contact` |
| 📦 **Manajemen Produk** | CRUD produk dengan gambar, stok, dan validasi per-user |
| 🛒 **Order & Stok** | Pembuatan order dengan validasi stok atomik dan harga server-side |
| 🎙️ **Voice Order AI** | Input order via suara — fuzzy matching Jaro-Winkler ke produk database |
| 📊 **Dashboard Penjualan** | Overview, grafik harian/bulanan, top produk, dan analisis tren pertumbuhan |
| 📄 **Laporan PDF** | Generate PDF laporan penjualan dengan insight AI dari Gemini |
| 💬 **AI Chat** | Asisten bisnis umum berbahasa Indonesia via Gemini API |
| 📣 **Feedback System** | Kirim dan kelola feedback publik/privat |

---

## 🛠️ Tech Stack

| Lapisan | Teknologi |
|---|---|
| Bahasa | Rust (2021 edition) |
| Web Framework | Axum 0.8 |
| Database | MySQL 8.0+ via SQLx 0.8 |
| Autentikasi | JWT — `jsonwebtoken 9` dengan HS256 |
| Hashing Password | `bcrypt` |
| Validasi Input | `validator 0.18` |
| Konfigurasi | `dotenvy` |
| Fuzzy Matching | `strsim 0.11` — algoritma Jaro-Winkler |
| AI / Voice | `reqwest` → Gemini API |
| PDF Generation | `genpdf 0.2` — pure Rust, tanpa binary eksternal |
| Error Handling | `thiserror` |
| Logging | `tracing` + `tracing-subscriber` |

---

## 🚀 Persiapan

### 1. Prasyarat

- [Rust](https://rustup.rs/) (stable)
- MySQL 8.0+
- Font `LiberationSans` di direktori `./fonts/` _(sudah disertakan di repo)_

### 2. Konfigurasi Environment

Salin `.env.example` ke `.env` lalu isi nilainya:

```env
APP_HOST=127.0.0.1
APP_PORT=8000
DATABASE_URL=mysql://root:password@localhost:3306/kasir
JWT_SECRET=ganti-dengan-secret-yang-aman-minimal-32-karakter
AI_API_KEY=api-key-gemini-anda
AI_API_URL=https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent
FONT_DIR=./fonts
```

> [!NOTE]
> `AI_API_KEY` bersifat opsional. Jika tidak diset, laporan PDF tetap berfungsi dengan insight statis yang dihitung dari data penjualan aktual.

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

Server berjalan di `http://127.0.0.1:8000` 🎉

---

## 🏗️ Arsitektur

Proyek mengikuti arsitektur berlapis yang ketat dengan pemisahan tanggung jawab yang jelas:

```
src/
├── config.rs                      # Konfigurasi environment
├── state.rs                       # AppState — db pool + config
├── main.rs                        # Entry point & router setup
│
├── models/                        # Struct database (SQLx FromRow)
│   ├── user.rs                    # + address, contact
│   ├── product.rs                 # + image_url, stock
│   ├── order.rs
│   └── feedback.rs
│
├── dto/                           # Data Transfer Objects
│   ├── auth/                      # Register, Login, Profile (+ address, contact)
│   ├── product.rs
│   ├── order.rs
│   ├── feedback.rs
│   ├── dashboard.rs
│   ├── report.rs                  # ReportData, ReportRangeQuery
│   └── ai.rs                      # AiChat, ParseOrder, MatchedOrderItem
│
├── repositories/                  # Layer akses database (SQL murni)
│   ├── user_repository.rs
│   ├── product_repository.rs
│   ├── order_repository.rs
│   ├── feedback_repository.rs
│   └── dashboard_repository.rs    # Agregasi SQL berbasis hari
│
├── services/                      # Layer business logic
│   ├── auth/
│   ├── product_service.rs
│   ├── order_service.rs           # Validasi stok atomik
│   ├── feedback_services.rs
│   ├── dashboard_service.rs
│   ├── ai_service.rs              # Chat + fuzzy matching voice order
│   ├── ai_insight_service.rs      # Generate insight teks via Gemini
│   └── report_service.rs          # Agregasi data + render PDF (genpdf)
│
├── handlers/                      # HTTP request handlers
│   ├── auth/
│   ├── products.rs
│   ├── orders.rs
│   ├── feedback.rs
│   ├── dashboard.rs
│   ├── ai.rs
│   └── report_handler.rs          # GET /reports/sales/pdf
│
├── routes/                        # Registrasi route + middleware
│   ├── auth.rs
│   ├── product.rs
│   ├── order.rs
│   ├── feedback.rs
│   ├── dashboard.rs
│   ├── ai.rs
│   └── report.rs
│
├── middleware/
│   └── jwt.rs                     # JWT extractor + Claims
│
└── errors/
    └── app_error.rs               # AppError enum + IntoResponse
```

---

## 📡 Endpoint API

Dokumentasi API lengkap dengan contoh request dan response tersedia di:

**[📖 api-docs.md](./api-docs.md)**

Berikut ringkasan semua endpoint yang tersedia:

### Auth — `/api/auth`

| Method | Path | Auth | Deskripsi |
|---|---|---|---|
| `POST` | `/api/auth/register` | Publik | Daftar akun baru |
| `POST` | `/api/auth/login` | Publik | Login, terima JWT |
| `POST` | `/api/auth/logout` | 🔒 JWT | Logout |
| `GET` | `/api/auth/me` | 🔒 JWT | Lihat profil (+ address & contact) |
| `PUT` | `/api/auth/me` | 🔒 JWT | Update profil |

### Produk — `/api/products` 🔒

| Method | Path | Deskripsi |
|---|---|---|
| `GET` | `/api/products` | Daftar produk (paginasi, search) |
| `GET` | `/api/products/:id` | Detail produk |
| `POST` | `/api/products` | Buat produk baru |
| `PUT` | `/api/products/:id` | Update produk |
| `DELETE` | `/api/products/:id` | Hapus produk (soft delete) |

### Order — `/api/orders` 🔒

| Method | Path | Deskripsi |
|---|---|---|
| `GET` | `/api/orders` | Daftar order (filter status & tanggal) |
| `GET` | `/api/orders/:id` | Detail order beserta items |
| `POST` | `/api/orders` | Buat order baru |
| `PUT` | `/api/orders/:id` | Update order |
| `DELETE` | `/api/orders/:id` | Hapus order (soft delete) |

### Feedback — `/api/feedback`

| Method | Path | Auth | Deskripsi |
|---|---|---|---|
| `GET` | `/api/feedback` | Publik | Daftar feedback publik |
| `GET` | `/api/feedback/:id` | Publik | Detail feedback |
| `POST` | `/api/feedback` | 🔒 JWT | Kirim feedback |
| `PUT` | `/api/feedback/:id` | 🔒 JWT | Update feedback sendiri |
| `DELETE` | `/api/feedback/:id` | 🔒 JWT | Hapus feedback sendiri |

### Dashboard — `/api/dashboard` 🔒

| Method | Path | Query | Deskripsi |
|---|---|---|---|
| `GET` | `/api/dashboard` | — | Ringkasan penjualan |
| `GET` | `/api/dashboard/sales` | `range=7d\|30d\|1y` | Grafik penjualan |
| `GET` | `/api/dashboard/top-products` | `range=7d\|30d\|1y` | Produk terlaris |
| `GET` | `/api/dashboard/trends` | `range=7d\|30d\|1y` | Tren pertumbuhan |

### AI & Voice — `/api/ai` 🔒

| Method | Path | Deskripsi |
|---|---|---|
| `POST` | `/api/ai/chat` | Chat dengan AI assistant |
| `POST` | `/api/ai/parse-order` | Parse order dari input suara (fuzzy matching) |

### Laporan PDF — `/api/reports` 🔒

| Method | Path | Query | Deskripsi |
|---|---|---|---|
| `GET` | `/api/reports/sales/pdf` | `range=7d\|30d\|1y` | Download laporan PDF bertenaga AI |

---

## 📋 Format Response

Semua endpoint menggunakan format JSON yang konsisten:

<details>
<summary>Sukses — single item</summary>

```json
{
    "success": true,
    "message": "...",
    "data": { }
}
```

</details>

<details>
<summary>Sukses — list & paginasi</summary>

```json
{
    "success": true,
    "message": "...",
    "data": [ ],
    "total": 100,
    "page": 1,
    "limit": 10
}
```

</details>

<details>
<summary>Error</summary>

```json
{
    "success": false,
    "message": "Deskripsi error",
    "data": null
}
```

| HTTP Status | Penyebab |
|---|---|
| `400` | Request tidak valid |
| `401` | JWT hilang atau tidak valid |
| `403` | Akses ke resource milik user lain |
| `404` | Resource tidak ditemukan |
| `409` | Duplikasi data (email, nama produk) |
| `422` | Validasi field gagal, stok tidak cukup |
| `500` | Internal server error |

</details>

---

## 🔒 Keamanan

- Password di-hash dengan **bcrypt** sebelum disimpan ke database
- Token JWT menggunakan **HS256** dengan secret yang dikonfigurasi via env
- Semua operasi tulis dibatasi per-user — akses lintas user mengembalikan `403 Forbidden`
- **Soft delete** menjaga integritas data referensial
- Harga order **selalu dihitung server-side** — harga dari klien diabaikan sepenuhnya
- Stok divalidasi atomik: `UPDATE ... WHERE stock >= qty`
- Filter dashboard & laporan selalu di-scope ke `user_id` dari JWT

---

## 🗄️ Skema Database

```sql
users
  id, name, email (UNIQUE), password, description,
  address VARCHAR(255),   -- untuk header laporan PDF
  contact VARCHAR(100),   -- untuk header laporan PDF
  created_at, updated_at, deleted_at

products
  id, user_id (FK), name, price DECIMAL,
  description, image_url, stock INT,
  created_at, updated_at, deleted_at

orders
  id, user_id (FK), total_amount DECIMAL,
  status TINYINT (0=pending, 1=completed),
  created_at, updated_at, deleted_at

order_items
  id, order_id (FK), product_id (FK),
  quantity, unit_price, subtotal

feedback
  id, user_id (FK), message TEXT,
  is_public TINYINT,
  created_at, updated_at, deleted_at
```

---

## 📦 Catatan Deployment

> [!IMPORTANT]
> Pastikan langkah-langkah berikut dilakukan sebelum menjalankan server di lingkungan produksi.

**1. Jalankan semua migration database:**
```bash
for f in migrations/*.sql; do mysql -u root -p kasir < "$f"; done
```

**2. Pastikan direktori `fonts/` tersedia:**
```
fonts/
├── LiberationSans-Regular.ttf
├── LiberationSans-Bold.ttf
├── LiberationSans-Italic.ttf
└── LiberationSans-BoldItalic.ttf
```
Atau set variabel `FONT_DIR=/path/absolut/ke/fonts` di `.env`.

**3. Lengkapi profil UMKM untuk laporan PDF yang optimal:**

Pastikan user mengisi `address` dan `contact` via `PUT /api/auth/me` — data ini akan tampil sebagai header di setiap laporan PDF yang digenerate.

---

<div align="center">
  <br/>
  <p>Dibangun dengan ❤️ untuk pelaku UMKM Indonesia</p>
  <p>
    <strong>IDCamp Developer Challenge — Generative AI Track</strong>
  </p>
  <br/>
  <a href="./api-docs.md">📖 Lihat Dokumentasi API Lengkap →</a>
</div>