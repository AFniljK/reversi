use std::collections::HashMap;

use ggez::{
    conf, event,
    graphics::Color,
    input::keyboard::{KeyCode, KeyInput},
    ContextBuilder, GameResult,
};
use reversi::{
    available_captures,
    gui::{Board, BoardConfig, Move, PieceConfig, Player},
};

const BOARD_SIZE: f32 = 800.0 * 2.0;

struct Client {}

impl Player for Client {
    fn play_move(&mut self, _config: &PieceConfig) -> Move {
        return Move::Board;
    }
    fn enemy_move(&mut self, _current_move: u64) {}
}

fn main() -> GameResult {
    let board = Board::new(
        BOARD_SIZE / 8.0,
        PieceConfig {
            white_pieces: 34493956096,
            black_pieces: 68987912192,
            blacks_play: true,
        },
        Box::new(handler),
        Box::new(capture),
        Box::new(Client {}),
        Box::new(Client {}),
    );

    let mut config = conf::Conf::new();
    config.window_setup.title = String::from("Reversi Debugger");
    config.window_mode.height = BOARD_SIZE;
    config.window_mode.width = BOARD_SIZE;
    let (context, event_loop) = ContextBuilder::new("Reversi", "Miyamizu")
        .default_conf(config)
        .build()?;
    event::run(context, event_loop, board);
}

fn handler(input: KeyInput, config: &BoardConfig) -> BoardConfig {
    let mut config = config.clone();
    match input.keycode {
        Some(KeyCode::W) => config.piece_config.blacks_play = false,
        Some(KeyCode::B) => config.piece_config.blacks_play = true,
        Some(KeyCode::D) => config.mesh = HashMap::new(),
        Some(KeyCode::R) => {
            config.piece_config.white_pieces = 34493956096;
            config.piece_config.black_pieces = 68987912192;
            config.mesh = HashMap::new();
        }
        Some(KeyCode::S) => {
            let (ally, foe) = config.piece_config.ally_foe();
            if let Some(captures) = available_captures(ally, foe) {
                let mut mesh: u64 = 0;
                for capture in captures.keys() {
                    mesh |= capture;
                }
                config.mesh.insert(mesh, Color::MAGENTA);
            }
        }
        Some(KeyCode::Return) => println!(
            "White Pieces: {:?}\tBlack Pieces: {:?}",
            config.piece_config.white_pieces, config.piece_config.black_pieces
        ),
        _ => (),
    }
    return config;
}

fn capture(config: &PieceConfig, position: u64) -> PieceConfig {
    let mut config = config.clone();
    if config.blacks_play {
        config.white_pieces &= !position;
        config.black_pieces |= position;
    } else {
        config.black_pieces &= !position;
        config.white_pieces |= position;
    }
    config
}
