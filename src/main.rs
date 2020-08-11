extern crate ggez;
mod chess;
use ggez::*;

struct State {
    dt: std::time::Duration,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // * Clear to background
        graphics::clear(ctx, graphics::WHITE);

        // * Present updates
        match graphics::present(ctx) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error occured: {}", e);
                panic!();
            }
        }
    }
}

fn main() {
    // * Create players
    let player1: &dyn chess::player::Player = &chess::player::new_human_player();
    let player2: &dyn chess::player::Player = &chess::player::new_human_player();

    // * Create game board
    let mut game_board: chess::board::ChessBoard = chess::board::create_board(player1, player2);

    match game_board.get_piece(0, 0) {
        chess::board::BoardSpot::None => {}
        chess::board::BoardSpot::Piece(piece) => match game_board.move_piece(3, 3, *piece) {
            Err(e) => println!("{}", e),
            Ok(_) => {}
        },
    }

    let r = chess::board::ROW_SIZE;
    for i in 0..r {
        for j in 0..r {
            match game_board.get_piece(i, j) {
                chess::board::BoardSpot::None => print!("X "),
                chess::board::BoardSpot::Piece(_) => print!("H "),
            }
        }
        print!("\n");
    }
    let state = &mut State {
        dt: std::time::Duration::new(0, 0),
    };
    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .conf(c)
        .build()
        .unwrap();
    event::run(ctx, event_loop, state).unwrap();
}
