use super::{Colors, Pieces};

#[derive(Copy, Clone)]
pub enum Changes {
	Capture,
	EnPassant,
	QCastle,
	RCastle,
	Promotion,
}

#[derive(Copy, Clone)]
pub struct Change {
	// * Location data
	from_location: (usize, usize),
	to_location: (usize, usize),

	// * Color of piece making the move and type of move
	color: Colors,
	change: Changes,

	// * Titles and which piece was captured
	captured: Pieces,
	titles: (Pieces, Pieces),
}
