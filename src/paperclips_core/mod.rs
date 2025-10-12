use std::time::Instant;

use rand::random_bool;

use crate::{business::Business, paperclips_core::{computational::Computational, investments::Investments, messages::Messages, qchips::QChips, wire::Wire}, strategy::Strategy};

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
    session_start: Instant,
    ticks: u128,

    messages: Messages,

    business: Business,
    wire: Wire,
    computational: Computational, 
    qchips: QChips,
    investments: Investments,
    strategy: Strategy,
}

impl Default for PaperClips {
    fn default() -> Self {
        Self { 
            session_start: Instant::now(),
            ticks: 0,

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

        // if human_flag {
        //     calculate_trust();
        // }

        if self.qchips.q_flag {
            self.quantum_compute();
        }

        // update_stats();
        // manage_projects();
        // milestone_check();

        // // Clip Rate Tracker
        // clip_rate_tracker += 1;
        // if clip_rate_tracker < 100 {
        //     let cr = clips - prev_clips;
        //     clip_rate_temp += cr;
        //     prev_clips = clips;
        // } else {
        //     clip_rate_tracker = 0;
        //     clip_rate = clip_rate_temp;
        //     clip_rate_temp = 0;
        // }

        // // Stock Report
        // stock_report_counter += 1;
        // if (investment_engine_flag == 1 && stock_report_counter >= 10000) {
        //     let r = (ledger + portTotal).toLocaleString();
        //     displayMessage("Lifetime investment revenue report: $" + r);
        //     stock_report_counter = 0;
        // }

        // // WireBuyer
        // if wire_buyer_flag && wire_buyer_status && self.wire.count <= 1.0 {
        //     self.buy_wire();
        // }

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

        // // Demand Curve 
        // if human_flag {
            // put everything into this function
            self.business.update_demand();
        // }

        // // // Creativity
        // if (creativityOn && operations >= (memory * 1000)) {
        //     calculateCreativity();
        // }

        // Ending

        // lots of code
    }

    /// Should run once every 1000ms
    pub fn update_stock_shop_tick(&mut self) {
        // if human_flag {
            self.stock_shop();
        // }
    }

    /// Should run once every 2500ms
    pub fn update_stocks_tick(&mut self) {
        // sell_delay += 1;

        // if portfolio_size > 0 && sell_delay >= 5 && random_bool(0.3) && human_flag {
            self.sell_stock();
        //     sell_delay = 0;
        // }

        // if portfolio_size > 0 && human_flag {
            self.update_stocks();
        // }

    }

    /// Should run once every 100ms
    pub fn update_wire_price_and_demand_tick(&mut self) {
        // Wire Price Fluctuation
        self.wire.adjust_wire_price();

        // if humanflag {
            // Sales Calculator
            if random_bool(self.business.demand as f64 / 100.0) {
                self.sell_clips((0.7 * self.business.demand.powf(1.15)).floor());
            }

            // Fire Once a Second
        // }


    }
}
