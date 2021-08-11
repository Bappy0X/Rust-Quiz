#![allow(non_snake_case)]

mod lib;

fn main() {
    let this_game = lib::Game::new();

    let input = this_game.take_input("What is your name? (Max 4 characters)".to_string());
    let this_player = lib::Player::new(input);

    this_player.ask_question(&this_game);

    this_player.print_score();
}