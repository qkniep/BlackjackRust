// Copyright (C) 2020 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use crate::rules::*;


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
            pair: card1 == card2,
            blackjack: card1.value() + card2.value() == 21,  // TODO: no blackjack on split
            last_card: card2,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.value += card.value();

        if self.soft {
            if self.value > 21 {
                self.value -= 10;
                self.soft = card == Card::Ace;  // ???
            }
        } else {
            if card == Card::Ace {
                if self.value > 21 { self.value -= 10; }
                else { self.soft = true; }
            }
        }

        self.last_card = card;
    }
}

pub struct Dealer(pub Hand);

impl Dealer {
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
