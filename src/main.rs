// Copyright (C) 2020 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

mod deck;
mod game;
mod hand;
mod strategy;
mod rules;

use game::*;


fn main() {
    let mut game = Game::new();

    for _ in 0..100 {
        game.play_round();
    }

    println!("{:?}", game.bankrolls());
}
