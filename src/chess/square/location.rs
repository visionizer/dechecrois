use super::super::{Side, Direction, Color};

pub struct Location {
    rank: u8,
    file: u8,
    side: Side,
    color: Color,
}

impl Location {
    pub const A1: Self = Self::new(1, 1);
    pub const A2: Self = Self::new(1, 2);
    pub const A3: Self = Self::new(1, 3);
    pub const A4: Self = Self::new(1, 4);
    pub const A5: Self = Self::new(1, 5);
    pub const A6: Self = Self::new(1, 6);
    pub const A7: Self = Self::new(1, 7);
    pub const A8: Self = Self::new(1, 8);

    pub const B1: Self = Self::new(2, 1);
    pub const B2: Self = Self::new(2, 2);
    pub const B3: Self = Self::new(2, 3);
    pub const B4: Self = Self::new(2, 4);
    pub const B5: Self = Self::new(2, 5);
    pub const B6: Self = Self::new(2, 6);
    pub const B7: Self = Self::new(2, 7);
    pub const B8: Self = Self::new(2, 8);

    pub const C1: Self = Self::new(3, 1);
    pub const C2: Self = Self::new(3, 2);
    pub const C3: Self = Self::new(3, 3);
    pub const C4: Self = Self::new(3, 4);
    pub const C5: Self = Self::new(3, 5);
    pub const C6: Self = Self::new(3, 6);
    pub const C7: Self = Self::new(3, 7);
    pub const C8: Self = Self::new(3, 8);

    pub const D1: Self = Self::new(4, 1);
    pub const D2: Self = Self::new(4, 2);
    pub const D3: Self = Self::new(4, 3);
    pub const D4: Self = Self::new(4, 4);
    pub const D5: Self = Self::new(4, 5);
    pub const D6: Self = Self::new(4, 6);
    pub const D7: Self = Self::new(4, 7);
    pub const D8: Self = Self::new(4, 8);

    pub const E1: Self = Self::new(5, 1);
    pub const E2: Self = Self::new(5, 2);
    pub const E3: Self = Self::new(5, 3);
    pub const E4: Self = Self::new(5, 4);
    pub const E5: Self = Self::new(5, 5);
    pub const E6: Self = Self::new(5, 6);
    pub const E7: Self = Self::new(5, 7);
    pub const E8: Self = Self::new(5, 8);

    pub const F1: Self = Self::new(6, 1);
    pub const F2: Self = Self::new(6, 2);
    pub const F3: Self = Self::new(6, 3);
    pub const F4: Self = Self::new(6, 4);
    pub const F5: Self = Self::new(6, 5);
    pub const F6: Self = Self::new(6, 6);
    pub const F7: Self = Self::new(6, 7);
    pub const F8: Self = Self::new(6, 8);

    pub const G1: Self = Self::new(7, 1);
    pub const G2: Self = Self::new(7, 2);
    pub const G3: Self = Self::new(7, 3);
    pub const G4: Self = Self::new(7, 4);
    pub const G5: Self = Self::new(7, 5);
    pub const G6: Self = Self::new(7, 6);
    pub const G7: Self = Self::new(7, 7);
    pub const G8: Self = Self::new(7, 8);

    pub const H1: Self = Self::new(8, 1);
    pub const H2: Self = Self::new(8, 2);
    pub const H3: Self = Self::new(8, 3);
    pub const H4: Self = Self::new(8, 4);
    pub const H5: Self = Self::new(8, 5);
    pub const H6: Self = Self::new(8, 6);
    pub const H7: Self = Self::new(8, 7);
    pub const H8: Self = Self::new(8, 8);


    const fn new(file: u8, rank: u8) -> Self {
        assert!(file <= 8);
        assert!(file > 0);
        assert!(rank <= 8);
        assert!(rank > 0);

        let side = if file <= 4 {
            Side::Queenside
        } else {
            Side::Kingside
        };

        let color = {
            if file % 2 == 0 {
                if rank % 2 == 0 { Color::Black } else { Color::White }
            } else {
                if rank % 2 == 0 { Color::White } else { Color::Black }
            }
        };

        Self {
            rank,
            file,
            side,
            color
        }
    }

    // Returns None if the move is impossible
    pub const fn move_to(&self, dir: Direction, amount: u8) -> Option<Self> {
        let me = match dir {
            Direction::Down => {
                if (self.rank as i8 - amount as i8) <= 0 {
                    return None;
                }
                Self::new(self.file, self.rank - amount)
            }
            Direction::Up => {
                if (self.rank + amount) > 8 {
                    return None;
                }
                Self::new(self.file, self.rank + amount)
            }
            Direction::Left => {
                if (self.file as i8 - amount as i8) <= 0 {
                    return None;
                }
                Self::new(self.file - amount, self.rank)
            }
            Direction::Right => {
                if (self.file + amount) > 8 {
                    return None;
                }
                Self::new(self.file + amount, self.rank)
            }

            Direction::LeftUpDiag => {
                if ((self.file as i8 - amount as i8) <= 0) || ((self.rank + amount) > 8) {
                    return None;
                }
                Self::new(self.file - amount, self.rank + amount)
            }

            Direction::RightUpDiag => {
                if ((self.file + amount) > 8) || ((self.rank + amount) > 8) {
                    return None;
                }
                Self::new(self.file + amount, self.rank + amount)
            }

            Direction::LeftDownDiag => {
                if ((self.file as i8 - amount as i8) <= 0) || ((self.rank as i8 - amount as i8) <= 0) {
                    return None;
                }
                Self::new(self.file - amount, self.rank - amount)
            }

            Direction::RightDownDiag => {
                if ((self.file + amount) > 8) || ((self.rank as i8 - amount as i8) <= 0) {
                    return None;
                }
                Self::new(self.file + amount, self.rank - amount)
            }
        };

        
        Some(me)
    }
}

impl core::fmt::Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chr = (self.file + 64) as char;
        write!(f, "{}{} ({:?}|{:?})", chr, self.rank, self.side, self.color)?;
        Ok(())
    }
}