use self::location::Location;

use super::{piece::Piece, Move, board::Board};

pub mod location;

pub struct Square {
    pub has_king: bool,  // Is a king standing here?
    pub location: Location,
    pub piece: Option<Piece>,
    pub claimant: Option<Vec<Square>>, // Occupied squares, which have a piece on it which 'claims' aka may take this square
    pub claims: Option<Vec<Square>>, // Occupied squares, which this piece could take
    pub theoretical_check: bool, // Can a king move here?
    pub protecting_king: bool, // Whether this piece is protecting a king
    pub protectors_value: u8, // Value of the protecting pieces
    pub protecting_value: u8, // Value of the pieces that this piece protects
}

impl Square {
    pub fn is_empty(&self) -> bool { self.piece.is_some() }
    pub fn get_legal_moves(&self, board: &Board) -> Vec<Move> {
        if self.protecting_king && self.theoretical_check {
            //vec![] // Moving would check the king 
        }
        vec![]
    }
}