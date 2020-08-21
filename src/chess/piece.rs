use super::{Bishop, Empty, Horse, King, Pawn, Queen, Rook};
use super::{Black, None, White};
use super::{Change, Colors, Pieces};

#[derive(Copy, Clone)]
pub struct Piece {
	// * Piece's type and color
	color: Colors,
	title: Pieces,

	// * Piece game info
	moved: bool,
}

impl Piece {
	pub fn new(color: Colors, title: Pieces) -> Self {
		// * Return the created Piece
		Piece {
			moved: false,
			color,
			title,
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
		// * Value of the piece
		let value = match self.title {
			Pawn => 1,
			Queen => 9,
			Rook => 5,
			Horse | Bishop => 3,
			Empty | King => 0,
		};

		// * Multiplier of the value to allow fro easy summation scoring
		let multiplier = match self.color {
			White => 1,
			Black => -1,
			None => 0,
		};

		return value * multiplier;
	}
}
