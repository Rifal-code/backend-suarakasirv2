Berikut adalah terjemahan dokumen `README.md` ke dalam bahasa Indonesia:

# API Kasir

REST API siap produksi untuk sistem Point-of-Sale (Kasir) yang dibangun menggunakan Rust, Axum, SQLx, dan MySQL.

## Teknologi yang Digunakan

| Lapisan | Teknologi |
| --- | --- |
| Bahasa | Rust (edisi 2021) |
| Framework Web | Axum 0.8 |
| Database | MySQL (via SQLx 0.8) |
| Autentikasi | JWT (jsonwebtoken 9) |
| Hashing Kata Sandi | bcrypt |
| Validasi | validator 0.18 |
| Konfigurasi | dotenvy |
| Integrasi AI | Reqwest → Gemini API |

---

## Persiapan

### 1. Prasyarat

* Rust (versi stable)
* MySQL 8.0+

### 2. Konfigurasi

Salin `.env.example` menjadi `.env` dan isi nilai-nilainya:

```env
APP_HOST=127.0.0.1
APP_PORT=8000
DATABASE_URL=mysql://root:password@localhost:3306/kasir
JWT_SECRET=rahasia-kunci-jwt-anda-ganti-di-produksi
AI_API_KEY=kunci-api-gemini-anda
AI_API_URL=https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent

```

### 3. Persiapan Database

Jalankan migrasi secara berurutan:

```bash
mysql -u root -p kasir < migrations/001_create_users_table.sql
mysql -u root -p kasir < migrations/002_create_products_table.sql
mysql -u root -p kasir < migrations/003_create_orders_tables.sql
mysql -u root -p kasir < migrations/004_create_feedback_table.sql

```

### 4. Menjalankan Aplikasi

```bash
cargo run

```

Server akan berjalan pada `http://127.0.0.1:8000`.

---

## Arsitektur

```text
src/
├── config.rs             # Konfigurasi environment
├── state.rs              # AppState (pool db + konfigurasi)
├── main.rs               # Titik masuk (Entry point)
├── database/
│   └── connetion.rs      # Pembuatan pool MySQL
├── models/               # Struct SQLx FromRow
│   ├── user.rs
│   ├── product.rs
│   ├── order.rs
│   └── feedback.rs
├── dto/                  # DTO Request/Response
│   ├── auth/
│   ├── product.rs
│   ├── order.rs
│   ├── feedback.rs
│   └── ai.rs
├── repositories/         # Lapisan akses database
│   ├── user_repository.rs
│   ├── product_repository.rs
│   ├── order_repository.rs
│   └── feedback_repository.rs
├── services/             # Lapisan logika bisnis
│   ├── auth/
│   ├── product_service.rs
│   ├── order_service.rs
│   ├── feedback_services.rs
│   └── ai_service.rs
├── handlers/             # Handler HTTP
│   ├── auth/
│   ├── products.rs
│   ├── orders.rs
│   ├── feedback.rs
│   └── ai.rs
├── routes/               # Registrasi rute
│   ├── auth.rs
│   ├── product.rs
│   ├── order.rs
│   ├── feedback.rs
│   └── ai.rs
├── middleware/
│   └── jwt.rs            # Middleware autentikasi JWT
└── errors/
    └── app_error.rs      # Penanganan error terpadu

```

---

## Format Respons

Semua endpoint mengembalikan struktur JSON yang konsisten.

**Sukses (satu item):**

```json
{
  "success": true,
  "message": "...",
  "data": { ... }
}

```

**Sukses (daftar dengan paginasi):**

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

Endpoint yang dilindungi memerlukan token Bearer pada header `Authorization`:

```text
Authorization: Bearer <token_jwt>

```

Token JWT berlaku selama **7 hari**.

---

## Endpoint API

### Auth — `/api/auth`

| Metode | Path | Autentikasi | Deskripsi |
| --- | --- | --- | --- |
| `POST` | `/api/auth/register` | Publik | Mendaftarkan pengguna baru |
| `POST` | `/api/auth/login` | Publik | Login dan mendapatkan JWT |
| `POST` | `/api/auth/logout` | 🔒 JWT | Logout (klien harus membuang token) |
| `GET` | `/api/auth/me` | 🔒 JWT | Dapatkan profil pengguna yang terautentikasi |
| `PUT` | `/api/auth/me` | 🔒 JWT | Perbarui profil pengguna |

#### `POST /api/auth/register`

