// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::io::{stdin, stdout, Write};

use blackjack_simulator::shoe;

pub const COUNTS: [[i32; 10]; 3] = [
//   2  3  4  5  6  7  8  9  10   A
    [1, 1, 1, 1, 1, 0, 0, 0, -1, -1],  // HiLo
    [1, 1, 1, 1, 1, 1, 0, 0, -1, -1],  // KO
    [2, 2, 2, 3, 2, 1, 0, -1, -2, -2], // Uston SS
];

fn main() {
    loop {
        println!("== Card Counting Trainer ==");
        println!("(1) HiLo Count");
        println!("(2) KO Count");
        println!("(3) Uston SS");
        print!("Which count do you want to practice? ");
        let _ = stdout().flush();
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Reading input failed!");
        train_count(s.lines().next().unwrap().parse().unwrap());
    }
}

fn train_count(count: usize) {
    let counting_table = COUNTS[count-1];
    let mut actual_count = 0;

    let mut deck = shoe::Shoe::new(1);
    deck.shuffle();

    while deck.num_cards() > 10 {
        let card = deck.draw_card();
        actual_count += counting_table[card.index()];
        print!("{:?}", card);
        let _ = stdout().flush();

        // wait for user to continue
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Reading input failed!");
    }

    print!("What was the final count? ");
    let _ = stdout().flush();
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Reading input failed!");
    let user_count = s.lines().next().unwrap().parse().unwrap();

    if actual_count == user_count {
        println!("Perfect! That was the exact count.");
    } else {
        let delta = actual_count - user_count;
        println!("Almost, the correct count was {}, you were off by {}.", actual_count, delta);
    }
    println!("");
}
