// Copyright (C) 2020 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

mod deck;
mod hand;
mod strategy;
mod rules;

use deck::*;
use hand::*;
use strategy::*;
use rules::*;


fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let dealer_open_card = deck.draw_card();  // TODO: move to Dealer struct
    let mut dealer = Dealer(Hand::new(dealer_open_card, deck.draw_card()));

    let mut cash = [100isize; PLAYERS];
    let mut hands = [vec![Hand::new(deck.draw_card(), deck.draw_card())]; PLAYERS];
    let bets = [2usize; PLAYERS];

    for _ in 0..100 {
        // Players' turns
        for player in 0..PLAYERS {
            while hands[player][0].value < 21 {
                println!("{:?}", optimal_action(&hands[player][0], &dealer_open_card));
                hands[player][0].add_card(deck.draw_card());
            }
        }

        // Dealer's turn
        while dealer.0.value < 17 || (DEALER_HITS_S17 && dealer.0.soft && dealer.0.value == 17) {
            dealer.0.add_card(deck.draw_card());
        }

        // Scoring
        for i in 0..PLAYERS {
            cash[i] += calculate_scoring(&hands[i], &dealer.0, bets[i]);
            println!("new balance of Player {}: {}", i, cash[i])
        }
    }
}
