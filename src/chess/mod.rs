pub mod square;
pub mod board;


pub mod piece;

#[derive(Debug)]
pub enum Side {
    Kingside,
    Queenside,
}

pub enum Move {
    Castle(Side),
    Movement {
        direction: Direction,
        amount: u8
    },
    RequestDraw,
    AcceptDraw,
    Resign,
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
#[derive(Debug, PartialEq)]
pub enum Color  {
    White = 0x8,
    Black = 0x10,
}