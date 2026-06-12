-- Migration: Add image_url and stock to products table
-- Created: 2026-06-12

ALTER TABLE products
    ADD COLUMN image_url VARCHAR(500) NULL AFTER description,
    ADD COLUMN stock     INT          NOT NULL DEFAULT 0 AFTER image_url;
