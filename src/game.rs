use crate::{
    card::{Card, self},
    player::{Dealer, User, Player, self},
};
use rand::{seq::SliceRandom, thread_rng};
use std::{
    collections::VecDeque,
    io::{stdin, stdout, Write},
};

macro_rules! input {
    () => {
        stdout().flush().unwrap();
    };
    ($input: expr) => {
        stdout().flush().unwrap();
        stdin().read_line(&mut $input).unwrap();
    };
}

#[derive(Clone)]
pub struct Game {
    deck: VecDeque<Card>,
    users: Vec<User>,
    dealer: Dealer,
}

impl Game {
    pub fn new() -> Self {
        println!("Loading deck...");
        let mut deck = VecDeque::<Card>::new();
        let suits = vec!['D', 'C', 'S', 'H'];
        let faces = vec![
            "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
            "Queen", "King",
        ];
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10];

        for suit in suits {
            let mut idx = 0;
            for face in &faces {
                deck.push_back(Card::new(suit, face.to_owned(), values[idx]));
                idx += 1;
            }
        }

        // ! Will default to one player for now. Will change if adding online capabilities.
        /*println!("How many players will be playing today?");
        print!(">>> ");
        let mut inp = String::new();
        input!(inp);
        inp = inp.trim().to_owned();
        while let Err(_) = inp.parse::<i32>() {
            println!("That was not a number! Please enter a number");
            print!(">>> ");

            input!(inp);
            inp = inp.trim().to_owned();
        }
        let player_count = inp.parse::<i32>().unwrap();

        println!("Loading players..."); */
        let player_count = 1;
        if player_count >= 2 {
            println!("Loading Players...");
        }
        else {
            println!("Loading Player...");
        }
        let server = Dealer::new();
        let mut users = Vec::<User>::new();
        for _x in 0..player_count {
            users.push(User::new());
        }
        Game {
            deck,
            users,
            dealer: server,
        }
    }

    pub fn get_dealer(&self) -> &Dealer {
        &self.dealer
    }

    pub fn get_players(&self) -> &Vec<User> {
        &self.users
    }

    pub fn game_loop(&mut self) -> bool {
        println!("Dealing...");
        self.deal();
        let mut players = self.users.clone();
        for idx in 0..players.len() {
            players[idx].do_turn();
        }
        true
    }

    //Shhh... since im too lazy to try and implement the standard method of dealing, im just gonna shuffle it between every drawing of 5 cards
    pub fn deal(&mut self) {
        let mut players = self.users.clone();
        for idx in 0..players.len() {
            players[idx].take_cards(self.draw(5));
            self.users[idx] = players[idx].clone();
            self.shuffle(false);
        }
        let drawn = self.draw(5);
        self.dealer.take_cards(drawn);
    }

    fn draw(&mut self, i: i32) -> Vec<Card> {
        let mut ret = Vec::new();
        for _x in 0..i {
            ret.push(match self.deck.pop_front() {
                Some(e) => e,
                None => Card::new('I', "I", 0),
            });
        }

        ret
    }

    pub fn shuffle(&mut self, verbose: bool) {
        if verbose {
            println!("Shuffling deck...");
        }
        let mut vec = Vec::from(self.deck.clone());
        vec.shuffle(&mut thread_rng());

        self.deck = VecDeque::from(vec);
    }
}

//Tests
#[test]
fn draw_test() {
    let mut gayme = Game::new();
    let mut g = VecDeque::<Card>::new();
    for x in 1..=2 {
        g.push_back(Card::new('A', "A", x));
    }
    let mut y = VecDeque::<Card>::new();
    for a in 3..=5 {
        y.push_back(Card::new('A', "A", a));
    }
    let t = gayme.draw(2);
    assert_eq!(VecDeque::from(t), g);
    assert_eq!(gayme.deck, y);
}

#[test]
fn deck_test() {
    let mut gayme = Game::new();
    println!("{:?}", gayme.deck);
    println!("Testing shuffling\n\n");
    gayme.shuffle(true);
    println!("{:?}", gayme.deck);
}

#[test]
fn players_test() {
    let mut gayme = Game::new();
    //gayme.shuffle();
    gayme.deal();
    println!("{:?}", gayme.users);
    println!("\n\n{:?}", gayme.deck);
}


#[test]
fn hand_test() {
    let mut user = player::User::new();
    let mut cards = Vec::<card::Card>::new();
    cards.push(card::Card::new('D', "King", 10));
    cards.push(card::Card::new('S', "Queen", 10));
    cards.push(card::Card::new('C', "Jack", 10));
    cards.push(card::Card::new('H', "Ace", 1));

    user.take_cards(cards);
    user.show_cards();
}

#[test]
fn game_shuffle_hand_test() {
    let mut gayme = Game::new();
    gayme.shuffle(true);
    gayme.deal();

    println!("Dealer:");
    gayme.get_dealer().show_cards();

    let showdown = gayme.get_players().clone();
    for user in &showdown {
        println!(
            "Player: {}",
            showdown.iter().position(|x| x == user).unwrap() + 1
        );
        user.show_cards();
    }
}

#[test]
fn invalid_card_test() {
    let mut user = User::new();
    user.take_cards(vec![Card::new('I', "Invalid", 0), Card::new('S', "King", 10)]);
    user.show_cards();
}
