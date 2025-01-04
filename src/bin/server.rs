use clap::Parser;
use reversi::cli::Args;
use std::net::TcpListener;

fn main() {
    let args = Args::parse();
    let addr = args.addr();
    let server = TcpListener::bind(args.addr()).expect(&format!("cannot bind to address: {:?}", addr));
    println!("listening on address: {:?}", addr);

    let (_kawasaki_stream, kawasaki_addr) = server.accept().expect(&format!("couldn't connect player"));
    println!("address: {:?} is connected!", kawasaki_addr);

    let (_yamauchi_stream, yamauchi_addr) = server.accept().expect(&format!("couldn't connect player"));
    println!("address: {:?} is connected!", yamauchi_addr);
}
