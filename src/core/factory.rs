use std::mem::take;

use serde::{Deserialize, Serialize};

use crate::{Float, PaperClips};

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

    /// # factoryLevel
    pub factory_level: u32,
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

    /// # harvesterLevel
    pub harvester_level: u32,
    /// # harvesterRate
    pub harvester_rate: Float,
    /// # harvesterBill
    pub harvester_bill: Float,
    /// # harvesterCost
    pub harvester_cost: Float,

    /// # wireDroneLevel
    pub wire_drone_level: u32,
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

            factory_level: 0,
            factory_rate: 1000000000.0,
            factory_boost: 1.0,
            factory_bill: 0.0,
            factory_cost: 100000000.0,

            drone_boost: 1.0,

            harvester_level: 0,
            harvester_rate: 26180337.0, // what the fuck
            harvester_bill: 0.0,
            harvester_cost: 1000000.0,

            wire_drone_level: 0,
            wire_drone_rate: 16180339.0, // what the fuck part 2
            wire_drone_bill: 0.0,
            wire_drone_cost: 1000000.0,

            farm_level: 0,
            farm_bill: 0.0,
            farm_cost: 10000000.0,

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
        }
    }
}

impl PaperClips {
    pub fn factory_reboot(&mut self) {
        self.factory.factory_level = 0;
        self.business.unused_clips += take(&mut self.factory.factory_bill);
        self.factory.factory_cost = 100000000.0;
    }
    pub fn harvester_reboot(&mut self) {
        self.factory.harvester_level = 0;
        self.business.unused_clips += take(&mut self.factory.harvester_bill);
        self.factory.update_drone_prices();
        self.factory.harvester_cost = 1000000.0;
    }
    pub fn wire_drone_reboot(&mut self) {
        self.factory.wire_drone_level = 0;
        self.business.unused_clips += take(&mut self.factory.wire_drone_bill);
        self.factory.update_drone_prices();
        self.factory.wire_drone_cost = 1000000.0;
    }
    pub fn farm_reboot(&mut self) {
        self.factory.farm_level = 0;
        self.business.unused_clips += take(&mut self.factory.farm_bill);
        self.factory.update_pow_prices();
        self.factory.farm_cost = 10000000.0;
    }
    pub fn battery_reboot(&mut self) {
        self.factory.battery_level = 0;
        self.business.unused_clips += take(&mut self.factory.battery_bill);
        self.factory.update_pow_prices();
        self.factory.battery_cost = 1000000.0;
    }
}

macro_rules! update_prices {
    ($($store:expr => $amt:literal $lvl:expr)*) => {
        $({
            let start_lvl = $lvl + 1;
            $store = (start_lvl..start_lvl + $amt).map(|i| (i as Float).powf(2.25)).sum::<Float>() * 1000000.0;
        })*
    };
}

impl Factory {
    pub fn update_drone_prices(&mut self) {
        update_prices!{
            self.p10h => 10 self.harvester_level
            self.p100h => 100 self.harvester_level
            self.p1000h => 1000 self.harvester_level
            self.p10w => 10 self.wire_drone_level
            self.p100w => 100 self.wire_drone_level
            self.p1000w => 1000 self.wire_drone_level
        }
    }
    pub fn update_pow_prices(&mut self) {
        update_prices!{
            self.p10f => 10 self.farm_level
            self.p100f => 100 self.farm_level
            self.p10b => 10 self.battery_level
            self.p100b => 100 self.battery_level
        }
    }
}
