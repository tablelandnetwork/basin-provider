CREATE TABLE IF NOT EXISTS jobs
(
    id BIGSERIAL PRIMARY KEY,
    ns_id BIGINT, 
    cid BYTEA NOT NULL,
    relation TEXT NOT NULL,
    activated TIMESTAMP,
    CONSTRAINT fk_namespace
        FOREIGN KEY(ns_id) 
	          REFERENCES namespaces(id)
);