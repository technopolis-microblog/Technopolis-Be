CREATE TABLE IF NOT EXISTS oauth_app (
    id VARCHAR(26) PRIMARY KEY,
    name VARCHAR(32) NOT NULL,
    secret VARCHAR(128) NOT NULL,
    redirect_uri VARCHAR NOT NULL,
    scopes VARCHAR NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    website VARCHAR NOT NULL
);
