// Copyright (C) 2020 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use crate::hand::Hand;
use crate::rules::*;
use crate::rules::Action::*;

const DAS: bool = DOUBLE_AFTER_SPLIT;

const PAIR_SPLITTING: [[bool; 10]; 10] = [
//   2      3      4      5      6      7      8      9      10     A     <- DEALER
    [DAS,   DAS,   true,  true,  true,  true,  false, false, false, false],  // 2
    [DAS,   DAS,   true,  true,  true,  true,  false, false, false, false],  // 3
    [false, false, false, DAS,   DAS,   false, false, false, false, false],  // 4
    [false, false, false, false, false, false, false, false, false, false],  // 5
    [DAS,   true,  true,  true,  true,  false, false, false, false, false],  // 6
    [true,  true,  true,  true,  true,  true,  false, false, false, false],  // 7
    [true,  true,  true,  true,  true,  true,  true,  true,  true,  true ],  // 8
    [true,  true,  true,  true,  true,  false, true,  true,  false, false],  // 9
    [false, false, false, false, false, false, false, false, false, false],  // 10
    [true,  true,  true,  true,  true,  true,  true,  true,  true,  true ],  // A
];

const SOFT_TOTALS: [[Action; 10]; 9] = [
//   2      3      4      5      6      7      8      9      10     A     <- DEALER
    [Hit,   Hit,   Hit,   Hit,   DH,    Hit,   Hit,   Hit,   Hit,   Hit  ],  // 12
    [Hit,   Hit,   Hit,   DH,    DH,    Hit,   Hit,   Hit,   Hit,   Hit  ],  // 13
    [Hit,   Hit,   Hit,   DH,    DH,    Hit,   Hit,   Hit,   Hit,   Hit  ],  // 14
    [Hit,   Hit,   DH,    DH,    DH,    Hit,   Hit,   Hit,   Hit,   Hit  ],  // 15
    [Hit,   Hit,   DH,    DH,    DH,    Hit,   Hit,   Hit,   Hit,   Hit  ],  // 16
    [Hit,   DH,    DH,    DH,    DH,    Hit,   Hit,   Hit,   Hit,   Hit  ],  // 17
    [Stand, DS,    DS,    DS,    DS,    Stand, Stand, Hit,   Hit,   Hit  ],  // 18
    [Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand],  // 19
    [Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand],  // 20
];

const HARD_TOTALS: [[Action; 10]; 17] = [
//   2      3      4      5      6      7      8      9      10     A     <- DEALER
    [Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit  ],  // 4
    [Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit  ],  // 5
    [Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit  ],  // 6
    [Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit  ],  // 7
    [Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit  ],  // 8

    [Hit,   DH,    DH,    DH,    DH,    Hit,   Hit,   Hit,   Hit,   Hit  ],  // 9
    [DH,    DH,    DH,    DH,    DH,    DH,    DH,    DH,    Hit,   Hit  ],  // 10
    [DH,    DH,    DH,    DH,    DH,    DH,    DH ,   DH,    DH,    Hit  ],  // 11
    [Hit,   Hit,   Stand, Stand, Stand, Hit,   Hit,   Hit,   Hit,   Hit  ],  // 12
    [Stand, Stand, Stand, Stand, Stand, Hit,   Hit,   Hit,   Hit,   Hit  ],  // 13
    [Stand, Stand, Stand, Stand, Stand, Hit,   Hit,   Hit,   Hit,   Hit  ],  // 14
    [Stand, Stand, Stand, Stand, Stand, Hit,   Hit,   Hit,   RH,    Hit  ],  // 15
    [Stand, Stand, Stand, Stand, Stand, Hit,   Hit,   RH,    RH,    Hit  ],  // 16

    [Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand],  // 17
    [Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand],  // 18
    [Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand],  // 19
    [Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand],  // 20
];

// CARD COUNTING STRATEGIES
pub const NO_COUNT: [i32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub const HILO_COUNT: [i32; 10] = [1, 1, 1, 1, 1, 0, 0, 0, -1, -1];
pub const KO_COUNT: [i32; 10] = [1, 1, 1, 1, 1, 1, 0, 0, -1, -1];
pub const USTON_SS_COUNT: [i32; 10] = [2, 2, 2, 3, 2, 1, 0, -1, -2, -2];
//pub const REVERE_POINT_COUNT: [i32; 10] = [1, 2, 2, 2, 2, 1, 0, 0, -2, -2];
//pub const RAPC_COUNT: [i32; 10] = [2, 3, 3, 4, 3, 2, 0, -1, -3, -4];

pub fn optimal_action(hand: &Hand, dealer_card: Card) -> Action {
    let dealer_index = dealer_card.index();
    let action;

    if hand.pair && PAIR_SPLITTING[hand.last_card.index()][dealer_index] {
        action = Split;
    } else if hand.soft {
        action = SOFT_TOTALS[hand.value-12][dealer_index];
    } else {
        action = HARD_TOTALS[hand.value-4][dealer_index];
    }

    if !DOUBLE || hand.num_cards > 2 || (!hand.natural && !DAS) {
        if action == Action::DH {
            return Hit
        } else if action == Action::DS {
            return Stand;
        }
    }
    if !SURRENDER || !hand.natural {
        if action == Action::RH {
            return Hit;
        } else if action == Action::RS {
            return Stand;
        }
    }
    action
}
