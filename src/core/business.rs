use std::collections::VecDeque;

use crate::{util::{floor_to, round_to}, Float, PaperClips};

#[derive(Debug, Clone)]
pub struct Business {
    /// # funds
    pub funds: Float,
    /// # clips
    /// Total amount of clips produced
    pub clips: Float,
    /// # unsoldClips
    pub unsold_clips: Float,
    /// # margin
    /// Price per Clip
    pub margin: Float,
    /// # demand
    pub demand: Float,
    /// # adCost
    pub ad_cost: Float,
    /// # marketingLvl
    pub marketing_lvl: u8,
    /// # marketingEffectiveness
    pub marketing_effectiveness: Float,
    /// # demandBoost
    pub demand_boost: Float,
    /// # prestigeU
    pub prestige_u: Float,
    
    /// # revPerSecFlag
    pub rev_per_sec_flag: bool,
    /// # avgSales
    pub avg_sales: Float,
    /// # avgRev
    pub avg_rev: Float,
    /// # income
    pub income: Float,
    /// # incomeTracker
    pub income_tracker: VecDeque<Float>,
    /// # incomeThen
    pub income_then: Float,
    /// # incomeNow
    pub income_now: Float,

    // # clipRateTracker
    // Used as a "run this code once every 100 cycles"
    // Removed for the reason above
    // pub clip_rate_tracker: u8,
    
    // Autoclippers
    /// # autoClipperFlag
    pub clipper_flag: bool,
    /// # clipmakerLevel
    pub clipper_level: Float,
    /// # clipperBoost
    pub clipper_boost: Float,
    /// # clipperCost
    pub clipper_cost: Float,
    /// # megaClipperFlag
    pub mega_clipper_flag: bool,
    /// # megaClipperLevel
    pub mega_clipper_level: Float,
    /// # megaClipperBoost
    pub mega_clipper_boost: Float,
    /// # megaClipperCost
    pub mega_clipper_cost: Float,

    /// # prevClips
    pub prev_clips: Float,
    /// # clipRateTemp
    pub clip_rate_temp: Float,
    /// # clipRate
    pub clip_rate: Float,

    /// # unusedClips
    pub unused_clips: Float,
}

impl Default for Business {
    fn default() -> Self {
        Self {
            funds: 0.0,
            clips: 0.0,
            unsold_clips: 0.0,
            margin: 0.25,
            demand: 5.0,
            ad_cost: 100.0,
            marketing_lvl: 1,
            marketing_effectiveness: 1.0,
            demand_boost: 1.0,
            prestige_u: 0.0,

            rev_per_sec_flag: false,
            avg_sales: 0.0,
            avg_rev: 0.0,
            income: 0.0,
            income_tracker: VecDeque::with_capacity(10),
            income_then: 0.0,
            income_now: 0.0,

            clipper_flag: false,
            clipper_level: 0.0,
            clipper_boost: 1.0,
            clipper_cost: 5.0,
            mega_clipper_flag: false,
            mega_clipper_level: 0.0,
            mega_clipper_boost: 1.0,
            mega_clipper_cost: 500.0,

            prev_clips: 0.0,
            clip_rate_temp: 0.0,
            clip_rate: 0.0,

            unused_clips: 0.0,
        }
    }
}

impl PaperClips {
    pub fn sell_clips(&mut self, amount: Float) {
        let amount = self.business.unsold_clips.min(amount);

        let transaction = floor_to(amount * self.business.margin, -3);
        self.business.funds = floor_to(self.business.funds + transaction, -2);
        self.business.income += transaction;
        // clipsSold += amount; // UNUSED
        self.business.unsold_clips -= amount;
    }
    /// Adds clips to `business.clips` and `business.unsold_clips`. Does not subtract wire.
    pub fn create_clips(&mut self, amount: Float) {
        self.business.clips += amount;
        self.business.unsold_clips += amount;
        self.business.unused_clips += amount;
    }
    pub fn clip_click(&mut self, amount: Float) {
        // if dismantle >= 4 {
        //     final_clips += 1;
        // }

        if self.wire.count >= 1.0 {
            let amount = amount.min(self.wire.count);

            self.create_clips(amount);
            self.wire.count -= amount;
        }
    }
}

impl Business {
    pub fn buy_ads(&mut self) {
        if self.funds >= self.ad_cost {
            self.marketing_lvl += 1;
            self.funds -= self.ad_cost;
            self.ad_cost *= 2.0;
        }
    }
    #[inline]
    pub fn raise_price(&mut self) {
        self.margin += 0.01;
        self.standardize_margin();

    }
    #[inline]
    pub fn lower_price(&mut self) {
        self.margin -= 0.01;
        self.standardize_margin();
    }
    #[inline]
    pub fn standardize_margin(&mut self) {
        self.margin = round_to(self.margin.max(0.01), -2);
    }

    pub fn update_demand(&mut self) {
        let marketing = (1.1 as Float).powf((self.marketing_lvl - 1) as Float);
        self.demand = 0.8 / self.margin * marketing * self.marketing_effectiveness * self.demand_boost;
        self.demand += self.demand / 10.0 * self.prestige_u;
    }
    pub fn make_clipper(&mut self) {
        if self.funds >= self.clipper_cost {
            self.funds -= self.clipper_cost;
            self.clipper_level += 1.0;
        }
        self.clipper_cost = (1.1 as Float).powf(self.clipper_level) + 5.0;
    }
    pub fn make_mega_clipper(&mut self) {
        if self.funds >= self.mega_clipper_cost {
            self.funds -= self.mega_clipper_cost;
            self.mega_clipper_level += 1.0;
        }
        self.mega_clipper_cost = (1.07 as Float).powf(self.mega_clipper_level) * 1000.0;
    }
    #[inline]
    pub fn scaled_demand(&self) -> Float {
        0.7 * self.demand.powf(1.15)
    }
    pub fn calculate_rev(&mut self) {
        self.income_then = self.income_now;
        self.income_now = self.income;

        let income_last_second = round_to(self.income_now - self.income_then, -2);

        self.income_tracker.push_back(income_last_second);
        if self.income_tracker.len() > 10 {
            self.income_tracker.pop_front();
        }

        let sum = round_to(self.income_tracker.iter().sum(), -2);

        
        let chance_of_purchase = match self.unsold_clips {
            1.0.. => (self.demand / 100.0).min(1.0),
            _ => 0.0,
        };
        
        let scaled_demand = self.scaled_demand();
        
        if self.demand > self.unsold_clips {
            let true_avg_rev = sum / self.income_tracker.len() as Float;
            self.avg_rev = true_avg_rev;
            self.avg_sales = self.avg_rev / self.margin;
        } else {
            self.avg_sales = chance_of_purchase * scaled_demand * 10.0;
            self.avg_rev = chance_of_purchase * scaled_demand * self.margin * 10.0;
        }
    }
}
