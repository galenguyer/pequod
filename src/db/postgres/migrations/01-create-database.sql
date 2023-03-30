CREATE TABLE IF NOT EXISTS repositories (
    name TEXT NOT NULL UNIQUE PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS manifests (
    digest TEXT NOT NULL PRIMARY KEY,
    repository TEXT NOT NULL,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS blobs (
    digest TEXT NOT NULL PRIMARY KEY,
    value BYTEA NOT NULL
);

CREATE TABLE IF NOT EXISTS tags (
    name TEXT NOT NULL,
    repository TEXT NOT NULL,
    manifest TEXT NOT NULL,
    updated BIGINT NOT NULL,
    PRIMARY KEY (name, repository)
);

CREATE TABLE IF NOT EXISTS manifest_blobs (
    manifest TEXT NOT NULL,
    blob TEXT NOT NULL,
    PRIMARY KEY (manifest, blob)
);
