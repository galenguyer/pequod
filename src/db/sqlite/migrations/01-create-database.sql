CREATE TABLE IF NOT EXISTS repositories (
    name TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS manifests (
    digest TEXT NOT NULL PRIMARY KEY ON CONFLICT IGNORE,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS blobs (
    digest TEXT NOT NULL PRIMARY KEY ON CONFLICT REPLACE,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tags (
    name TEXT NOT NULL,
    repository TEXT NOT NULL,
    manifest TEXT NOT NULL,
    FOREIGN KEY (repository) REFERENCES repositories (name) ON DELETE CASCADE,
    FOREIGN KEY (manifest) REFERENCES manifests (digest) ON DELETE CASCADE,
    PRIMARY KEY (name, repository) ON CONFLICT REPLACE
);
