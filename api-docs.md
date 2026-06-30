# Suara Kasir — API Documentation

**Base URL:** `http://127.0.0.1:8000`

Semua endpoint yang dilindungi memerlukan header:
```
Authorization: Bearer <token>
```

Token JWT diperoleh dari endpoint Login dan berlaku selama **7 hari**.

---

## Format Response

**Sukses (single data):**
```json
{
    "success": true,
    "message": "...",
    "data": { }
}
```

**Sukses (list + paginasi):**
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

**Error:**
```json
{
    "success": false,
    "message": "Deskripsi error",
    "data": null
}
```

---

## Error Codes

| HTTP Status | Penyebab |
|---|---|
| `400` | Request tidak valid |
| `401` | Token JWT hilang atau tidak valid |
| `403` | Akses ke resource milik user lain |
| `404` | Resource tidak ditemukan |
| `409` | Duplikasi data (email, nama produk) |
| `422` | Validasi field gagal atau stok tidak cukup |
| `500` | Internal server error |

---

## AUTH

### Register

- **URL**
    - `/api/auth/register`
- **Method**
    - `POST`
- **Headers**
    - `Content-Type: application/json`
- **Request Body**
    - `name` as string, required, min 2 max 100 karakter
    - `email` as string, required, harus format email valid
    - `password` as string, required, min 6 karakter
    - `description` as string, optional
- **Response**

    ```json
    {
        "success": true,
        "message": "Registration successful",
        "data": {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "Budi Santoso",
            "email": "budi@example.com",
            "description": "Pemilik warung makan"
        }
    }
    ```

---

### Login

- **URL**
    - `/api/auth/login`
- **Method**
    - `POST`
- **Headers**
    - `Content-Type: application/json`
- **Request Body**
    - `email` as string, required, harus format email valid
    - `password` as string, required, min 6 karakter
- **Response**

    ```json
    {
        "success": true,
        "message": "Login successful",
        "data": {
            "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
            "user": {
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "name": "Budi Santoso",
                "email": "budi@example.com",
                "description": "Pemilik warung makan"
            }
        }
    }
    ```

---

### Logout

- **URL**
    - `/api/auth/logout`
- **Method**
    - `POST`
- **Headers**
    - `Authorization: Bearer <token>`
- **Request Body**
    - Tidak ada
- **Response**

    ```json
    {
        "success": true,
        "message": "Logout successful. Please discard your token.",
        "data": null
    }
    ```

> **Catatan:** Server tidak menyimpan daftar token. Logout dilakukan di sisi klien dengan membuang token yang tersimpan.

---

### Get Profile

- **URL**
    - `/api/auth/me`
- **Method**
    - `GET`
- **Headers**
    - `Authorization: Bearer <token>`
- **Response**

    ```json
    {
        "success": true,
        "message": "Profile fetched successfully",
        "data": {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "Budi Santoso",
            "email": "budi@example.com",
            "description": "Pemilik warung makan",
            "address": "Jl. Merdeka No. 10, Jakarta",
            "contact": "+6281234567890"
        }
    }
    ```

---

### Update Profile

- **URL**
    - `/api/auth/me`
- **Method**
    - `PUT`
- **Headers**
    - `Content-Type: application/json`
    - `Authorization: Bearer <token>`
- **Request Body**
    - `name` as string, optional, min 2 max 100 karakter
    - `email` as string, optional, harus format email valid dan belum digunakan
    - `password` as string, optional, min 6 karakter
    - `description` as string, optional
    - `address` as string, optional, max 255 karakter — tampil di laporan PDF
    - `contact` as string, optional, max 100 karakter — tampil di laporan PDF
- **Response**

    ```json
    {
        "success": true,
        "message": "Profile updated successfully",
        "data": {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "Budi Santoso",
            "email": "budi@example.com",
            "description": "Pemilik warung makan",
            "address": "Jl. Merdeka No. 10, Jakarta",
            "contact": "+6281234567890"
        }
    }
    ```

> **Catatan:** `id`, `created_at`, dan `updated_at` tidak dapat diubah. Field yang tidak disertakan dalam request tidak akan berubah.

---

## PRODUK

Semua endpoint produk memerlukan autentikasi JWT. User hanya dapat mengakses produk miliknya sendiri.

### Get All Products

- **URL**
    - `/api/products`
