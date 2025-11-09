use std::time::Instant;

use crate::{util::powf, Float};

pub const TOTAL_MATTER: Float = powf(10.0, 54);
pub const STARTING_AVAILABLE_MATTER: Float = powf(10.0, 24) * 6000.0;

#[derive(Debug, Clone, Copy)]
pub struct Space {
    /// # spaceFlag
    pub space_flag: bool,

    pub hypno_drone_event: Option<Instant>,

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
}

impl Default for Space {
    fn default() -> Self {
        Self {
            space_flag: false,

            hypno_drone_event: None,

            boredom_level: 0.0,

            available_matter: STARTING_AVAILABLE_MATTER,
            found_matter: STARTING_AVAILABLE_MATTER,
            acquired_matter: 0.0,
            processed_matter: 0.0,
        }
    }
}
