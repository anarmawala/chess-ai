use super::colors::Colors;
use super::colors::Colors::{BLACK, NONE, WHITE};
use super::pieces::Pieces;
use super::pieces::Pieces::{BISHOP, EMPTY, HORSE, KING, PAWN, QUEEN, ROOK};

#[derive(Copy, Clone)]
pub struct Piece {
    // * Piece's type and color
    color: Colors,
    title: Pieces,

    // * Piece game info
    moved: bool,
    value: i32,
}

impl Piece {
    pub fn new(color: Colors, title: Pieces) -> Self {
        // * Value of the piece
        let _value = match title {
            PAWN => 1,
            QUEEN => 9,
            ROOK => 5,
            HORSE => 3,
            BISHOP => 3,
            EMPTY | KING => 0,
        };

        // * Multiplier of the value to allow fro easy summation scoring
        let _multiplier = match color {
            WHITE => 1,
            BLACK => -1,
            NONE => 0,
        };

        // * Return the created Piece
        Piece {
            moved: false,
            color,
            title,

            value: _value * _multiplier,
        }
    }

    pub fn color(&self) -> &Colors {
        &self.color
    }

    pub fn title(&self) -> &Pieces {
        &self.title
    }

    pub fn moved(&self) -> bool {
        self.moved
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn get_moves(&self, row: usize, col: usize) {
        match self.title {
            PAWN => {}
            KING => {}
            QUEEN => {}
            HORSE => {}
            ROOK => {}
            BISHOP => {}
            EMPTY => panic!("No piece to be moved"),
        }
    }
}
