use sqlite::Connection;



fn search_reading(reading : &str, conn : &Connection) {
    let s = r#"
        SELECT * FROM entry_full
        WHERE ent_seq IN (
            SELECT ent_seq
            FROM entries_readings
            WHERE reb = ?
        );
    "#;

    for row in conn
        .prepare(s)
        .unwrap()
        .into_iter()
        .bind((1, reading))
        .unwrap()
        .map(|row| row.unwrap())
    {
        println!("{}|{}|{}",
            row.read::<i64, _>("ent_seq"),
            row.read::<&str, _>("kanji_list"),
            row.read::<&str, _>("reading_list")
    );
    }
}

fn select_by_ent_seq(ent_seq : i64, conn : &Connection) {

    let s = r#"
        SELECT * FROM entry_full WHERE ent_seq = ?
    "#;

    for row in conn
        .prepare(s)
        .unwrap()
        .into_iter()
        .bind((1, ent_seq))
        .unwrap()
        .map(|row| row.unwrap())
    {
        println!("{}|{}|{}",
            row.read::<i64, _>("ent_seq"),
            row.read::<&str, _>("kanji_list"),
            row.read::<&str, _>("reading_list")
        )
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
