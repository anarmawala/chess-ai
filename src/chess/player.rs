use super::pieces;

pub trait Player<'a> {
    fn get_pieces(&self) -> &Vec<&'a dyn pieces::Piece>;
    fn add_piece(&mut self, piece: &'a dyn pieces::Piece);
}

pub struct HumanPlayer<'a> {
    pieces: Vec<&'a dyn pieces::Piece>,
}

impl<'a> Player<'a> for HumanPlayer<'a> {
    fn get_pieces(&self) -> &Vec<&'a dyn pieces::Piece> {
        return &self.pieces;
    }

    fn add_piece(&mut self, piece: &'a dyn pieces::Piece) {
        self.pieces.push(piece);
    }
}

pub fn new_human_player<'a>() -> HumanPlayer<'a> {
    return HumanPlayer { pieces: vec![] };
}

impl<'a> PartialEq for &dyn Player<'a> {
    fn eq(&self, other: &&dyn Player<'a>) -> bool {
        return self == other;
    }

    fn ne(&self, other: &&dyn Player<'a>) -> bool {
        return self != other;
    }
}
