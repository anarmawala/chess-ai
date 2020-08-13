mod chess;

use chess::colors::Colors::{BLACK, NONE, WHITE};
use chess::Game;

fn main() {
    let game: Game = Game::new(true, 5);

    let board = game.board();

    for _row in (0..8).rev() {
        for _column in 0..8 {
            let info = board[_row * 8 + _column].get_info();

            let _color = match info.0 {
                BLACK => "B",
                WHITE => "W",
                NONE => " ",
            };

            print!("{:^5} ", info.3);
        }
        println!("\n");
    }
}
