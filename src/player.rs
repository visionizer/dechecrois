use crate::chess::{
    board::{Board, RoChessBoard},
    Color, Move,
};
use std::sync::Arc;

pub trait Player<'a> {
    fn init(&mut self, board: RoChessBoard<'a>, color: Color);
    fn apply_move(&mut self, moved: &Move);
    fn send_move(&mut self) -> Move;
    fn name(&self) -> &str;
}
