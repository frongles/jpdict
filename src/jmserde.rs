#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::collections::HashMap;

use serde::Deserialize;
use serde_xml_rs::from_reader;

const CLEAN_JMDICT : &str = "JMdictCLEAN";


#[derive(Debug, Deserialize)]
pub struct JMdict {
    pub entry : Vec<Entry>
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub ent_seq : i32,
    pub r_ele   : Vec<Reading>,
    pub sense   : Vec<Sense>,
}

#[derive(Debug, Deserialize)]
pub struct Reading {
    pub reb     : String,
}

#[derive(Debug, Deserialize)]
pub struct Sense {
    pub pos     : Vec<String>,
    pub x_ref   : Option<String>,
    pub gloss   : Vec<String>,
}



pub fn parse(file : &str) -> JMdict {
    let file = std::fs::File::open(file).unwrap();
    from_reader(file).unwrap()
}


/// ---------------------------------------------------------
/// Preprocess JMDict file, expanding XML entities as their 
/// full string
/// ---------------------------------------------------------
fn preprocess(filename : &str) {
    println!("Preprocessing {}", filename);
    let file = File::open(filename).unwrap();
    let mut entities = HashMap::new(); 
    
    let mut reader = BufReader::new(&file);

    let mut outputfile = File::create(CLEAN_JMDICT).unwrap();

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

    writeln!(outputfile, "<JMdict>").unwrap();
    // Replace entities will full string in each line
    for line in reader.lines() {
        let mut line = line.unwrap();
        // replace entities
        if line.contains("&") {
            for (entity, replacement) in &entities {
                let pattern = format!("&{};", entity);
                line = line.replace(&pattern, replacement);
            }
        }
        // Write to output file
        writeln!(outputfile, "{}", line).unwrap();
    }
}






/// ------------------------------------------------------------
/// Test
/// ------------------------------------------------------------

#[test]
fn test_parse() {
    preprocess("test.xml");
    let file = File::open(CLEAN_JMDICT).unwrap();

    let dict : JMdict = from_reader(file).unwrap();
    println!("{:?}", dict);
}