use crate::{util::{floor_to, round_to}, Float, PaperClips};

pub struct Business {
    /// # funds
    pub funds: Float,
    /// # unsoldClips
    pub unsold_clips: Float,
    /// # margin
    /// Price per Clip
    pub margin: Float,
    /// # demand
    pub demand: Float,
    /// # marketingLvl
    pub marketing_lvl: u8,
    /// # marketingEffectiveness
    pub marketing_effectiveness: Float,
    /// # demandBoost
    pub demand_boost: Float,
    /// # prestigeU
    pub prestige_u: Float,
}

impl Default for Business {
    fn default() -> Self {
        Self {
            funds: 0.0,
            unsold_clips: 0.0,
            margin: 0.25,
            demand: 5.0,
            marketing_lvl: 1,
            marketing_effectiveness: 1.0,
            demand_boost: 1.0,
            prestige_u: 0.0,
        }
    }
}

impl PaperClips {
    pub fn sell_clips(&mut self, amount: Float) {
        let amount = self.business.unsold_clips.min(amount);

        let transaction = floor_to(amount * self.business.margin, 3);
        self.business.funds = floor_to(self.business.funds + transaction, 2);
        // income += transaction;
        // clipsSold += amount; // UNUSED
        self.business.unsold_clips -= amount;
    }
}

impl Business {
    pub fn raise_price(&mut self) {
        self.margin += round_to(self.margin, -2);

    }
    pub fn lower_price(&mut self) {
        self.margin = round_to((self.margin - 0.01).max(0.01), -2);
    }

    pub fn update_demand(&mut self) {
        let marketing = (1.1 as Float).powf((self.marketing_lvl - 1) as Float);
        self.demand = 0.8 / self.margin * marketing * self.marketing_effectiveness * self.demand_boost;
        self.demand += self.demand / 10.0 * self.prestige_u;
    }
}
