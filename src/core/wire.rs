use serde::{Deserialize, Serialize};

use crate::{core::{Float, PaperClips}, rng::PCRng};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Wire {
    /// # wirePriceTimer
    pub price_timer: u8,
    /// # wireBasePrice
    pub base_price: Float,
    /// # wirePriceCounter
    pub price_counter: u32,
    /// # wireCost
    pub cost: Float,
    /// # wire
    pub count: Float,
    /// # wireSupply
    pub supply: Float,
    /// # wirePurchase
    pub purchase: u64,
    /// # wireBuyerFlag
    /// If the wire buyer is usable
    pub buyer_flag: bool,
    /// # wireBuyerStatus
    /// If the wire buyers is active
    pub buyer_status: bool,
    /// # wireProductionFlag
    pub production_flag: bool,
}

impl Default for Wire {
    fn default() -> Self {
        Self {
            price_timer: 0,
            base_price: 20.0,
            price_counter: 0,
            cost: 20.0,
            count: 1000.0,
            supply: 1000.0,
            purchase: 0,
            buyer_flag: false,
            buyer_status: true,
            production_flag: false,
        }
    }
}

impl Wire {
    pub fn adjust_wire_price(&mut self, rng: &mut PCRng) {
        self.price_timer = self.price_timer.saturating_add(1);

        if self.price_timer >= 250 && self.base_price > 15.0 {
            self.base_price *= 0.999; 
            self.price_timer = 0;
        }

        let wire_adjust = 6.0 * (self.price_counter as Float).sin();
        let new_cost = self.base_price + wire_adjust;

        let best_rng = rng.is_best() && new_cost < self.cost;
        let worst_rng = rng.is_worst() && new_cost > self.cost;
        
        if best_rng || worst_rng || rng.random_bool(0.015, false) {
            self.price_counter += 1;
            self.cost = new_cost;
        }
    }

    // toggle_wire_buyer()
}

impl PaperClips {
    pub fn buy_wire(&mut self) {
        if self.business.funds >= self.wire.cost {
            self.wire.price_timer = 0;
            self.wire.count += self.wire.supply;
            self.business.funds -= self.wire.cost;
            self.wire.purchase += 1;
            self.wire.base_price += 0.05;
            // update wire
            // update funds
        }
    }
}
