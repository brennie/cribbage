// Copyright 2019 Barret Rennie
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Cards, decks, and their components.

use std::fmt;

use rand::seq::SliceRandom;
use rand::Rng;

/// A card suit.
///
/// A card has one of four suits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

/// A static array of all suits used by [`Suit::iter()`](enum.Suit.html#method.iter) to generate an
/// iterator over all suits.
static SUITS: [Suit; 4] = [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club];

impl Suit {
    /// Return an iterator over all suits.
    pub fn iter() -> impl Iterator<Item = Suit> {
        SUITS.iter().cloned()
    }
}

/// A card's rank.
///
/// A card has one of thirteen ranks, starting at [`Ace`](enum.Rank.html#variant.Ace) up to
/// [`King`](enum.Rank.html#variant.King).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

/// A static array of all ranks used [`Rank::iter()`](enum.Rank.html#method.iter) to generate an
/// iterator over all ranks.
static RANKS: [Rank; 13] = [
    Rank::Ace,
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
];

impl Rank {
    /// Return an iterator over all ranks.
    pub fn iter() -> impl Iterator<Item = Rank> {
        RANKS.iter().cloned()
    }
}

/// A card, which is a combination of a suit and a rank.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            CARD_DISPLAY[self.suit as usize][self.rank as usize]
        )
    }
}

/// A lookup table mapping cards indexed by suit and rank to their characters for display.
static CARD_DISPLAY: [[char; 13]; 4] = [
    [
        '\u{1F0A1}',
        '\u{1F0A2}',
        '\u{1F0A3}',
        '\u{1F0A4}',
        '\u{1F0A5}',
        '\u{1F0A6}',
        '\u{1F0A7}',
        '\u{1F0A8}',
        '\u{1F0A9}',
        '\u{1F0AA}',
        '\u{1F0AB}',
        '\u{1F0AD}',
        '\u{1F0AE}',
    ],
    [
        '\u{1F0B1}',
        '\u{1F0B2}',
        '\u{1F0B3}',
        '\u{1F0B4}',
        '\u{1F0B5}',
        '\u{1F0B6}',
        '\u{1F0B7}',
        '\u{1F0B8}',
        '\u{1F0B9}',
        '\u{1F0BA}',
        '\u{1F0BB}',
        '\u{1F0BD}',
        '\u{1F0BE}',
    ],
    [
        '\u{1F0C1}',
        '\u{1F0C2}',
        '\u{1F0C3}',
        '\u{1F0C4}',
        '\u{1F0C5}',
        '\u{1F0C6}',
        '\u{1F0C7}',
        '\u{1F0C8}',
        '\u{1F0C9}',
        '\u{1F0CA}',
        '\u{1F0CB}',
        '\u{1F0CD}',
        '\u{1F0CE}',
    ],
    [
        '\u{1F0D1}',
        '\u{1F0D2}',
        '\u{1F0D3}',
        '\u{1F0D4}',
        '\u{1F0D5}',
        '\u{1F0D6}',
        '\u{1F0D7}',
        '\u{1F0D8}',
        '\u{1F0D9}',
        '\u{1F0DA}',
        '\u{1F0DB}',
        '\u{1F0DD}',
        '\u{1F0DE}',
    ],
];

/// A deck of cards.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deck(Vec<Card>);

