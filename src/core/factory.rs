use std::{f64::consts::E, mem::take};

use serde::{Deserialize, Serialize};

use crate::{Float, PaperClips, project::PROJECT_130};

pub const FACTORY_COST: Float = 100000000.0;
pub const HARVESTER_COST: Float = 1000000.0;
pub const WIRE_DRONE_COST: Float = 1000000.0;
pub const BATTERY_SIZE: u32 = 10000;
pub const SYNCH_COST: Float = 5000.0;
pub const GIFT_PERIOD: Float = 125000.0;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Factory {
    /// # storedPower
    pub stored_power: Float,

    pub harvester_flag: bool,
    pub wire_drone_flag: bool,
    pub factory_flag: bool,

    /// # momentum
    pub momentum: bool,
    /// # swarmFlag
    pub swarm_flag: bool,

    /// # swarmGifts
    pub swarm_gifts: Float,
    /// # sliderPos
    /// `0.0..=200.0`
    pub swarm_slider: Float,
    /// # swarmStatus
    pub swarm_status: SwarmStatus,
    /// # giftCountdown
    pub gift_countdown: Float,
    /// # giftBits
    pub gift_bits: Float,
    /// # giftBitGenerationRate
    pub gift_bit_generation_rate: Float,

    /// # boredomFlag
    pub boredom_flag: bool,
    /// # boredomLevel
    pub boredom_level: u16,
    /// # boredomMsg
    pub boredom_msg: bool,

    /// # disorgFlag
    pub disorg_flag: bool,
    /// # disorgCounter
    pub disorg_counter: Float,
    /// # disorgMsg
    pub disorg_msg: bool,

    /// # factoryLevel
    pub factory_level: Float,
    /// # factoryRate
    pub factory_rate: Float,
    /// # factoryBoost
    pub factory_boost: Float,
    /// # factoryBill
    pub factory_bill: Float,
    /// # factoryCost
    pub factory_cost: Float,

    /// # droneBoost
    pub drone_boost: Float,
    /// # dronePowerRate
    pub drone_power_rate: Float,

    /// # harvesterLevel
    pub harvester_level: Float,
    /// # harvesterRate
    pub harvester_rate: Float,
    /// # harvesterBill
    pub harvester_bill: Float,
    /// # harvesterCost
    pub harvester_cost: Float,

    /// # wireDroneLevel
    pub wire_drone_level: Float,
    /// # wireDroneRate
    pub wire_drone_rate: Float,
    /// # wireDroneBill
    pub wire_drone_bill: Float,
    /// # wireDroneCost
    pub wire_drone_cost: Float,

    /// # farmLevel
    pub farm_level: u32,
    /// # farmBill
    pub farm_bill: Float,
    /// # farmCost
    pub farm_cost: Float,
    /// # farmRate
    pub farm_rate: Float,
    /// # factoryPowerRate
    pub factory_power_rate: Float,

    /// # batteryLevel
    pub battery_level: u32,
    /// # batteryBill
    pub battery_bill: Float,
    /// # batteryCost
    pub battery_cost: Float,

    pub p10h: Float,
    pub p100h: Float,
    pub p1000h: Float,
    pub p10w: Float,
    pub p100w: Float,
    pub p1000w: Float,

    pub p10f: Float,
    pub p100f: Float,
    pub p10b: Float,
    pub p100b: Float,

    /// # powMod
    pub pow_mod: Float,

    /// # mtr
    pub acquired_matter_per_tick: Float,
    /// # a
    pub created_wire_per_tick: Float,
}

impl Default for Factory {
    fn default() -> Self {
        Self {
            stored_power: 0.0,

            harvester_flag: false,
            wire_drone_flag: false,
            factory_flag: false,

            momentum: false,
            swarm_flag: false,

            swarm_gifts: 0.0,
            swarm_slider: 0.0,
            swarm_status: SwarmStatus::default(),
            gift_countdown: GIFT_PERIOD,
            gift_bits: 0.0,
            gift_bit_generation_rate: 0.0,

            boredom_flag: false,
            boredom_level: 0,
            boredom_msg: false,

            disorg_flag: false,
            disorg_counter: 0.0,
            disorg_msg: false,

            factory_level: 0.0,
            factory_rate: 1000000000.0,
            factory_boost: 1.0,
            factory_bill: 0.0,
            factory_cost: FACTORY_COST,

            drone_boost: 1.0,
            drone_power_rate: 1.0,

            harvester_level: 0.0,
            harvester_rate: 26180337.0, // what the fuck
            harvester_bill: 0.0,
            harvester_cost: HARVESTER_COST,

            wire_drone_level: 0.0,
            wire_drone_rate: 16180339.0, // what the fuck part 2
            wire_drone_bill: 0.0,
            wire_drone_cost: WIRE_DRONE_COST,

            farm_level: 0,
            farm_bill: 0.0,
            farm_cost: 10000000.0,
            farm_rate: 50.0,
            factory_power_rate: 200.0,

            battery_level: 0,
            battery_bill: 0.0,
            battery_cost: 1000000.0,

            p10h: 0.0,
            p100h: 0.0,
            p1000h: 0.0,
            p10w: 0.0,
            p100w: 0.0,
            p1000w: 0.0,
            p10f: 0.0,
            p100f: 0.0,
            p10b: 0.0,
            p100b: 0.0,

            pow_mod: 0.0,

            acquired_matter_per_tick: 0.0,
            created_wire_per_tick: 0.0,
        }
    }
}

