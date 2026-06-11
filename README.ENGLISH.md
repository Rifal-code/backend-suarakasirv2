# Kasir API

A production-ready REST API for a Point-of-Sale (Kasir) system built with Rust, Axum, SQLx, and MySQL.

## Tech Stack

| Layer | Technology |
|---|---|
| Language | Rust (2021 edition) |
| Web Framework | Axum 0.8 |
| Database | MySQL (via SQLx 0.8) |
| Authentication | JWT (jsonwebtoken 9) |
| Password Hashing | bcrypt |
| Validation | validator 0.18 |
| Config | dotenvy |
| AI Integration | Reqwest → Gemini API |

---

## Setup

### 1. Prerequisites

- Rust (stable)
- MySQL 8.0+

### 2. Configuration

Copy `.env.example` to `.env` and fill in the values:

```env
APP_HOST=127.0.0.1
APP_PORT=8000
DATABASE_URL=mysql://root:password@localhost:3306/kasir
JWT_SECRET=your-super-secret-jwt-key-change-in-production
AI_API_KEY=your-gemini-api-key
AI_API_URL=https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent
```

### 3. Database Setup

Run migrations in order:

```bash
mysql -u root -p kasir < migrations/001_create_users_table.sql
mysql -u root -p kasir < migrations/002_create_products_table.sql
mysql -u root -p kasir < migrations/003_create_orders_tables.sql
mysql -u root -p kasir < migrations/004_create_feedback_table.sql
```

### 4. Run

```bash
cargo run
```

The server starts at `http://127.0.0.1:8000`.

---

## Architecture

```
src/
├── config.rs             # Environment configuration
├── state.rs              # AppState (db pool + config)
├── main.rs               # Entry point
├── database/
│   └── connetion.rs      # MySQL pool creation
├── models/               # SQLx FromRow structs
│   ├── user.rs
│   ├── product.rs
│   ├── order.rs
│   └── feedback.rs
├── dto/                  # Request/Response DTOs
│   ├── auth/
│   ├── product.rs
│   ├── order.rs
│   ├── feedback.rs
│   └── ai.rs
├── repositories/         # Database access layer
│   ├── user_repository.rs
│   ├── product_repository.rs
│   ├── order_repository.rs
│   └── feedback_repository.rs
├── services/             # Business logic layer
│   ├── auth/
│   ├── product_service.rs
│   ├── order_service.rs
│   ├── feedback_services.rs
│   └── ai_service.rs
├── handlers/             # HTTP handlers
│   ├── auth/
│   ├── products.rs
│   ├── orders.rs
│   ├── feedback.rs
│   └── ai.rs
├── routes/               # Route registration
│   ├── auth.rs
│   ├── product.rs
│   ├── order.rs
│   ├── feedback.rs
│   └── ai.rs
├── middleware/
│   └── jwt.rs            # JWT authentication middleware
└── errors/
    └── app_error.rs      # Unified error handling
```

---

## Response Format

All endpoints return a consistent JSON structure.

**Success (single item):**
```json
{
  "success": true,
  "message": "...",
  "data": { ... }
}
```

