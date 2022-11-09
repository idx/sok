use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   query: String,
}

fn main() {
    let args = Args::parse();

    println!("QUERY: {}", args.query)
}