```json
{
  "name": "John Doe",
  "email": "john@example.com",
  "password": "secret123",
  "description": "Bio opsional"
}

```

**Respons `201`:**

```json
{
  "success": true,
  "message": "Registrasi berhasil",
  "data": {
    "id": "uuid",
    "name": "John Doe",
    "email": "john@example.com",
    "description": "Bio opsional"
  }
}

```

#### `POST /api/auth/login`

```json
{
  "email": "john@example.com",
  "password": "secret123"
}

```

**Respons `200`:**

```json
{
  "success": true,
  "message": "Login berhasil",
  "data": {
    "token": "eyJ...",
    "user": {
      "id": "uuid",
      "name": "John Doe",
      "email": "john@example.com",
      "description": null
    }
  }
}

```

#### `PUT /api/auth/me` 🔒

Semua field bersifat opsional:

```json
{
  "name": "Nama Baru",
  "email": "new@example.com",
  "password": "passwordbaru",
  "description": "Bio yang diperbarui"
}

```

---

### Produk — `/api/products` 🔒

Semua endpoint produk memerlukan JWT. Pengguna hanya dapat mengakses produk mereka sendiri.

| Metode | Path | Deskripsi |
| --- | --- | --- |
| `GET` | `/api/products` | Tampilkan daftar produk sendiri (paginasi) |
| `GET` | `/api/products/:id` | Dapatkan detail produk |
| `POST` | `/api/products` | Buat produk baru |
| `PUT` | `/api/products/:id` | Perbarui produk |
| `DELETE` | `/api/products/:id` | Hapus produk (soft-delete) |

#### Parameter Query untuk `GET /api/products`

| Parameter | Tipe | Default | Deskripsi |
| --- | --- | --- | --- |
| `page` | int | 1 | Nomor halaman |
| `limit` | int | 10 | Item per halaman (maks 100) |
| `search` | string | — | Pencarian berdasarkan nama |

#### `POST /api/products`

```json
{
  "name": "Kopi Susu",
  "price": "15000.00",
  "description": "Deskripsi opsional"
}

```

**Validasi:**

* `name`: 2–255 karakter, harus unik per pengguna
* `price`: harus > 0

#### `PUT /api/products/:id`

Semua field bersifat opsional:

```json
{
  "name": "Kopi Hitam",
  "price": "12000.00",
  "description": "Deskripsi yang diperbarui"
}

```

**Error:**

* `404` — produk tidak ditemukan
* `403` — produk milik pengguna lain
* `409` — nama duplikat

---

### Pesanan — `/api/orders` 🔒

Semua endpoint pesanan memerlukan JWT. Pengguna hanya dapat mengakses pesanan mereka sendiri.

> ⚠️ **Perhitungan harga di sisi server:** Harga dimuat dari database. Harga yang diberikan klien akan diabaikan.

| Metode | Path | Deskripsi |
| --- | --- | --- |
| `GET` | `/api/orders` | Tampilkan pesanan sendiri (paginasi, dapat disaring) |
| `GET` | `/api/orders/:id` | Dapatkan detail pesanan beserta itemnya |
| `POST` | `/api/orders` | Buat pesanan |
| `PUT` | `/api/orders/:id` | Perbarui pesanan |
| `DELETE` | `/api/orders/:id` | Hapus pesanan (soft-delete) |

#### Parameter Query untuk `GET /api/orders`

| Parameter | Tipe | Default | Deskripsi |
| --- | --- | --- | --- |
| `page` | int | 1 | Nomor halaman |
| `limit` | int | 10 | Item per halaman (maks 100) |
| `status` | int | — | Saring berdasarkan status (0=menunggu, 1=selesai) |
| `start_date` | ISO 8601 | — | Saring dari tanggal |
| `end_date` | ISO 8601 | — | Saring sampai tanggal |

#### `POST /api/orders`

```json
{
  "items": [
    { "product_id": "uuid", "quantity": 2 },
    { "product_id": "uuid", "quantity": 1 }
  ]
}

```

**Respons `201`:**

```json
{
  "success": true,
  "message": "Pesanan berhasil dibuat",
  "data": {
    "id": "uuid",
    "total_amount": "45000.00",
    "status": 0,
    "items": [
      {
        "id": "uuid",
        "product_id": "uuid",
        "product_name": "Kopi Susu",
        "quantity": 2,
        "unit_price": "15000.00",
        "subtotal": "30000.00"
      }
    ],
    "created_at": "2026-06-10T03:00:00Z"
  }
}

```

**Aturan Bisnis:**