- **Method**
    - `GET`
- **Headers**
    - `Authorization: Bearer <token>`
- **Query Parameters**
    - `page` as integer, optional, default `1`
    - `limit` as integer, optional, default `10`
    - `search` as string, optional, filter berdasarkan nama produk
- **Response**

    ```json
    {
        "success": true,
        "message": "Products fetched successfully",
        "data": [
            {
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "name": "Kopi Susu",
                "price": "15000.00",
                "description": "Kopi susu segar",
                "image_url": "https://example.com/kopi.jpg",
                "stock": 50,
                "created_at": "2026-06-01T08:00:00Z"
            }
        ],
        "total": 1,
        "page": 1,
        "limit": 10
    }
    ```

---

### Get Product Detail

- **URL**
    - `/api/products/:id`
- **Method**
    - `GET`
- **Headers**
    - `Authorization: Bearer <token>`
- **URL Parameters**
    - `id` as string, UUID produk
- **Response**

    ```json
    {
        "success": true,
        "message": "Product fetched successfully",
        "data": {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "Kopi Susu",
            "price": "15000.00",
            "description": "Kopi susu segar",
            "image_url": "https://example.com/kopi.jpg",
            "stock": 50,
            "created_at": "2026-06-01T08:00:00Z"
        }
    }
    ```

---

### Create Product

- **URL**
    - `/api/products`
- **Method**
    - `POST`
- **Headers**
    - `Content-Type: application/json`
    - `Authorization: Bearer <token>`
- **Request Body**
    - `name` as string, required, min 2 max 255 karakter, unik per user
    - `price` as decimal, required, harus lebih dari 0
    - `description` as string, optional
    - `image_url` as string, optional, harus URL valid
    - `stock` as integer, optional, default `0`
- **Response**

    ```json
    {
        "success": true,
        "message": "Product created successfully",
        "data": {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "Kopi Susu",
            "price": "15000.00",
            "description": "Kopi susu segar",
            "image_url": "https://example.com/kopi.jpg",
            "stock": 50,
            "created_at": "2026-06-01T08:00:00Z"
        }
    }
    ```

---

### Update Product

- **URL**
    - `/api/products/:id`
- **Method**
    - `PUT`
- **Headers**
    - `Content-Type: application/json`
    - `Authorization: Bearer <token>`
- **URL Parameters**
    - `id` as string, UUID produk
- **Request Body**
    - `name` as string, optional, min 2 max 255 karakter
    - `price` as decimal, optional
    - `description` as string, optional
    - `image_url` as string, optional, harus URL valid
    - `stock` as integer, optional
- **Response**

    ```json
    {
        "success": true,
        "message": "Product updated successfully",
        "data": {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "Kopi Susu Premium",
            "price": "18000.00",
            "description": "Kopi susu segar premium",
            "image_url": "https://example.com/kopi-premium.jpg",
            "stock": 45,
            "created_at": "2026-06-01T08:00:00Z"
        }
    }
    ```

---

### Delete Product

- **URL**
    - `/api/products/:id`
- **Method**
    - `DELETE`
- **Headers**
    - `Authorization: Bearer <token>`
- **URL Parameters**
    - `id` as string, UUID produk
- **Response**

    ```json
    {
        "success": true,
        "message": "Product deleted successfully",
        "data": null
    }
    ```

> **Catatan:** Penghapusan bersifat soft delete. Data tidak benar-benar dihapus dari database.

---

## ORDER

Semua endpoint order memerlukan autentikasi JWT. Harga selalu dihitung server-side — harga dari klien diabaikan. Stok divalidasi sebelum order disimpan dan dikurangi otomatis setelah berhasil.

### Get All Orders

- **URL**
    - `/api/orders`
- **Method**
    - `GET`
- **Headers**
    - `Authorization: Bearer <token>`
- **Query Parameters**
    - `page` as integer, optional, default `1`
    - `limit` as integer, optional, default `10`
    - `status` as integer, optional, `0` = pending, `1` = completed
    - `start_date` as datetime (ISO 8601), optional
    - `end_date` as datetime (ISO 8601), optional
