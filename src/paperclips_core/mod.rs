use std::time::{Duration, Instant};

use rand::random_bool;

use crate::{business::Business, paperclips_core::{computational::Computational, investments::Investments, messages::Messages, qchips::QChips, wire::Wire}, strategy::Strategy, util::ticks_10ms};

// Can easily get changed with f128 in the future
pub type Float = f32;

pub mod business;
pub mod messages;
pub mod wire;
pub mod computational;
pub mod qchips;
pub mod projects;
pub mod strategy;
pub mod util;
pub mod investments;

pub struct PaperClips {
    pub session_start: Instant,
    pub ticks: u128,

    pub human_flag: bool,

    pub messages: Messages,

    pub business: Business,
    pub wire: Wire,
    pub computational: Computational, 
    pub qchips: QChips,
    pub investments: Investments,
    pub strategy: Strategy,
}

impl Default for PaperClips {
    fn default() -> Self {
        Self { 
            session_start: Instant::now(),
            ticks: 0,

            human_flag: true,

            business: Business::default(),
            wire: Wire::default(),
            qchips: QChips::default(),
            computational: Computational::default(),
            messages: Messages::default(),
            investments: Investments::default(),
            strategy: Strategy::default(),
        }
    }
}

impl PaperClips {
    /// Should run once every 10ms
    pub fn main_loop_tick(&mut self) {
        self.ticks += 1;
        
        // milestone_check();
        // button_update();

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
        // milestone_check();

        // Clip Rate Tracker
        let Business { prev_clips, clip_rate_temp, clip_rate, clips, .. } = &mut self.business;
        if self.ticks % ticks_10ms(Duration::from_secs(1)) == 0 {
            *clip_rate = *clip_rate_temp;
            *clip_rate_temp = 0.0;
        } else {
            let cr = *clips - *prev_clips;
            *clip_rate_temp += cr;
            *prev_clips = *clips;
        }

        // Stock Report
        if self.investments.engine_flag && self.ticks % ticks_10ms(Duration::from_secs(100)) == 0 {
            let r = self.investments.ledger + self.investments.port_total;
            self.messages.push(format!("Lifetime investment revenue report: ${r}"));
        }

        // WireBuyer
        if self.wire.buyer_flag && self.wire.count <= 1.0 {
            self.buy_wire();
        }

        // // First, Explore
        // explore_universe();
        // // Then, Drones
        // if (humanFlag == 0 && spaceFlag == 0) {
        //     update_drone_buttons();
        // }
        // update_power();
        // update_swarm();
        // acquire_matter();
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
        //     clip_click(clipperBoost * (clipmakerLevel / 100));
        //     clip_click(megaClipperBoost * (megaClipperLevel * 5));
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

        // Ending

        // lots of code
    }

    /// Should run once every 1000ms
    pub fn update_stock_shop_tick(&mut self) {
        if self.human_flag {
            self.stock_shop();
        }
    }

    /// Should run once every 2500ms
    pub fn update_stocks_tick(&mut self) {
        self.investments.sell_delay += 1;
        if self.human_flag && !self.investments.stocks.is_empty() {
            if self.investments.sell_delay >= 5 && random_bool(0.3) {
                self.sell_stock();
                self.investments.sell_delay = 0;
            }
            self.update_stocks();
        }
    }

    /// Should run once every 100ms
    pub fn update_wire_price_and_demand_tick(&mut self) {
        // Wire Price Fluctuation
        self.wire.adjust_wire_price();

        if self.human_flag {
            // Sales Calculator
            if random_bool(self.business.demand as f64 / 100.0) {
                self.sell_clips((0.7 * self.business.demand.powf(1.15)).floor());
            }
            // Fire Once a Second
        }
    }
}
