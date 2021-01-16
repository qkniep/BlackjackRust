// Copyright (C) 2020 Quentin Kniep <hello@quentinkniep.com> Distributed under terms of the MIT license.

mod dealer;
mod game;
mod hand;
mod player;
mod rules;
mod shoe;
mod strategy;

use indicatif::{ProgressBar, ProgressStyle};

use game::Game;
use player::Player;
use strategy::*;

const TESTS: u32 = 1000;
const ROUNDS: u32 = 1000;
const TOTAL_ROUNDS: u64 = TESTS as u64 * ROUNDS as u64;

fn main() {
    let mut roi_no = 0.0;
    let mut roi_hilo = 0.0;
    let mut roi_ko = 0.0;
    let mut roi_uston = 0.0;

    let mut avg_final_bankroll_no = 0.0;
    let mut avg_final_bankroll_hilo = 0.0;
    let mut avg_final_bankroll_ko = 0.0;
    let mut avg_final_bankroll_uston = 0.0;

    let mut co_bankrupt_no = 0.0;
    let mut co_bankrupt_hilo = 0.0;
    let mut co_bankrupt_ko = 0.0;
    let mut co_bankrupt_uston = 0.0;

    println!("Running {} tests with {} games each, a total of {} games...", TESTS, ROUNDS, TOTAL_ROUNDS);

    let pb = ProgressBar::new(TOTAL_ROUNDS);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} - {per_sec} ({eta})")
        .progress_chars("#>-"));

    for _ in 0..TESTS {
        pb.inc(ROUNDS as u64);

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
            let bet_sizes = game.bet_sizes();
            let bank = game.bankrolls();

            roi_no += (bank[0] - bank_before[0]) as f64 / bet_sizes[0] as f64;
            roi_hilo += (bank[1] - bank_before[1]) as f64 / bet_sizes[1] as f64;
            roi_ko += (bank[2] - bank_before[2]) as f64 / bet_sizes[2] as f64;
            roi_uston += (bank[3] - bank_before[3]) as f64 / bet_sizes[3] as f64;

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

        avg_final_bankroll_no += game.bankrolls()[0] as f64 / TESTS as f64;
        avg_final_bankroll_hilo += game.bankrolls()[1] as f64 / TESTS as f64;
        avg_final_bankroll_ko += game.bankrolls()[2] as f64 / TESTS as f64;
        avg_final_bankroll_uston += game.bankrolls()[3] as f64 / TESTS as f64;

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

    pb.finish_with_message("Done!");

    println!(
        "Average final bankroll after {} rounds (no count): {}",
        ROUNDS,
        avg_final_bankroll_no
    );
    println!(
        "Average final bankroll after {} rounds (HL count): {}",
        ROUNDS,
        avg_final_bankroll_hilo
    );
    println!(
        "Average final bankroll after {} rounds (KO count): {}",
        ROUNDS,
        avg_final_bankroll_ko
    );
    println!(
        "Average final bankroll after {} rounds (SS count): {}",
        ROUNDS,
        avg_final_bankroll_uston
    );

    println!(
        "Average ROI after {} rounds (no count): {:.2}%",
        ROUNDS,
        roi_no as f64 * 100.0 / (ROUNDS * TESTS) as f64,
    );
    println!(
        "Average ROI after {} rounds (HL count): {:.2}%",
        ROUNDS,
        roi_hilo as f64 * 100.0 / (ROUNDS * TESTS) as f64,
    );
    println!(
        "Average ROI after {} rounds (KO count): {:.2}%",
        ROUNDS,
        roi_ko as f64 * 100.0 / (ROUNDS * TESTS) as f64,
    );
    println!(
        "Average ROI after {} rounds (SS count): {:.2}%",
        ROUNDS,
        roi_uston as f64 * 100.0 / (ROUNDS * TESTS) as f64,
    );

    println!(
        "Chance of bankrupt after {} rounds (no count): {:.2}%",
        ROUNDS,
        co_bankrupt_no * 100.0
    );
    println!(
        "Chance of bankrupt after {} rounds (HL count): {:.2}%",
        ROUNDS,
        co_bankrupt_hilo * 100.0
    );
    println!(
        "Chance of bankrupt after {} rounds (KO count): {:.2}%",
        ROUNDS,
        co_bankrupt_ko * 100.0
    );
    println!(
        "Chance of bankrupt after {} rounds (SS count): {:.2}%",
        ROUNDS,
        co_bankrupt_uston * 100.0
    );
}