- **Response**

    ```json
    {
        "success": true,
        "message": "Orders fetched successfully",
        "data": [
            {
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "total_amount": "45000.00",
                "status": 1,
                "items": [
                    {
                        "id": "item-uuid",
                        "product_id": "product-uuid",
                        "product_name": "Kopi Susu",
                        "quantity": 2,
                        "unit_price": "15000.00",
                        "subtotal": "30000.00"
                    },
                    {
                        "id": "item-uuid-2",
                        "product_id": "product-uuid-2",
                        "product_name": "Teh Manis",
                        "quantity": 1,
                        "unit_price": "8000.00",
                        "subtotal": "8000.00"
                    }
                ],
                "created_at": "2026-06-29T10:00:00Z"
            }
        ],
        "total": 1,
        "page": 1,
        "limit": 10
    }
    ```

---

### Get Order Detail

- **URL**
    - `/api/orders/:id`
- **Method**
    - `GET`
- **Headers**
    - `Authorization: Bearer <token>`
- **URL Parameters**
    - `id` as string, UUID order
- **Response**

    ```json
    {
        "success": true,
        "message": "Order fetched successfully",
        "data": {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "total_amount": "45000.00",
            "status": 1,
            "items": [
                {
                    "id": "item-uuid",
                    "product_id": "product-uuid",
                    "product_name": "Kopi Susu",
                    "quantity": 2,
                    "unit_price": "15000.00",
                    "subtotal": "30000.00"
                }
            ],
            "created_at": "2026-06-29T10:00:00Z"
        }
    }
    ```

---

### Create Order

- **URL**
    - `/api/orders`
- **Method**
    - `POST`
- **Headers**
    - `Content-Type: application/json`
    - `Authorization: Bearer <token>`
- **Request Body**
    - `items` as array, required, minimal 1 item
        - `product_id` as string, required, UUID produk
        - `quantity` as integer, required, harus lebih dari 0
- **Response**

    ```json
    {
        "success": true,
        "message": "Order created successfully",
        "data": {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "total_amount": "45000.00",
            "status": 0,
            "items": [
                {
                    "id": "item-uuid",
                    "product_id": "product-uuid",
                    "product_name": "Kopi Susu",
                    "quantity": 2,
                    "unit_price": "15000.00",
                    "subtotal": "30000.00"
                }
            ],
            "created_at": "2026-06-29T10:00:00Z"
        }
    }
    ```

- **Error — Stok tidak cukup (422)**

    ```json
    {
        "success": false,
        "message": "Insufficient stock for 'Kopi Susu'. Available: 3, requested: 5",
        "data": null
    }
    ```

---

### Update Order

- **URL**
    - `/api/orders/:id`
- **Method**
    - `PUT`
- **Headers**
    - `Content-Type: application/json`
    - `Authorization: Bearer <token>`
- **URL Parameters**
    - `id` as string, UUID order
- **Request Body**
    - `items` as array, required, minimal 1 item
        - `product_id` as string, required
        - `quantity` as integer, required, harus lebih dari 0
    - `status` as integer, optional, `0` = pending, `1` = completed
- **Response**

    ```json
    {
        "success": true,
        "message": "Order updated successfully",
        "data": {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "total_amount": "30000.00",
            "status": 1,
            "items": [
                {
                    "id": "item-uuid",
                    "product_id": "product-uuid",
                    "product_name": "Kopi Susu",
                    "quantity": 2,
                    "unit_price": "15000.00",
                    "subtotal": "30000.00"
                }
            ],
            "created_at": "2026-06-29T10:00:00Z"
        }
    }
    ```

---

### Delete Order

- **URL**
    - `/api/orders/:id`
- **Method**
    - `DELETE`
- **Headers**
    - `Authorization: Bearer <token>`
- **URL Parameters**
    - `id` as string, UUID order
- **Response**

    ```json
    {
        "success": true,
        "message": "Order deleted successfully",
        "data": null
    }
    ```

---

## FEEDBACK

### Get All Feedback

- **URL**
    - `/api/feedback`
- **Method**
    - `GET`
- **Headers**
    - Tidak diperlukan (publik)
- **Query Parameters**
    - `page` as integer, optional, default `1`
    - `limit` as integer, optional, default `10`
- **Response**

    ```json
    {
        "success": true,
        "message": "Feedback fetched successfully",
        "data": [
            {
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "user_name": "Budi Santoso",
                "message": "Aplikasi ini sangat membantu!",
                "created_at": "2026-06-29T10:00:00Z"
            }
        ],
        "total": 1,
        "page": 1,
        "limit": 10
    }
    ```

