-- Migration: Create products table
-- Created: 2026-06-10

CREATE TABLE IF NOT EXISTS products (
    id          VARCHAR(36)     NOT NULL PRIMARY KEY,
    user_id     VARCHAR(36)     NOT NULL,
    name        VARCHAR(255)    NOT NULL,
    price       DECIMAL(15, 2)  NOT NULL,
    description TEXT            NULL,
    created_at  DATETIME(6)     NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at  DATETIME(6)     NOT NULL DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6),
    deleted_at  DATETIME(6)     NULL,
    INDEX idx_products_user_id (user_id),
    INDEX idx_products_deleted_at (deleted_at),
    INDEX idx_products_user_name (user_id, name),
    CONSTRAINT fk_products_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