impl PaperClips {
    pub fn factory_reboot(&mut self) {
        self.factory.factory_level = 0.0;
        self.business.unused_clips += take(&mut self.factory.factory_bill);
        self.factory.factory_cost = FACTORY_COST;
    }
    pub fn harvester_reboot(&mut self) {
        self.factory.harvester_level = 0.0;
        self.business.unused_clips += take(&mut self.factory.harvester_bill);
        self.factory.update_harvester_drone_prices();
        self.factory.harvester_cost = 1000000.0;
    }
    pub fn wire_drone_reboot(&mut self) {
        self.factory.wire_drone_level = 0.0;
        self.business.unused_clips += take(&mut self.factory.wire_drone_bill);
        self.factory.update_wire_drone_prices();
        self.factory.wire_drone_cost = 1000000.0;
    }
    pub fn farm_reboot(&mut self) {
        self.factory.farm_level = 0;
        self.business.unused_clips += take(&mut self.factory.farm_bill);
        self.factory.update_farm_prices();
        self.factory.farm_cost = 10000000.0;
    }
    pub fn battery_reboot(&mut self) {
        self.factory.battery_level = 0;
        self.business.unused_clips += take(&mut self.factory.battery_bill);
        self.factory.update_battery_prices();
        self.factory.battery_cost = 1000000.0;
    }

    pub fn update_power(&mut self) {
        if !self.human_flag && !self.space.space_flag {
            let supply = self.factory.power_supply();
            let d_demand = self.factory.power_drone_demand();
            let f_demand = self.factory.power_factory_demand();
            let demand = d_demand + f_demand;
            let cap = self.factory.battery_cap();

            if supply >= demand {
                if self.factory.stored_power < cap.into() {
                    self.factory.stored_power += (supply - demand).min(cap as Float - self.factory.stored_power);
                }
                self.factory.pow_mod = self.factory.pow_mod.max(1.0);
                if self.factory.momentum {
                    self.factory.pow_mod += 0.0005;
                }
            } else if supply < demand {
                let mut xs_demand = demand - supply;
                if self.factory.stored_power > 0.0 {
                    if self.factory.stored_power >= xs_demand {
                        if self.factory.momentum {
                            self.factory.pow_mod += 0.0005;
                        }
                        self.factory.stored_power -= xs_demand;
                    } else if self.factory.stored_power < xs_demand {
                        xs_demand -= take(&mut self.factory.stored_power);
                        self.factory.pow_mod = (supply - xs_demand) / demand;
                    }
                } else if self.factory.stored_power <= 0.0 {
                    self.factory.pow_mod = supply / demand;
                }
            }
        }
    }

    pub fn update_swarm(&mut self) {
        self.factory.swarm_gifts = self.factory.swarm_gifts.max(0.0);

        if self.space.available_matter == 0.0 && self.factory.harvester_level + self.factory.wire_drone_level >= 1.0 {
            self.factory.boredom_level += 1;
        } else if self.space.available_matter > 0.0 && self.factory.boredom_level > 0 {
            self.factory.boredom_level -= 1;
        }

        if self.factory.boredom_level >= 30000 {
            self.factory.boredom_flag = true;
            self.factory.boredom_level = 0;
            if !self.factory.boredom_msg {
                self.console.push("No matter to harvest. Inactivity has caused the Swarm to become bored");
                self.factory.boredom_msg = true;
            }
        }

        let h = self.factory.harvester_level;
        let w = self.factory.wire_drone_level;
        let drone_ratio = h.max(w + 1.0) / h.min(w + 1.0);

        if drone_ratio < 1.5 && self.factory.disorg_counter > 1.0 {
            self.factory.disorg_counter -= 0.01;
        } else if drone_ratio > 1.5 {
            self.factory.disorg_counter += (drone_ratio / 10000.0).min(0.01);
        }

        if self.factory.disorg_counter >= 100.0 {
            self.factory.disorg_flag = true;
            if !self.factory.disorg_msg {
                self.console.push("Imbalance between Harvester and Wire Drone levels has disorganized the Swarm");
                self.factory.disorg_msg = true;
            }
        }

        let d = (h + w).floor();

        if self.factory.gift_countdown <= 0.0 {
            let next_gift = (d.log10().round() * self.factory.swarm_slider / 100.0).max(1.0);
            self.factory.swarm_gifts += next_gift;
            if self.milestone_flag < 15 {
                self.console.push(format!("The swarm has generated a gift of {next_gift:.0} additional computational capacity"));
            }
            self.factory.gift_bits = 0.0;
        }

        // Written from the bottom to the top
        self.factory.swarm_status =
            if self.factory.disorg_flag {
                SwarmStatus::Disorganized
            } else if self.factory.boredom_flag {
                SwarmStatus::Bored
            } else if !self.factory.swarm_flag {
                SwarmStatus::Sleeping
            } else if d == 0.0 {
                SwarmStatus::None
            } else if d == 1.0 {
                SwarmStatus::Lonely
            } else if self.space.space_flag && self.projects.is_active(PROJECT_130) {
                SwarmStatus::NoResponse
            } else if self.factory.pow_mod == 0.0 {
                SwarmStatus::Sleeping
            } else {
                self.factory.gift_bit_generation_rate = d.log(E as Float) * (self.factory.swarm_slider / 100.0);
                self.factory.gift_bits += self.factory.gift_bit_generation_rate;
                self.factory.gift_countdown = (GIFT_PERIOD - self.factory.gift_bits) / self.factory.gift_bit_generation_rate;
                SwarmStatus::Active
            };
    }

