use tic_tac_toer::Game;
fn main() {
    let game = Game::new();
    Game::render(&game);
    Game::get_move();
}
