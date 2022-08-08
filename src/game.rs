use crate::{
    card::{Card, self},
    player::{Dealer, User, Player, Hand, self},
    compare::ToCompare,
};
use rand::{seq::SliceRandom, thread_rng};
use std::collections::VecDeque;

pub enum GameState {
    Stopping,
    Losing,
    Winning,
}

#[derive(Clone)]
pub struct Game {
    compare: ToCompare,
    deck: VecDeque<Card>,
    users: Vec<User>,
    dealer: Dealer,
    flop: Hand,
    turn: Hand,
    river: Hand,
}

impl Game {
    pub fn new() -> Self {
        let compare = ToCompare::new();
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
                deck.push_back(Card::new(suit, *face, values[idx]));
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
        let user_count = 1;
        if user_count >= 2 {
            println!("Loading Players...");
        }
        else {
            println!("Loading Player...");
        }
        let dealer = Dealer::new();
        let mut users = Vec::<User>::new();
        for _x in 0..user_count {
            users.push(User::new());
        }
        Game {
            compare,
            deck,
            users,
            dealer,
            flop: Hand::new(),
            turn: Hand::new(),
            river: Hand::new(),
        }
    }

    pub fn get_dealer(&self) -> &Dealer {
        &self.dealer
    }

    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }

    pub fn game_loop(&mut self) -> GameState {
        println!("Dealing...");
        self.deal();

        //Initial betting
        //let mut users = self.users.clone();
        for idx in 0..self.users.len() {
            if self.users[idx].do_turn(idx) == false {
                return GameState::Stopping;
            }
        }

        //Burn
        let _ = self.draw(1);

        //Draw the flop
        println!("The Flop:");
        self.flop.contents = self.draw(3);
        self.flop.show_hand();

        //Add to dealer deck
        self.dealer.append_hand(&mut self.flop.to_vec());

        for idx in 0..self.users.len() {
            self.users[idx].append_hand(&mut self.flop.to_vec());
            if self.users[idx].do_turn(idx) == false {
                return GameState::Stopping;
            }
        }

        //Burn
        let _ = self.draw(1);

        //Draw the turn
        println!("The Turn:");
        self.turn.contents = self.draw(1);
        self.turn.show_hand();

        //Add to dealer deck
        self.dealer.append_hand(&mut self.turn.to_vec());

        for idx in 0..self.users.len() {
            self.users[idx].append_hand(&mut self.turn.to_vec());
            if self.users[idx].do_turn(idx) == false {
                return GameState::Stopping;
            }
        }

        //Burn
        let _ = self.draw(1);

        //Draw the river
        println!("The River:");
        self.river.contents = self.draw(1);
        self.river.show_hand();

        //Add to dealer deck
        self.dealer.append_hand(&mut self.river.to_vec());

        for idx in 0..self.users.len() {
            self.users[idx].append_hand(&mut self.river.to_vec());
            if self.users[idx].do_turn(idx) == false {
                return GameState::Stopping;
            }
        }

        let score = self.compare.compare(&self.users[0].get_hand_as_vec());
        let dealer_score = self.compare.compare(&self.dealer.get_hand_as_vec());

        return match score >= dealer_score {
            true => GameState::Winning,
            false => GameState::Losing,
        }
    }

    //Shhh... since im too lazy to try and implement the standard method of dealing, im just gonna shuffle it between every drawing of 5 cards
    pub fn deal(&mut self) {
        let mut users = self.users.clone();
        for idx in 0..users.len() {
            users[idx].take_cards(self.draw(2));
            self.users[idx] = users[idx].clone();
            self.shuffle(false);
        }
        let drawn = self.draw(2);
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

//? Tests

#[test]
fn deck_test() {
    let mut gayme = Game::new();
    println!("{:?}", gayme.deck);
    println!("Testing shuffling\n\n");
    gayme.shuffle(true);
    println!("{:?}", gayme.deck);
}

#[test]
fn users_test() {
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

    let showdown = gayme.get_users().clone();
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
