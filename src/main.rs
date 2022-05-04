use chess::{square::Square, Direction, piece::{Piece, Kind}, Color};

extern crate gl;

pub mod graphics;
pub mod chess;

fn main() {
    let piece = Piece::new(Color::Black, Kind::Pawn);
    println!("{:?}", piece);
}
