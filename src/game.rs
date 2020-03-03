// Copyright (C) 2020 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use crate::deck::*;
use crate::hand::*;
use crate::rules::*;
use crate::strategy::*;


struct Player {
    hands: Vec<Hand>,
    bankroll: isize,
    bets: Vec<usize>,

    count: i32,
}

impl Player {
    fn new() -> Player {
        Player {
            hands: Vec::new(),
            bankroll: 1000,
            bets: Vec::new(),
            count: 0,
        }
    }

    fn reveal(&mut self, card: Card) {
        self.count += USTON_SS_COUNT[card.index()];
    }
}

pub struct Game {
    deck: Deck,
    players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Self {
            deck: Deck::new(),
            players: Vec::new(),
        };

        for _ in 0..PLAYERS {
            game.join(Player::new());
        }

        game
    }

    fn join(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn play_round(&mut self) {
        self.new_round();

        let mut dealer = Dealer::new(self.draw_reveal(), self.draw_reveal());

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
        let mut result = Vec::new();
        for player in &self.players {
            result.push(player.bankroll);
        }
        result
    }

    fn new_round(&mut self) {
        if self.deck.num_cards() <= SHUFFLE_AT*52 {
            self.deck.shuffle();
        }

        let card1 = self.draw_reveal();
        let card2 = self.draw_reveal();
        for player in &mut self.players {
            player.hands.push(Hand::new(card1, card2));
            player.bets.push(MINIMUM_BET);
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
                        self.players[player].hands[hand].add_card(card)
                    },
                    Action::Stand => break,
                    Action::DH | Action::DS => {  // Double
                        assert!(DOUBLE);
                        self.players[player].bets[hand] *= 2;
                        let card = self.draw_reveal();
                        self.players[player].hands[hand].add_card(card);
                        break;
                    },
                    Action::RH | Action::RS => {  // Surrender
                        assert!(SURRENDER);
                        unimplemented!();
                        //break;
                    },
                    Action::Split => {
                        let card = self.players[player].hands[hand].last_card;
                        self.players[player].hands[hand] = Hand::new(card, self.draw_reveal());
                        let drawn_card = self.draw_reveal();
                        self.players[player].hands.push(Hand::new(card, drawn_card));
                        let bet = self.players[player].bets[hand];
                        self.players[player].bets.push(bet);
                    },
                }
            }
            hand += 1;
        }
    }

    fn draw_reveal(&mut self) -> Card {
        let card = self.deck.draw_card();
        self.reveal_card(card);
        card
    }

    fn reveal_card(&mut self, card: Card) {
        for player in &mut self.players {
            player.reveal(card);
        }
    }
}
