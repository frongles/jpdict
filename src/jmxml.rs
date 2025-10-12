#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::collections::HashMap;


use serde::Deserialize;
use serde_xml_rs::from_reader;

const CLEAN_JMDICT : &str = "JMdictCLEAN";


#[derive(Debug, Deserialize)]
struct JMdict {
    entry : Vec<Entry>
}

#[derive(Debug, Deserialize)]
struct Entry {
    ent_seq : i32,
    r_ele   : Reading,
    sense   : Sense,
}

#[derive(Debug, Deserialize)]
struct Reading {
    reb     : String,
}

#[derive(Debug, Deserialize)]
struct Sense {
    pos     : String,
    x_ref   : Option<String>,
    gloss   : String,
}



fn parse(file : std::fs::File) {
    let dict : JMdict = from_reader(file).unwrap();
    
}

/// ---------------------------------------------------------
/// Preprocess JMDict file, expanding XML entities as their 
/// full string
/// ---------------------------------------------------------
fn preprocess(file : File) {
    let entities : HashMap<&str, &str> = [
        ("unc", "Unclassified"),
    ]
    .iter()
    .cloned()
    .collect();

    let reader = BufReader::new(file);

    let mut outputfile = File::create(CLEAN_JMDICT).unwrap();

    for line in reader.lines() {
        let mut line = line.unwrap();
        for (entity, replacement) in &entities {
            let pattern = format!("&{}", entity);
            line = line.replace(&pattern, replacement);
        }
        writeln!(outputfile, "{}", line).unwrap();
    }

}






/// ------------------------------------------------------------
/// Test
/// ------------------------------------------------------------

#[test]
fn test_parse() {
    let file = std::fs::File::open("JMdictTest").unwrap();
    preprocess(file);
    let file = File::open(CLEAN_JMDICT).unwrap();

    let dict : JMdict = from_reader(file).unwrap();
    println!("{:?}", dict);
}