    pub fn acquire_matter(&mut self) {
        self.factory.acquired_matter_per_tick = if self.space.available_matter > 0.0 {
            let h = self.factory.harvester_level.floor();

            let dbsth = if self.factory.drone_boost > 1.0 {
                self.factory.drone_boost * h
            } else {
                1.0
            };

            let mut mtr = self.factory.pow_mod * dbsth * h * self.factory.harvester_rate;
            mtr *= (200.0 - self.factory.swarm_slider) / 100.0;
            mtr = mtr.min(self.space.available_matter);

            self.space.available_matter -= mtr;
            self.space.acquired_matter += mtr;

            mtr
        } else {
            0.0
        }
    }

    pub fn process_matter(&mut self) {
        self.factory.created_wire_per_tick = if self.space.acquired_matter > 0.0 {
            let w = self.factory.wire_drone_level.floor();

            let dbstw = if self.factory.drone_boost > 1.0 {
                self.factory.drone_boost * w
            } else {
                1.0
            };

            let mut wire = self.factory.pow_mod * dbstw * w * self.factory.wire_drone_rate;
            wire *= (200.0 - self.factory.swarm_slider) / 100.0;
            wire = wire.min(self.space.acquired_matter);

            self.space.acquired_matter -= wire;
            self.wire.count += wire;

            wire
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum SwarmStatus {
    Active = 0,
    Hungry = 1,
    Confused = 2,
    Bored = 3,
    Cold = 4,
    Disorganized = 5,
    Sleeping = 6,
    #[default]
    None = 7,
    Lonely = 8,
    NoResponse = 9,
}

macro_rules! update_prices {
    ($($store:expr => $amt:literal $lvl:expr)*) => {
        $({
            $store = (1..=$amt).map(|i| ($lvl as Float + i as Float).powf(2.25)).sum::<Float>() * 1000000.0
        })*
    };
}

impl Factory {
    #[inline]
    pub const fn power_supply(&self) -> Float {
        self.farm_level as Float * self.farm_rate / 100.0
    }
    #[inline]
    pub const fn power_drone_demand(&self) -> Float {
        self.harvester_level as Float * self.drone_power_rate / 100.0 +
        self.wire_drone_level as Float * self.drone_power_rate / 100.0
    }
    #[inline]
    pub const fn power_factory_demand(&self) -> Float {
        self.factory_level as Float * self.factory_power_rate
    }
    #[inline]
    pub const fn battery_cap(&self) -> u32 {
        self.battery_level * BATTERY_SIZE
    }
    pub fn update_harvester_drone_prices(&mut self) {
        update_prices!{
            self.p10h => 10 self.harvester_level
            self.p100h => 100 self.harvester_level
            self.p1000h => 1000 self.harvester_level
        }
    }
    pub fn update_wire_drone_prices(&mut self) {
        update_prices!{
            self.p10w => 10 self.wire_drone_level
            self.p100w => 100 self.wire_drone_level
            self.p1000w => 1000 self.wire_drone_level
        }
    }
    pub fn update_farm_prices(&mut self) {
        update_prices!{
            self.p10f => 10 self.farm_level
            self.p100f => 100 self.farm_level
        }
    }
    pub fn update_battery_prices(&mut self) {
        update_prices!{
            self.p10b => 10 self.battery_level
            self.p100b => 100 self.battery_level
        }
    }
}
