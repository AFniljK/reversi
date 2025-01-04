use ggez::input::keyboard::{KeyCode, KeyInput};
use reversi;

use ggez::glam::Vec2;
use ggez::graphics::{Color, FillOptions, MeshBuilder};
use ggez::{conf, event, graphics};
use ggez::{event::EventHandler, ContextBuilder, GameError, GameResult};

const BOARD_SIZE: f32 = 8.0 * 100.0;

struct MainState {
    square_size: f32,
    desired_fps: u32,
    white_pieces: u64,
    black_pieces: u64,
    blacks_play: bool,
}

impl MainState {
    fn new(desired_fps: u32) -> MainState {
        MainState {
            square_size: BOARD_SIZE / 8.0,
            desired_fps,
            /*
            white_pieces: 34493956096, // Default White Pieces' Position
            black_pieces: 68987912192, // Default Black Pieces' Position
             */
            white_pieces: 0,
            black_pieces: 0,
            blacks_play: true,
        }
    }

    fn capture(&mut self, position: u64) {
        if self.blacks_play {
            self.black_pieces |= position;
        } else {
            self.white_pieces |= position;
        }
    }

    fn colored_mesh(
        &mut self,
        ctx: &mut ggez::Context,
        pieces: u64,
        color: Color,
    ) -> GameResult<graphics::Mesh> {
        let mesh_builder = &mut MeshBuilder::new();
        if let Some(positions) = reversi::piece_positions(pieces) {
            for position in positions {
                let (row, column) = reversi::bitboard_rowcol(position);
                mesh_builder.rectangle(
                    graphics::DrawMode::Fill(FillOptions::default()),
                    graphics::Rect::new(
                        column as f32 * self.square_size,
                        row as f32 * self.square_size,
                        self.square_size,
                        self.square_size,
                    ),
                    color,
                )?;
            }
        }

        Ok(graphics::Mesh::from_data(ctx, mesh_builder.build()))
    }

    fn is_occupied(&self, position: u64) -> bool {
        (self.black_pieces | self.white_pieces) & position != 0
    }

    fn grid(&self, ctx: &mut ggez::Context) -> GameResult<graphics::Mesh> {
        let mesh_builder = &mut MeshBuilder::new();
        for i in 1..8 {
            mesh_builder.line(
                &[
                    Vec2::new(self.square_size * i as f32, 0.0),
                    Vec2::new(self.square_size * i as f32, BOARD_SIZE),
                ],
                2.0,
                Color::WHITE,
            )?;
            mesh_builder.line(
                &[
                    Vec2::new(0.0, self.square_size * i as f32),
                    Vec2::new(BOARD_SIZE as f32, self.square_size * i as f32),
                ],
                2.0,
                Color::WHITE,
            )?;
        }
        Ok(graphics::Mesh::from_data(ctx, mesh_builder.build()))
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        if ctx.time.check_update_time(self.desired_fps) {
            if ctx.mouse.button_pressed(event::MouseButton::Left) {
                let position = ctx.mouse.position();
                let row = (position.y / (BOARD_SIZE / 8.0)) as u8;
                let column = (position.x / (BOARD_SIZE / 8.0)) as u8;
                let position = reversi::bitboard_position(row, column);
                if !self.is_occupied(position) {
                    self.capture(position);
                }
            }
        };

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut ggez::Context, input: KeyInput, repeat: bool) -> GameResult {
        if repeat {
            return Ok(());
        }
        match input.keycode {
            Some(KeyCode::W) => self.blacks_play = false,
            Some(KeyCode::B) => self.blacks_play = true,
            Some(KeyCode::R) => {self.white_pieces = 0; self.black_pieces = 0;},
            Some(KeyCode::Return) => println!("White Pieces: {:?}\tBlack Pieces: {:?}", self.white_pieces, self.black_pieces),
            _ => (),
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(255, 162, 103));
        canvas.draw(
            &self.colored_mesh(ctx, self.black_pieces, Color::from_rgb(66, 66, 66))?,
            graphics::DrawParam::new(),
        );
        canvas.draw(
            &self.colored_mesh(ctx, self.white_pieces, Color::from_rgb(224, 224, 224))?,
            graphics::DrawParam::new(),
        );
        canvas.draw(&self.grid(ctx)?, graphics::DrawParam::new());
        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let mainstate = MainState::new(60);
    let mut config = conf::Conf::new();
    config.window_setup.title = String::from("Reversi");
    config.window_mode.height = BOARD_SIZE.into();
    config.window_mode.width = BOARD_SIZE.into();
    let (context, event_loop) = ContextBuilder::new("Reversi", "Miyamizu")
        .default_conf(config)
        .build()?;
    event::run(context, event_loop, mainstate);
}