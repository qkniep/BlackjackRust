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
            pair: if SPLIT_BY_VALUE {
                card1.value() == card2.value()
            } else {
                card1 == card2
            },
            blackjack: card1.value() + card2.value() == 21,
            surrendered: false,
            last_card: card2,
        };
        if hand.value == 22 {
            hand.value -= 10
        }
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
            if self.value > 21 {
                self.value -= 10;
            } else {
                self.soft = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(Hand::new(Card::Number(4), Card::Number(7)).value, 11);
        assert_eq!(Hand::new(Card::King, Card::Queen).value, 20);
        assert_eq!(Hand::new(Card::Ace, Card::Ace).value, 12);
        assert_eq!(Hand::new(Card::Number(10), Card::Ace).value, 21);
        assert_eq!(Hand::new(Card::Number(9), Card::Ace).value, 20);
        assert_eq!(Hand::new(Card::Number(9), Card::Number(9)).value, 18);
    }

    #[test]
    fn test_value_after_add() {
        let mut hand = Hand::new(Card::Number(4), Card::Number(4));
        hand.add_card(Card::Number(3));
        assert_eq!(hand.value, 11);
        hand.add_card(Card::Number(6));
        assert_eq!(hand.value, 17);

        let mut hand = Hand::new(Card::Jack, Card::Number(8));
        hand.add_card(Card::Ace);
        assert_eq!(hand.value, 19);
    }

    #[test]
    fn test_blackjack() {
        assert_eq!(Hand::new(Card::Number(10), Card::Ace).blackjack, true);
        assert_eq!(Hand::new(Card::King, Card::Ace).blackjack, true);
        assert_eq!(Hand::new(Card::King, Card::Number(10)).blackjack, false);
        let mut hand = Hand::new(Card::King, Card::Queen);
        hand.add_card(Card::Number(1));
        assert_eq!(hand.blackjack, false);
    }
}
