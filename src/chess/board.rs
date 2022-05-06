use spin::RwLock;
use std::ops::{Index, IndexMut};

use super::square::{location::Location, Square};
use spin::RwLockReadGuard;
use std::sync::Arc;
pub type ChessBoard = Arc<RwLock<Board>>;
pub type RoChessBoard<'a> = RwLockReadGuard<'a, Board>;

#[derive(Clone, Copy)]
pub struct Board {
    squares: [Square; 64],
}

impl From<[Square; 64]> for Board {
    fn from(sq: [Square; 64]) -> Self {
        Self { squares: sq }
    }
}

impl Index<usize> for Board {
    type Output = Square;
    fn index(&self, index: usize) -> &Self::Output {
        &self.squares[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.squares[index]
    }
}

impl Index<Location> for Board {
    type Output = Square;
    fn index(&self, index: Location) -> &Self::Output {
        &self[index.index()]
    }
}

impl IndexMut<Location> for Board {
    fn index_mut(&mut self, index: Location) -> &mut Self::Output {
        &mut self[index.index()]
    }
}
