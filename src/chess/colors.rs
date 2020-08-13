use std::fmt;

#[derive(Copy, Clone)]
pub enum Colors {
    WHITE,
    BLACK,
    NONE,
}

impl fmt::Debug for Colors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Colors::WHITE => write!(f, "W"),
            Colors::BLACK => write!(f, "B"),
            Colors::NONE => write!(f, " "),
        }
    }
}
