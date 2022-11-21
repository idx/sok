use clap::Parser;
use sok::query;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   query: String,

   #[clap(short, long, default_value = "~/.sok", help = "Search index")]
   index: String,
}

fn main() {
    let args = Args::parse();

    println!("QUERY: {}", args.query);

    query::parse();

    sok::new_searcher();
}
