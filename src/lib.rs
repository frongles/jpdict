pub mod jmdict;


/// -----------------------------------------------------------------
/// Test 
/// -----------------------------------------------------------------
#[cfg(test)]
mod test {
    use rusqlite::Connection;
    use rusqlite::params;

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
            println!("{}|{}|{}|{}",
                row.0.unwrap(),
                row.1.unwrap(),
                row.2.unwrap(),
                row.3.unwrap());
        }

    }


    fn get_by_gloss(gloss : &str, conn : &Connection) {
        let s = r#"
    SELECT
        s.id AS sense_id,
        s.ent_seq AS ent_seq,
        
        -- First kanji by highest priority
        (SELECT kr.keb
         FROM kanji kr
         WHERE kr.ent_seq = s.ent_seq
         ORDER BY 
            CASE 
                WHEN kr.pri LIKE 'news%' THEN 1
                WHEN kr.pri LIKE 'nf%' THEN 2
                ELSE 3
            END,
            kr.keb
         LIMIT 1) AS kanji,

        -- First reading by highest priority
        (SELECT rr.reb
         FROM readings rr
         WHERE rr.ent_seq = s.ent_seq
         ORDER BY 
            CASE 
                WHEN rr.pri LIKE 'news%' THEN 1
                WHEN rr.pri LIKE 'nf%' THEN 2
                ELSE 3
            END,
            rr.reb
         LIMIT 1) AS reading,

        -- You can still concatenate glosses
        GROUP_CONCAT(DISTINCT se.gloss) AS gloss_list

    FROM sense s
    LEFT JOIN sense_eng se ON s.id = se.sense_id
    WHERE s.id IN (
    SELECT sense_id 
    FROM sense_eng
    WHERE gloss = ?
    )
    GROUP BY s.id;
        "#;
        let mut stmnt = conn.prepare(s).unwrap();
        let rows = stmnt.query_map(params![gloss], |row| {
            Ok((
                row.get::<&str, i64>("sense_id"),
                row.get::<&str, i64>("ent_seq"),
                row.get::<&str, String>("kanji"),
                row.get::<&str, String>("reading"),
                row.get::<&str, String>("gloss_list")
            ))}).unwrap();
        for row in rows {
            let row = row.unwrap();
            println!("{}|{}|{}|{}|{}",
                row.0.unwrap(),
                row.1.unwrap(),
                row.2.unwrap(),
                row.3.unwrap(),
                row.4.unwrap());
        }
    }


    fn get_by_ent_seq(ent_seq : i64, conn : &Connection) {
        let s = r#"
    SELECT se.sense_id,
        GROUP_CONCAT(DISTINCT se.gloss) AS gloss_list
    FROM 
    sense s
    LEFT JOIN
    sense_eng se ON se.sense_id = s.id
    WHERE
    s.ent_seq = ?
    GROUP BY se.sense_id;
        "#;
       let mut stmnt = conn.prepare(s).unwrap();
       let rows = stmnt.query_map(params![ent_seq], |row| {
           Ok((
                   row.get::<&str, i64>("sense_id"),
                   row.get::<&str, String>("gloss_list")
           ))}).unwrap();
       for row in rows {
           let row = row.unwrap();
            println!("{}|{}",
                row.0.unwrap(),
                row.1.unwrap());
        }
    }



    fn init_db_test() -> Connection {
        Connection::open("jmdict.db").unwrap()
    }

    #[test]
    fn test_get_by_ent_seq() {
        let conn = init_db_test();
        let ent_seq = 1001980;
        let start = std::time::Instant::now();
        get_by_ent_seq(ent_seq, &conn);
        println!("Elapsed: {:?}", start.elapsed());
        
    }


    #[test]
    fn test_size() {
        let conn = init_db_test();
        let start = std::time::Instant::now();
        get_db_size(&conn);
        println!("Elapsed: {:?}", start.elapsed());
    }


    #[test]
    fn test_get_gloss() {
        let conn = init_db_test();
        let start = std::time::Instant::now();
        get_by_gloss("to run", &conn);
        println!("Elapsed: {:?}", start.elapsed());
    }
}
