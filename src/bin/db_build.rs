use std::{fs, io};
use std::io::copy;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::collections::HashMap;

use flate2::read::GzDecoder;

use sqlite::Connection;
use ureq::get;

use jpdict::jmdict;


const DICT_SERVER       : &str = "http://ftp.edrdg.org/pub/Nihongo/JMdict_b.gz";
const XMLFILE           : &str = "test.xml";
pub const DB_FILE       : &str = "jmdict.db";

fn main() {
/*    let args = std::env::args();
    for argument in args {
        match argument.as_str() {
            "--fetch" => fetch_data(DICT_SERVER, XMLFILE),
            _ => (),
        }
    }
    let conn = Connection::open(DB_FILE).unwrap();
    rebuild_db(conn);

    let jmdict = jmserde::parse(XMLFILE);

    for entry in jmdict.entry {
        //insert entry
    }
*/

    read_xml("test.xml");


    println!("Exit success");
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
fn rebuild_db(conn : Connection) {

    match fs::remove_file(DB_FILE) {
        Ok(()) => println!("Removed database"),
        Err(e) => eprintln!("Failed to remove database: {}", e),
    }

    // Disable foreign key enforcement in SQLite
    conn.execute("PRAGMA foreign_keys = OFF;").unwrap();
    conn.execute("PRAGMA journal_mode = WAL;").unwrap();
    conn.execute("PRAGMA synchronous = OFF;").unwrap();

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
        conn.execute(stmt).unwrap();
    }

    // Re-enable foreign key enforcement
    conn.execute("PRAGMA foreign_keys = ON;").unwrap();
    //Ok(())
}
/// -----------------------------------------------------
/// read_xml
/// -----------------------------------------------------
/// Reads data from JMdict_b.xml, inserting into database
/// while parsing
/// -----------------------------------------------------
fn read_xml(filename : &str) {
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

    // State machine for reading entries into database
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
                    else if line == "</entry>" { break }

                }
                println!("{:?}", entry);
            },
            "</JMdict>" => break,
            _ => panic!("Your loop logic failed"),
        }
    }
}

// Strips xml tags from an inline element and returns the value
fn strip_xml(line : &str, tag : &str, map : &HashMap<String, String>) -> String {
    let mut line = line.to_string();
    if line.contains('&') {
        for (entity, replacement) in map.iter(){
            let pattern = format!("&{};", entity);
            line = line.replace(&pattern, replacement);
        }
    }
    line.strip_prefix(&format!("<{}>", tag))
        .and_then(|s| s.strip_suffix(&format!("</{}>", tag)))
        .unwrap()
        .to_string()
}

fn insert_entry(entry : jmdict::Entry) {



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


#[test]
fn test_read_xml() {
    read_xml("JMdict_b.xml");
}
