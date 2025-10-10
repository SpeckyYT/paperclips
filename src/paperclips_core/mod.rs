use std::time::Instant;

use rand::random_bool;

use crate::{business::Business, paperclips_core::{computational::Computational, investments::Investments, messages::Messages, qchips::QChips, wire::Wire}, strategy::Strategy};

pub type Float = f32;

pub mod business;
pub mod messages;
pub mod wire;
pub mod computational;
pub mod qchips;
pub mod projects;
pub mod strategy;
pub mod util;
pub mod investments;

pub struct PaperClips {
    session_start: Instant,
    ticks: u128,

    messages: Messages,

    business: Business,
    wire: Wire,
    computational: Computational, 
    qchips: QChips,
    investments: Investments,
    strategy: Strategy,
}

impl Default for PaperClips {
    fn default() -> Self {
        Self { 
            session_start: Instant::now(),
            ticks: 0,

            business: Business::default(),
            wire: Wire::default(),
            qchips: QChips::default(),
            computational: Computational::default(),
            messages: Messages::default(),
            investments: Investments::default(),
            strategy: Strategy::default(),
        }
    }
}
