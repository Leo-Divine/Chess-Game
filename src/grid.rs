use std::ops::{Index, IndexMut};
use std::sync::{Arc, Mutex};

const ROWS: usize = 8;
const COLS: usize = 8;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Pieces {
    #[default]
    None,
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

pub struct Grid {
    items: Vec<Pieces>,
}

impl Default for Grid {
    fn default() -> Self {
        Self { 
            vec![Pieces::BlackRook, Pieces::BlackKnight, Pieces::BlackBishop, Pieces::BlackKing, Pieces::BlackQueen, Pieces::BlackBishop, Pieces::BlackKnight, Pieces::BlackRook,
                Pieces::BlackPawn, Pieces::BlackPawn, Pieces::BlackPawn, Pieces::BlackPawn, Pieces::BlackPawn, Pieces::BlackPawn, Pieces::BlackPawn, Pieces::BlackPawn,
                Pieces::None, Pieces::None, Pieces::None, Pieces::None, Pieces::None, Pieces::None, Pieces::None, Pieces::None,
                Pieces::None, Pieces::None, Pieces::None, Pieces::None, Pieces::None, Pieces::None, Pieces::None, Pieces::None,
                Pieces::None, Pieces::None, Pieces::None, Pieces::None, Pieces::None, Pieces::None, Pieces::None, Pieces::None,
                Pieces::None, Pieces::None, Pieces::None, Pieces::None, Pieces::None, Pieces::None, Pieces::None, Pieces::None,
                Pieces::WhitePawn, Pieces::WhitePawn, Pieces::WhitePawn, Pieces::WhitePawn, Pieces::WhitePawn, Pieces::WhitePawn, Pieces::WhitePawn, Pieces::WhitePawn,
                Pieces::WhiteRook, Pieces::WhiteKnight, Pieces::WhiteBishop, Pieces::WhiteKing, Pieces::WhiteQueen, Pieces::WhiteBishop, Pieces::WhiteKnight, Pieces::WhiteRook
            ]
        }
    }
}

impl Grid {
    pub fn move_piece(&mut self, old_position: (usize, usize), new_position: (usize, usize)) {
        if self[old_position] == Pieces::None {
            return;
        }
        self[new_position] = self[old_position].clone();
        self[old_position] = Pieces::None;
    }

    fn get_index(col: usize, row: usize) -> usize {
        ROWS * row + col
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Pieces;

    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        &self.items[ROWS * row + col]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
        &mut self.items[ROWS * row + col]
    }
}

pub type Board = Arc<Mutex<Grid>>;
