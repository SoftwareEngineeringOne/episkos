CREATE TABLE metadata (
    id BLOB PRIMARY KEY NOT NULL,
    directory TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    repository_url TEXT,
    preferred_ide INTEGER,
    created BLOB NOT NULL,
    updated BLOB NOT NULL,
    FOREIGN KEY (preferred_ide) REFERENCES ide(id)
);

CREATE TABLE build_system (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    CONSTRAINT unq UNIQUE (name, version)
);

CREATE TABLE ide (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    CONSTRAINT unq UNIQUE (name, version)
);

CREATE TABLE language (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    CONSTRAINT unq UNIQUE (name, version)
);

CREATE TABLE category (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE rel_metadata_category (
    metadata_id BLOB NOT NULL,
    category_id INTEGER NOT NULL,
    FOREIGN KEY (metadata_id) REFERENCES metadata(id),
    FOREIGN KEY (category_id) REFERENCES category(id)
);

CREATE TABLE rel_metadata_language (
    metadata_id BLOB NOT NULL,
    language_id INTEGER NOT NULL,
    FOREIGN KEY (metadata_id) REFERENCES metadata(id),
    FOREIGN KEY (language_id) REFERENCES language(id)
);

CREATE TABLE rel_metadata_build_system (
    metadata_id BLOB NOT NULL,
    build_system_id INTEGER NOT NULL,
    FOREIGN KEY (metadata_id) REFERENCES metadata(id),
    FOREIGN KEY (build_system_id) REFERENCES build_system(id)
);
