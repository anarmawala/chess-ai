use super::pieces;
use super::player;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum BoardSpot<T> {
    Piece(T),
    None,
}

pub enum PlayerType<T> {
    Player(T),
    None,
}

pub const ROW_SIZE: usize = 8;

pub struct ChessBoard<'a> {
    board: Vec<BoardSpot<&'a dyn pieces::Piece>>,
    player_one: &'a dyn player::Player<'a>,
    player_two: &'a dyn player::Player<'a>,
    current_player: PlayerType<&'a dyn player::Player<'a>>,
}

impl<'a> ChessBoard<'a> {
    pub fn reset(&mut self) {
        // * Set P1
        for i in ROW_SIZE - 2..ROW_SIZE {
            for j in 0..ROW_SIZE {
                let piece = &pieces::Horseman { x: 0, y: 0 };
                let idx = (i * ROW_SIZE) + j;
                self.board[idx] = BoardSpot::Piece(piece);
            }
        }

        // * Set P2
        for i in 0..2 {
            for j in 0..ROW_SIZE {
                let piece = &pieces::Horseman { x: 0, y: 0 };
                let idx = (i * ROW_SIZE) + j;
                self.board[idx] = BoardSpot::Piece(piece);
            }
        }

        // * Empty board spaces
        for i in 2..ROW_SIZE - 2 {
            for j in 0..ROW_SIZE {
                let idx = (i * ROW_SIZE) + j;
                self.board[idx] = BoardSpot::None;
            }
        }
    }

    pub fn move_piece(
        self: &mut ChessBoard<'a>,
        dx: usize,
        dy: usize,
        piece: &'a dyn pieces::Piece,
    ) -> Result<u32, String> {
        // * Check if piece already in destination
        if self.board[(ROW_SIZE * dy) + dx] != BoardSpot::None {
            return Err(String::from("Piece already in location!"));
        }
        // * nullify previous location
        let (x, y) = piece.get_location();
        self.board[(ROW_SIZE * dy) + dx] = BoardSpot::None;

        // * Set piece
        self.board[(ROW_SIZE * dy) + dx] = BoardSpot::Piece(piece);
        return Ok(((ROW_SIZE * dy) + dx) as u32);
    }

    pub fn get_piece(&self, row: usize, col: usize) -> &BoardSpot<&'a dyn pieces::Piece> {
        return &self.board[(ROW_SIZE * row) + col];
    }

    pub fn get_board(&self) -> &Vec<BoardSpot<&'a dyn pieces::Piece>> {
        return &self.board;
    }

    pub fn set_current_player(&mut self, player: &'a dyn player::Player<'a>) {
        self.current_player = PlayerType::Player(player);
    }
}

pub fn create_board<'a>(
    p1: &'a dyn player::Player<'a>,
    p2: &'a dyn player::Player<'a>,
) -> ChessBoard<'a> {
    let mut gb: ChessBoard<'a> = ChessBoard {
        board: vec![BoardSpot::None; ROW_SIZE * ROW_SIZE],
        player_one: p1,
        player_two: p2,
        current_player: PlayerType::None,
    };

    gb.reset();

    gb
}
