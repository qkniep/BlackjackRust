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
    let mut cash = [100isize; PLAYERS];
    let mut bets: [Vec<usize>; PLAYERS];

    let mut count = 0;

    deck.shuffle();
    for _ in 0..100 {
        if deck.num_cards() <= SHUFFLE_AT*52 {
            deck.shuffle();
            count = 0;
        }
        let mut dealer = Dealer::new(deck.draw_card(), deck.draw_card());
        count += USTON_SS_COUNT[dealer.open_card.index()];
        let mut hands: [Vec<Hand>; PLAYERS] = Default::default();
        bets = Default::default();

        for p in 0..PLAYERS {
            let card1 = deck.draw_card();
            let card2 = deck.draw_card();
            count += USTON_SS_COUNT[card1.index()] + USTON_SS_COUNT[card2.index()];
            hands[p].push(Hand::new(card1, card2));
            bets[p].push(1);
        }

        // increase counter's bet
        if count >= 10 {
            bets[0] = vec![4];
        }

        // Players' turns
        for player in 0..PLAYERS {
            let mut hand = 0;
            while hand < hands[player].len() {
                while hands[player][hand].value < 21 {
                    let action = optimal_action(&hands[player][hand], dealer.open_card);
                    match action {
                        Action::Hit => hands[player][hand].add_card(deck.draw_card()),
                        Action::Stand => break,
                        Action::DH => {
                            if DOUBLE { bets[player][hand] *= 2; }
                            let card = deck.draw_card();
                            count += USTON_SS_COUNT[card.index()];
                            hands[player][hand].add_card(card);
                        },
                        Action::DS => if DOUBLE {
                            bets[player][hand] *= 2;
                            let card = deck.draw_card();
                            count += USTON_SS_COUNT[card.index()];
                            hands[player][hand].add_card(card);
                        },
                        Action::Split => {
                            let card = hands[player][hand].last_card;
                            let card1 = deck.draw_card();
                            count += USTON_SS_COUNT[card1.index()];
                            hands[player][hand] = Hand::new(card, card1);
                            let card2 = deck.draw_card();
                            count += USTON_SS_COUNT[card2.index()];
                            hands[player].push(Hand::new(card, card2));
                            bets[player].push(bets[player][hand]);
                        },
                        Action::Surrender => {},
                    }
                }
                hand += 1;
            }
        }

        // Dealer's turn
        while dealer.should_hit() {
            dealer.hand.add_card(deck.draw_card());
        }

        // Scoring
        for i in 0..PLAYERS {
            cash[i] += &dealer.score_hands(&hands[i], &bets[i]);
            println!("new balance of Player {}: {}", i, cash[i])
        }
    }
}
