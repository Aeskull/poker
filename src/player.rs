use crate::card::Card;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Player {
    hand: Vec<Card>,
}

impl Player {
    pub fn new() -> Self {
        let hand = Vec::<Card>::new();
        Player { hand }
    }

    pub fn print_hand(&self) {
        let top = vec![" _____ ".to_owned(); self.hand.len()].join("");

        let suits = self
            .hand
            .iter()
            .map(|x| format!("|{}    |", x.get_suit()))
            .collect::<String>();

        let faces = self
            .hand
            .iter()
            .map(|x| format!("|  {}  |", x.get_face()))
            .collect::<String>();

        let values = self
            .hand
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
                            1 => x.get_value().to_string(),
                            2 => format!("\x08{}", x.get_value()),
                            _ => "".to_owned(),
                        }
                    }
                )
            })
            .collect::<String>();

        let bot = vec![" ‾‾‾‾‾ "; self.hand.len()].join("");
        println!("{top}\n{suits}\n{faces}\n{values}\n{bot}");
    }

    pub fn do_turn(&mut self) {}

    pub fn take_cards(&mut self, drawn: Vec<Card>) {
        self.hand = drawn;
    }
}

#[derive(Clone)]
pub struct Dealer {
    contents: Player,
}

impl Dealer {
    pub fn new() -> Self {
        Self {
            contents: Player::new(),
        }
    }

    pub fn do_turn(&mut self) {
        self.contents.do_turn();
    }

    pub fn take_cards(&mut self, drawn: Vec<Card>) {
        self.contents.hand = drawn;
    }

    pub fn show_cards(&self) {
        self.contents.print_hand();
    }
}
