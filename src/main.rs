use game::{Game, GameState};
use player::*;
use std::io::{stdin, stdout, Write};
use compare::ToCompare;

mod game;
mod card;
mod player;
mod compare;

macro_rules! input {
    () => {
        stdout().flush().unwrap();
    };
    ($inp: expr) => {
        stdin().read_line(&mut $inp).unwrap();
        stdout().flush().unwrap();
    }
}

fn main() {
    let compare = ToCompare::new();

    println!("Welcome to poker!\n\nTo begin a new game, press 'y'. To exit, press 'n'.");
    println!("Just an FYI, Invalid cards appear with a letter 'E'. They cannot be used.");
    let mut round = 0;
    while menu(round) {
        let mut game = Game::new();
        game.shuffle(true);
        let _ = match game.game_loop() {
            GameState::Stopping => {
                println!("You have folded :(");

                println!("Dealer's hand:");
                let mut t = game.get_dealer().get_hand();
                t.sort();
                t.show_hand();
                
            },
            GameState::Losing => {
                println!("Sorry, but you did not win...");
                println!("You had a {}", get_results(&game.get_users()[0], &compare));

                println!("Dealer's hand:");
                game.get_dealer().show_cards();

                println!("The dealer had a {}", get_results(game.get_dealer(), &compare));

                let temp = game.get_users();
                if temp.len() >= 1 {
                    for idx in 1..temp.len() {
                        println!("Player {}'s hand:", idx);
                        temp[idx].show_cards();
                    }
                }
                
            },
            GameState::Winning => {
                println!("Congrats! You Won!");
                println!("You had a {}", get_results(&game.get_users()[0], &compare));

                println!("Dealer's hand:");
                game.get_dealer().show_cards();

                println!("The dealer only had a {}", get_results(game.get_dealer(), &compare));

                let temp = game.get_users();
                if temp.len() >= 1 {
                    for idx in 1..temp.len() {
                        println!("Player {}'s hand:", idx);
                        temp[idx].show_cards();
                        println!("They had a {}", get_results(&temp[idx], &compare));
                    }
                }
            },
        };
        round += 1;
    }
}

fn menu(round: i32) -> bool {
    if round > 0 {
        println!("Want to play again?");
    }
    input!();

    let mut inp = String::new();
    input!(inp);
    if let Some(c) = inp.chars().nth(0) {
        if c == 'y' || c == 'Y' {
            return true;
        } else {
            return false;
        }
    }
    false
}

fn get_results<T>(player: &T, comp: &ToCompare) -> String 
where T: Player
{

    let ret = match comp.compare(&player.get_hand_as_vec()) {
        0 => "High Card!",
        1 => "One Pair!",
        2 => "Two Pair!",
        3 => "3-of-a-Kind!",
        4 => "Straight!",
        5 => "Flush!",
        6 => "Full House!",
        7 => "4-of-a-Kind!",
        8 => "Straight Flush!",
        9 => "Royal Flush!",
        _ => "Error!",
    };

    ret.to_owned()
}
