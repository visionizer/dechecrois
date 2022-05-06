use std::fmt::Debug;

use super::board::ChessBoard;
use super::board::RoChessBoard;
use super::{board::Board, piece::Kind, square::location::Location, Color, Direction, Side};
use std::sync::Arc;

pub enum Victory {
    Resign(Color),
    Checkmate(Color), // Color of the checkmated person
    Draw,
    TimeControl(Color), // The party that lost
    Forfeit(Color),
}

impl Debug for Victory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Resign(col) => {
                write!(f, "{} ({:?} resigns)", (-col).indicate_victory(), col)
            }
            Self::Checkmate(col) => {
                write!(f, "{:?} (Checkmate)", (-col).indicate_victory())
            }
            Self::Draw => {
                write!(f, "1/2-1/2 (Draw)")
            }
            Self::TimeControl(col) => {
                write!(f, "{:?} (Time control)", (-col).indicate_victory())
            }
            Self::Forfeit(col) => {
                write!(f, "{:?} ({:?} forfeited)", (-col).indicate_victory(), col)
            }
        }
    }
}

pub enum Move {
    Castle(Side),
    Movement {
        notation_char: char, // K, Q, B, R, ...
        start: Location,
        direction: Direction,
        amount: u8,
        checkmate: bool,
    },
    RequestDraw,
    Victory(Victory),
}

impl Move {
    pub fn fmt(&self, board: &RoChessBoard) -> String {
        match self {
            Self::Castle(side) => match side {
                Side::Kingside => "0-0".to_owned(),
                Side::Queenside => "0-0-0".to_owned(),
            },
            Self::RequestDraw => "=".to_owned(),
            Self::Victory(vic) => format!("{:?}", vic),
            Self::Movement {
                notation_char,
                start,
                direction,
                amount,
                checkmate,
            } => {
                let new_loc = start.apply(direction, *amount).expect("Illegal move");
                let (hit, check) = {
                    let square = board[new_loc];
                    if square.is_occupied() {
                        if square.inhabitant_kind() == Kind::King {
                            (true, true)
                        } else {
                            (true, false)
                        }
                    } else {
                        (false, false)
                    }
                };
                let hit = if hit { "x" } else { "" };
                let check = if *checkmate {
                    "#"
                } else if check {
                    "+"
                } else {
                    ""
                };

                format!("{}{}{:?}{}", notation_char, hit, new_loc, check)
            }
        }
    }
}
