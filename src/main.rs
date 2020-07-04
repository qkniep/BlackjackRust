// Copyright (C) 2020 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

mod dealer;
mod game;
mod hand;
mod player;
mod rules;
mod shoe;
mod strategy;

use game::Game;
use player::Player;
use strategy::*;

const TESTS: u32 = 1000;
const ROUNDS: u32 = 1000;

fn main() {
    let mut roi_no = 0;
    let mut roi_hilo = 0;
    let mut roi_ko = 0;
    let mut roi_uston = 0;

    let mut co_bankrupt_no = 0.0;
    let mut co_bankrupt_hilo = 0.0;
    let mut co_bankrupt_ko = 0.0;
    let mut co_bankrupt_uston = 0.0;

    println!("Running {} tests...", TESTS);

    for _ in 0..TESTS {
        let mut game = Game::new();
        game.join(Player::new(&NO_COUNT));
        game.join(Player::new(&HILO_COUNT));
        game.join(Player::new(&KO_COUNT));
        game.join(Player::new(&USTON_SS_COUNT));

        let mut bankrupt_no = false;
        let mut bankrupt_hilo = false;
        let mut bankrupt_ko = false;
        let mut bankrupt_uston = false;

        for _ in 0..ROUNDS {
            let bank_before = game.bankrolls();
            game.play_round();

            let bank = game.bankrolls();
            roi_no += bank[0] - bank_before[0];
            roi_hilo += bank[1] - bank_before[1];
            roi_ko += bank[2] - bank_before[2];
            roi_uston += bank[3] - bank_before[3];

            if bank[0] < 0 {
                bankrupt_no = true;
            }
            if bank[1] < 0 {
                bankrupt_hilo = true;
            }
            if bank[2] < 0 {
                bankrupt_ko = true;
            }
            if bank[3] < 0 {
                bankrupt_uston = true;
            }
        }

        if bankrupt_no {
            co_bankrupt_no += 1.0 / TESTS as f64;
        }
        if bankrupt_hilo {
            co_bankrupt_hilo += 1.0 / TESTS as f64;
        }
        if bankrupt_ko {
            co_bankrupt_ko += 1.0 / TESTS as f64;
        }
        if bankrupt_uston {
            co_bankrupt_uston += 1.0 / TESTS as f64;
        }
    }

    println!(
        "Average ROI after {} rounds (no count): {}",
        ROUNDS,
        roi_no as f64 / (ROUNDS * TESTS) as f64
    );
    println!(
        "Average ROI after {} rounds (HL count): {}",
        ROUNDS,
        roi_hilo as f64 / (ROUNDS * TESTS) as f64
    );
    println!(
        "Average ROI after {} rounds (KO count): {}",
        ROUNDS,
        roi_ko as f64 / (ROUNDS * TESTS) as f64
    );
    println!(
        "Average ROI after {} rounds (SS count): {}",
        ROUNDS,
        roi_uston as f64 / (ROUNDS * TESTS) as f64
    );

    println!(
        "Chance of bankrupt after {} rounds (no count): {}",
        ROUNDS, co_bankrupt_no
    );
    println!(
        "Chance of bankrupt after {} rounds (HL count): {}",
        ROUNDS, co_bankrupt_hilo
    );
    println!(
        "Chance of bankrupt after {} rounds (KO count): {}",
        ROUNDS, co_bankrupt_ko
    );
    println!(
        "Chance of bankrupt after {} rounds (SS count): {}",
        ROUNDS, co_bankrupt_uston
    );
}
