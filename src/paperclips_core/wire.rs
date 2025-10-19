use std::time::{Duration, Instant};

use rand::random_bool;

use crate::paperclips_core::{Float, PaperClips};

const PRICE_TIMER: Duration = Duration::from_secs(25);

pub struct Wire {
    /// # wirePriceTimer
    pub price_timer: Instant,
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
    pub buyer_flag: bool,
}

impl Default for Wire {
    fn default() -> Self {
        Self {
            price_timer: Instant::now(),
            base_price: 20.0,
            price_counter: 0,
            cost: 20.0,
            count: 1000.0,
            supply: 1000.0,
            purchase: 0,
            buyer_flag: false,
        }
    }
}

impl Wire {
    pub fn adjust_wire_price(&mut self) {
        if self.price_timer.elapsed() >= PRICE_TIMER && self.base_price > 15.0 {
            self.base_price *= 0.999; 
            self.price_timer = Instant::now();
        }

        if random_bool(0.015) {
            self.price_counter += 1;
            let wire_adjust = 6.0 * (self.price_counter as Float).sin();
            self.cost = self.base_price + wire_adjust;
            // update wire cost
        }
    }

    // toggle_wire_buyer()
}

impl PaperClips {
    pub fn buy_wire(&mut self) {
        if self.business.funds >= self.wire.cost {
            self.wire.price_timer = Instant::now();
            self.wire.count += self.wire.supply;
            self.business.funds -= self.wire.cost;
            self.wire.purchase += 1;
            self.wire.base_price += 0.05;
            // update wire
            // update funds
        }
    }
}
