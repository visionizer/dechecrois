pub mod board;
pub mod game;
pub mod moves;
pub mod square;
use std::ops::Neg;

pub use moves::Move;
pub mod board_builder;

pub mod piece;

#[derive(Debug, Copy, Clone)]
pub enum Side {
    Kingside,
    Queenside,
}

// All these directions are from Whites POV
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    LeftUpDiag,
    LeftDownDiag,
    RightUpDiag,
    RightDownDiag,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White = 0x8,
    Black = 0x10,
}

impl Color {
    pub const fn indicate_victory(&self) -> &'static str {
        match *self {
            Self::White => "0-1",
            Self::Black => "1-0",
        }
    }
}

impl Neg for Color {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
