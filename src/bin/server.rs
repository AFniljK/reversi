use clap::Parser;
use reversi::cli::Args;

fn main() {
    let args = Args::parse();
    println!("listening on: {:?}", args.addr());
}
