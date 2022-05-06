use self::location::Location;

use super::{
    board::Board,
    piece::{Kind, Piece},
    Move,
};

pub mod location;

#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub has_king: bool, // Is a king standing here?
    pub location: Location,
    pub piece: Option<Piece>,
    pub claimants_value: u8, // Value of attacking pieces
    pub claims_value: u8,
    pub theoretical_check: bool, // Can a king move here?
    pub protecting_king: bool,   // Whether this piece is protecting a king
    pub protectors_value: u8,    // Value of the protecting pieces
    pub protecting_value: u8,    // Value of the pieces that this piece protects
}

impl Square {
    pub const fn is_empty(&self) -> bool {
        self.piece.is_none()
    }
    pub fn get_legal_moves(&self, board: &Board) -> Vec<Move> {
        if self.protecting_king && self.theoretical_check {
            //vec![] // Moving would check the king
        }
        vec![]
    }

    pub const fn is_occupied(&self) -> bool {
        self.piece.is_some()
    }
    pub fn inhabitant(&self) -> &Piece {
        self.piece
            .as_ref()
            .expect("tried to get inhabitant of empty square")
    }

    pub fn inhabitant_kind(&self) -> Kind {
        self.inhabitant().kind()
    }
}

impl Default for Square {
    fn default() -> Self {
        Self {
            has_king: false,
            location: Location::A1,
            piece: None,
            claimants_value: 0,
            claims_value: 0,
            theoretical_check: false,
            protecting_king: false,
            protectors_value: 0,
            protecting_value: 0,
        }
    }
}
