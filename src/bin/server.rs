use std::{
    io::{Read, Write},
    net::TcpListener,
};

use clap::Parser;
use reversi::cli::Args;

fn main() {
    let args = Args::parse();

    let server = TcpListener::bind(args.addr()).expect("cannot bind on given address");
    println!("listening on: {:?}", args.addr());
    let (mut stream, addr) = server.accept().expect("cannot accept connection");
    println!("connected from: {:?}", addr);

    stream
        .write("pick".as_bytes())
        .expect("cannot write to connected stream");

    let mut buf = [0; 1024];
    let read = stream.read(&mut buf).unwrap();
    let response = String::from_utf8(buf[0..read].to_vec()).unwrap();
    println!("{addr} plays {response}");

    let (mut challenger, addr) = server.accept().expect("cannot accept connection");
    challenger.set_read_timeout(None).unwrap();
    println!("connected from: {:?}", addr);
    if response == "black".to_string() {
        stream.write(b"black").unwrap();
        challenger.write(b"white").unwrap();
    } else {
        stream.write(b"white").unwrap();
        challenger.write(b"black").unwrap();
    }

    let mut challenger_plays = response == "white".to_string();

    loop {
        let mut buf = [0; 1024];
        if challenger_plays {
            challenger.read(&mut buf).unwrap();
            stream.write(&buf).unwrap();
        } else {
            stream.read(&mut buf).unwrap();
            challenger.write(&buf).unwrap();
        }
        challenger_plays = !challenger_plays;
    }
}
