pub mod colors;
pub mod piece;
pub mod pieces;

use colors::Colors::{BLACK, NONE, WHITE};
use pieces::Pieces::{BISHOP, EMPTY, HORSE, KING, PAWN, QUEEN, ROOK};

pub struct Chess {
    // * Chess Board
    board: Vec<piece::Piece>,

    // * Player preferences
    player_white: bool,
    difficulty: u8,

    // * Game state variables
    num_move: usize,
}

impl Chess {
    pub fn new(play_first: bool, difficulty: u8) -> Self {
        // * Initialize the game board
        let mut game_board = Vec::new();
        for _row in 0..8 {
            match _row {
                1 | 6 => {
                    // ? Decide the color base on row number
                    let _color = if _row == 1 { WHITE } else { BLACK };

                    // * Add 8 pawns
                    for _i in 0..8 {
                        game_board.push(piece::Piece::new(_color, PAWN, _row + 1, _i + 1))
                    }
                }
                0 | 7 => {
                    // ? Decide the color base on row number
                    let _color = if _row == 0 { WHITE } else { BLACK };

                    game_board.push(piece::Piece::new(_color, ROOK, _row + 1, 1));
                    game_board.push(piece::Piece::new(_color, HORSE, _row + 1, 2));
                    game_board.push(piece::Piece::new(_color, BISHOP, _row + 1, 3));
                    game_board.push(piece::Piece::new(_color, QUEEN, _row + 1, 4));
                    game_board.push(piece::Piece::new(_color, KING, _row + 1, 5));
                    game_board.push(piece::Piece::new(_color, BISHOP, _row + 1, 6));
                    game_board.push(piece::Piece::new(_color, HORSE, _row + 1, 7));
                    game_board.push(piece::Piece::new(_color, ROOK, _row + 1, 8));
                }
                2 | 3 | 4 | 5 => {
                    // * Add 8 empty blocks
                    for _i in 0..8 {
                        game_board.push(piece::Piece::new(NONE, EMPTY, _row + 1, _i + 1))
                    }
                }
                _ => panic!("Error, Chess only has eight rows"),
            }
        }

        Chess {
            player_white: play_first,
            difficulty,
            num_move: 0,
            board: game_board,
        }
    }

    pub fn get_board(&self) -> &Vec<piece::Piece> {
        &self.board
    }
}
