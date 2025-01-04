use clap::Parser;
use reversi::cli::Args;
use tungstenite::connect;

fn main() {
    let args = Args::parse();
    let addr = args.addr();
    let (mut stream, response) = connect("ws://".to_owned() + &addr).expect("cannot connect");
    println!(
        "connected to address: {:?}\tstatus: {:?}",
        addr,
        response.status()
    );
    println!("{}", stream.read().unwrap());
}
