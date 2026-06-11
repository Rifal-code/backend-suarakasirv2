-- Migration: Create orders and order_items tables
-- Created: 2026-06-10

CREATE TABLE IF NOT EXISTS orders (
    id           VARCHAR(36)    NOT NULL PRIMARY KEY,
    user_id      VARCHAR(36)    NOT NULL,
    total_amount DECIMAL(15, 2) NOT NULL DEFAULT 0.00,
    status       TINYINT        NOT NULL DEFAULT 0 COMMENT '0=pending, 1=completed',
    created_at   DATETIME(6)    NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at   DATETIME(6)    NOT NULL DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6),
    deleted_at   DATETIME(6)    NULL,
    INDEX idx_orders_user_id (user_id),
    INDEX idx_orders_status (status),
    INDEX idx_orders_created_at (created_at),
    INDEX idx_orders_deleted_at (deleted_at),
    CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS order_items (
    id         VARCHAR(36)    NOT NULL PRIMARY KEY,
    order_id   VARCHAR(36)    NOT NULL,
    product_id VARCHAR(36)    NOT NULL,
    quantity   INT            NOT NULL DEFAULT 1,
    unit_price DECIMAL(15, 2) NOT NULL COMMENT 'Snapshot of product price at order time',
    subtotal   DECIMAL(15, 2) NOT NULL COMMENT 'quantity * unit_price',
    INDEX idx_order_items_order_id (order_id),
    INDEX idx_order_items_product_id (product_id),
    CONSTRAINT fk_order_items_order   FOREIGN KEY (order_id)   REFERENCES orders   (id) ON DELETE CASCADE,
    CONSTRAINT fk_order_items_product FOREIGN KEY (product_id) REFERENCES products (id) ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
