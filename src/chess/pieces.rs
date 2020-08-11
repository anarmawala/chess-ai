use super::board;

pub trait Piece {
    // * Base move set
    fn get_base_move_set(&self) -> Vec<(i8, i8)>;

    // * For calculating dynamic movesets like bishops
    fn calculate_valid_move_set(&self, board: &board::ChessBoard) -> Vec<(i8, i8)>;

    // * Return piece type for rendering
    fn get_piece_type(&self) -> String;

    fn set_location(&mut self, x: usize, y: usize);
    fn get_location(&self) -> (usize, usize);
}

impl PartialEq for &dyn Piece {
    fn eq(&self, other: &&dyn Piece) -> bool {
        return self == other;
    }

    fn ne(&self, other: &&dyn Piece) -> bool {
        return self != other;
    }
}

/**
 * @piece-type Horseman
 */
pub struct Horseman {
    pub x: usize,
    pub y: usize,
}

impl Piece for Horseman {
    fn get_base_move_set(&self) -> Vec<(i8, i8)> {
        let mut vec = vec![];
        vec.push((-1, 2));
        vec.push((1, 2));
        vec.push((-2, 1));
        vec.push((-2, -1));
        vec.push((2, 1));
        vec.push((2, -1));
        vec.push((-1, -2));
        vec.push((1, -2));
        return vec;
    }
    fn calculate_valid_move_set(&self, board: &board::ChessBoard) -> Vec<(i8, i8)> {
        return self.get_base_move_set(); // * Horseman has no dynamic movesets
    }

    fn get_piece_type(&self) -> String {
        String::from("Horseman")
    }

    fn set_location(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
    fn get_location(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}
