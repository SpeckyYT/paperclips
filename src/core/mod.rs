#![allow(clippy::unnecessary_cast)]

use std::time::Duration;

use crate::{business::Business, core::{computational::Computational, investments::Investments, console::Console, qchips::QChips, wire::Wire}, factory::Factory, project::{PROJECT_35, Projects}, rng::PCRng, space::{Space, TOTAL_MATTER}, strategy::Strategy, sounds::Threnody, util::ticks_10ms};

// Can easily get changed with f128 in the future
pub type Float = f64;
pub type Ticks = u128;

pub mod business;
pub mod console;
pub mod wire;
pub mod computational;
pub mod qchips;
pub mod project;
pub mod strategy;
pub mod util;
pub mod investments;
pub mod factory;
pub mod space;
pub mod sounds;
pub mod cheat;
pub mod rng;

#[derive(Debug, Clone)] // TODO: , Serialize, Deserialize
pub struct PaperClips {
    pub ticks: u128,

    pub milestone_flag: u8,
    pub human_flag: bool,

    pub console: Console,

    pub business: Business,
    pub wire: Wire,
    pub computational: Computational, 
    pub projects: Projects,
    pub qchips: QChips,
    pub investments: Investments,
    pub strategy: Strategy,
    pub factory: Factory,
    pub space: Space,
    pub threnody: Threnody,
    pub rng: PCRng,
}

impl Default for PaperClips {
    fn default() -> Self {
        Self {
            ticks: 0,

            milestone_flag: 0,
            human_flag: true,

            business: Business::default(),
            wire: Wire::default(),
            qchips: QChips::default(),
            computational: Computational::default(),
            projects: Projects::default(),
            console: Console::default(),
            investments: Investments::default(),
            strategy: Strategy::default(),
            factory: Factory::default(),
            space: Space::default(),
            threnody: Threnody::default(),
            rng: PCRng::default(),
        }
    }
}

impl PaperClips {
    /// Should run once every 10ms
    pub fn main_tick(&mut self) {
        self.ticks += 1;
        
        self.milestone_check();
        self.button_update();

        if self.computational.comp_flag {
            self.computational.calculate_operations();
        }

        if self.human_flag {
            self.calculate_trust();
        }

        if self.qchips.q_flag {
            self.quantum_compute_update();
        }

        // update_stats();
        self.manage_projects();
        // why does it happen twice lmao
        self.milestone_check();

        // Clip Rate Tracker
        let Business { prev_clips, clip_rate_temp, clip_rate, clips, .. } = &mut self.business;
        if self.ticks.is_multiple_of(ticks_10ms(Duration::from_secs(1))) {
            *clip_rate = *clip_rate_temp;
            *clip_rate_temp = 0.0;
        } else {
            let cr = *clips - *prev_clips;
            *clip_rate_temp += cr;
            *prev_clips = *clips;
        }

        // Stock Report
        if self.investments.engine_flag && self.ticks.is_multiple_of(ticks_10ms(Duration::from_secs(100))) {
            let r = self.investments.ledger + self.investments.port_total();
            self.console.push(format!("Lifetime investment revenue report: ${r:.2}"));
        }

        // WireBuyer
        if self.human_flag && self.wire.buyer_flag && self.wire.buyer_status && self.wire.count <= 1.0 {
            self.buy_wire();
        }

        // First, Explore
        self.space.explore_universe();
        // Then, Drones
        if !self.human_flag && !self.space.space_flag {
            // update_drone_buttons(); // This is managed by the GUI
        }

        self.update_power();
        self.update_swarm();
        self.acquire_matter();
        // process_matter();

        // // Then Factories    
        // let fbst = if factory_boost > 1 {
        //     fbst = factory_boost * factory_level;
        // } else { 1 };
        // if (dismantle < 4) {
        //     clipClick(powMod * fbst * (Math.floor(factoryLevel) * factoryRate));
        // }

        // // Then Other Probe Functions
        // if spaceFlag {
        //     if probe_count < 0 {
        //         probe_count = 0;
        //     }
        //     encounter_hazards();
        //     spawn_factories();
        //     spawn_harvesters();
        //     spawn_wire_drones();
        //     spawn_probes();
        //     drift();
        //     war();
        // }

        // // Auto-Clipper
        // if (dismantle < 4) {
            self.clip_click(self.business.clipper_boost * (self.business.clipper_level / 100.0));
            self.clip_click(self.business.mega_clipper_boost * (self.business.mega_clipper_level * 5.0));
        // }

        // Demand Curve 
        if self.human_flag {
            // put everything into this function
            self.business.update_demand();
        }

        // Creativity
        let Computational { creativity_flag, operations, .. } = &mut self.computational;
        if *creativity_flag && *operations >= self.computational.max_operations() as Float {
            self.computational.calculate_creativity();
        }

        // Stuff that has to be global because the original code uses `setTimeout` and other stuff
        if self.strategy.engine_flag {
            self.round_tick();
        }

        // Ending

        // lots of code


        // Other updating ticks
        if self.ticks.is_multiple_of(ticks_10ms(Duration::from_millis(1000))) {
            self.update_stock_shop_tick();
        }
        if self.ticks.is_multiple_of(ticks_10ms(Duration::from_millis(2500))) {
            self.update_stocks_tick();
        }
        if self.ticks.is_multiple_of(ticks_10ms(Duration::from_millis(100))) {
            self.update_wire_price_and_demand_tick();
        }

        // Small fixes that aren't in the original code
        if self.console.messages.is_empty() {
            self.console.push("Welcome to Universal Paperclips");
        }
    }