---

### Get Feedback Detail

- **URL**
    - `/api/feedback/:id`
- **Method**
    - `GET`
- **Headers**
    - Tidak diperlukan (publik)
- **URL Parameters**
    - `id` as string, UUID feedback
- **Response**

    ```json
    {
        "success": true,
        "message": "Feedback fetched successfully",
        "data": {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "user_name": "Budi Santoso",
            "message": "Aplikasi ini sangat membantu!",
            "created_at": "2026-06-29T10:00:00Z"
        }
    }
    ```

---

### Create Feedback

- **URL**
    - `/api/feedback`
- **Method**
    - `POST`
- **Headers**
    - `Content-Type: application/json`
    - `Authorization: Bearer <token>`
- **Request Body**
    - `message` as string, required, min 3 max 1000 karakter
    - `is_public` as boolean, optional, default `false`
- **Response**

    ```json
    {
        "success": true,
        "message": "Feedback created successfully",
        "data": {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "user_name": "Budi Santoso",
            "message": "Aplikasi ini sangat membantu!",
            "created_at": "2026-06-29T10:00:00Z"
        }
    }
    ```

---

### Update Feedback

- **URL**
    - `/api/feedback/:id`
- **Method**
    - `PUT`
- **Headers**
    - `Content-Type: application/json`
    - `Authorization: Bearer <token>`
- **URL Parameters**
    - `id` as string, UUID feedback
- **Request Body**
    - `message` as string, optional, min 3 max 1000 karakter
    - `is_public` as boolean, optional
- **Response**

    ```json
    {
        "success": true,
        "message": "Feedback updated successfully",
        "data": {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "user_name": "Budi Santoso",
            "message": "Diperbarui: Sangat membantu bisnis saya!",
            "created_at": "2026-06-29T10:00:00Z"
        }
    }
    ```

---

### Delete Feedback

- **URL**
    - `/api/feedback/:id`
- **Method**
    - `DELETE`
- **Headers**
    - `Authorization: Bearer <token>`
- **URL Parameters**
    - `id` as string, UUID feedback
- **Response**

    ```json
    {
        "success": true,
        "message": "Feedback deleted successfully",
        "data": null
    }
    ```

> **Catatan:** User hanya dapat mengupdate atau menghapus feedback miliknya sendiri. Akses ke feedback milik user lain mengembalikan `403 Forbidden`.

---

## DASHBOARD

Semua endpoint dashboard memerlukan autentikasi JWT dan hanya mengembalikan data milik user yang sedang login.

### Overview

- **URL**
    - `/api/dashboard`
- **Method**
    - `GET`
- **Headers**
    - `Authorization: Bearer <token>`
