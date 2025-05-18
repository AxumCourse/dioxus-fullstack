CREATE TABLE IF NOT EXISTS "images"(
    "id" CHAR(20) PRIMARY KEY,
    "hash" CHAR(64) NOT NULL,
    "file_path" VARCHAR(255) NOT NULL,
    "file_size" BIGINT NOT NULL DEFAULT 0
);