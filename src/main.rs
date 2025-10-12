use clap::Parser;
mod jmxml;



#[derive(Parser, Debug)]
struct Args {

    #[arg(short, long)]
    fetch_data : bool,

    #[arg(short, long)]
    rebuild_db : bool,
}


fn main() {
    let args = Args::parse();

    //if args.fetch_data { fetch_data(); }
    if args.rebuild_db { rebuild_db(); }


}



fn rebuild_db() {

}