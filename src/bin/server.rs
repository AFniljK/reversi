use clap::Parser;
use reversi::cli::Args;
use std::net::{TcpListener, TcpStream};
use tungstenite::{accept, Message, WebSocket};

fn handle_player(black: &mut WebSocket<TcpStream>, white: &mut WebSocket<TcpStream>) {
    black.send(Message::text(String::from("black"))).unwrap();
    white.send(Message::text(String::from("white"))).unwrap();

    let message = black.read().unwrap();
    println!("black: {:?}", message);
    let message = white.read().unwrap();
    println!("white: {:?}", message);

    let mut position: Message;
    loop {
        position = black.read().unwrap();
        white.send(position.clone()).unwrap();
        position = white.read().unwrap();
        black.send(position.clone()).unwrap();
    }
}

fn main() {
    let args = Args::parse();
    let server = TcpListener::bind(args.addr())
        .expect(&format!("cannot bind to address: {:?}", args.addr()));
    println!("listening on address: {:?}", args.addr());

    let (stream, kawasaki_address) = server.accept().expect(&format!("couldn't connect player"));
    let mut kawasaki = accept(stream).expect("cannot upgrade connection to websocket");
    println!("address: {:?} is connected!", kawasaki_address);

    kawasaki.send(Message::text(String::from("pick"))).unwrap();
    let message: String = kawasaki.read().unwrap().into_text().unwrap().to_string();
    let kawasaki_black = message == String::from("black");

    let (stream, yamauchi_address) = server.accept().expect(&format!("couldn't connect player"));
    let yamauchi = accept(stream).expect("cannot upgrade connection to websocket");
    println!("address: {:?} is connected!", yamauchi_address);

    let (mut black, mut white) = if kawasaki_black {
        (kawasaki, yamauchi)
    } else {
        (yamauchi, kawasaki)
    };
    handle_player(&mut black, &mut white);
}
