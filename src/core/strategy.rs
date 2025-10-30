use std::borrow::Cow;

use arrayvec::ArrayVec;
use rand::random_range;

use crate::{Float, PaperClips, project::PROJECT_128, strategy::strategies::{RANDOM, STRAT_COUNT, Strat}};

mod strategies;
mod util;

#[derive(Debug, Clone, Copy)]
pub enum TourneyDisplay {
    RunTournament,
    /// true => "payoff"
    Results(bool),
    Round,
}

#[derive(Debug, Clone, Copy)]
pub struct StrategyGrid {
    aa: u8,
    ab: u8,
    ba: u8,
    bb: u8,
    previous_horizontal_move: Move,
    previous_vertical_move: Move,
    choice_names: (&'static str, &'static str),
}

fn random_value() -> u8 { random_range(1..=10) }
impl StrategyGrid {
    fn random_self(&mut self) {
        self.aa = random_value();
        self.ab = random_value();
        self.ba = random_value();
        self.bb = random_value();
        self.update_choice_names();
    }
    fn random() -> StrategyGrid {
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

#[derive(Debug, Clone)]
pub struct Strategy {
    pub engine_flag: bool,

    pub grid: StrategyGrid,

    pub strats: ArrayVec<(&'static Strat, u16), STRAT_COUNT>,

    pub yomi: Float,

    pub tourney_cost: Float,
    pub tourney_report_display: TourneyDisplay,

    /// # (vStrat, hStrat)
    pub fight: (&'static Strat, &'static Strat),
    // var stratCounter = 0;
    // var roundNum = 0;
    // var hMove = 1;
    // var vMove = 1;
    // var hMovePrev = 1;
    // var vMovePrev = 1;
    // var rounds = 0;
    current_round: u8,
    // var rCounter = 0;
    tourney_in_prog: bool,
    // var winnerPtr = 0;
    // var placeScore = 0;
    // var showScore = 0;
    pick: usize,
    // var yomi = 0;
    yomi_boost: Float,

    // var allStrats = [];
    // var strats = [];

    // var resultsTimer = 0;
    // var results = [];
    results_flag: bool,


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
            engine_flag: false,
            grid: StrategyGrid::random(),
            strats: ArrayVec::from_iter([(&RANDOM, 0)]),
            yomi: 0.0,

            tourney_cost: 1000.0,
            tourney_report_display: TourneyDisplay::RunTournament,
            fight: (&RANDOM, &RANDOM),
            current_round: 0,
            tourney_in_prog: false,
            pick: 0,
            yomi_boost: 1.0,
            results_flag: false,
        }
    }
}

impl Strategy {
    #[inline]
    pub fn pick_strats(&mut self, round_num: usize) {
        let h = round_num / self.strats.len();
        let v = round_num % self.strats.len();

        self.fight = (self.strats[v].0, self.strats[h].0);
    }
    #[inline]
    pub fn reset_strats(&mut self) {
        self.strats.sort_by(|a, b| a.0.index.cmp(&b.0.index));
        self.strats.iter_mut().for_each(|(_, cs)| *cs = 0);
    }
    #[inline]
    pub const fn rounds(&self) -> u8 {
        self.strats.len() as u8 * self.strats.len() as u8
    }
    #[inline]
    pub fn generate_grid(&mut self) {
        self.grid.random_self();
    }
    pub fn round(&mut self, current_round: u8) {
        // TODO implement round
    }
    #[inline]
    pub fn pick_winner(&mut self) {
        self.strats.sort_by(|a, b| a.1.cmp(&b.1));
    }
    #[inline]
    pub fn picked_strat(&mut self) -> (&'static Strat, u16) {
        self.strats.iter().copied().find(|s| s.0.index == self.pick).unwrap_or((&RANDOM, 0))
    }
    /// Requires the strats to be sorted by score
    #[inline]
    pub fn calculate_strats_beat(&self) -> usize {
        self.strats.len() - self.strats.iter().position(|s| s.0.index == self.pick).unwrap_or(0)
    }
    #[inline]
    pub fn display_tourney_report(&mut self) {
        self.results_flag = true;
    }
    #[inline]
    pub fn tourney_report(&mut self, display: TourneyDisplay) {
        self.tourney_report_display = display;
    }
}

impl PaperClips {
    pub fn new_tourney(&mut self) {
        self.strategy.results_flag = false;

        self.strategy.tourney_in_prog = true;
        self.strategy.current_round = 0;
        self.strategy.reset_strats();
        self.computational.standard_ops -= self.strategy.tourney_cost;
        self.strategy.grid.random_self();
    }
    pub fn run_tourney(&mut self) {
        if self.strategy.current_round < self.strategy.rounds() {
            self.strategy.round(self.strategy.current_round);
        } else {
            self.strategy.tourney_in_prog = false;
            self.strategy.pick_winner();
            // calculate_place_score();
            // calculate_show_score();
            self.declare_winner();
        }
    }
    pub fn declare_winner(&mut self) {
        // if pick < 10 {} // this is assumed to be valid by default

        let mut beat_boost = self.strategy.calculate_strats_beat() - 1;
        let strat_s = match beat_boost == 1 {
            true => "strat",
            false => "strats",
        };
        let bb = match beat_boost == 0 {
            true => {
                beat_boost = 0;
                0
            }
            false => beat_boost,
        };

        let (picked_strat, picked_score) = self.strategy.picked_strat();

        self.strategy.tourney_report(TourneyDisplay::Results(true));
        let yomi_increse = picked_score as Float * self.strategy.yomi_boost * beat_boost as Float;
        self.strategy.yomi += yomi_increse;

        if self.milestone_flag < 15 {
            self.console.push(format!(
                "{} scored {picked_score} and beat {bb} {strat_s}. Yomi increased by {yomi_increse:.0}",
                picked_strat.name,
            ));
        }

        let place = rank_of(picked_score, self.strategy.strats.iter().map(|v| v.1));

        let result = match place {
            1 => Some((50000.0, "Selected strategy won the tournament (or tied for first). +50,000 yomi")),
            2 => Some((30000.0, "Selected strategy finished in (or tied for) second place. +30,000 yomi")),
            3 => Some((20000.0, "Selected strategy finished in (or tied for) third place. +20,000 yomi")),
            _ => None,
        };

        match (self.projects.is_active(PROJECT_128), result) {
            (true, Some((yomi_reward, text))) => {
                self.strategy.yomi += yomi_reward;
                if self.milestone_flag < 15 {
                    self.console.push(text);
                }
            }
            _ => {
                self.strategy.tourney_report(TourneyDisplay::Results(false));
            }
        } 

        // populateTourneyReport(); // this is entirely done at runtime
        self.strategy.display_tourney_report();
    }
}

fn rank_of<T: Copy+PartialEq, I: Iterator<Item = T>>(value: T, mut iter: I) -> usize {
    let mut rank = 1;
    let mut prev = match iter.next() {
        Some(v) => v,
        None => return 0,
    };
    for item in iter {
        if item != prev {
            rank += 1;
            prev = item;
        }
        if item == value {
            return rank;
        }
    }
    // Check if the first value was the match
    if prev == value {
        return 1;
    }
    0
}
