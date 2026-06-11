-- Migration: Create feedback table
-- Created: 2026-06-10

CREATE TABLE IF NOT EXISTS feedback (
    id         VARCHAR(36) NOT NULL PRIMARY KEY,
    user_id    VARCHAR(36) NOT NULL,
    message    TEXT        NOT NULL,
    is_public  TINYINT(1)  NOT NULL DEFAULT 1 COMMENT '0=private, 1=public',
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6),
    deleted_at DATETIME(6) NULL,
    INDEX idx_feedback_user_id (user_id),
    INDEX idx_feedback_is_public (is_public),
    INDEX idx_feedback_deleted_at (deleted_at),
    CONSTRAINT fk_feedback_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
