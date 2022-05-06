use super::{
    board::Board,
    piece::{Kind, Piece},
    square::{location::Location, Square},
    Color, Direction,
};

pub const EMPTY_SQUARE: Square = Square {
    has_king: false,
    location: Location::A1,
    piece: None,
    claimants_value: 0,
    claims_value: 0,
    theoretical_check: false,
    protecting_king: false,
    protectors_value: 0,
    protecting_value: 0,
};

pub struct BoardBuilder {
    board: Board,
}

impl BoardBuilder {
    pub fn new() -> Self {
        Self {
            board: Board::from([EMPTY_SQUARE; 64]),
        }
    }

    pub fn fill_rank(&mut self, start: Location, piece: Piece) -> &mut Self {
        for i in 0..7 {
            let place = start.apply(&Direction::Right, 1).expect("Illegal move");
            self.place(place, piece);
        }

        self
    }

    pub fn place(&mut self, location: Location, piece: Piece) -> &mut Self {
        self.board[location].piece = Some(piece);
        self
    }

    pub fn standard() -> Self {
        let mut me = Self::new();
        // White pieces
        me.place(Location::A1, Piece::new(Color::White, Kind::Rook))
            .place(Location::B1, Piece::new(Color::White, Kind::Knight))
            .place(Location::C1, Piece::new(Color::White, Kind::Bishop))
            .place(Location::D1, Piece::new(Color::White, Kind::Queen))
            .place(Location::E1, Piece::new(Color::White, Kind::King))
            .place(Location::F1, Piece::new(Color::White, Kind::Bishop))
            .place(Location::G1, Piece::new(Color::White, Kind::Knight))
            .place(Location::H1, Piece::new(Color::White, Kind::Rook))
            .fill_rank(Location::A2, Piece::new(Color::White, Kind::Pawn));

        // Black pieces
        me.place(Location::A8, Piece::new(Color::Black, Kind::Rook))
            .place(Location::B8, Piece::new(Color::Black, Kind::Knight))
            .place(Location::C8, Piece::new(Color::Black, Kind::Bishop))
            .place(Location::D8, Piece::new(Color::Black, Kind::Queen))
            .place(Location::E8, Piece::new(Color::Black, Kind::King))
            .place(Location::F8, Piece::new(Color::Black, Kind::Bishop))
            .place(Location::G8, Piece::new(Color::Black, Kind::Knight))
            .place(Location::H8, Piece::new(Color::Black, Kind::Rook))
            .fill_rank(Location::A7, Piece::new(Color::Black, Kind::Pawn));

        me
    }

    pub fn build(self) -> Board {
        self.board
    }
}
