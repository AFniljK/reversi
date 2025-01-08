use std::collections::HashMap;

use ggez::glam::Vec2;
use ggez::graphics::{Color, FillOptions, MeshBuilder};
use ggez::input::keyboard::KeyInput;
use ggez::{event, graphics};
use ggez::{event::EventHandler, GameError, GameResult};

use crate::{bitboard_position, bitboard_rowcol, piece_positions};

pub enum Move {
    Position(u64),
    Board,
}

pub trait Player {
    fn play_move(&self, config: &PieceConfig) -> Move;
}

#[derive(Clone, PartialEq)]
pub struct PieceConfig {
    pub white_pieces: u64,
    pub black_pieces: u64,
    pub blacks_play: bool,
}

impl PieceConfig {
    pub fn ally_foe(&self) -> (u64, u64) {
        if self.blacks_play {
            return (self.black_pieces, self.white_pieces);
        }
        return (self.white_pieces, self.black_pieces);
    }
}

#[derive(Clone)]
pub struct BoardConfig {
    pub piece_config: PieceConfig,
    pub mesh: HashMap<u64, Color>,
}

impl BoardConfig {
    pub fn new(piece_config: PieceConfig) -> BoardConfig {
        BoardConfig {
            piece_config,
            mesh: HashMap::new(),
        }
    }
}

pub struct Board {
    square_size: f32,
    pub config: BoardConfig,
    handle_keypress: Box<dyn Fn(KeyInput, &BoardConfig) -> BoardConfig>,
    capture: Box<dyn Fn(&PieceConfig, u64) -> PieceConfig>,
    black: Box<dyn Player>,
    white: Box<dyn Player>,
}

impl Board {
    pub fn new(
        square_size: f32,
        piece_config: PieceConfig,
        handle_keypress: Box<dyn Fn(KeyInput, &BoardConfig) -> BoardConfig>,
        capture: Box<dyn Fn(&PieceConfig, u64) -> PieceConfig>,
        black: Box<dyn Player>,
        white: Box<dyn Player>,
    ) -> Board {
        Board {
            square_size,
            config: BoardConfig::new(piece_config),
            handle_keypress,
            capture,
            black,
            white,
        }
    }

    fn colored_mesh(
        &mut self,
        ctx: &mut ggez::Context,
        pieces: u64,
        color: Color,
    ) -> GameResult<graphics::Mesh> {
        let mesh_builder = &mut MeshBuilder::new();
        if let Some(positions) = piece_positions(pieces) {
            for position in positions {
                let (row, column) = bitboard_rowcol(position);
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

    fn grid(&self, ctx: &mut ggez::Context) -> GameResult<graphics::Mesh> {
        let mesh_builder = &mut MeshBuilder::new();
        for i in 1..8 {
            mesh_builder.line(
                &[
                    Vec2::new(self.square_size * i as f32, 0.0),
                    Vec2::new(self.square_size * i as f32, self.square_size * 8f32),
                ],
                2.0,
                Color::WHITE,
            )?;
            mesh_builder.line(
                &[
                    Vec2::new(0.0, self.square_size * i as f32),
                    Vec2::new(self.square_size * 8f32, self.square_size * i as f32),
                ],
                2.0,
                Color::WHITE,
            )?;
        }
        Ok(graphics::Mesh::from_data(ctx, mesh_builder.build()))
    }
}

impl EventHandler<GameError> for Board {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let desired_fps: u32 = 60;
        if ctx.time.check_update_time(desired_fps) {
            return Ok(());
        }

        let player = if self.config.piece_config.blacks_play {
            &self.black
        } else {
            &self.white
        };

        let position = match player.play_move(&self.config.piece_config) {
            Move::Position(position) => position,
            Move::Board => {
                if !ctx.mouse.button_pressed(event::MouseButton::Left) {
                    return Ok(());
                }
                let position = ctx.mouse.position();
                let row = (position.y / self.square_size) as u8;
                let column = (position.x / self.square_size) as u8;
                bitboard_position(row, column)
            }
        };
        self.config.piece_config = (self.capture)(&self.config.piece_config, position);

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        input: KeyInput,
        repeat: bool,
    ) -> GameResult {
        if !repeat {
            self.config = (self.handle_keypress)(input, &self.config);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let brown = Color::from_rgb(255, 162, 103);
        let black = Color::from_rgb(66, 66, 66);
        let grey = Color::from_rgb(224, 224, 224);
        let mut canvas = graphics::Canvas::from_frame(ctx, brown);

        canvas.draw(
            &self.colored_mesh(ctx, self.config.piece_config.black_pieces, black)?,
            graphics::DrawParam::new(),
        );
        canvas.draw(
            &self.colored_mesh(ctx, self.config.piece_config.white_pieces, grey)?,
            graphics::DrawParam::new(),
        );
        for (mesh, color) in self.config.mesh.clone().into_iter() {
            let drawable = self.colored_mesh(ctx, mesh, color)?;
            canvas.draw(&drawable, graphics::DrawParam::new());
        }

        canvas.draw(&self.grid(ctx)?, graphics::DrawParam::new());
        canvas.finish(ctx)?;
        Ok(())
    }
}
