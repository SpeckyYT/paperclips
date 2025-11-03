use rand::random_bool;

use crate::core::strategy::{util::{find_biggest_payoff, what_beats_last}, Move::{self, *}, Position, Side::*, StrategyGrid};

#[derive(Debug, Clone, Copy)]
pub struct Strat {
    pub name: &'static str,
    pub index: usize,
    pub pick_move: fn(board: StrategyGrid, position: Position) -> Move,
}

impl PartialEq for Strat {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

macro_rules! strats {
    (@gen $i:expr; ) => {};
    (@gen $i:expr; $name:ident { $(# $str:literal)? $($prop:ident: $val:expr),* $(,)? } $($rest:tt)*) => {
        pub const $name: Strat = Strat {
            name: [$($str,)? stringify!($name)][0],
            index: $i,
            $($prop: $val,)*
        };
        strats!(@gen ($i + 1usize); $($rest)*);
    };
    ($($name:ident { $(# $str:literal)? $($prop:ident: $val:expr),* $(,)? })*) => {
        strats!(@gen 0usize; $($name { $(# $str)? $($prop: $val),* })*);
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
