use std::{fs, io};
use std::io::copy;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::collections::HashMap;

use flate2::read::GzDecoder;

use ureq::get;
use rusqlite::Connection;
use rusqlite::params;


use jpdict::jmdict;


const DICT_SERVER       : &str = "http://ftp.edrdg.org/pub/Nihongo/JMdict_b.gz";
const XMLFILE           : &str = "JMdict_b.xml";
pub const DB_FILE       : &str = "jmdict.db";

fn main() {
    let args = std::env::args();
    for argument in args {
        match argument.as_str() {
            "--fetch" => fetch_data(DICT_SERVER, XMLFILE),
            _ => (),
        }
    }
    let conn = rebuild_db();

    read_xml(XMLFILE, &conn);

    build_ind(&conn);

    println!("Imported data success");

}

/// -------------------------------------------------
/// Fetch Data
/// -------------------------------------------------
/// Fetches dictionary data and decompresses the file
/// -------------------------------------------------
fn fetch_data (url : &str, fileout : &str) {
    println!("Fetching data from {}", url);
    let gzfile = "temp.gz";
    match fs::remove_file(gzfile) {
        Ok(()) => eprintln!("Removed {}", gzfile),
        Err(e) => eprintln!("Failed to remove file {}: {}", gzfile, e),
    }
    let resp = get(url)
        .call()
        .unwrap()
        .body_mut()
        .with_config()
        .limit(20 * 1024 * 1024)
        .read_to_vec()
        .unwrap();
    fs::write(gzfile, resp).unwrap();
    decompress(gzfile, fileout);
    fs::remove_file(gzfile).unwrap();
}

/// ----------------------------------------------------
/// decompress
/// ----------------------------------------------------
/// decompresses the fetched .gz file
/// ----------------------------------------------------
fn decompress(filename : &str, fileout : &str) {
    let f = File::open(filename).unwrap();
    let mut gz = GzDecoder::new(f);

    let out = File::create(fileout).unwrap();

    copy(&mut gz, &mut io::BufWriter::new(out)).unwrap();

}
/// -----------------------------------------------------
/// rebuild_db
/// -----------------------------------------------------
/// Deletes database and sets up tables
/// -----------------------------------------------------
fn rebuild_db() -> Connection {

    match fs::remove_file(DB_FILE) {
        Ok(()) => println!("Removed database"),
        Err(e) => eprintln!("Failed to remove database: {}", e),
    }
    let conn = Connection::open(DB_FILE).unwrap();
    // Disable foreign key enforcement in SQLite
    conn.execute("PRAGMA foreign_keys = OFF;", ()).unwrap();
    let _ = conn.query_row("PRAGMA journal_mode = WAL;", [], |_| Ok(())).unwrap();
    conn.execute("PRAGMA synchronous = OFF;", ()).unwrap();
    conn.execute("BEGIN TRANSACTION;", ()).unwrap();

    // Recreate tables in dependency order
    let create_statements = [
        r#"
        CREATE TABLE IF NOT EXISTS entries (
            ent_seq INTEGER PRIMARY KEY
        );
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS japanese_readings (
            reb TEXT PRIMARY KEY
        );
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS kanji (
            keb TEXT PRIMARY KEY
        );
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS sense(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ent_seq INTEGER NOT NULL,
            FOREIGN KEY (ent_seq) REFERENCES entries(ent_seq)
        );
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS eng(
            gloss TEXT PRIMARY KEY
        );
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS entries_kanji (
            ent_seq INTEGER NOT NULL,
            keb TEXT NOT NULL,
            PRIMARY KEY (ent_seq, keb),
            FOREIGN KEY (ent_seq) REFERENCES entries(ent_seq),
            FOREIGN KEY (keb) REFERENCES kanji(keb)
        );
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS entries_readings (
            ent_seq INTEGER NOT NULL,
            reb TEXT NOT NULL,
            PRIMARY KEY (ent_seq, reb),
            FOREIGN KEY (ent_seq) REFERENCES entries(ent_seq),
            FOREIGN KEY (reb) REFERENCES japanese_readings(reb)
        );
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS sense_eng(
            sense_id INTEGER NOT NULL,
            gloss TEXT NOT NULL,
            PRIMARY KEY (sense_id, gloss),
            FOREIGN KEY (sense_id) REFERENCES sense(sense_id),
            FOREIGN KEY (gloss) REFERENCES eng(gloss)
        );
        "#

    ];

    for stmt in create_statements.iter() {
        conn.execute(stmt, ()).unwrap();
    }

    // Re-enable foreign key enforcement
    conn.execute("PRAGMA foreign_keys = ON;", ()).unwrap();
    conn.execute("COMMIT;", ()).unwrap();
    println!("Re initialised database");
    conn
    //Ok(())
}
/// -----------------------------------------------------
/// build_ind
/// -----------------------------------------------------
/// Builds indexes and concatenated tables after forming
/// database
/// -----------------------------------------------------
fn build_ind(conn : &Connection) {

    let statements = [
        r#"
        CREATE TABLE entry_full AS
        SELECT e.ent_seq,
            GROUP_CONCAT(DISTINCT ek.keb) AS kanji_list,
            GROUP_CONCAT(DISTINCT er.reb) AS reading_list,
            GROUP_CONCAT(DISTINCT eng.gloss) AS gloss_list
        FROM entries e
        LEFT JOIN entries_kanji ek ON e.ent_seq = ek.ent_seq
        LEFT JOIN entries_readings er ON e.ent_seq = er.ent_seq
        LEFT JOIN sense s ON s.ent_seq = e.ent_seq
        LEFT JOIN sense_eng se ON s.id = se.sense_id
        LEFT JOIN eng ON eng.gloss = se.gloss
        GROUP BY s.id;
        "#,
        r#"
        CREATE TABLE gloss_entry AS
        SELECT g.gloss,
            GROUP_CONCAT(DISTINCT kr.keb) AS kanji_list,
            GROUP_CONCAT(DISTINCT rr.reb) AS reading_list,
            GROUP_CONCAT(DISTINCT related.gloss) AS related_gloss
        FROM eng g
        LEFT JOIN sense_eng se ON se.gloss = g.gloss
        LEFT JOIN sense s ON se.sense_id = s.id
        LEFT JOIN sense_eng related ON se.sense_id = related.sense_id
        LEFT JOIN entries_kanji kr ON s.ent_seq = kr.ent_seq
        LEFT JOIN entries_readings rr ON s.ent_seq = rr.ent_seq
        GROUP BY se.sense_id;
        "#,
        r#"
        CREATE INDEX IF NOT EXISTS idx_entries_kanji_ent ON entries_kanji(ent_seq);
        "#,
        r#"
        CREATE INDEX IF NOT EXISTS idx_entries_kanji_keb ON entries_kanji(keb);
        "#,
        r#"
        CREATE INDEX IF NOT EXISTS idx_entries_readings_ent
        ON entries_readings(ent_seq);
        "#,
        r#"
        CREATE INDEX IF NOT EXISTS idx_entries_readings_reb 
        ON entries_readings(reb);
        "#,
        r#"
        CREATE INDEX IF NOT EXISTS idx_entry_full ON entry_full(ent_seq);
        "#,
        r#"
        CREATE VIRTUAL TABLE readings_fts
        USING fts5(ent_seq UNINDEXED, reb);
        "#,
        r#"
        CREATE VIRTUAL TABLE gloss_fts
        USING fts5(ent_seq UNINDEXED, gloss);
        "#,
        r#"
        INSERT INTO gloss_fts
        SELECT * FROM sense_eng;
        "#
    ];
    for stmt in statements.iter() {
        conn.execute(stmt, ()).unwrap();
    }




}

