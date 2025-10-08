use std::time::Instant;

use crate::paperclips_core::{computational::Computational, investments::Investments, messages::Messages, qchips::QChips, wire::Wire};

pub type Float = f32;

pub mod wire;
pub mod computational;
pub mod qchips;
pub mod projects;
pub mod strategy;
pub mod util;
pub mod messages;
pub mod investments;

pub struct PaperClips {
    session_start: Instant,

    funds: Float,
    wire: Wire,
    qchips: QChips,
    computational: Computational, 
    messages: Messages,
    investments: Investments,
}

impl Default for PaperClips {
    fn default() -> Self {
        Self { 
            session_start: Instant::now(),
            funds: 0.0,
            wire: Wire::default(),
            qchips: QChips::default(),
            computational: Computational::default(),
            messages: Messages::default(),
            investments: Investments::default(),
        }
    }
}
