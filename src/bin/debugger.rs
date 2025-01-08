use ggez::{
    conf, event,
    input::keyboard::{KeyCode, KeyInput},
    ContextBuilder, GameResult,
};
use reversi::gui::{Board, PieceConfig};

const BOARD_SIZE: f32 = 800.0 * 2.0;

fn keyboard_handler(input: KeyInput, board: &Board) -> PieceConfig {
    let mut config = board.piece_config.clone();
    match input.keycode {
        Some(KeyCode::W) => config.blacks_play = false,
        Some(KeyCode::B) => config.blacks_play = true,
        Some(KeyCode::R) => {
            config.white_pieces = 0;
            config.black_pieces = 0;
        }
        Some(KeyCode::Return) => println!(
            "White Pieces: {:?}\tBlack Pieces: {:?}",
            config.white_pieces, config.black_pieces
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

fn main() -> GameResult {
    let mut board = Board::new(
        BOARD_SIZE,
        60,
        true,
        Box::new(keyboard_handler),
        Box::new(capture),
        PieceConfig {
            white_pieces: 0,
            black_pieces: 0,
            blacks_play: true,
        },
        None,
    );
    board.debugging = true;

    let mut config = conf::Conf::new();
    config.window_setup.title = String::from("Reversi");
    config.window_mode.height = BOARD_SIZE;
    config.window_mode.width = BOARD_SIZE;
    let (context, event_loop) = ContextBuilder::new("Reversi", "Miyamizu")
        .default_conf(config)
        .build()?;
    event::run(context, event_loop, board);
}