/// -----------------------------------------------------
/// read_xml
/// -----------------------------------------------------
/// Reads data from JMdict_b.xml, inserting into database
/// while parsing
/// -----------------------------------------------------
fn read_xml(filename : &str, conn : &Connection) {
    let file = File::open(filename).unwrap();
    let mut entities = HashMap::new(); 

    let mut reader = BufReader::new(&file);
    // Collect entity tags for replacement
    for line in reader.by_ref().lines() {
        let line = line.unwrap();
        // Read until first XML tag
        if line.as_str() == "<JMdict>" {
            break;
        }
        // Ignore any lines that aren't entities
        if !line.starts_with("<!ENTITY") {
            continue;
        }
        // Add entity to hash map
        let parts : Vec<&str> = line.splitn(3, ' ').collect();
        let name = parts[1].to_string();
        let value = parts[2]
            .trim_start_matches("\"")
            .trim_end_matches("\">")
            .to_string();
        entities.insert(name, value);
    }
    let mut lines = reader.lines();
    let mut count = 0;
    // State machine for reading entries into database
    conn.execute("BEGIN TRANSACTION;",()).unwrap();
    loop {
        let line = lines.next().unwrap().unwrap();
        let line = line.trim();
        let mut entry = jmdict::Entry::default();

        match line {
            "<entry>" => {
                loop {
                    let line = lines.next().unwrap().unwrap();
                    let line = line.trim();
                    // get entry sequence
                    if line.starts_with("<ent_seq>") { 
                        let content = strip_xml(line, "ent_seq", &entities);
                        entry.ent_seq = content.parse().unwrap();
                    }
                    // get kanji elements
                    else if line.starts_with("<k_ele>") {
                        let mut kanji = jmdict::Kanji::default();
                        loop {
                            let line = lines.next().unwrap().unwrap();
                            let line = line.trim();
                            if line.starts_with("<keb>") {
                                kanji.keb = strip_xml(line, "keb", &entities);
                            }
                            if line == "</k_ele>" { break }
                        }
                        entry.k_ele.push(kanji);
                    }
                    // get reading elements
                    else if line.starts_with("<r_ele>") {
                        let mut reading = jmdict::Reading::default();
                        loop {
                            let line = lines.next().unwrap().unwrap();
                            let line = line.trim();
                            if line.starts_with("<reb>") {  
                                reading.reb = strip_xml(line, "reb", &entities);
                            } 
                            else if line.starts_with("<re_pri>") {}
                            else if line.starts_with("<re_inf>") {}
                            else if line == "</r_ele>" { break }
                        }
                        entry.r_ele.push(reading);
                    }
                    // get sense elements
                    else if line == "<sense>" {
                        let mut sense = jmdict::Sense::default();
                        loop {
                            let line = lines.next().unwrap().unwrap();
                            let line = line.trim();
                            if line.starts_with("<pos>") { 
                                sense.pos.push(strip_xml(line, "pos", &entities))
                            }
                            else if line.starts_with("<gloss>") {
                                sense.gloss.push(strip_xml(line, "gloss", &entities))
                            }
                            else if line.starts_with("<x_ref>") {
                                sense.x_ref.push(strip_xml(line, "x_ref", &entities))
                            }
                            else if line.starts_with("<misc>") {
                                sense.misc.push(strip_xml(line, "misc", &entities))
                            }
                            else if line == "</sense>" { break }
                        }
                        entry.sense.push(sense);

                    }
                    else if line == "</entry>" { 
                        insert_entry(entry, &conn);
                        count += 1;
                        break;
                    }

                }
            },
            "</JMdict>" => {
                println!("Total entries: {}", count);
                conn.execute("COMMIT;", ()).unwrap();
                break;
            }
            _ => panic!("Your loop logic failed"),
        }
    }
}



