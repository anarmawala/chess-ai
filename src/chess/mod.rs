pub mod colors;
pub mod piece;
pub mod pieces;

use colors::Colors::{BLACK, NONE, WHITE};
use piece::Piece;
use pieces::Pieces::{BISHOP, EMPTY, HORSE, KING, PAWN, QUEEN, ROOK};

pub struct Chess {
    // * Chess Board
    board: Vec<Piece>,

    // * Player preferences
    player_white: bool,
    difficulty: u8,

    // * Game state variables
    num_move: usize,
}

impl Chess {
    // * Initialize the game board
    pub fn new(play_first: bool, difficulty: u8) -> Self {
        let mut game_board = Vec::new();
        for _row in 1..=8 {
            match _row {
                2 | 7 => {
                    // ? Decide the color base on row number
                    let _color = if _row == 1 { WHITE } else { BLACK };

                    // * Add 8 pawns
                    for _i in 1..=8 {
                        game_board.push(Piece::new(_color, PAWN, _row, _i))
                    }
                }
                1 | 8 => {
                    // ? Decide the color base on row number
                    let _color = if _row == 0 { WHITE } else { BLACK };

                    game_board.push(Piece::new(_color, ROOK, _row, 1));
                    game_board.push(Piece::new(_color, HORSE, _row, 2));
                    game_board.push(Piece::new(_color, BISHOP, _row, 3));
                    game_board.push(Piece::new(_color, QUEEN, _row, 4));
                    game_board.push(Piece::new(_color, KING, _row, 5));
                    game_board.push(Piece::new(_color, BISHOP, _row, 6));
                    game_board.push(Piece::new(_color, HORSE, _row, 7));
                    game_board.push(Piece::new(_color, ROOK, _row, 8));
                }
                3 | 4 | 5 | 6 => {
                    // * Add 8 empty blocks
                    for _i in 1..=8 {
                        game_board.push(Piece::new(NONE, EMPTY, _row, _i))
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

    pub fn board(&self) -> &Vec<Piece> {
        &self.board
    }
}
