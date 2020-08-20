mod chess;

use chess::Game;

fn main() {
	let game: Game = Game::new(true, 5);
	let board = game.board();

	for _row in (0..8).rev() {
		for _column in 0..8 {
			let _piece = board[_row * 8 + _column];

			// print!("{:^5?} ", piece.row());
			// print!("{:^5?} ", piece.col());
			// print!("{:^5?} ", piece.title());
			// print!("{:^5?} ", piece.value());
			// print!("{:^5?} ", piece.color());
			// print!("{:^5?} ", piece.moved());
		}
		println!("\n");
	}
}