// Strips xml tags from an inline element and returns the value
fn strip_xml(line : &str, tag : &str, map : &HashMap<String, String>) -> String {
    let line = line.to_string();
    
    // Get value from xml
    let mut val = line.strip_prefix(&format!("<{}>", tag))
        .and_then(|s| s.strip_suffix(&format!("</{}>", tag)))
        .unwrap()
        .to_string();

    // Expand if it's an entity
    if line.contains("&") {
        match map.get(&val[1..val.len()-1]) {
            Some(value) => val = value.to_string(),
            None => (),
        }
    }
    val
}


fn insert_entry(entry: jmdict::Entry, conn: &Connection) {
    // Prepare each statement once
    let insert_entry = "INSERT INTO entries (ent_seq) VALUES (?);";
    let insert_reading = "INSERT OR IGNORE INTO japanese_readings (reb) VALUES (?);";
    let insert_kanji = "INSERT OR IGNORE INTO kanji (keb) VALUES (?);";
    let link_rd = "INSERT INTO entries_readings (ent_seq, reb) VALUES (?, ?);";
    let link_kj = "INSERT INTO entries_kanji (ent_seq, keb) VALUES (?, ?);";
    let insert_sense = "INSERT INTO sense(ent_seq) VALUES (?);";
    let insert_gloss = "INSERT OR IGNORE INTO eng(gloss) VALUES (?);";
    let link_sense_gloss = "INSERT INTO sense_eng(sense_id, gloss) VALUES (?, ?);";

    // Insert the entry
    conn.execute(insert_entry, params![entry.ent_seq]).unwrap();

    // Insert readings and links
    for reading in entry.r_ele {
        conn.execute(insert_reading, params![reading.reb.as_str()]).unwrap();

        conn.execute(link_rd, params![entry.ent_seq, reading.reb.as_str()]).unwrap();
    }

    // Insert kanji and links
    for kanji in entry.k_ele {
        conn.execute(insert_kanji, params![kanji.keb.as_str()]).unwrap();
        conn.execute(link_kj, params![entry.ent_seq, kanji.keb.as_str()]).unwrap();
    }
    
    // Insert english meanings
    for sense in entry.sense {
        conn.execute(insert_sense, params![entry.ent_seq]).unwrap();
        let sense_id = conn.last_insert_rowid();
        
        for gloss in sense.gloss {
            conn.execute(insert_gloss, params![gloss.as_str()]).unwrap();

            conn.execute(link_sense_gloss,
                params![sense_id, gloss])
                .unwrap();
        }
    }

}


/// ----------------------------------
/// Tests
/// ----------------------------------
#[cfg(test)]
use std::path::Path;

#[test]
#[ignore]
fn test_fetch_data() {
    fetch_data(DICT_SERVER, XMLFILE);
    let path = Path::new(XMLFILE);

    assert!(path.exists());
}

#[test]
#[ignore]
fn test_decompress() {
    decompress("JMdict_b.gz", XMLFILE);
}
