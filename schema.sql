-- 1. Main dictionary entries
CREATE TABLE entries (
    ent_seq INTEGER PRIMARY KEY
);

-- 2. Japanese readings
CREATE TABLE japanese_readings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ent_seq INTEGER NOT NULL,
    reading TEXT NOT NULL,
    FOREIGN KEY (ent_seq) REFERENCES entries(ent_seq)
);

-- 3. Metadata (part-of-speech, misc)
CREATE TABLE metadata (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ent_seq INTEGER NOT NULL,
    type TEXT NOT NULL,   -- e.g., "pos" or "misc"
    value TEXT NOT NULL,
    FOREIGN KEY (ent_seq) REFERENCES entries(ent_seq)
);

-- 4. English glosses
CREATE TABLE english_glosses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ent_seq INTEGER NOT NULL,
    gloss TEXT NOT NULL,
    FOREIGN KEY (ent_seq) REFERENCES entries(ent_seq)
);

