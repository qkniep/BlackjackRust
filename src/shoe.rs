// Copyright (C) 2020 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use rand::{seq::SliceRandom, thread_rng};

use crate::rules::*;


pub struct Shoe(Vec<Card>);

impl Shoe {
    pub fn new() -> Self {
        Self(Vec::with_capacity(DECKS*52))
    }

    pub fn draw_card(&mut self) -> Card {
        assert!(self.0.len() > 0);
        self.0.pop().unwrap()
    }

    pub fn shuffle(&mut self) {
        self.0.clear();
        for _ in 0..DECKS*4 {
            for n in 2..=10 {
                self.0.push(Card::Number(n));
            }
            self.0.push(Card::Jack);
            self.0.push(Card::Queen);
            self.0.push(Card::King);
            self.0.push(Card::Ace);
        }
        self.0.as_mut_slice().shuffle(&mut thread_rng());
        assert_eq!(self.0.len(), self.0.capacity());
    }

    pub fn num_cards(&self) -> usize {
        self.0.len()
    }
}
