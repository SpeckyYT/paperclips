use std::time::Instant;

use arrayvec::ArrayString;

use crate::{Float, PaperClips, factory::{FACTORY_COST, HARVESTER_COST, WIRE_DRONE_COST}, project::{PROJECT_129, PROJECT_148}, util::powf};

pub const TOTAL_MATTER: Float = powf(10.0, 54);
pub const STARTING_AVAILABLE_MATTER: Float = powf(10.0, 24) * 6000.0;

pub const DEFAULT_BATTLENAME: &str = "Durenstein 1";
pub const THRENODY_START: &str = "Threnody for the Heroes of ";
pub const MAX_BATTLENAME_LEN: usize = 24+1 + 20;
pub const MAX_THRENODY_LEN: usize = THRENODY_START.len() + MAX_BATTLENAME_LEN;

pub const PROBE_COST: Float = powf(10.0, 17);

/// # probeXBaseRate
pub const PROBE_X_BASE_RATE: Float = 1750000000000000000.0;

/// # probeHazBaseRate
pub const PROBE_HAZ_BASE_RATE: Float = 0.01;
/// # probeFacBaseRate
pub const PROBE_FAC_BASE_RATE: Float = 0.000001;
/// # probeHarvBaseRate
pub const PROBE_HARV_BASE_RATE: Float = 0.000002;
/// # probeWireBaseRate
pub const PROBE_WIRE_BASE_RATE: Float = 0.000002;
/// # probeDriftBaseRate
pub const PROBE_DRIFT_BASE_RATE: Float = 0.000001;
/// # probeRepBaseRate
pub const PROBE_REP_BASE_RATE: Float = 0.00005;

#[derive(Debug, Clone, Copy)]
pub struct Space {
    /// # spaceFlag
    pub space_flag: bool,

    pub hypno_drone_event: Option<Instant>,

    /// # honor
    pub honor: Float,

    /// # threnodyTitle
    pub threnody_title: ArrayString<MAX_BATTLENAME_LEN>,
    pub threnody_project: ArrayString<MAX_BATTLENAME_LEN>,

    /// # boredomLevel
    pub boredom_level: Float,

    /// # availableMatter
    pub available_matter: Float,
    /// # foundMatter
    pub found_matter: Float,
    /// # acquiredMatter
    pub acquired_matter: Float,
    /// # processedMatter
    pub processed_matter: Float,

    /// # probeCount
    pub probe_count: Float,

    // TODO: change to u8 or something
    /// # probeTrust
    pub probe_trust: Float,

    /// # probeSpeed
    pub probe_speed: Float,
    /// # probeNav
    /// "Exploration"
    pub probe_nav: Float,
    /// # probeRep
    pub probe_rep: Float,
    /// # probeHaz
    pub probe_haz: Float,
    /// # probeFac
    pub probe_fac: Float,
    /// # probeHarv
    pub probe_harv: Float,
    /// # probeWire
    pub probe_wire: Float,

    /// # partialProbeHaz
    pub partial_probe_haz: Float,
    /// # partialProbeSpawn
    pub partial_probe_spawn: Float,

    /// # probesLostHaz
    pub probes_lost_haz: Float,
    /// # probesLostDrift
    pub probes_lost_drift: Float,

    /// # drifterCount
    pub drifter_count: Float,
    /// # probeDescendents
    pub probe_descendents: Float,
}

impl Default for Space {
    fn default() -> Self {
        Self {
            space_flag: false,

            hypno_drone_event: None,

            honor: 0.0,

            threnody_title: ArrayString::from(DEFAULT_BATTLENAME).expect("Always valid"),
            threnody_project: ArrayString::from(DEFAULT_BATTLENAME).expect("Always valid"),

            boredom_level: 0.0,

            available_matter: STARTING_AVAILABLE_MATTER,
            found_matter: STARTING_AVAILABLE_MATTER,
            acquired_matter: 0.0,
            processed_matter: 0.0,

            probe_count: 0.0,

            probe_trust: 0.0,

            probe_speed: 0.0,
            probe_nav: 0.0,
            probe_rep: 0.0,
            probe_haz: 0.0,
            probe_fac: 0.0,
            probe_harv: 0.0,
            probe_wire: 0.0,

            partial_probe_haz: 0.0,
            partial_probe_spawn: 0.0,

            probes_lost_haz: 0.0,
            probes_lost_drift: 0.0,

            drifter_count: 0.0,
            probe_descendents: 0.0,
        }
    }
}

