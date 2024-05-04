use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, EnumIter)]
pub enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

#[derive(Debug, Clone, EnumIter)]
pub enum Rank {
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

impl Rank {
    pub fn value(&self) -> u8 {
        match self {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

pub struct Deck {
    pub deck: Vec<Card>
}

impl Deck {
    pub fn create_deck() -> Self {
        let mut deck = Vec::new();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                deck.push(Card { rank: rank.clone(), suit: suit.clone() });
            }
        }

        Deck { deck }
    }
}

pub fn get_deck() {
    let deck = Deck::create_deck();
    for card in deck.deck {
        println!("{:?} of {:?}", card.rank, card.suit);
    }
}