impl Deck {
    /// Generate a new sorted (i.e., unshuffled) deck.
    pub fn new_sorted() -> Self {
        let mut cards = Vec::with_capacity(52);

        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card { suit, rank })
            }
        }

        Deck(cards)
    }

    /// Generate a new shuffled deck of cards.
    pub fn new_shuffled<R>(rng: &mut R) -> Self
    where
        R: Rng,
    {
        let mut deck = Deck::new_sorted();
        deck.shuffle(rng);
        deck
    }

    /// Shuffle the deck of cards.
    pub fn shuffle<R>(&mut self, rng: &mut R)
    where
        R: Rng,
    {
        self.0.shuffle(rng)
    }

    /// Return a reference to the cards in the deck.
    ///
    /// The first element of the returned slice is the bottom of the deck and the last element is
    /// the top of the deck.
    pub fn cards(&self) -> &[Card] {
        &self.0
    }

    /// Deal out two hands.
    pub fn deal(&mut self) -> (Hand, Hand) {
        let mut dealer = Vec::with_capacity(6);
        let mut opponent = Vec::with_capacity(6);

        opponent.push(self.0.pop().unwrap());
        dealer.push(self.0.pop().unwrap());
        opponent.push(self.0.pop().unwrap());
        dealer.push(self.0.pop().unwrap());
        opponent.push(self.0.pop().unwrap());
        dealer.push(self.0.pop().unwrap());
        opponent.push(self.0.pop().unwrap());
        dealer.push(self.0.pop().unwrap());
        opponent.push(self.0.pop().unwrap());
        dealer.push(self.0.pop().unwrap());
        opponent.push(self.0.pop().unwrap());
        dealer.push(self.0.pop().unwrap());

        (
            Hand {
                unplayed: dealer,
                played: Vec::with_capacity(6),
            },

            Hand {
                unplayed: opponent,
                played: Vec::with_capacity(6),
            },
        )
    }

    /// Cut the deck randomly and return the cut card.
    pub fn cut<R>(&mut self, rng: &mut R) -> Card
    where
        R: Rng,
    {
        let index: usize = rng.gen_range(0, self.0.len());
        self.0.remove(index)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Hand {
    unplayed: Vec<Card>,
    played: Vec<Card>,
}

impl Hand {
    pub fn cards(&self) -> &[Card] {
        &self.unplayed
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sorted_deck() {
        use Rank::*;
        use Suit::*;

        assert_eq!(
            Deck::new_sorted(),
            Deck(vec![
                Card {
                    suit: Spade,
                    rank: Ace
                },
                Card {
                    suit: Spade,
                    rank: Two
                },
                Card {
                    suit: Spade,
                    rank: Three
                },
                Card {
                    suit: Spade,
                    rank: Four
                },
                Card {
                    suit: Spade,
                    rank: Five
                },
                Card {
                    suit: Spade,
                    rank: Six
                },
                Card {
                    suit: Spade,
                    rank: Seven
                },
                Card {
                    suit: Spade,
                    rank: Eight
                },
                Card {
                    suit: Spade,
                    rank: Nine
                },
                Card {
                    suit: Spade,
                    rank: Ten
                },
                Card {
                    suit: Spade,
                    rank: Jack
                },
                Card {
                    suit: Spade,
                    rank: Queen
                },
                Card {
                    suit: Spade,
                    rank: King
                },
                Card {
                    suit: Heart,
                    rank: Ace
                },
                Card {
                    suit: Heart,
                    rank: Two
                },
                Card {
                    suit: Heart,
                    rank: Three
                },
                Card {
                    suit: Heart,
                    rank: Four
                },
                Card {
                    suit: Heart,
                    rank: Five
                },
                Card {
                    suit: Heart,
                    rank: Six
                },
                Card {
                    suit: Heart,
                    rank: Seven
                },
                Card {
                    suit: Heart,
                    rank: Eight
                },
                Card {
                    suit: Heart,
                    rank: Nine
                },
                Card {
                    suit: Heart,
                    rank: Ten
                },
                Card {
                    suit: Heart,
                    rank: Jack
                },
                Card {
                    suit: Heart,
                    rank: Queen
                },
                Card {
                    suit: Heart,
                    rank: King
                },
                Card {
                    suit: Diamond,
                    rank: Ace
                },
                Card {
                    suit: Diamond,
                    rank: Two
                },
                Card {
                    suit: Diamond,
                    rank: Three
                },
                Card {
                    suit: Diamond,
                    rank: Four
                },
                Card {
                    suit: Diamond,
                    rank: Five
                },
                Card {
                    suit: Diamond,
                    rank: Six
                },
                Card {
                    suit: Diamond,
                    rank: Seven
                },
                Card {
                    suit: Diamond,
                    rank: Eight
                },
                Card {
                    suit: Diamond,
                    rank: Nine
                },
                Card {
                    suit: Diamond,
                    rank: Ten
                },
                Card {
                    suit: Diamond,
                    rank: Jack
                },
                Card {
                    suit: Diamond,
                    rank: Queen
                },
                Card {
                    suit: Diamond,
                    rank: King
                },
                Card {
                    suit: Club,
                    rank: Ace
                },
                Card {
                    suit: Club,
                    rank: Two
                },
                Card {
                    suit: Club,
                    rank: Three
                },
                Card {
                    suit: Club,
                    rank: Four
                },
                Card {
                    suit: Club,
                    rank: Five
                },
                Card {
                    suit: Club,
                    rank: Six
                },
                Card {
                    suit: Club,
                    rank: Seven
                },
                Card {
                    suit: Club,
                    rank: Eight
                },
                Card {
                    suit: Club,
                    rank: Nine
                },
                Card {
                    suit: Club,
                    rank: Ten
                },
                Card {
                    suit: Club,
                    rank: Jack
                },
                Card {
                    suit: Club,
                    rank: Queen
                },
                Card {
                    suit: Club,
                    rank: King
                }
            ])
        );
    }
}
