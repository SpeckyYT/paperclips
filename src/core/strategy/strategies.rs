use rand::random_bool;

use crate::core::strategy::{util::{find_biggest_payoff, what_beats_last}, Move::{self, *}, Position, Side::*, StrategyBoard};

#[derive(Debug, Clone, Copy)]
pub struct Strat {
    name: &'static str,
    pick_move: fn(board: StrategyBoard, position: Position) -> Move,
}

macro_rules! strats {
    ($($name:ident { $(# $str:literal)? $($prop:ident: $val:expr),* $(,)? })*) => {
        $(
            pub const $name: Strat = Strat {
                name: [$($str,)? stringify!($name)][0],
                $(
                    $prop: $val,
                )*
            };
        )*
        pub const STRAT_COUNT: usize = [$(stringify!($name),)*].len();
        pub const ALL_STRATS: [Strat; STRAT_COUNT] = [$($name,)*];
    };
}

strats!{
    RANDOM {
        pick_move: |_, _| {
            match random_bool(0.5) {
                true => A,
                false => B,
            }
        },
    }
    A100 {
        pick_move: |_, _| A,
    }
    B100 {
        pick_move: |_, _| B,
    }
    GREEDY {
        pick_move: |board, _| {
            match find_biggest_payoff(board) {
                AA|AB => A,
                BA|BB => B,
            }
        }
    }
    GENEROUS {
        pick_move: |board, _| {
            match find_biggest_payoff(board) {
                AA|BA => A,
                AB|BB => B,
            }
        }
    }
    MINIMAX {
        pick_move: |board, _| {
            match find_biggest_payoff(board) {
                AA|BA => B,
                AB|BB => A,
            }
        }
    }
    TIT_FOR_TAT {
        # "TIT FOR TAT"
        pick_move: |board, position| {
            match position {
                Position::H => board.previous_vertical_move,
                Position::V => board.previous_horizontal_move,
            }
        }
    }
    BEAT_LAST {
        # "BEAT LAST"
        pick_move: |board, position| {
            what_beats_last(position, &board)
        }
    }
}
