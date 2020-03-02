// Copyright (C) 2020 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

// SETTINGS
pub const PLAYERS: usize = 4;
//pub const STARTING_CASH: usize = 1;
//pub const STARTING_BET: usize = 1;

// RULES
pub const DECKS: usize = 4;
pub const SHUFFLE_AT: usize = 1;
pub const SPLIT_BY_VALUE: bool = false;
//pub const SPLIT_TO_X_HANDS: usize = 4;
pub const DOUBLE: bool = true;
pub const DOUBLE_AFTER_SPLIT: bool = false;
pub const SURRENDER: bool = false;
pub const DEALER_HITS_S17: bool = false;

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Card {
    Number(usize), Jack, Queen, King, Ace
}

impl Card {
    pub fn value(&self) -> usize {
        match self {
            Card::Number(n) => *n,
            Card::Jack => 10,
            Card::Queen => 10,
            Card::King => 10,
            Card::Ace => 11,
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Card::Number(n) => n-2,
            Card::Jack => 8,
            Card::Queen => 8,
            Card::King => 8,
            Card::Ace => 9,
        }
    }
}

#[derive(Clone,Copy,Debug)]
pub enum Action {
    Hit, Stand, DH, DS, Split, Surrender
}
