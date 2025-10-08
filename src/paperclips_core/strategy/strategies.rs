use rand::random_bool;

use crate::paperclips_core::strategy::{util::{find_biggest_payoff, what_beats_last}, Move::{self, *}, Position, Side::*, StratPickMove, StrategyBoard};

macro_rules! strats {
    ($(struct $name:ident $impl:tt)*) => {
        $(
            #[derive(Debug, Clone, Copy)]
            pub struct $name {}
            impl StratPickMove for $name $impl
        )*
        #[derive(Debug, Clone, Copy)]
        pub enum StratFunction {
            $(
                $name($name),
            )*
        }
        impl StratFunction {
            fn pick_move(&mut self, board: StrategyBoard, position: Position) -> Move {
                match self {
                    $(
                        Self::$name(strat) => strat.pick_move(board, position),
                    )*
                }
            }
        }
    };
}

strats!{
    struct Random {
        fn pick_move(&mut self, _: StrategyBoard, _: Position) -> Move {
            match random_bool(0.5) {
                true => A,
                false => B,
            }
        }
    }
    struct A100 {
        fn pick_move(&mut self, _: StrategyBoard, _: Position) -> Move { A }
    }
    struct B100 {
        fn pick_move(&mut self, _: StrategyBoard, _: Position) -> Move { B }
    }
    struct Greedy {
        fn pick_move(&mut self, board: StrategyBoard, _: Position) -> Move {
            match find_biggest_payoff(board) {
                AA|AB => A,
                BA|BB => B,
            }
        }
    }
    struct Generous {
        fn pick_move(&mut self, board: StrategyBoard, _: Position) -> Move {
            match find_biggest_payoff(board) {
                AA|BA => A,
                AB|BB => B,
            }
        }
    }
    struct MiniMax {
        fn pick_move(&mut self, board: StrategyBoard, _: Position) -> Move {
            match find_biggest_payoff(board) {
                AA|BA => B,
                AB|BB => A,
            }
        }
    }
    struct TitForTat {
        fn pick_move(&mut self, board: StrategyBoard, position: Position) -> Move {
            match position {
                Position::H => board.previous_vertical_move,
                Position::V => board.previous_horizontal_move,
            }
        }
    }
    struct BeatLast {
        fn pick_move(&mut self, board: StrategyBoard, position: Position) -> Move {
            what_beats_last(position, &board)
        }
    }

}
