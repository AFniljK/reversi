use std::{
    io::{Read, Write},
    net::TcpStream,
};

use clap::Parser;
use ggez::{conf, event, ContextBuilder};
use reversi::{
    available_captures,
    cli::Args,
    gui::{Board, Move, PieceConfig, Player},
};

struct Enemy {
    stream: TcpStream,
}

impl Player for Enemy {
    fn play_move(&mut self, _config: &PieceConfig) -> Move {
        let position = loop {
            let mut buf = [0; 8];
            match self.stream.read(&mut buf) {
                Ok(0) => continue,
                Ok(_) => {
                    let position = u64::from_le_bytes(buf);
                    if position != 0 {
                        break position;
                    }
                }
                _ => {}
            }
        };
        Move::Position(position)
    }

    fn enemy_move(&mut self, current_move: u64) {
        let bytes = &current_move.to_le_bytes();
        println!("sent buf: {:?}", bytes);
        self.stream.write(bytes).unwrap();
    }
}

struct Ally {}

impl Player for Ally {
    fn play_move(&mut self, _config: &PieceConfig) -> Move {
        Move::Board
    }

    fn enemy_move(&mut self, _current_move: u64) {}
}

fn capture(config: &PieceConfig, position: u64) -> PieceConfig {
    let mut config = config.clone();
    let (ally, foe) = config.ally_foe();
    let mesh = available_captures(ally, foe).unwrap();
    println!("{:?}", mesh);
    let mesh = *mesh.get(&position).unwrap() | position;
    if config.blacks_play {
        config.white_pieces &= !mesh;
        config.black_pieces |= mesh;
    } else {
        config.black_pieces &= !mesh;
        config.white_pieces |= mesh;
    }
    config.blacks_play = !config.blacks_play;
    config
}

fn main() {
    let args = Args::parse();
    let mut stream = TcpStream::connect(args.addr()).expect("cannot connect on given address");
    println!("connection on {:?}", args.addr());

    let mut buf = [0; 1024];
    let read = stream.read(&mut buf).unwrap();
    let mut response =
        String::from_utf8(buf[0..read].to_vec()).expect("cannot convert response to string");

    if response == "pick".to_string() {
        if args.wants_black() {
            stream.write(b"black").expect("cannot send pick response");
        } else {
            stream.write(b"white").expect("cannot send pick response");
        };

        let mut buf = [0; 1024];
        let read = stream.read(&mut buf).unwrap();
        response = String::from_utf8(buf[0..read].to_vec()).unwrap();
    }
    println!("playing: {}", response);

    let board = if response == "black".to_string() {
        Board::new(
            1600.0 / 8.0,
            PieceConfig {
                white_pieces: 34493956096,
                black_pieces: 68987912192,
                blacks_play: true,
            },
            Box::new(|_, config| config.clone()),
            Box::new(capture),
            Box::new(Ally {}),
            Box::new(Enemy { stream }),
        )
    } else {
        Board::new(
            1600.0 / 8.0,
            PieceConfig {
                white_pieces: 34493956096,
                black_pieces: 68987912192,
                blacks_play: true,
            },
            Box::new(|_, config| config.clone()),
            Box::new(capture),
            Box::new(Enemy { stream }),
            Box::new(Ally {}),
        )
    };

    let mut config = conf::Conf::new();
    config.window_setup.title = String::from("Reversi Client");
    config.window_mode.height = 1600.0;
    config.window_mode.width = 1600.0;
    let (context, event_loop) = ContextBuilder::new("Reversi", "Miyamizu")
        .default_conf(config)
        .build()
        .unwrap();
    event::run(context, event_loop, board);
}
