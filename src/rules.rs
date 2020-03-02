// Copyright (C) 2020 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use crate::hand::*;


// RULES
pub const PLAYERS: usize = 1;
pub const DECKS: usize = 4;
//pub const SPLIT_BY_VALUE: bool = true;
//pub const SPLIT_TO_X_HANDS: usize = 4;
pub const DOUBLE: bool = true;
pub const DOUBLE_AFTER_SPLIT: bool = false;
//pub const SURRENDER: bool = false;
pub const DEALER_HITS_S17: bool = false;

#[derive(Clone,Copy,PartialEq)]
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
    Stand, Hit, Split, DH, DS
}

pub fn calculate_scoring(player_hands: &Vec<Hand>, dealer_hand: &Hand, bet: usize) -> isize {
    let bet = bet as isize;
    if player_hands[0].blackjack {
        return bet * 3 / 2;
    }
    let mut score = 0;
    for hand in player_hands {
        if hand.value > 21 {
            score -= bet;
        } else if dealer_hand.value > 21 {
            score += bet;
        } else if hand.value < dealer_hand.value {
            score -= bet;
        } else if hand.value > dealer_hand.value {
            score += bet;
        }
    }
    return score;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
