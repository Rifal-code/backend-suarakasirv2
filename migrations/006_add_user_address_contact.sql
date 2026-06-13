-- Migration: Add address and contact to users table
-- Created: 2026-06-13

ALTER TABLE users
    ADD COLUMN address VARCHAR(255) NULL AFTER description,
    ADD COLUMN contact VARCHAR(100) NULL AFTER address;
