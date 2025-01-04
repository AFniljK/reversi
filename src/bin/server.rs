use std::{
    net::{SocketAddr, TcpListener},
    str::FromStr,
};

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    // set server host address
    #[arg(long, short, default_value = "127.0.0.1")]
    address: String,
    // set server host port
    #[arg(long, short, default_value_t = 3000)]
    port: u16,
}

impl Args {
    fn addr(&self) -> SocketAddr {
        SocketAddr::from_str(&(self.address.clone() + ":" + &self.port.to_string()))
            .expect("invalid socket address")
    }
}

fn main() {
    let args = Args::parse();
    let addr = args.addr();
    let server =
        TcpListener::bind(args.addr()).expect(&format!("cannot bind to address: {:?}", addr));
    println!("listening on address: {:?}", addr);

    let (_kawasaki_stream, kawasaki_addr) =
        server.accept().expect(&format!("couldn't connect player"));
    println!("address: {:?} is connected!", kawasaki_addr);

    let (_yamauchi_stream, yamauchi_addr) =
        server.accept().expect(&format!("couldn't connect player"));
    println!("address: {:?} is connected!", yamauchi_addr);
}
