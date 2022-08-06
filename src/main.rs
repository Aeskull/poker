use game::Game;
use player::*;
use std::io::stdin;

mod card;
mod game;
mod player;

fn main() {
    while menu() {
        let mut game = Game::new();
        game.shuffle(true);
        while game.game_loop() {}
    }
}

fn menu() -> bool {
    println!("Welcome to poker!\n\n To begin a new game, press 'y'. To exit, press 'n'.");
    println!("Just an FYI, Invalid cards appear with a letter 'E'. They cannot be used.");
    let mut inp = String::new();
    stdin().read_line(&mut inp).unwrap();
    if let Some(c) = inp.chars().nth(0) {
        if c == 'y' || c == 'Y' {
            return true;
        } else {
            return false;
        }
    }
    false
}
