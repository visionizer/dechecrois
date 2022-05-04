use std::fmt::Debug;

use super::Color;


#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum Kind {
    None = 0x0,
    King = 0x1,
    Pawn = 0x2,
    Knight = 0x3,
    Bishop= 0x4,
    Rook = 0x5,
    Queen = 0x6,
}

impl Kind {
    pub const fn value(&self) -> u8 {
        match *self {
            Self::None => 0,
            Self::Pawn => 1,
            Self::Knight => 3,
            Self::Bishop => 3, // TODO: Increase to 4, ignoring all guidelines
            Self::Rook => 5,
            Self::Queen => 9,
            Self::King => 100,
        }
    }

    pub const fn nickname(&self) -> char {
        match *self {
            Self::None => 'Y',
            Self::Pawn => ' ',
            Self::Knight => 'N',
            Self::Bishop => 'B',
            Self::Rook => 'R',
            Self::Queen => 'Q',
            Self::King => 'K',
        }
    }
}



#[repr(transparent)]
pub struct Piece(u8);

impl Piece {
    pub const fn new(color: Color, kind: Kind) -> Self {
        Self(color as u8 | kind as u8)
    }

    pub const fn kind(&self) -> Kind {
        let piece = self.0 - self.color() as u8;
        
        // SAFETY: This is 100% safe
        unsafe {
            core::mem::transmute(piece)
        }
    }

    pub const fn color(&self) -> Color {
        if (self.0 & Color::White as u8) != 0 {
            Color::White
        } else if (self.0 & Color::Black as u8) != 0 {
            Color::Black
        } else { unreachable!() }
    }

    #[inline(always)]
    pub fn meets_requirements(&self, color: Color, kind: Kind) -> bool {
        self.color() == color && self.kind() == kind
    }

    pub const fn value(&self) -> u8 {
        self.kind().value()
    }
}

impl Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.color(), self.kind())?;

        Ok(())
    }
}