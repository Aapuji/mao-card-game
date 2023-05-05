use enum_iterator::Sequence;
use std::fmt::Display;

#[derive(Debug, Clone, Sequence)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Self {
        Self { value, suit }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

#[derive(Debug, Clone, Sequence)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from(match self {
                Self::Ace => "A",
                Self::Two => "2",
                Self::Three => "3",
                Self::Four => "4",
                Self::Five => "5",
                Self::Six => "6",
                Self::Seven => "7",
                Self::Eight => "8",
                Self::Nine => "9",
                Self::Ten => "10",
                Self::Jack => "J",
                Self::Queen => "Q",
                Self::King => "K",
            })
        )
    }
}

#[derive(Debug, Clone, Sequence)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from(match self {
                Self::Clubs => "\u{2663}",
                Self::Diamonds => "\u{2666}",
                Self::Hearts => "\u{2665}",
                Self::Spades => "\u{2660}",
            })
        )
    }
}
