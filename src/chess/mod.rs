mod change;
mod colors;
mod piece;
mod pieces;

pub use change::Change;
pub use colors::Colors;
pub use colors::Colors::{BLACK, NONE, WHITE};
pub use piece::Piece;
pub use pieces::Pieces;
pub use pieces::Pieces::{BISHOP, EMPTY, HORSE, KING, PAWN, QUEEN, ROOK};

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
                    let _color = if _row <= 2 { WHITE } else { BLACK };

                    // * Add 8 pawns
                    for _i in 1..=8 {
                        game_board.push(Piece::new(_color, PAWN))
                    }
                }
                1 | 8 => {
                    // ? Decide the color base on row number
                    let _color = if _row <= 1 { WHITE } else { BLACK };

                    game_board.push(Piece::new(_color, ROOK));
                    game_board.push(Piece::new(_color, HORSE));
                    game_board.push(Piece::new(_color, BISHOP));
                    game_board.push(Piece::new(_color, QUEEN));
                    game_board.push(Piece::new(_color, KING));
                    game_board.push(Piece::new(_color, BISHOP));
                    game_board.push(Piece::new(_color, HORSE));
                    game_board.push(Piece::new(_color, ROOK));
                }
                3 | 4 | 5 | 6 => {
                    // * Add 8 empty blocks
                    for _i in 1..=8 {
                        game_board.push(Piece::new(NONE, EMPTY))
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
