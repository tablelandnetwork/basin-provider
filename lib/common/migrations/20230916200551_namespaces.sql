CREATE TABLE IF NOT EXISTS namespaces
(
    id     BIGSERIAL   PRIMARY KEY,
    name   VARCHAR(32) UNIQUE NOT NULL,
    owner  BYTEA NOT NULL,
    last_export TIMESTAMPTZ
);
