use clap::Parser;
use sok::query;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   query: String,
}

fn main() {
    let args = Args::parse();

    println!("QUERY: {}", args.query);

    query::parse();

    sok::new_searcher();
}
