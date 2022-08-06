use crate::card::Card;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct User {
    hand: Hand,
}

impl Player for User {
    fn new() -> Self {
        Self { hand: Hand::new() }
    }

    fn do_turn(&mut self) {
        self.do_turn();
    }

    fn take_cards(&mut self, drawn: Vec<Card>) {
        self.hand.contents = drawn;
    }

    fn show_cards(&self) {
        self.hand.show_hand();
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

    fn do_turn(&mut self) {
        self.do_turn();
    }

    fn take_cards(&mut self, drawn: Vec<Card>) {
        self.hand.contents = drawn;
    }

    fn show_cards(&self) {
        self.hand.show_hand();
    }
}

pub trait Player {
    fn new() -> Self;
    fn do_turn(&mut self);
    fn take_cards(&mut self, drawn: Vec<Card>);
    fn show_cards(&self);
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Hand {
    contents: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        let temp = Vec::<Card>::new();
        Self { contents: temp }
    }

    fn show_hand(&self) {
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
}