**Success (paginated list):**
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
  "message": "Error description",
  "data": null
}
```

---

## Authentication

Protected endpoints require a Bearer token in the `Authorization` header:

```
Authorization: Bearer <jwt_token>
```

JWT tokens are valid for **7 days**.

---

## API Endpoints

### Auth — `/api/auth`

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| `POST` | `/api/auth/register` | Public | Register a new user |
| `POST` | `/api/auth/login` | Public | Login and receive JWT |
| `POST` | `/api/auth/logout` | 🔒 JWT | Logout (client must discard token) |
| `GET`  | `/api/auth/me` | 🔒 JWT | Get authenticated user profile |
| `PUT`  | `/api/auth/me` | 🔒 JWT | Update user profile |

#### `POST /api/auth/register`

```json
{
  "name": "John Doe",
  "email": "john@example.com",
  "password": "secret123",
  "description": "Optional bio"
}
```

**Response `201`:**
```json
{
  "success": true,
  "message": "Registration successful",
  "data": {
    "id": "uuid",
    "name": "John Doe",
    "email": "john@example.com",
    "description": "Optional bio"
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

**Response `200`:**
```json
{
  "success": true,
  "message": "Login successful",
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

All fields are optional:
```json
{
  "name": "New Name",
  "email": "new@example.com",
  "password": "newpassword",
  "description": "Updated bio"
}
```

---

### Products — `/api/products` 🔒

All product endpoints require JWT. Users can only access their own products.

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/products` | List own products (paginated) |
| `GET` | `/api/products/:id` | Get product detail |
| `POST` | `/api/products` | Create product |
| `PUT` | `/api/products/:id` | Update product |
| `DELETE` | `/api/products/:id` | Soft-delete product |

#### Query Parameters for `GET /api/products`

| Param | Type | Default | Description |
|-------|------|---------|-------------|
| `page` | int | 1 | Page number |
| `limit` | int | 10 | Items per page (max 100) |
| `search` | string | — | Search by name |

#### `POST /api/products`

```json
{
  "name": "Kopi Susu",
  "price": "15000.00",
  "description": "Optional description"
}
```

**Validation:**
- `name`: 2–255 characters, unique per user
- `price`: must be > 0

#### `PUT /api/products/:id`

All fields optional:
```json
{
  "name": "Kopi Hitam",
  "price": "12000.00",
  "description": "Updated desc"
}
```

**Errors:**
- `404` — product not found
- `403` — belongs to another user
- `409` — duplicate name

---

### Orders — `/api/orders` 🔒

All order endpoints require JWT. Users can only access their own orders.

> ⚠️ **Server-side price calculation:** Prices are loaded from the database. Client-provided prices are ignored.

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/orders` | List own orders (paginated, filterable) |
| `GET` | `/api/orders/:id` | Get order detail with items |
| `POST` | `/api/orders` | Create order |
| `PUT` | `/api/orders/:id` | Update order |
| `DELETE` | `/api/orders/:id` | Soft-delete order |

#### Query Parameters for `GET /api/orders`

| Param | Type | Default | Description |
|-------|------|---------|-------------|
| `page` | int | 1 | Page number |
| `limit` | int | 10 | Items per page (max 100) |
| `status` | int | — | Filter by status (0=pending, 1=completed) |
| `start_date` | ISO 8601 | — | Filter from date |
| `end_date` | ISO 8601 | — | Filter to date |

#### `POST /api/orders`

```json
{
  "items": [
    { "product_id": "uuid", "quantity": 2 },
    { "product_id": "uuid", "quantity": 1 }
  ]
}
```

**Response `201`:**
```json
{
  "success": true,
  "message": "Order created successfully",
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

**Business Rules:**
- `unit_price` is fetched from the products table (never trusted from client)
- `subtotal = quantity × unit_price`
- `total_amount = Σ subtotals`
- Changing a product's price does NOT affect historical orders

#### Order Status

| Value | Meaning |
|-------|---------|
| `0` | Pending |
| `1` | Completed |

---

### Feedback — `/api/feedback`

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| `GET` | `/api/feedback` | Public | List public feedback (paginated) |
| `GET` | `/api/feedback/:id` | Public | Get public feedback detail |
| `POST` | `/api/feedback` | 🔒 JWT | Create feedback |
| `PUT` | `/api/feedback/:id` | 🔒 JWT | Update own feedback |
| `DELETE` | `/api/feedback/:id` | 🔒 JWT | Soft-delete own feedback |

#### `POST /api/feedback`

```json
{
  "message": "Great service!",
  "is_public": true
}
```

**Validation:**
- `message`: 3–1000 characters (required)
- `is_public`: boolean, defaults to `true`

**Response `201`:**
```json
{
  "success": true,
  "message": "Feedback created successfully",
  "data": {
    "id": "uuid",
    "user_name": "John Doe",
    "message": "Great service!",
    "created_at": "2026-06-10T03:00:00Z"
  }
}
```

**Security:**
- `GET` endpoints return only `is_public = true` feedback
- `PUT`/`DELETE` enforce ownership — `403` if accessing another user's feedback

---

### AI Chat — `/api/ai` 🔒

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| `POST` | `/api/ai/chat` | 🔒 JWT | Send a message to the AI assistant |

#### `POST /api/ai/chat`

```json
{
  "message": "Rekomendasikan menu untuk siang hari"
}
```

**Response `200`:**
```json
{
  "success": true,
  "message": "AI response generated",
  "data": {
    "reply": "Berikut beberapa rekomendasi menu siang..."
  }
}
```

> Requires `AI_API_KEY` and `AI_API_URL` to be configured.

---

## Error Reference

| HTTP Status | AppError Variant | Typical Cause |
|-------------|-----------------|---------------|
| `400` | `BadRequest` | Malformed request |
| `401` | `Unauthorized` | Missing/invalid JWT or wrong credentials |
| `403` | `Forbidden` | Accessing another user's resource |
| `404` | `NotFound` | Resource does not exist or is soft-deleted |
| `409` | `Conflict` | Duplicate email, duplicate product name |
| `422` | `ValidationError` | Failed field validation |
| `500` | `InternalServerError` | Database or server error |

---

## Security

- Passwords are hashed with **bcrypt** (default cost)
- JWT tokens use **HS256** with a configurable secret
- All writes are user-scoped — cross-user access returns `403`
- Soft deletes preserve data integrity (deleted records are excluded from all queries)
- Order prices are **always calculated server-side** — client prices are ignored

---

## Database Schema

```sql
users
  id (PK), name, email (UNIQUE), password, description, created_at, updated_at, deleted_at

products
  id (PK), user_id (FK→users), name, price (DECIMAL), description, created_at, updated_at, deleted_at
  UNIQUE: (user_id, name) per active products

orders
  id (PK), user_id (FK→users), total_amount (DECIMAL), status (TINYINT), created_at, updated_at, deleted_at

order_items
  id (PK), order_id (FK→orders), product_id (FK→products), quantity, unit_price (snapshot), subtotal

feedback
  id (PK), user_id (FK→users), message (TEXT), is_public (TINYINT), created_at, updated_at, deleted_at
```
