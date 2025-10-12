#![allow(dead_code)]
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Error;
use std::fs;


pub const DB_FILE       : &str = "jmdict.db";

fn refresh() { }

fn insert_entry() { }

fn query() {}


pub async fn rebuild_db() -> Result<(), Error> {

    fs::remove_file(DB_FILE).unwrap();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(DB_FILE)
        .await?;
    // Disable foreign key enforcement in SQLite
    sqlx::query("PRAGMA foreign_keys = OFF;")
        .execute(&pool)
        .await?;

    // Recreate tables in dependency order
    let create_statements = [
        r#"
        CREATE TABLE IF NOT EXISTS entries (
            ent_seq INTEGER PRIMARY KEY
        );
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS japanese_readings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ent_seq INTEGER NOT NULL,
            reading TEXT NOT NULL,
            FOREIGN KEY (ent_seq) REFERENCES entries(ent_seq)
        );
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS metadata (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ent_seq INTEGER NOT NULL,
            type TEXT NOT NULL,
            value TEXT NOT NULL,
            FOREIGN KEY (ent_seq) REFERENCES entries(ent_seq)
        );
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS english_glosses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ent_seq INTEGER NOT NULL,
            gloss TEXT NOT NULL,
            FOREIGN KEY (ent_seq) REFERENCES entries(ent_seq)
        );
        "#
    ];

    for stmt in create_statements.iter() {
        sqlx::query(stmt).execute(&pool).await?;
    }

    // Re-enable foreign key enforcement
    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await?;

    Ok(())
}