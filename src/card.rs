use crate::render::ansi::ANSIColor;
use crate::render::engine::{BoxDrawingProfile, RenderResult, RenderableElement, TextFrameBuffer};
use enum_iterator::Sequence;
use std::cmp::PartialEq;
use std::fmt::Display;
use std::hash::Hash;

/// A struct representing a card, with `value` and `suit` fields.
#[derive(Debug, Copy, Clone, PartialEq, Hash, Sequence)]
pub struct Card {
    value: Value,
    suit: Suit,
}

impl Card {
    /// Makes a new `Card` with value `value` and suit `suit`.
    pub fn new(value: Value, suit: Suit) -> Self {
        Self { value, suit }
    }

    /// Returns `self.suit`.
    pub fn suit(&self) -> Suit {
        self.suit
    }

    /// Returns `self.value`.
    pub fn value(&self) -> Value {
        self.value
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

/// Renderer for cards so they can be displayed to the screen.
pub enum RenderableCard {
    Front(Card),
    Back,
}

impl RenderableElement for RenderableCard {
    const W: usize = 5;
    const H: usize = 5;

    fn render(&self, fb: &mut TextFrameBuffer, x: usize, y: usize) -> RenderResult<()> {
        fb.outline_box(BoxDrawingProfile::Normal, x, y, Self::W, Self::H)?;
        fb.fill_box(' ', x + 1, y + 1, Self::W - 2, Self::H - 2)?;

        match self {
            Self::Front(card) => {
                let value = card.value();
                let value_str = value.name();
                let suit = card.suit();
                let suit_str = suit.name();
                let suit_color = card.suit().color();

                fb.text(value_str, x + 1, y + 1)?;
                fb.text(
                    value_str,
                    x + Self::W - 1 - value_str.len(),
                    y + Self::H - 2,
                )?;
                fb.text(suit_str, x + Self::W / 2, y + Self::H / 2)?;
                fb.style_fg_box(suit_color, x + 1, y + 1, Self::W - 2, Self::H - 2)?;
            }

            Self::Back => {
                fb.fill_box(
                    BoxDrawingProfile::SHADING[2],
                    x + 1,
                    y + 1,
                    Self::W - 2,
                    Self::H - 2,
                )?;
            }
        }

        Ok(())
    }
}

/// An enum of all the possible values a card can have.
#[derive(Debug, Copy, Clone, PartialEq, Hash, Sequence)]
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

impl Value {
    /// Returns the "name" of the value, or what would be displayed on a standard set of cards.
    fn name(&self) -> &str {
        &match self {
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
        }
    }

    /// Returns the effective count value for the card. Eg. Ace: 1, 2: 2, ..., J: 11, Q: 12, K: 13.
    fn count(&self) -> u8 {
        match self {
            Self::Ace => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 10,
            Self::Jack => 11,
            Self::Queen => 12,
            Self::King => 13,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Enum representing all the possible suits for a card.
#[derive(Debug, Copy, Clone, PartialEq, Hash, Sequence)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    /// Returns the name of the suit (what would be displayed).
    fn name(&self) -> &str {
        &match self {
            Self::Clubs => "\u{2663}",
            Self::Diamonds => "\u{2666}",
            Self::Hearts => "\u{2665}",
            Self::Spades => "\u{2660}",
        }
    }

    /// Returns the style of the displayed output.
    ///
    /// Either `ANSIColor::Default` or `ANSIColor::LightRed`.
    fn color(&self) -> ANSIColor {
        match self {
            Self::Clubs => ANSIColor::Default,
            Self::Spades => ANSIColor::Default,
            Self::Diamonds => ANSIColor::Red,
            Self::Hearts => ANSIColor::Red,
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
