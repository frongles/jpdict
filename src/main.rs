use std::fs;
use clap::Parser;
use sqlx::sqlite;
use reqwest::blocking::get;
use tokio;

mod jmxml;
mod sql;

const DICT_SERVER   : &str = "http://ftp.edrdg.org/pub/Nihongo/JMdict_b.gz";
const TARGET_FILE   : &str = "JMdict_b.gz";

#[derive(Parser, Debug)]
struct Args {

    #[arg(short, long)]
    fetch_data : bool,

    #[arg(short, long)]
    rebuild_db : bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.fetch_data { fetch_data(); }
    if args.rebuild_db { 
        rebuild_db().await;
    }
    
    let pool = sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect(sql::DB_FILE)
        .await
        .unwrap();



}


fn fetch_data () {
    match fs::remove_file(TARGET_FILE) {
        Ok(()) => eprintln!("Removed {}", TARGET_FILE),
        Err(e) => eprintln!("Failed to remove file {}: {}", TARGET_FILE, e),
    }
    let mut resp = get(DICT_SERVER).unwrap();
    let mut out = fs::File::create(TARGET_FILE).unwrap();
    std::io::copy(&mut resp, &mut out).unwrap();

}

async fn rebuild_db () {

    sql::rebuild_db().await.unwrap();

}





/// ------------------------------------------------------------------------
/// Tests
/// ------------------------------------------------------------------------

#[test]
#[ignore]
fn test_fetch_data() {

    fetch_data();

} 