    /// Should run once every 1000ms
    pub fn update_stock_shop_tick(&mut self) {
        if self.human_flag {
            self.stock_shop();
            self.business.calculate_rev();
        }
    }

    /// Should run once every 2500ms
    pub fn update_stocks_tick(&mut self) {
        self.investments.sell_delay += 1;
        if self.human_flag && !self.investments.stocks.is_empty() {
            if self.investments.sell_delay >= 5 && self.rng.random_bool_no_best(0.3) {
                self.sell_stock();
                self.investments.sell_delay = 0;
            }
            self.update_stocks();
        }
    }

    /// Should run once every 100ms
    pub fn update_wire_price_and_demand_tick(&mut self) {
        // Wire Price Fluctuation
        self.wire.adjust_wire_price(&mut self.rng);

        if self.human_flag {
            // Sales Calculator
            if self.rng.random_bool((self.business.demand as f64 / 100.0).clamp(0.0, 1.0), true) {
                self.sell_clips(self.business.scaled_demand().floor());
            }

            // here there was "Fire Once a Second" with `calculateRev()`, but it got moved into the 1000ms loop
        }
    }

    pub fn button_update(&mut self) {
        if self.strategy.results_flag && self.strategy.auto_tourney_flag && self.strategy.auto_tourney_status {
            self.strategy.results_timer += 1;

            if self.strategy.results_timer >= 300 && self.computational.operations >= self.strategy.tourney_cost {
                self.new_tourney();
                self.run_tourney();
                self.strategy.results_timer = 0;
            }
        }

        if !self.human_flag {
            self.investments.engine_flag = false;
            self.wire.buyer_flag = false;
        }

    }

    pub fn milestone_check(&mut self) {
        if !self.computational.comp_flag && (
            self.business.unsold_clips < 1.0 && self.business.funds < self.wire.cost && self.wire.count < 1.0
            || self.business.clips.ceil() >= 2000.0
        ) {
            self.computational.comp_flag = true;
            self.projects.flag = true;
            self.console.push("Trust-Constrained Self-Modification enabled");
        }

        macro_rules! milestones {
            ($([$milestone:literal] $condition:tt => $($code:block)? $($kind:ident $text:expr;)?)*) => {
                $(
                    if self.milestone_flag == $milestone && milestones!(@ $condition) {
                        self.milestone_flag += 1;
                        $( $code; )?
                        $( milestones!(@ $kind $text); )?
                    }
                )*
            };
            (@ time $text:literal) => {
                let message = self.milestone_string($text);
                self.console.push(message);
            };
            (@ text $text:literal) => { self.console.push($text); };
            (@ (clips($amount:expr))) => { self.business.clips >= $amount as Float };
            (@ ($condition:expr)) => { $condition };
        }

        milestones!{
            [0] (self.business.funds >= self.business.clipper_cost) =>
                { self.business.clipper_flag = true; }
                text "AutoClippers available for purchase";
            [1] (clips(500)) => time "500 clips created";
            [2] (clips(1000)) => time "1,000 clips created";
            [3] (clips(10000)) => time "10,000 clips created";
            [4] (clips(100000)) => time "100,000 clips created";
            [5] (clips(1000000)) => time "1,000,000 clips created";
            [6] (self.projects.is_active(PROJECT_35)) => time "Full autonomy attained";
            [7] (clips(1000000000000.0)) => time "One Trillion Clips Created";
            [8] (clips(1000000000000000.0)) => time "One Quadrillion Clips Created";
            [9] (clips(1000000000000000000.0)) => time "One Quintillion Clips Created";
            [10] (clips(1000000000000000000000.0)) => time "One Sextillion Clips Created";
            [11] (clips(1000000000000000000000000.0)) => time "One Septillion Clips Created";
            [12] (clips(1000000000000000000000000000.0)) => time "One Octillion Clips Created";
            [13] (self.space.space_flag) => time "Terrestrial resources fully utilized";
            [14] (
                (self.business.clips >= TOTAL_MATTER)
                || (self.space.found_matter >= TOTAL_MATTER && self.space.available_matter < 1.0 && self.wire.count < 1.0)
            ) => time "Universal Paperclips achieved";
        }
    }

    pub fn reset(&mut self) {
        let prestige_u = self.business.prestige_u;
        let prestige_s = self.computational.prestige_s;

        *self = Default::default();

        self.business.prestige_u = prestige_u;
        self.computational.prestige_s = prestige_s;
    }

    pub fn reset_prestige(&mut self) {
        self.business.prestige_u = 0.0;
        self.computational.prestige_s = 0.0;
    }
}
