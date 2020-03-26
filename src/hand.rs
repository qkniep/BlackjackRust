// Copyright (C) 2020 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use crate::rules::*;


#[derive(Debug)]
pub struct Hand {
    pub value: usize,
    pub natural: bool,
    pub soft: bool,
    pub pair: bool,
    pub blackjack: bool,
    pub surrendered: bool,
    pub last_card: Card,
}

impl Hand {
    pub fn new(card1: Card, card2: Card) -> Self {
        let mut hand = Self {
            value: card1.value() + card2.value(),
            natural: true,
            soft: card1 == Card::Ace || card2 == Card::Ace,
            pair: if SPLIT_BY_VALUE { card1.value() == card2.value() } else { card1 == card2 },
            blackjack: card1.value() + card2.value() == 21,
            surrendered: false,
            last_card: card2,
        };
        if hand.value == 22 { hand.value -= 10 }
        hand
    }

    pub fn add_card(&mut self, card: Card) {
        self.value += card.value();
        self.natural = false;
        self.last_card = card;

        if self.soft {
            if self.value > 21 {
                self.value -= 10;
                self.soft = card == Card::Ace;
            }
        } else if card == Card::Ace {
            if self.value > 21 { self.value -= 10; }
            else { self.soft = true; }
        }
    }
}

#[derive(Debug)]
pub struct Dealer {
    pub hand: Hand,
    pub open_card: Card,
}

impl Dealer {
    pub fn new(card1: Card, card2: Card) -> Self {
        Self {
            hand: Hand::new(card1, card2),
            open_card: card1,
        }
    }

    pub fn should_hit(&self) -> bool {
        return self.hand.value < 17 || (DEALER_HITS_S17 && self.hand.soft && self.hand.value == 17);
    }

    pub fn score_hands(&self, hands: &Vec<Hand>, bets: &Vec<usize>) -> isize {
        let mut score = 0;
        let bets = bets.iter().map(|x| *x as isize);
        for (hand, bet) in hands.iter().zip(bets) {
            if hand.surrendered {  // Player Surrender
                score -= bet / 2;
            } else if self.hand.blackjack {  // Dealer Blackjack
                if hand.blackjack && hand.natural { continue; }
                score -= bet;
            } else if hand.blackjack && hand.natural {  // Player Blackjack
                score += bet * 3 / 2;
            } else if hand.value > 21 {  // Player Bust
                score -= bet;
            } else if self.hand.value > 21 {  // Dealer Bust
                score += bet;
            } else if hand.value < self.hand.value {  // Dealer stronger hand
                score -= bet;
            } else if hand.value > self.hand.value {  // Player stronger hand
                score += bet;
            }
        }
        score
    }
}
