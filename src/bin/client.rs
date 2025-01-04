use clap::Parser;
use reversi::cli::Args;
use tungstenite::connect;

fn main() {
    let args = Args::parse();
    let addr = args.addr();
    let (_stream, response) = connect(addr.to_string()).expect("cannot connect");
    println!(
        "connected to address: {:?}\tstatus: {:?}",
        addr,
        response.status()
    );
}
