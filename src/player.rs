use crate::{card::Card, game::Game};
use std::io::{stdout, stdin, Write};

macro_rules! input {
    () => {
        stdout().flush().unwrap();
    };
    ($input: expr) => {
        stdout().flush().unwrap();
        stdin().read_line(&mut $input).unwrap();
    };
}

pub trait Player {
    fn new() -> Self;
    fn do_turn(&mut self, pos: usize) -> bool;
    fn take_cards(&mut self, drawn: Vec<Card>);
    fn append_hand(&mut self, drawn: &Vec<Card>);
    fn show_cards(&self);
    fn get_hand_as_vec(&self) -> Vec<Card>;
    fn get_hand(&self) -> Hand;
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct User {
    hand: Hand,
}

impl Player for User {
    fn new() -> Self {
        Self { hand: Hand::new() }
    }

    fn do_turn(&mut self, pos: usize) -> bool {
        self.hand.sort();
        println!("Your Hand:");
        self.show_cards();
        println!("Want to continue? (y/n)");
        let mut str = String::new();
        input!(&mut str);
        str = str.trim().to_owned();
        let mut ret: Option<bool> = None;
        while ret == None {
            ret = match str.as_str() {
                "y" => Some(true),
                "n" => Some(false),
                _ => None,
            };
        }
        ret.unwrap()
    }

    fn take_cards(&mut self, drawn: Vec<Card>) {
        self.hand.contents = drawn;
    }

    fn append_hand(&mut self, drawn: &Vec<Card>) {
        self.hand.contents.append(&mut drawn.clone());
    }

    fn show_cards(&self) {
        self.hand.show_hand();
    }

    fn get_hand_as_vec(&self) -> Vec<Card> {
        self.hand.to_vec()
    }

    fn get_hand(&self) -> Hand {
        self.hand.clone()
    }
}


#[derive(Clone)]
pub struct Dealer {
    hand: Hand,
}

impl Player for Dealer {
    fn new() -> Self {
        Self { hand: Hand::new() }
    }

    fn do_turn(&mut self, pos: usize) -> bool {
        println!("Dealer: ");
        self.hand.sort();
        self.show_cards();
        true
    }

    fn take_cards(&mut self, drawn: Vec<Card>) {
        self.hand.contents = drawn;
    }

    fn append_hand(&mut self, drawn: &Vec<Card>) {
        self.hand.contents.append(&mut drawn.clone());
    }

    fn show_cards(&self) {
        self.hand.show_hand();
    }

    fn get_hand_as_vec(&self) -> Vec<Card> {
        self.hand.to_vec()
    }

    fn get_hand(&self) -> Hand {
        self.hand.clone()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Hand {
    pub contents: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        let temp = Vec::<Card>::new();
        Self { contents: temp }
    }

    pub fn show_hand(&self) {
        let top = vec![" _____ ".to_owned(); self.contents.len()].join("");

        let suits = self
            .contents
            .iter()
            .map(|x| format!("|{}    |", x.get_suit()))
            .collect::<String>();

        let faces = self
            .contents
            .iter()
            .map(|x| format!("|  {}  |", x.get_face()))
            .collect::<String>();

        let values = self
            .contents
            .iter()
            .map(|x| {
                format!(
                    "|    {}|",
                    if x.get_value().to_string().chars().nth(0).unwrap() == x.get_face() {
                        x.get_suit().to_string()
                    } else if x.get_value() == 0 {
                        " ".to_owned()
                    } else {
                        match x.get_value().to_string().len() {
                            1 if x.get_value() != 0 => x.get_value().to_string(),
                            2 => format!("\x08{}", x.get_value()),
                            _ => "".to_owned(),
                        }
                    }
                )
            })
            .collect::<String>();

        let bot = vec![" ‾‾‾‾‾ "; self.contents.len()].join("");
        println!("{top}\n{suits}\n{faces}\n{values}\n{bot}");
    }

    pub fn to_vec(&self) -> Vec<Card> {
        self.contents.clone()
    }

    pub fn sort(&mut self) {
        let mut spades = Vec::<Card>::new();
        let mut hearts = Vec::<Card>::new();
        let mut diamonds = Vec::<Card>::new();
        let mut clubs = Vec::<Card>::new();

        self.contents.drain(..).for_each(|x| {
            let _ = match x.get_suit() {
                'S' => spades.push(x),
                'H' => hearts.push(x),
                'D' => diamonds.push(x),
                'C' => clubs.push(x),
                _ => {},
            };
        });

        spades.sort_by(|a, b| a.get_face_value().cmp(&b.get_face_value()));
        hearts.sort_by(|a, b| a.get_face_value().cmp(&b.get_face_value()));
        diamonds.sort_by(|a, b| a.get_face_value().cmp(&b.get_face_value()));
        clubs.sort_by(|a, b| a.get_face_value().cmp(&b.get_face_value()));

        self.contents.append(&mut spades);
        self.contents.append(&mut hearts);
        self.contents.append(&mut diamonds);
        self.contents.append(&mut clubs);
    }
}