use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    // Connect to SQLite database
    let path = "backend/data/parish.db";
    let db = Connection::open(path)?;

    // Enable foreign keys
    db.execute("PRAGMA foreign_keys = ON", [])?;

    // USERS table
    db.execute(
        "CREATE TABLE IF NOT EXISTS users (
            user_id     INTEGER PRIMARY KEY,
            first_name  TEXT NOT NULL,
            last_name   TEXT NOT NULL,
            phone       TEXT UNIQUE,
            birth_date  TEXT NOT NULL,
            password    TEXT NOT NULL,
            active      INTEGER NOT NULL DEFAULT 1,
            needs_swap  INTEGER NOT NULL DEFAULT 0,
            swap_notes  TEXT,
            notes       TEXT
        )",
        [],
    )?;

    // ALB table
    db.execute(
        "CREATE TABLE IF NOT EXISTS albs (
            alb_id          INTEGER PRIMARY KEY,
            alb_code        TEXT NOT NULL,
            alb_size        INTEGER NOT NULL,
            adult_alb       INTEGER NOT NULL DEFAULT 1,
            has_accessory   INTEGER NOT NULL DEFAULT 0,
            notes           TEXT
        )",
        [],
    )?;

    // CHURCH table
    db.execute(
        "CREATE TABLE IF NOT EXISTS church (
            church_id       INTEGER PRIMARY KEY,
            church_name     TEXT NOT NULL
        )",
        [],
    )?;

    // ALTAR SERVER <-> ALB ASSIGNMENT table
    db.execute(
        "CREATE TABLE IF NOT EXISTS user_alb (
            id          INTEGER PRIMARY KEY,
            user_id     INTEGER REFERENCES users(user_id),
            alb_id      INTEGER REFERENCES albs(alb_id),
            date        TEXT NOT NULL,

            CHECK (user_id IS NOT NULL OR alb_id IS NOT NULL)
        )",
        [],
    )?;

    // CHURCH <-> ALB ASSIGNMENT table
    db.execute(
        "CREATE TABLE IF NOT EXISTS church_alb (
            id          INTEGER PRIMARY KEY,
            alb_id      INTEGER REFERENCES albs(alb_id),
            church_id   INTEGER REFERENCES church(church_id),
            date        TEXT NOT NULL,

            CHECK (alb_id IS NOT NULL OR church_id IS NOT NULL)
        )",
        [],
    )?;


    // EXAMPLE ROW
    //db.execute(
    //    "INSERT INTO users (first_name, last_name, phone, birth_date, password) VALUES (?1, ?2, ?3, ?4, ?5)",
    //    ("Marco", "Ramos", "964924660", "2000-12-17", "###"),
    //)?;

    println!("Database created and row inserted.");
    Ok(())
}