- **Response**

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
                "product_id": "product-uuid",
                "product_name": "Kopi Susu",
                "total_quantity": 120,
                "total_revenue": "1800000.00"
            },
            "recent_orders_count": 210
        }
    }
    ```

> **Catatan:** `total_sales_week` dan `total_sales_month` dihitung dari awal hari (00:00:00) N hari lalu, bukan N×24 jam ke belakang.

---

### Sales Chart

- **URL**
    - `/api/dashboard/sales`
- **Method**
    - `GET`
- **Headers**
    - `Authorization: Bearer <token>`
- **Query Parameters**
    - `range` as string, optional, nilai: `7d` (default), `30d`, `1y`
- **Response**

    ```json
    {
        "success": true,
        "message": "Sales chart data",
        "data": {
            "range": "7d",
            "data": [
                {
                    "label": "2026-06-23",
                    "total_sales": "150000.00",
                    "total_orders": 10
                },
                {
                    "label": "2026-06-24",
                    "total_sales": "0.00",
                    "total_orders": 0
                },
                {
                    "label": "2026-06-29",
                    "total_sales": "200000.00",
                    "total_orders": 14
                }
            ]
        }
    }
    ```

> **Catatan:** Hari tanpa transaksi tetap muncul dengan nilai `0`. Range `1y` menggunakan agregasi bulanan (`YYYY-MM`).

---

### Top Products

- **URL**
    - `/api/dashboard/top-products`
- **Method**
    - `GET`
- **Headers**
    - `Authorization: Bearer <token>`
- **Query Parameters**
    - `range` as string, optional, nilai: `7d` (default), `30d`, `1y`
- **Response**

    ```json
    {
        "success": true,
        "message": "Top products",
        "data": [
            {
                "product_id": "product-uuid",
                "product_name": "Kopi Susu",
                "total_quantity": 120,
                "total_revenue": "1800000.00"
            },
            {
                "product_id": "product-uuid-2",
                "product_name": "Teh Manis",
                "total_quantity": 85,
                "total_revenue": "680000.00"
            }
        ]
    }
    ```

---

### Sales Trends

- **URL**
    - `/api/dashboard/trends`
- **Method**
    - `GET`
- **Headers**
    - `Authorization: Bearer <token>`
- **Query Parameters**
    - `range` as string, optional, nilai: `7d` (default), `30d`, `1y`
- **Response**

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

> `sales_trend` dan `order_trend` bernilai `"up"`, `"down"`, atau `"flat"`.

---

## AI & VOICE ORDER

### AI Chat

- **URL**
    - `/api/ai/chat`
- **Method**
    - `POST`
- **Headers**
    - `Content-Type: application/json`
    - `Authorization: Bearer <token>`
- **Request Body**
    - `message` as string, required, min 1 max 2000 karakter
- **Response**

    ```json
    {
        "success": true,
        "message": "AI response generated",
        "data": {
            "reply": "Berikut rekomendasi menu untuk siang hari: ..."
        }
    }
    ```

---

### Parse Voice Order

Menerima hasil konversi Gemini dari suara ke JSON, lalu melakukan fuzzy matching ke produk di database. Digunakan untuk konfirmasi order sebelum dikirim ke `POST /api/orders`.

- **URL**
    - `/api/ai/parse-order`
- **Method**
    - `POST`
- **Headers**
    - `Content-Type: application/json`
    - `Authorization: Bearer <token>`
- **Request Body**
    - `items` as array, required, minimal 1 item
        - `n` as string, nama produk yang diucapkan (bisa typo atau singkatan)
        - `q` as integer, jumlah produk
- **Response**

    ```json
    {
        "success": true,
        "message": "Order parsed from voice input",
        "data": {
            "items": [
                {
                    "product_id": "product-uuid",
                    "name": "Bakso",
                    "input_name": "baxo",
                    "quantity": 3,
                    "unit_price": "15000.00",
                    "confidence": 0.9333,
                    "needs_confirmation": false
                },
                {
                    "product_id": "product-uuid-2",
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

> **Catatan:**
> - `confidence` bernilai antara `0.0` hingga `1.0` (4 desimal)
> - `needs_confirmation: true` jika `confidence < 0.80` — tampilkan konfirmasi ke user
> - Setelah user mengkonfirmasi, kirim hasil ke `POST /api/orders`

---

## LAPORAN PDF

### Download Laporan Penjualan PDF

Generate dan download laporan penjualan dalam format PDF berdasarkan data transaksi user dalam rentang waktu yang dipilih.

- **URL**
    - `/api/reports/sales/pdf`
- **Method**
    - `GET`
- **Headers**
    - `Authorization: Bearer <token>`
- **Query Parameters**
    - `range` as string, optional, nilai: `7d` (default), `30d`, `1y`
- **Response**
    - Content-Type: `application/pdf`
    - File PDF langsung diunduh dengan nama `laporan-penjualan-{range}.pdf`

    ```
    HTTP/1.1 200 OK
    Content-Type: application/pdf
    Content-Disposition: attachment; filename="laporan-penjualan-7d.pdf"
    Content-Length: <ukuran file dalam bytes>
    ```

> **Isi laporan PDF:**
> 1. Header UMKM — nama, alamat, dan kontak dari profil user
> 2. Informasi periode laporan
> 3. Ringkasan — total omzet, jumlah transaksi, total item, produk terlaris
> 4. Insight AI — analisis bisnis singkat dalam bahasa Indonesia
> 5. Tabel produk terlaris (top 10)
> 6. Tabel detail transaksi lengkap dengan subtotal dan total akhir
> 7. Tanda tangan pemilik dan tanggal cetak
>
> **Tips:** Lengkapi `address` dan `contact` di `PUT /api/auth/me` agar informasi UMKM muncul lengkap di laporan.
