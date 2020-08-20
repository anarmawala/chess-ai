use super::piece::Piece;

#[derive(Copy, Clone)]
pub struct Change {
	old_row: usize,
	old_col: usize,

	new_row: usize,
	new_col: usize,

	piece: Piece,
}
