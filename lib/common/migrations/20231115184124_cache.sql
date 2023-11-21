CREATE TABLE IF NOT EXISTS cache_config
(
    ns_id BIGINT NOT NULL,
    relation TEXT NOT NULL,
    duration BIGINT,
    CONSTRAINT fk_namespace
    FOREIGN KEY(ns_id)
    REFERENCES namespaces(id)
);

ALTER TABLE jobs ADD COLUMN cache_path TEXT;
ALTER TABLE jobs ADD COLUMN expires_at TIMESTAMP;