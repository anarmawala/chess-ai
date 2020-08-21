mod change;
mod colors;
mod piece;
mod pieces;

pub use change::Change;
pub use colors::Colors;
pub use colors::Colors::{Black, None, White};
pub use piece::Piece;
pub use pieces::Pieces;
pub use pieces::Pieces::{Bishop, Empty, Horse, King, Pawn, Queen, Rook};

pub struct Game {
	// * Chess Board
	board: Vec<Piece>,

	// * Log of moves
	log: Vec<Change>,

	// * Player preferences
	player_white: bool,
	difficulty: u8,
}

impl Game {
	// * Initialize the game board
	pub fn new(play_first: bool, difficulty: u8) -> Self {
		let mut game_board = Vec::new();
		for _row in 1..=8 {
			match _row {
				2 | 7 => {
					// ? Decide the color base on row number
					let _color = if _row <= 2 { White } else { Black };

					// * Add 8 pawns
					for _i in 1..=8 {
						game_board.push(Piece::new(_color, Pawn))
					}
				}
				1 | 8 => {
					// ? Decide the color base on row number
					let _color = if _row <= 1 { White } else { Black };

					game_board.push(Piece::new(_color, Rook));
					game_board.push(Piece::new(_color, Horse));
					game_board.push(Piece::new(_color, Bishop));
					game_board.push(Piece::new(_color, Queen));
					game_board.push(Piece::new(_color, King));
					game_board.push(Piece::new(_color, Bishop));
					game_board.push(Piece::new(_color, Horse));
					game_board.push(Piece::new(_color, Rook));
				}
				3 | 4 | 5 | 6 => {
					// * Add 8 empty blocks
					for _i in 1..=8 {
						game_board.push(Piece::new(None, Empty))
					}
				}
				_ => panic!("Error, Chess only has eight rows"),
			}
		}

		Game {
			player_white: play_first,
			difficulty,
			log: Vec::new(),
			board: game_board,
		}
	}

	pub fn board(&self) -> &Vec<Piece> {
		&self.board
	}
}
