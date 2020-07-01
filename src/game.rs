// Copyright (C) 2020 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use crate::hand::*;
use crate::rules::*;
use crate::shoe::*;
use crate::strategy::*;

pub struct Player {
    bankroll: isize,
    hands: Vec<Hand>,
    bets: Vec<usize>,

    counting_strategy: &'static [i32; 10],
    count: i32,
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

    fn deal(&mut self, hand: Hand) {
        self.hands.clear();
        self.bets.clear();

        let bet_size = std::cmp::max(1, self.count / DECKS as i32 - 1);
        self.bets.push(MINIMUM_BET * bet_size as usize);
        self.hands.push(hand);
    }

    fn reveal(&mut self, card: Card) {
        self.count += self.counting_strategy[card.index()];
    }
}

pub struct Game {
    shoe: Shoe,
    players: Vec<Player>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            shoe: Shoe::new(),
            players: Vec::new(),
        }
    }

    pub fn join(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn play_round(&mut self) {
        self.new_round();

        let mut dealer = Dealer::new(self.draw_reveal(), self.shoe.draw_card());

        // Players' turns
        for player in 0..self.players.len() {
            self.player_turn(player, &dealer);
        }

        // Dealer's turn
        self.reveal_card(dealer.hand.last_card);
        while dealer.should_hit() {
            dealer.hand.add_card(self.draw_reveal());
        }

        // Scoring
        for player in &mut self.players {
            player.bankroll += &dealer.score_hands(&player.hands, &player.bets);
        }
    }

    pub fn bankrolls(&self) -> Vec<isize> {
        self.players.iter().map(|p| p.bankroll).collect()
    }

    fn new_round(&mut self) {
        if self.shoe.num_cards() <= SHUFFLE_AT * 52 {
            self.shoe.shuffle();
            for player in &mut self.players {
                player.count = 0;
            }
        }

        for player in 0..self.players.len() {
            let card1 = self.draw_reveal();
            let card2 = self.draw_reveal();
            self.players[player].deal(Hand::new(card1, card2));
        }
    }

    fn player_turn(&mut self, player: usize, dealer: &Dealer) {
        let mut hand = 0;
        while hand < self.players[player].hands.len() {
            while self.players[player].hands[hand].value < 21 {
                let action = optimal_action(&self.players[player].hands[hand], dealer.open_card);
                match action {
                    Action::Hit => {
                        let card = self.draw_reveal();
                        self.players[player].hands[hand].add_card(card);
                    }
                    Action::Stand => break,
                    Action::DH | Action::DS => {
                        // Double
                        self.players[player].bets[hand] *= 2;
                        let card = self.draw_reveal();
                        self.players[player].hands[hand].add_card(card);
                        break;
                    }
                    Action::RH | Action::RS => {
                        // Surrender
                        self.players[player].hands[hand].surrendered = true;
                        break;
                    }
                    Action::Split => {
                        let card = self.players[player].hands[hand].last_card;
                        let mut hand1 = Hand::new(card, self.draw_reveal());
                        let mut hand2 = Hand::new(card, self.draw_reveal());
                        hand1.natural = false;
                        hand2.natural = false;
                        self.players[player].hands[hand] = hand1;
                        self.players[player].hands.push(hand2);
                        let bet = self.players[player].bets[hand];
                        self.players[player].bets.push(bet);

                        if self.players[player].hands.len() >= SPLIT_TO_X_HANDS {
                            for hand in &mut self.players[player].hands {
                                hand.pair = false; // can no longer split
                            }
                        }
                    }
                }
            }
            hand += 1;
        }
    }

    fn draw_reveal(&mut self) -> Card {
        let card = self.shoe.draw_card();
        self.reveal_card(card);
        card
    }

    fn reveal_card(&mut self, card: Card) {
        for player in &mut self.players {
            player.reveal(card);
        }
    }
}
