use std::{fs, io};
use std::io::copy;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::collections::HashMap;

use flate2::read::GzDecoder;

use sqlite::Connection;
use ureq::get;

use jpdict::jmserde;


const DICT_SERVER       : &str = "http://ftp.edrdg.org/pub/Nihongo/JMdict_b.gz";
const XMLFILE           : &str = "test.xml";
pub const DB_FILE       : &str = "jmdict.db";

fn main() {
    let args = std::env::args();
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


fn read_xml(filename : &str) {
    println!("Preprocessing {}", filename);
    let file = File::open(filename).unwrap();
    let mut entities = HashMap::new(); 
    
    let mut reader = BufReader::new(&file);

    let mut outputfile = File::create("placeholder").unwrap();

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
    while true {
        let line = lines.next().unwrap().unwrap();
        let line = line.as_str();
        let entry = jmserde::Entry {
            ent_seq : 0,
            r_ele   : Vec::new(),
            sense   : Vec::new(),
        };

        match line {
            "<entry>" => {

            },
            "</entry>" => {
                //push entry to database
            }
            _ => panic!("Your loop logic failed"),
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