#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Card {
    suit: Suit,
    face: String,
    value: i32,
}

impl Card {
    pub fn new(suit_c: char, face: String, value: i32) -> Self {
        let suit = match suit_c {
            'D' => Suit::Diamonds,
            'C' => Suit::Clubs,
            'S' => Suit::Spades,
            'H' => Suit::Hearts,
            _ => Suit::None,
        };
        Card { suit, face, value }
    }

    pub fn get_suit(&self) -> char {
        match self.suit {
            Suit::Clubs => 'C',
            Suit::Diamonds => 'D',
            Suit::Hearts => 'H',
            Suit::Spades => 'S',
            Suit::None => ' ',
        }
    }

    pub fn get_face(&self) -> char {
        match self.face.as_str() {
            "Ace" => 'A',
            "Two" => '2',
            "Three" => '3',
            "Four" => '4',
            "Five" => '5',
            "Six" => '6',
            "Seven" => '7',
            "Eight" => '8',
            "Nine" => '9',
            "Ten" => 'X',
            "Jack" => 'J',
            "Queen" => 'Q',
            "King" => 'K',
            _ => 'E',
        }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Suit {
    Diamonds,
    Clubs,
    Spades,
    Hearts,
    None,
}
