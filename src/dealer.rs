// Copyright (C) 2020 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use crate::hand::*;
use crate::rules::*;

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
        self.hand.value < 17 || (DEALER_HITS_S17 && self.hand.soft && self.hand.value == 17)
    }

    pub fn score_hands(&self, hands: &Vec<Hand>, bets: &Vec<usize>) -> isize {
        let mut score = 0;
        let bets = bets.iter().map(|x| *x as isize);
        for (hand, bet) in hands.iter().zip(bets) {
            if hand.surrendered {
                // Player Surrender
                score -= bet / 2;
            } else if self.hand.blackjack {
                // Dealer Blackjack
                if !(hand.blackjack && hand.natural) {
                    score -= bet;
                }
            } else if hand.blackjack && hand.natural {
                // Player Blackjack
                score += bet * 3 / 2;
            } else if hand.value > 21 {
                // Player Bust
                score -= bet;
            } else if self.hand.value > 21 {
                // Dealer Bust
                score += bet;
            } else if hand.value < self.hand.value {
                // Dealer stronger hand
                score -= bet;
            } else if hand.value > self.hand.value {
                // Player stronger hand
                score += bet;
            }
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scoring() {
        let dealer = Dealer::new(Card::Ace, Card::King);
        let hands = vec![Hand::new(Card::Number(8), Card::Queen)];
        assert_eq!(dealer.score_hands(&hands, &vec![2]), -2);

        let hands = vec![Hand::new(Card::Ace, Card::Queen)];
        assert_eq!(dealer.score_hands(&hands, &vec![7]), 0);

        let dealer = Dealer::new(Card::Number(3), Card::Number(3));
        let mut hands = vec![Hand::new(Card::Number(7), Card::Queen)];
        hands[0].add_card(Card::Number(9));
        assert_eq!(dealer.score_hands(&hands, &vec![3]), -3);

        let dealer = Dealer::new(Card::Jack, Card::Number(8));
        let hands = vec![Hand::new(Card::Number(9), Card::Queen)];
        assert_eq!(dealer.score_hands(&hands, &vec![4]), 4);
    }
}