impl Space {
    pub fn explore_universe(&mut self) {
        let x_rate = self.probe_count.floor() * PROBE_X_BASE_RATE * self.probe_speed * self.probe_nav;
        let x_rate = x_rate.min(TOTAL_MATTER - self.found_matter);
        self.found_matter += x_rate;
        self.available_matter += x_rate;
    }
}

impl PaperClips {
    pub fn encounter_hazards(&mut self) {
        let boost = self.space.probe_haz.powf(1.6);
        let amount = self.space.probe_count * PROBE_HAZ_BASE_RATE / (3.0 * boost + 1.0);
        let amount = amount * if self.projects.is_active(PROJECT_129) { 0.5 } else { 1.0 };

        if amount < 1.0 {
            self.space.partial_probe_haz += amount;
            if self.space.partial_probe_haz >= 1.0 {
                let amount = 1.0 as Float;
                self.space.partial_probe_haz = 0.0;
                self.space.probe_count -= amount.min(self.space.probe_count);
                self.space.probes_lost_haz += amount
            }
        } else {
            let amount = amount.min(self.space.probe_count);
            self.space.probe_count -= amount.min(self.space.probe_count);
            self.space.probes_lost_haz += amount;
        }
    }
    pub fn spawn_factories(&mut self) {
        let amount = self.space.probe_count * PROBE_FAC_BASE_RATE * self.space.probe_fac;
        let amount = amount.min(self.business.unused_clips / FACTORY_COST).floor();
        self.business.unused_clips -= amount * FACTORY_COST;
        self.factory.factory_level += amount;
    }
    pub fn spawn_harvesters(&mut self) {
        const SPAWN_HARVESTER_COST: Float = 2.0 * HARVESTER_COST;
        let amount = self.space.probe_count * PROBE_HARV_BASE_RATE * self.space.probe_harv;
        let amount = amount.min(self.business.unused_clips / SPAWN_HARVESTER_COST).floor();
        self.business.unused_clips -= amount * SPAWN_HARVESTER_COST;
        self.factory.harvester_level += amount;
    }
    pub fn spawn_wire_drones(&mut self) {
        const SPAWN_WIRE_DRONE_COST: Float = 2.0 * WIRE_DRONE_COST;
        let amount = self.space.probe_count * PROBE_WIRE_BASE_RATE * self.space.probe_wire;
        let amount = amount.min(self.business.unused_clips / SPAWN_WIRE_DRONE_COST).floor();
        self.business.unused_clips -= amount * SPAWN_WIRE_DRONE_COST;
        self.factory.wire_drone_level += amount;
    }
    pub fn drift(&mut self) {
        let amount = if self.projects.is_active(PROJECT_148) {
            0.0
        } else {
            (self.space.probe_count * PROBE_DRIFT_BASE_RATE * self.space.probe_trust.powf(1.2))
                .min(self.space.probe_count)
        };
        self.space.probe_count -= amount;
        self.space.drifter_count += amount;
        self.space.probes_lost_drift += amount;
    }
    pub fn spawn_probes(&mut self) {
        let mut next_gen = if self.space.probe_count >= 999999999999999999999999999999999999999999999999.0 {
            0.0
        } else {
            self.space.probe_count * PROBE_REP_BASE_RATE * self.space.probe_rep
        };

        // Partial Spawn = early slow growth
        if next_gen > 0.0 && next_gen < 1.0 {
            self.space.partial_probe_spawn += next_gen;
            if self.space.partial_probe_spawn >= 1.0 {
                next_gen = 1.0;
                self.space.partial_probe_spawn = 0.0;
            }
        }

        // Probes Cost Clips
        let next_gen = next_gen.min((self.business.unused_clips / PROBE_COST).floor());

        self.business.unused_clips -= next_gen * PROBE_COST;

        self.space.probe_descendents += next_gen;
        self.space.probe_count += next_gen;
    }
}
