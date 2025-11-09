use std::time::Duration;

use arrayvec::ArrayVec;
use rand::random_range;

use crate::{Float, PaperClips, Ticks, project::PROJECT_128, strategy::strategies::{RANDOM, STRAT_COUNT, Strat}, util::ticks_10ms};

pub mod strategies;
pub mod util;

#[derive(Debug, Clone, Copy)]
pub enum TourneyDisplay {
    RunTournament,
    /// true => "payoff"
    Results(bool),
    Round,
}

#[derive(Debug, Clone, Copy)]
pub struct StrategyGrid {
    pub aa: u8,
    pub ab: u8,
    pub ba: u8,
    pub bb: u8,
    pub h_move: Move,
    pub v_move: Move,
    pub h_move_prev: Move,
    pub v_move_prev: Move,
    pub choice_names: (&'static str, &'static str),
}

#[inline]
fn random_value() -> u8 { random_range(1..=10) }
impl StrategyGrid {
    pub fn random_self(&mut self) {
        self.aa = random_value();
        self.ab = random_value();
        self.ba = random_value();
        self.bb = random_value();
        self.update_choice_names();
    }
    pub fn random() -> StrategyGrid {
        Self {
            aa: random_value(),
            ab: random_value(),
            ba: random_value(),
            bb: random_value(),
            h_move: Move::A,
            v_move: Move::A,
            h_move_prev: Move::A,
            v_move_prev: Move::A,
            choice_names: Self::random_choice_names(),
        }
    }
    pub fn update_choice_names(&mut self) {
        self.choice_names = Self::random_choice_names();
    }
    fn random_choice_names() -> (&'static str, &'static str) {
        CHOICE_NAMES[random_range(0..CHOICE_NAMES.len())]
    }
    pub fn get_values(&self, h: Move, v: Move) -> (u8, u8) {
        use Move::*;
        match (h, v) {
            (A, A) => (self.aa, self.aa),
            (A, B) => (self.ab, self.ba),
            (B, A) => (self.ba, self.ab),
            (B, B) => (self.bb, self.bb),
        }
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
    pub yomi_boost: Float,

    pub tourney_cost: Float,
    pub tourney_report_display: TourneyDisplay,

    /// # (hStrat, vStrat)
    pub fight: (&'static Strat, &'static Strat),
    // var stratCounter = 0;
    // var roundNum = 0;
    // var rounds = 0;
    pub current_round: u8,
    /// # rCounter
    pub round_counter: u8,
    pub tourney_in_prog: bool,
    // var winnerPtr = 0;
    // var placeScore = 0;
    // var showScore = 0;
    pub pick: &'static Strat,

    pub auto_tourney_flag: bool,
    pub auto_tourney_status: bool,

    round_timer: Ticks,

    clear_grid: bool,

    // var allStrats = [];
    // var strats = [];

    pub results_timer: u64,
    // var results = [];
    pub results_flag: bool,


    // var payoffGrid = {
    //     valueAA:0,
    //     valueAB:0,
    //     valueBA:0,
    //     valueBB:0,
    // }

    pub disable_run_button: bool,
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
            round_counter: 10,
            tourney_in_prog: false,
            pick: &RANDOM,
            yomi_boost: 1.0,
            auto_tourney_flag: false,
            auto_tourney_status: false,
            round_timer: Ticks::MAX,
            clear_grid: false,
            results_timer: 0,
            results_flag: false,

            disable_run_button: true,
        }
    }
}

impl Strategy {
    #[inline]
    pub fn pick_strats(&mut self) {
        let round_num = self.current_round as usize;
        let h = round_num / self.strats.len();
        let v = round_num % self.strats.len();
        self.fight = (self.strats[h].0, self.strats[v].0);
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
    #[inline]
    pub fn pick_winner(&mut self) {
        self.strats.sort_by(|a, b| b.1.cmp(&a.1));
    }
    #[inline]
    pub fn picked_strat(&mut self) -> (&'static Strat, u16) {
        self.strats.iter().copied().find(|s| s.0.index == self.pick.index).unwrap_or((&RANDOM, 0))
    }
    #[inline]
    pub fn get_strat(&mut self, i: usize) -> &mut (&'static Strat, u16) {
        self.strats.iter_mut().find(|s| s.0.index == i).unwrap()
    }
    /// Requires the strats to be sorted by score
    #[inline]
    pub fn calculate_strats_beat(&self) -> usize {
        self.strats.len() - self.strats.iter().position(|s| s.0.index == self.pick.index).unwrap_or(0) + 1
    }
    #[inline]
    pub fn display_tourney_report(&mut self) {
        self.results_flag = true;
    }
    #[inline]
    pub fn tourney_report(&mut self, display: TourneyDisplay) {
        self.tourney_report_display = display;
    }
    #[inline]
    pub fn is_running_round(&self) -> bool {
        self.round_counter < 10
    }

    pub fn round_setup(&mut self) {
        self.round_counter = 0;
        self.pick_strats();
        self.tourney_report_display = TourneyDisplay::Round;
    }

    pub fn run_round(&mut self) {
        self.round_counter += 1;

        let new_h_move = (self.fight.0.pick_move)(self.grid, Position::H);
        let new_v_move = (self.fight.1.pick_move)(self.grid, Position::V);

        let StrategyGrid { h_move, v_move, h_move_prev, v_move_prev, .. } = &mut self.grid;
        (*h_move_prev, *h_move) = (*h_move, new_h_move);
        (*v_move_prev, *v_move) = (*v_move, new_v_move);

        self.calc_payoff(new_h_move, new_v_move);
    }

    pub fn calc_payoff(&mut self, hm: Move, vm: Move) {
        let (hv, vv) = self.grid.get_values(hm, vm);

        // TODO: optimize this stuff
        let h = self.get_strat(self.fight.0.index);
        h.1 += hv as u16;
        let v = self.get_strat(self.fight.1.index);
        v.1 += vv as u16;
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
        self.strategy.disable_run_button = false;
        self.strategy.tourney_report_display = TourneyDisplay::RunTournament;
    }
    pub fn run_tourney(&mut self) {
        self.strategy.disable_run_button = true;
        if self.strategy.current_round < self.strategy.rounds() {
            self.tourney_round();
        } else {
            self.strategy.tourney_in_prog = false;
            self.strategy.pick_winner();
            // calculate_place_score();
            // calculate_show_score();
            self.declare_winner();
        }
    }

    /// # round()
    pub fn tourney_round(&mut self) {
        self.strategy.round_setup();
        self.round_loop();
    }
    pub fn round_loop(&mut self) {
        if self.strategy.round_counter < 10 {
            self.strategy.run_round();
            self.strategy.round_timer = self.ticks;
        } else {
            self.strategy.current_round += 1;
            self.run_tourney();
        }
    }
    /// Can run at any frequency (preferrably 10ms) 
    pub fn round_tick(&mut self) {
        if self.strategy.round_counter <= 10 && self.ticks > self.strategy.round_timer  {
            let time_passed = self.ticks - self.strategy.round_timer;
            
            const FIFTY_MS: Duration = Duration::from_millis(50);
            if time_passed >= ticks_10ms(FIFTY_MS) {
                self.strategy.clear_grid = true;
            }
            if time_passed >= ticks_10ms(2 * FIFTY_MS) {
                self.strategy.clear_grid = false;
                self.strategy.round_timer = Ticks::MAX;
                self.round_loop();
            }
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
