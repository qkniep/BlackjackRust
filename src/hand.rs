// Copyright (C) 2020 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use crate::rules::*;


#[derive(Debug)]
pub struct Hand {
    pub value: usize,
    pub soft: bool,
    pub pair: bool,
    pub blackjack: bool,
    pub last_card: Card,
}

impl Hand {
    pub fn new(card1: Card, card2: Card) -> Hand {
        Hand {
            value: card1.value() + card2.value(),
            soft: card1 == Card::Ace || card2 == Card::Ace,
            pair: if SPLIT_BY_VALUE { card1.value() == card2.value() } else { card1 == card2 },
            blackjack: card1.value() + card2.value() == 21,  // TODO: no blackjack on split
            last_card: card2,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.value += card.value();

        if self.soft {
            if self.value > 21 {
                self.value -= 10;
                self.soft = card == Card::Ace;
            }
        } else if card == Card::Ace {
            if self.value > 21 { self.value -= 10; }
            else { self.soft = true; }
        }

        self.last_card = card;
    }
}

#[derive(Debug)]
pub struct Dealer {
    pub hand: Hand,
    pub open_card: Card,
}

impl Dealer {
    pub fn new(card1: Card, card2: Card) -> Dealer {
        Dealer {
            hand: Hand::new(card1, card2),
            open_card: card1,
        }
    }

    pub fn should_hit(&self) -> bool {
        return self.hand.value < 17 || (DEALER_HITS_S17 && self.hand.soft && self.hand.value == 17);
    }

    pub fn score_hands(&self, hands: &Vec<Hand>, bets: &Vec<usize>) -> isize {
        if hands[0].blackjack {
            return bets[0] as isize * 3 / 2;
        }
        let mut score = 0;
        for (hand, bet) in hands.iter().zip(bets) {
            if hand.value > 21 {
                score -= *bet as isize;
            } else if self.hand.value > 21 {
                score += *bet as isize;
            } else if hand.value < self.hand.value {
                score -= *bet as isize;
            } else if hand.value > self.hand.value {
                score += *bet as isize;
            }
        }
        return score;
    }
}
