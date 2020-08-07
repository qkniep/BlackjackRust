// Copyright (C) 2020 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use crate::hand::Hand;
use crate::rules::*;

pub struct Player {
    pub bankroll: isize,
    pub hands: Vec<Hand>,
    pub bets: Vec<usize>,

    counting_strategy: &'static [i32; 10],
    pub count: i32,
}

impl Player {
    pub fn new(strategy: &'static [i32; 10]) -> Self {
        Self {
            bankroll: 1000,
            hands: Vec::new(),
            bets: Vec::new(),
            counting_strategy: strategy,
            count: 0,
        }
    }

    pub fn deal(&mut self, hand: Hand, remaining_decks: i32) {
        self.hands.clear();
        self.bets.clear();

        // TODO: possible unwanted advantage:
        // player bets after seeing other player's hands
        let bet_size = std::cmp::max(0, self.count / remaining_decks + 1);
        self.bets.push(MINIMUM_BET * bet_size as usize);
        self.hands.push(hand);
    }

    pub fn reveal(&mut self, card: Card) {
        self.count += self.counting_strategy[card.index()];
    }
}