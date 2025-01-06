use clap::Parser;
use ggez::{
    conf, event,
    input::keyboard::{KeyCode, KeyInput},
    ContextBuilder, GameResult,
};

use reversi::{
    cli::Args,
    gui::{Board, PieceConfig},
};
use tungstenite::{connect, Message};

fn handle_keyboard(key: KeyInput, board: &Board) -> PieceConfig {
    let config = board.piece_config.clone();
    match key.keycode {
        Some(KeyCode::T) => {
            if config.blacks_play {
                println!("blacks' play");
            } else {
                println!("whites' play");
            }
        }
        _ => {}
    }
    return config;
}

fn capture(config: &PieceConfig, position: u64) -> PieceConfig {
    let mut config = config.clone();
    let (ally, foe) = if config.blacks_play {
        (config.black_pieces, config.white_pieces)
    } else {
        (config.white_pieces, config.black_pieces)
    };

    let captures = reversi::available_captures(ally, foe).unwrap();
    if let Some(mesh) = captures.get(&position) {
        if config.blacks_play {
            config.black_pieces |= mesh | position;
            config.white_pieces &= !mesh;
        } else {
            config.white_pieces |= mesh | position;
            config.black_pieces &= !mesh;
        }
    }
    config.blacks_play = !config.blacks_play;

    config
}

const BOARD_SIZE: f32 = 800.0 * 2.0;

fn main() -> GameResult {
    let args = Args::parse();
    let addr = args.addr();
    let (mut stream, response) = connect("ws://".to_owned() + &addr).expect("cannot connect");
    println!(
        "connected to address: {:?}\tstatus: {:?}",
        addr,
        response.status()
    );

    let mut message: String = stream.read().unwrap().into_text().unwrap().to_string();
    if message == String::from("pick") {
        let piece = if args.wants_black() {
            String::from("black")
        } else {
            String::from("white")
        };
        stream.send(Message::text(piece)).unwrap();
        message = stream.read().unwrap().into_text().unwrap().to_string();
    }
    let our_turn = message == String::from("black");
    println!("playing as {}", message);

    stream.send(Message::text(String::from("ready"))).unwrap();

    let board = Board::new(
        BOARD_SIZE,
        60,
        our_turn,
        Box::new(handle_keyboard),
        Box::new(capture),
        PieceConfig {
            white_pieces: 34493956096,
            black_pieces: 68987912192,
            blacks_play: true,
        },
        Some(stream),
    );

    let mut config = conf::Conf::new();
    config.window_setup.title = String::from("Reversi");
    config.window_mode.height = BOARD_SIZE;
    config.window_mode.width = BOARD_SIZE;
    let (context, event_loop) = ContextBuilder::new("Reversi", "Miyamizu")
        .default_conf(config)
        .build()?;
    event::run(context, event_loop, board);
}
