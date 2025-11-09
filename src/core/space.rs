use std::time::Instant;

use arrayvec::ArrayString;

use crate::{util::powf, Float};

pub const TOTAL_MATTER: Float = powf(10.0, 54);
pub const STARTING_AVAILABLE_MATTER: Float = powf(10.0, 24) * 6000.0;

pub const DEFAULT_BATTLENAME: &str = "Durenstein 1";
pub const THRENODY_START: &str = "Threnody for the Heroes of ";
pub const MAX_BATTLENAME_LEN: usize = 24+1 + 20;
pub const MAX_THRENODY_LEN: usize = THRENODY_START.len() + MAX_BATTLENAME_LEN;

#[derive(Debug, Clone, Copy)]
pub struct Space {
    /// # spaceFlag
    pub space_flag: bool,

    pub hypno_drone_event: Option<Instant>,

    /// # honor
    pub honor: Float,

    /// # threnodyTitle
    pub threnody_title: ArrayString<MAX_BATTLENAME_LEN>,
    pub threnody_project: ArrayString<MAX_THRENODY_LEN>,

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

            honor: 0.0,

            threnody_title: ArrayString::from(DEFAULT_BATTLENAME).expect("Always valid"),
            threnody_project: {
                let mut string = ArrayString::from(THRENODY_START).expect("Always valid");
                string.push_str(DEFAULT_BATTLENAME);
                string
            },

            boredom_level: 0.0,

            available_matter: STARTING_AVAILABLE_MATTER,
            found_matter: STARTING_AVAILABLE_MATTER,
            acquired_matter: 0.0,
            processed_matter: 0.0,
        }
    }
}
