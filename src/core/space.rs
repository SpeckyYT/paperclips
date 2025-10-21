use crate::{util::powf, Float};

pub const TOTAL_MATTER: Float = powf(10.0, 54);
pub const STARTING_AVAILABLE_MATTER: Float = powf(10.0, 24) * 6000.0;

pub struct Space {
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
            available_matter: STARTING_AVAILABLE_MATTER,
            found_matter: STARTING_AVAILABLE_MATTER,
            acquired_matter: 0.0,
            processed_matter: 0.0,
        }
    }
}
