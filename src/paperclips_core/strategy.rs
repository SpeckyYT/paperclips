use arrayvec::ArrayVec;
use rand::random_range;

use crate::{paperclips_core::{strategy::strategies::StratFunction, Float}, strategy::strategies::A100};

mod strategies;
mod util;

#[derive(Debug, Clone, Copy)]
pub struct StrategyBoard {
    aa: u8,
    ab: u8,
    ba: u8,
    bb: u8,
    previous_horizontal_move: Move,
    previous_vertical_move: Move,
    choice_names: (&'static str, &'static str),
}

fn random_value() -> u8 { random_range(1..=10) }
impl StrategyBoard {
    fn random_self(&mut self) {
        self.aa = random_value();
        self.ab = random_value();
        self.ba = random_value();
        self.bb = random_value();
        self.update_choice_names();
    }
    fn random() -> StrategyBoard {
        Self {
            aa: random_value(),
            ab: random_value(),
            ba: random_value(),
            bb: random_value(),
            previous_horizontal_move: Move::A,
            previous_vertical_move: Move::A,
            choice_names: Self::random_choice_names(),
        }
    }
    fn update_choice_names(&mut self) {
        self.choice_names = Self::random_choice_names();
    }
    fn random_choice_names() -> (&'static str, &'static str) {
        CHOICE_NAMES[random_range(0..CHOICE_NAMES.len())]
    }
}

pub const CHOICE_NAMES: &[(&str, &str)] = &[
    ("cooperate", "defect"),
    ("swerve", "straight"),
    ("macro", "micro"),
    ("fight", "back_down"),
    ("bet", "fold"),
    ("raise_price", "lower_price"),
    ("opera", "football"),
    ("go", "stay"),
    ("heads", "tails"),
    ("particle", "wave"),
    ("discrete", "continuous"),
    ("peace", "war"),
    ("search", "evaluate"),
    ("lead", "follow"),
    ("accept", "reject"),
    ("accept", "deny"),
    ("attack", "decay"),
]; 

#[derive(Debug, Clone, Copy)]
pub enum Move {
    A = 1,
    B = 2,
}

#[derive(Debug, Clone, Copy)]
pub enum Position {
    H = 1,
    V = 2,
}
impl Position {
    pub fn opposite(&self) -> Self {
        match self {
            Position::V => Position::H,
            Position::H => Position::V,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Side {
    AA = 1,
    AB = 2,
    BA = 3,
    BB = 4,
}

pub trait StratPickMove {
    fn pick_move(&mut self, board: StrategyBoard, current_position: Position) -> Move;
}

struct Strat {
    score: Float,
    function: StratFunction,
}

pub struct Strategy {
    board: StrategyBoard,

    strats: ArrayVec<StratFunction, 8>,

    horizontal_strat: StratFunction,
    vertical_strats: StratFunction,

    pub yomi: Float,

    // var tourneyCost = 1000;
    // var tourneyLvl = 1;
    // var stratCounter = 0;
    // var roundNum = 0;
    // var hMove = 1;
    // var vMove = 1;
    // var hMovePrev = 1;
    // var vMovePrev = 1;
    // var aa = 0;
    // var ab = 0;
    // var ba = 0;
    // var bb = 0;
    // var rounds = 0;
    // var currentRound = 0;
    // var rCounter = 0;
    // var tourneyInProg = 0;
    // var winnerPtr = 0;
    // var placeScore = 0;
    // var showScore = 0;
    // var high = 0;
    // var pick = 10;
    // var yomi = 0;
    // var yomiBoost = 1;

    // var allStrats = [];
    // var strats = [];

    // var resultsTimer = 0;
    // var results = [];
    // var resultsFlag = 0;


    // var payoffGrid = {
    //     valueAA:0,
    //     valueAB:0,
    //     valueBA:0,
    //     valueBB:0,
    // }

}

impl Default for Strategy {
    fn default() -> Self {
        Self {
            board: StrategyBoard::random(),
            strats: ArrayVec::new(),
            horizontal_strat: StratFunction::A100(A100 {}),
            vertical_strats: StratFunction::A100(A100{}),
            yomi: 0.0,
        }
    }
}

impl Strategy {
    pub fn pick_strats(&mut self, round_num: usize) {
        let h = round_num / self.strats.len();
        let v = round_num % self.strats.len();

        let v_strat = self.strats[v];
        let h_strat = self.strats[h];


    }
}
