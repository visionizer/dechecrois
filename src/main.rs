use chess::{
    board::Board,
    board_builder::BoardBuilder,
    game::Game,
    piece::{Kind, Piece},
    square::Square,
    Color, Direction,
};
use graphics::create_graphics_provider;
use spin::RwLock;
use std::sync::Arc;

pub mod chess;
pub mod engine;
pub mod graphics;
pub mod player;

fn main() {
    let board = Arc::new(RwLock::new(BoardBuilder::standard().build()));
    let mut provider = create_graphics_provider();
    let mut game = Game::new(board, provider);
    game.start();
}
