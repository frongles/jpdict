use rusqlite::Connection;
use rusqlite::params;

use crate::jmdict::Kanji;

fn search_reading(reading : &str, conn : &Connection) {
    let s = r#"
        SELECT * FROM entry_full
        WHERE ent_seq IN (
            SELECT ent_seq
            FROM readings
            WHERE reb = ?
        );
    "#;
    let mut stmnt = conn.prepare(s).unwrap();
    let rows = stmnt.query_map(params![reading], |row| {
        Ok((
            row.get::<&str, i64>("ent_seq"),
            row.get::<&str, String>("kanji_list"),
            row.get::<&str, String>("reading_list")
        ))
    }).unwrap();

    for row in rows {
        let row = row.unwrap();
        println!("{}|{}|{}", row.0.unwrap(), row.1.unwrap(), row.2.unwrap());
    }

}

fn select_by_ent_seq(ent_seq : i64, conn : &Connection) {

    let s = r#"
        SELECT * FROM entry_full WHERE ent_seq = ?
    "#;

    let mut stmnt = conn.prepare(s).unwrap();
    let rows = stmnt.query_map(params![ent_seq], |row| {
        Ok((
            row.get::<&str, i64>("ent_seq"),
            row.get::<&str, String>("kanji_list"),
            row.get::<&str, String>("reading_list")
        ))
    }).unwrap();

    for row in rows {
        let row = row.unwrap();
        println!("{}|{}|{}", row.0.unwrap(), row.1.unwrap(), row.2.unwrap());
    }

}

fn get_db_size(conn : &Connection) {

    let s = r#"
SELECT
    name,
    SUM(pgsize) AS total_bytes,
    SUM(pgsize)/1024.0 AS total_kb,
    SUM(pgsize)/1024.0/1024.0 AS total_mb
FROM dbstat
GROUP BY name
ORDER BY total_bytes DESC;
    "#;
    let mut stmnt = conn.prepare(s).unwrap();
    let rows = stmnt.query_map([], |row| {
        Ok((
            row.get::<&str, String>("name"),
            row.get::<&str, i64>("total_bytes"),
            row.get::<&str, f64>("total_kb"),
            row.get::<&str, f64>("total_mb")
        ))}).unwrap();
    for row in rows {
        let row = row.unwrap();
        println!("{}|{}|{}|{}", row.0.unwrap(), row.1.unwrap(), row.2.unwrap(), row.3.unwrap());
    }

}

/// -----------------------------------------------------------------
/// Test 
/// -----------------------------------------------------------------


fn init_db_test() -> Connection {
    Connection::open("jmdict.db").unwrap()
}

#[test]
fn test_select_ent_seq() {
    let conn = init_db_test();
    let ent_seq = 1001980;
    select_by_ent_seq(ent_seq, &conn);
    
}


#[test]
fn test_select_reading() {
    let conn = init_db_test();
    let reading = "ちょっと";
    search_reading(reading, &conn);
}

#[test]
fn test_size() {
    let conn = init_db_test();
    get_db_size(&conn);
}