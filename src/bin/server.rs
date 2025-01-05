use clap::Parser;
use reversi::cli::Args;
use tungstenite::{accept, Message};
use std::net::TcpListener;

fn main() {
    let args = Args::parse();
    let server = TcpListener::bind(args.addr()).expect(&format!("cannot bind to address: {:?}", args.addr()));
    println!("listening on address: {:?}", args.addr());

    let (stream, kawasaki_address) = server.accept().expect(&format!("couldn't connect player"));
    let mut kawasaki = accept(stream).unwrap();
    println!("address: {:?} is connected!", kawasaki_address);

    let (stream, yamauchi_address) = server.accept().expect(&format!("couldn't connect player"));
    let mut yamauchi = accept(stream).unwrap();
    println!("address: {:?} is connected!", yamauchi_address);

    kawasaki.send(Message::text(format!("{} is your opponent!", yamauchi_address))).unwrap();
    yamauchi.send(Message::text(format!("{} is your opponent!", kawasaki_address))).unwrap();
}