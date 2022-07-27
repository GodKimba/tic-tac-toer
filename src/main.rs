use std::process;

use anyhow::Result;
use tic_tac_toer::{Game, MoveStates};
fn main() -> Result<()> {
    let mut game = Game::new();
    Game::render(&game);

    let mut rounds = 0;
    while rounds < 10 {
        let player1 = MoveStates::X;
        let player2 = MoveStates::O;
        let mov = Game::get_move()?;

        Game::make_a_move(&mut game, mov, player1);
        Game::render(&game);
        if Game::get_winner(&game) {
            process::exit(1)
        }

        let mov = Game::get_move()?;
        Game::make_a_move(&mut game, mov, player2);
        Game::render(&game);
        if Game::get_winner(&game) {
            process::exit(1)
        }
        rounds += 1;
    }

    Ok(())
}
