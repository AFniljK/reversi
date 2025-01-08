use std::net::TcpStream;

use ggez::glam::Vec2;
use ggez::graphics::{Color, FillOptions, MeshBuilder};
use ggez::input::keyboard::KeyInput;
use ggez::{event, graphics};
use ggez::{event::EventHandler, GameError, GameResult};
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};

use crate::{available_captures, bitboard_position, bitboard_rowcol, piece_positions};

#[derive(Clone, PartialEq)]
pub struct PieceConfig {
    pub white_pieces: u64,
    pub black_pieces: u64,
    pub blacks_play: bool,
}

pub struct Board {
    square_size: f32,
    desired_fps: u32,
    our_turn: bool,
    pub debugging: bool,
    pub piece_config: PieceConfig,
    keyboard_handler: Box<dyn Fn(KeyInput, &Board) -> PieceConfig>,
    capture: Box<dyn FnMut(&PieceConfig, u64) -> PieceConfig>,
    stream: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
}

impl Board {
    pub fn new(
        board_size: f32,
        desired_fps: u32,
        our_turn: bool,
        keyboard_handler: Box<dyn Fn(KeyInput, &Board) -> PieceConfig>,
        capture: Box<dyn FnMut(&PieceConfig, u64) -> PieceConfig>,
        piece_config: PieceConfig,
        stream: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    ) -> Board {
        Board {
            square_size: board_size / 8.0,
            desired_fps,
            our_turn,
            debugging: false,
            piece_config,
            keyboard_handler,
            capture,
            stream,
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

    pub fn is_occupied(&self, position: u64) -> bool {
        (self.piece_config.black_pieces | self.piece_config.white_pieces) & position != 0
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
        if ctx.time.check_update_time(self.desired_fps) {
            if let Some(stream) = &mut self.stream {
                if !self.our_turn {
                    println!("Waiting For Position");
                    let position: u64 = stream.read().unwrap().to_text().unwrap().parse().unwrap();
                    println!("Got Position: {}", position);
                    self.piece_config = (self.capture)(&self.piece_config, position);
                }
                self.our_turn = true;
            } else {
                if ctx.mouse.button_pressed(event::MouseButton::Left) {
                    let position = ctx.mouse.position();
                    let row = (position.y / self.square_size) as u8;
                    let column = (position.x / self.square_size) as u8;
                    let position = bitboard_position(row, column);

                    let (ally, foe) = if self.piece_config.blacks_play {
                        (
                            self.piece_config.black_pieces,
                            self.piece_config.white_pieces,
                        )
                    } else {
                        (
                            self.piece_config.white_pieces,
                            self.piece_config.black_pieces,
                        )
                    };

                    if self.debugging {
                        self.piece_config = (self.capture)(&self.piece_config, position);
                        self.our_turn = false;
                        return Ok(());
                    }
                    let keys = match available_captures(ally, foe) {
                        Some(captures) => captures,
                        None => {
                            self.our_turn = false;
                            self.piece_config.blacks_play = !self.piece_config.blacks_play;
                            return Ok(());
                        }
                    };
                    if keys.contains_key(&position) {
                        let piece_config = (self.capture)(&self.piece_config, position);
                        if let Some(stream) = &mut self.stream {
                            stream.send(Message::text(position.to_string())).unwrap();
                            println!("Sent Position: {}", position);
                        }
                        self.piece_config = piece_config;
                        self.our_turn = false;
                    }
                }
            }
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        input: KeyInput,
        repeat: bool,
    ) -> GameResult {
        if !repeat {
            let config = (self.keyboard_handler)(input, self);
            self.piece_config = config;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(255, 162, 103));
        canvas.draw(
            &self.colored_mesh(
                ctx,
                self.piece_config.black_pieces,
                Color::from_rgb(66, 66, 66),
            )?,
            graphics::DrawParam::new(),
        );
        canvas.draw(
            &self.colored_mesh(
                ctx,
                self.piece_config.white_pieces,
                Color::from_rgb(224, 224, 224),
            )?,
            graphics::DrawParam::new(),
        );

        let (ally, foe) = if self.piece_config.blacks_play {
            (
                self.piece_config.black_pieces,
                self.piece_config.white_pieces,
            )
        } else {
            (
                self.piece_config.white_pieces,
                self.piece_config.black_pieces,
            )
        };
        if let Some(captures) = available_captures(ally, foe) {
            let mut mesh = 0;
            for key in captures.keys() {
                mesh |= key;
            }
            canvas.draw(
                &self.colored_mesh(ctx, mesh, Color::MAGENTA)?,
                graphics::DrawParam::new(),
            )
        }
        canvas.draw(&self.grid(ctx)?, graphics::DrawParam::new());
        canvas.finish(ctx)?;
        Ok(())
    }
}
