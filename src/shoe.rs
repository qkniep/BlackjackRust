// Copyright (C) 2020 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};

use crate::rules::*;

pub struct Shoe {
    rng: SmallRng,
    num_decks: usize,
    cards: Vec<Card>,
}

impl Shoe {
    pub fn new(decks: usize) -> Self {
        Self {
            rng: SmallRng::from_entropy(),
            num_decks: decks,
            cards: Vec::with_capacity(decks * 52),
        }
    }

    pub fn draw_card(&mut self) -> Card {
        assert!(self.cards.len() > 0);
        self.cards.pop().unwrap()
    }

    pub fn shuffle(&mut self) {
        self.cards.clear();
        for _ in 0..self.num_decks * 4 {
            for n in 2..=10 {
                self.cards.push(Card::Number(n));
            }
            self.cards.push(Card::Jack);
            self.cards.push(Card::Queen);
            self.cards.push(Card::King);
            self.cards.push(Card::Ace);
        }
        self.cards.as_mut_slice().shuffle(&mut self.rng);
    }

    pub fn num_cards(&self) -> usize {
        self.cards.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle() {
        let mut shoe = Shoe::new(8);
        shoe.shuffle();
        assert_eq!(shoe.num_cards(), 52 * 8);
    }
}
