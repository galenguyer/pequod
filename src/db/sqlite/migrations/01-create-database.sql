CREATE TABLE IF NOT EXISTS repositories (
    name TEXT NOT NULL PRIMARY KEY ON CONFLICT IGNORE
);

CREATE TABLE IF NOT EXISTS manifests (
    digest TEXT NOT NULL PRIMARY KEY ON CONFLICT IGNORE,
    repository TEXT NOT NULL,
    value TEXT NOT NULL,
    FOREIGN KEY (repository) REFERENCES repositories (name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS blobs (
    digest TEXT NOT NULL PRIMARY KEY ON CONFLICT REPLACE,
    value BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS tags (
    name TEXT NOT NULL,
    repository TEXT NOT NULL,
    manifest TEXT NOT NULL,
    updated INTEGER NOT NULL,
    FOREIGN KEY (repository) REFERENCES repositories (name) ON DELETE CASCADE,
    FOREIGN KEY (manifest) REFERENCES manifests (digest) ON DELETE CASCADE,
    PRIMARY KEY (name, repository) ON CONFLICT REPLACE
);

CREATE TABLE IF NOT EXISTS manifest_blobs (
    manifest TEXT NOT NULL,
    blob TEXT NOT NULL,
    CONSTRAINT fk_manifest FOREIGN KEY (manifest) REFERENCES manifests (digest) ON DELETE CASCADE,
    CONSTRAINT fk_blob FOREIGN KEY (blob) REFERENCES blobs (digest) ON DELETE CASCADE,
    PRIMARY KEY (manifest, blob) ON CONFLICT IGNORE
);
