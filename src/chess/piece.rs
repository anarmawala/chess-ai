use super::colors::Colors;
use super::colors::Colors::{BLACK, NONE, WHITE};
use super::pieces::Pieces;
use super::pieces::Pieces::{BISHOP, EMPTY, HORSE, KING, PAWN, QUEEN, ROOK};

#[derive(Copy, Clone)]
pub struct Piece {
    // * Piece's type and color
    color: Colors,
    p_typ: Pieces,

    // * Piece game info
    has_moved: bool,
    value: i32,

    // * Location
    column: usize,
    row: usize,
}

impl Piece {
    pub fn new(color: Colors, p_typ: Pieces, row: usize, column: usize) -> Self {
        let piece_value = match p_typ {
            PAWN => 1,
            QUEEN => 9,
            ROOK => 5,
            HORSE => 3,
            BISHOP => 3,
            _ => 0,
        };

        Piece {
            has_moved: false,
            color,
            p_typ,
            row,
            column,
            value: piece_value * if let BLACK = color { -1 } else { 1 },
        }
    }

    // * Returns a tuple of the pieces colors, type, and location
    pub fn get_info(&self) -> (Colors, Pieces, usize, usize, i32) {
        (self.color, self.p_typ, self.row, self.column, self.value)
    }
}
