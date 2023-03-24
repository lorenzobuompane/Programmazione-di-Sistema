use::clap::Parser;
use slugify::slugify;

#[derive(Parser, Debug)]
struct Args {
    /// Input string
    #[arg(short, long)]
    input: String
}

fn main() {
    let args = Args::parse();
    println!("{}", slugify(args.input.as_str()));
}

