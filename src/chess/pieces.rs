use std::fmt;

#[derive(Copy, Clone)]
pub enum Pieces {
    PAWN,
    HORSE,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
    EMPTY,
}

impl fmt::Debug for Pieces {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Pieces::PAWN => write!(f, "P"),
            Pieces::HORSE => write!(f, "H"),
            Pieces::BISHOP => write!(f, "B"),
            Pieces::ROOK => write!(f, "R"),
            Pieces::QUEEN => write!(f, "Q"),
            Pieces::KING => write!(f, "K"),
            Pieces::EMPTY => write!(f, " "),
        }
    }
}
