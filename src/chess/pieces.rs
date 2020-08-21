use std::fmt;

#[derive(Copy, Clone)]
pub enum Pieces {
	Pawn,
	Horse,
	Bishop,
	Rook,
	Queen,
	King,
	Empty,
}

impl fmt::Debug for Pieces {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Pieces::Pawn => write!(f, "P"),
			Pieces::Horse => write!(f, "H"),
			Pieces::Bishop => write!(f, "B"),
			Pieces::Rook => write!(f, "R"),
			Pieces::Queen => write!(f, "Q"),
			Pieces::King => write!(f, "K"),
			Pieces::Empty => write!(f, " "),
		}
	}
}
