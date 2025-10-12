
use serde::Deserialize;
use serde_xml_rs::from_reader;



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
struct Sense;



fn parse(file : std::fs::File) {
    let dict : JMdict = from_reader(file).unwrap();

}