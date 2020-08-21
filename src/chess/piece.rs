use super::{Change, Colors, Pieces};
use super::{BISHOP, EMPTY, HORSE, KING, PAWN, QUEEN, ROOK};
use super::{BLACK, NONE, WHITE};

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
			PAWN => 1,
			QUEEN => 9,
			ROOK => 5,
			HORSE => 3,
			BISHOP => 3,
			EMPTY | KING => 0,
		};

		// * Multiplier of the value to allow fro easy summation scoring
		let multiplier = match self.color {
			WHITE => 1,
			BLACK => -1,
			NONE => 0,
		};

		return value * multiplier;
	}

	pub fn get_moves(
		&self,
		row: usize,
		col: usize,
		board: &Vec<Piece>,
		log: &Vec<Change>,
	) -> Vec<Change> {
		// * Check if it is a valid piece
		if let NONE = self.color {
			panic!("No piece to be moved")
		} else if let EMPTY = self.title {
			panic!("No piece to be moved")
		}

		let mut moves: Vec<Change> = Vec::new();

		match self.title {
			PAWN => {}
			KING => {}
			QUEEN => {}
			HORSE => {}
			ROOK => {}
			BISHOP => {}
			EMPTY => panic!("No piece to be moved"),
		};

		return moves;
	}
}
