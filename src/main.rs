use game::Game;
use player::*;
use std::{fmt::format, io::stdin};

use crate::card::Card;

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

#[test]
fn hand_test() {
    let mut player = player::User::new();
    let mut cards = Vec::<card::Card>::new();
    cards.push(card::Card::new('D', "King".to_owned(), 10));
    cards.push(card::Card::new('S', "Queen".to_owned(), 10));
    cards.push(card::Card::new('C', "Jack".to_owned(), 10));
    cards.push(card::Card::new('H', "Ace".to_owned(), 1));

    player.take_cards(cards);
    player.show_cards();
}

#[test]
fn game_shuffle_hand_test() {
    let mut gayme = Game::new();
    gayme.shuffle(true);
    gayme.deal();

    println!("Dealer:");
    gayme.get_dealer().show_cards();

    let showdown = gayme.get_players().clone();
    for player in &showdown {
        println!(
            "Player: {}",
            showdown.iter().position(|x| x == player).unwrap() + 1
        );
        player.show_cards();
    }
}
