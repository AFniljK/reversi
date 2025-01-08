use clap::Parser;
use reversi::cli::Args;

fn main() {
    let args = Args::parse();
    println!("connection on {:?}", args.addr());
}
