use crate::chess::board::Board;
use crate::chess::board::RoChessBoard;
use crate::chess::Color;
use crate::chess::Move;
use crate::chess::Side;
use crate::player::Player;
use std::sync::Arc;

pub struct ChessEngine<'a> {
    board: Option<RoChessBoard<'a>>,
    my_eval: u64,
    enemy_eval: u64,
}

impl<'a> ChessEngine<'a> {
    pub fn new() -> Self {
        Self {
            board: None,
            my_eval: 0,
            enemy_eval: 0,
        }
    }
}

impl<'a> Player<'a> for ChessEngine<'a> {
    fn init(&mut self, board: RoChessBoard<'a>, color: Color) {
        println!("Initializing engine");
        self.board = Some(board);
    }
    fn apply_move(&mut self, moved: &Move) {
        println!(
            "Engine got move {}",
            moved.fmt(self.board.as_ref().unwrap())
        );
    }
    fn send_move(&mut self) -> Move {
        println!("Engine is moving...");
        Move::Castle(Side::Queenside)
    }

    fn name(&self) -> &str {
        "Engine"
    }
}
