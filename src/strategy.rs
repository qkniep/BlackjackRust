// Copyright (C) 2020 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use crate::hand::Hand;
use crate::rules::{Action, Card};
use crate::rules::Action::*;


const DAS: bool = crate::rules::DOUBLE_AFTER_SPLIT;

const PAIR_SPLITTING: [[bool; 10]; 10] = [
//   TWO    THREE  FOUR   FIVE   SIX    SEVEN  EIGHT  NINE   TEN    ACE   <- DEALER
    [DAS,   DAS,   true,  true,  true,  true,  false, false, false, false],  // TWO
    [DAS,   DAS,   true,  true,  true,  true,  false, false, false, false],  // THREE
    [false, false, false, DAS,   DAS,   false, false, false, false, false],  // FOUR
    [false, false, false, false, false, false, false, false, false, false],  // FIVE
    [DAS,   true,  true,  true,  true,  false, false, false, false, false],  // SIX
    [true,  true,  true,  true,  true,  true,  false, false, false, false],  // SEVEN
    [true,  true,  true,  true,  true,  true,  true,  true,  true,  true ],  // EIGHT
    [true,  true,  true,  true,  true,  false, true,  true,  false, false],  // NINE
    [false, false, false, false, false, false, false, false, false, false],  // TEN
    [true,  true,  true,  true,  true,  true,  true,  true,  true,  true ],  // ACE
];

const SOFT_TOTALS: [[Action; 10]; 8] = [
//   TWO    THREE  FOUR   FIVE   SIX    SEVEN  EIGHT  NINE   TEN    ACE   <- DEALER
    [Hit,   Hit,   Hit,   DH,    DH,    Hit,   Hit,   Hit,   Hit,   Hit  ],  // 2
    [Hit,   Hit,   Hit,   DH,    DH,    Hit,   Hit,   Hit,   Hit,   Hit  ],  // 3
    [Hit,   Hit,   DH,    DH,    DH,    Hit,   Hit,   Hit,   Hit,   Hit  ],  // 4
    [Hit,   Hit,   DH,    DH,    DH,    Hit,   Hit,   Hit,   Hit,   Hit  ],  // 5
    [Hit,   DH,    DH,    DH,    DH,    Hit,   Hit,   Hit,   Hit,   Hit  ],  // 6
    [DS,    DS,    DS,    DS,    DS,    Stand, Stand, Stand, Stand, Stand],  // 7
    [Stand, Stand, Stand, Stand, DS,    Stand, Stand, Stand, Stand, Stand],  // 8
    [Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand],  // 9
];

const HARD_TOTALS: [[Action; 10]; 17] = [
//   TWO    THREE  FOUR   FIVE   SIX    SEVEN  EIGHT  NINE   TEN    ACE   <- DEALER
    [Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit  ],  // 4
    [Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit  ],  // 5
    [Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit  ],  // 6
    [Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit  ],  // 7
    [Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit,   Hit  ],  // 8

    [Hit,   DH,    DH,    DH,    DH,    Hit,   Hit,   Hit,   Hit,   Hit  ],  // 9
    [DH,    DH,    DH,    DH,    DH,    DH,    DH,    DH,    Hit,   Hit  ],  // 10
    [DH,    DH,    DH,    DH,    DH,    DH,    DH ,   DH,    DH,    DH   ],  // 11
    [Hit,   Hit,   Stand, Stand, Stand, Hit,   Hit,   Hit  , Hit,   Hit  ],  // 12
    [Stand, Stand, Stand, Stand, Stand, Hit,   Hit,   Hit  , Hit,   Hit  ],  // 13
    [Stand, Stand, Stand, Stand, Stand, Hit,   Hit,   Hit  , Hit,   Hit  ],  // 14
    [Stand, Stand, Stand, Stand, Stand, Hit,   Hit,   Hit  , Hit,   Hit  ],  // 15
    [Stand, Stand, Stand, Stand, Stand, Hit,   Hit,   Hit  , Hit,   Hit  ],  // 16

    [Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand],  // 17
    [Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand],  // 18
    [Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand],  // 19
    [Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand],  // 20
];

// CARD COUNTING STRATEGIES
pub const HILO_COUNT: [i32; 10] = [1, 1, 1, 1, 1, 0, 0, 0, -1, -1];
pub const KO_COUNT: [i32; 10] = [1, 1, 1, 1, 1, 1, 0, 0, -1, -1];
pub const USTON_SS_COUNT: [i32; 10] = [2, 2, 2, 3, 2, 1, 0, -1, -2, -2];

pub fn optimal_action(hand: &Hand, dealer_card: Card) -> Action {
    let dealer_index = dealer_card.index();

    if hand.pair && PAIR_SPLITTING[hand.last_card.index()][dealer_index] {
        Split
    } else if hand.soft {
        SOFT_TOTALS[hand.value-11-2][dealer_index]
    } else {
        HARD_TOTALS[hand.value-4][dealer_index]
    }
}
