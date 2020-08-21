use std::fmt;

#[derive(Copy, Clone)]
pub enum Colors {
	White,
	Black,
	None,
}

impl fmt::Debug for Colors {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Colors::White => write!(f, "W"),
			Colors::Black => write!(f, "B"),
			Colors::None => write!(f, " "),
		}
	}
}
