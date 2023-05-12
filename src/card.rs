use crate::render::engine::{
    BoxDrawingProfile, RenderResult, RenderableElement, TextColor, TextFrameBuffer, TextStyle,
};
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

pub enum RenderableCard {
    Front(Card),
    Back,
}
impl RenderableElement for RenderableCard {
    const W: usize = 5;
    const H: usize = 5;
    fn render_size(&self) -> (usize, usize) {
        (Self::W, Self::H)
    }
    fn render(&self, fb: &mut TextFrameBuffer, x: usize, y: usize) -> RenderResult<()> {
        fb.outline_box(BoxDrawingProfile::Normal, x, y, Self::W, Self::H)?;
        match self {
            Self::Front(card) => {
                let value_str = card.value.name();
                let suit_str = card.suit.name();
                let suit_style = card.suit.draw_style();
                fb.text(value_str, x + 1, y + 1)?;
                fb.text(
                    value_str,
                    x + Self::W - 1 - value_str.len(),
                    y + Self::H - 2,
                )?;
                fb.text(suit_str, x + Self::W / 2, y + Self::H / 2)?;
                fb.style_box(suit_style, x + 1, y + 1, Self::W - 2, Self::H - 2)?;
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
impl Value {
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

#[derive(Debug, Clone, Sequence)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    fn name(&self) -> &str {
        &match self {
            Self::Clubs => "\u{2663}",
            Self::Diamonds => "\u{2666}",
            Self::Hearts => "\u{2665}",
            Self::Spades => "\u{2660}",
        }
    }
    fn draw_style(&self) -> TextStyle {
        match self {
            Self::Clubs => TextStyle::default(),
            Self::Diamonds => TextStyle::fg_only(TextColor::Red),
            Self::Hearts => TextStyle::bg_only(TextColor::Red),
            Self::Spades => TextStyle::default(),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