* `unit_price` diambil dari tabel produk (tidak pernah memercayai input dari klien)
* `subtotal = quantity × unit_price`
* `total_amount = Σ subtotals`
* Mengubah harga produk TIDAK memengaruhi riwayat pesanan sebelumnya

#### Status Pesanan

| Nilai | Arti |
| --- | --- |
| `0` | Menunggu (Pending) |
| `1` | Selesai (Completed) |

---

### Umpan Balik (Feedback) — `/api/feedback`

| Metode | Path | Autentikasi | Deskripsi |
| --- | --- | --- | --- |
| `GET` | `/api/feedback` | Publik | Tampilkan daftar umpan balik publik (paginasi) |
| `GET` | `/api/feedback/:id` | Publik | Dapatkan detail umpan balik publik |
| `POST` | `/api/feedback` | 🔒 JWT | Buat umpan balik |
| `PUT` | `/api/feedback/:id` | 🔒 JWT | Perbarui umpan balik sendiri |
| `DELETE` | `/api/feedback/:id` | 🔒 JWT | Hapus umpan balik sendiri (soft-delete) |

#### `POST /api/feedback`

```json
{
  "message": "Layanan yang luar biasa!",
  "is_public": true
}

```

**Validasi:**

* `message`: 3–1000 karakter (wajib)
* `is_public`: boolean, secara bawaan bernilai `true`

**Respons `201`:**

```json
{
  "success": true,
  "message": "Umpan balik berhasil dibuat",
  "data": {
    "id": "uuid",
    "user_name": "John Doe",
    "message": "Layanan yang luar biasa!",
    "created_at": "2026-06-10T03:00:00Z"
  }
}

```

**Keamanan:**

* Endpoint `GET` hanya mengembalikan umpan balik yang bernilai `is_public = true`
* `PUT`/`DELETE` memberlakukan aturan kepemilikan — mengembalikan `403` jika mengakses umpan balik milik pengguna lain

---

### Chat AI — `/api/ai` 🔒

| Metode | Path | Autentikasi | Deskripsi |
| --- | --- | --- | --- |
| `POST` | `/api/ai/chat` | 🔒 JWT | Kirim pesan ke asisten AI |

#### `POST /api/ai/chat`

```json
{
  "message": "Rekomendasikan menu untuk siang hari"
}

```

**Respons `200`:**

```json
{
  "success": true,
  "message": "Respons AI berhasil dibuat",
  "data": {
    "reply": "Berikut beberapa rekomendasi menu siang..."
  }
}

```

> Memerlukan pengaturan `AI_API_KEY` dan `AI_API_URL` pada konfigurasi.

---

## Referensi Error

| Status HTTP | Varian AppError | Penyebab Umum |
| --- | --- | --- |
| `400` | `BadRequest` | Request cacat (Malformed request) |
| `401` | `Unauthorized` | JWT tidak ada/tidak valid atau kredensial salah |
| `403` | `Forbidden` | Mengakses sumber daya pengguna lain |
| `404` | `NotFound` | Sumber daya tidak ada atau telah di-soft-delete |
| `409` | `Conflict` | Email duplikat, nama produk duplikat |
| `422` | `ValidationError` | Gagal validasi field |
| `500` | `InternalServerError` | Error pada database atau server |

---

## Keamanan

* Kata sandi di-hash dengan **bcrypt** (cost bawaan)
* Token JWT menggunakan **HS256** dengan rahasia (secret) yang dapat dikonfigurasi
* Semua penulisan (writes) dibatasi ruang lingkupnya per pengguna — akses lintas pengguna akan mengembalikan `403`
* Penghapusan sementara (Soft delete) menjaga integritas data (data yang dihapus dikecualikan dari semua kueri)
* Harga pesanan **selalu dihitung di sisi server** — harga dari klien diabaikan

---

## Skema Database

```text
users
  id (PK), name, email (UNIK), password, description, created_at, updated_at, deleted_at

products
  id (PK), user_id (FK→users), name, price (DECIMAL), description, created_at, updated_at, deleted_at
  UNIK: (user_id, name) per produk aktif

orders
  id (PK), user_id (FK→users), total_amount (DECIMAL), status (TINYINT), created_at, updated_at, deleted_at

order_items
  id (PK), order_id (FK→orders), product_id (FK→products), quantity, unit_price (cuplikan/snapshot), subtotal

feedback
  id (PK), user_id (FK→users), message (TEXT), is_public (TINYINT), created_at, updated_at, deleted_at

```