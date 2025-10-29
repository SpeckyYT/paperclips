use std::time::Instant;

use eframe::egui::{Color32, RichText, Ui};
use paperclips::Ticks;

use crate::gui::Gui;

const TEXT_SIZE: f32 = 115.0;
const SPACE_SIZE: f32 = TEXT_SIZE * 1.6;

const LONG_BLINK_INTERVAL: Ticks = 32;

impl Gui {
    /// Returns if the long blink finished
    pub fn long_blink(&mut self, ui: &mut Ui, start: Instant) -> bool {
        let time = start.elapsed().as_millis();

        let step = time / LONG_BLINK_INTERVAL;

        let enabled = step.is_multiple_of(2);
        if enabled { return false }

        let mut huge_text = |space: f32, text: &str| {
            if space > 0.0 {
                ui.add_space(space);
            }
            ui.label(RichText::new(text).size(TEXT_SIZE).color(Color32::WHITE));    
        };

        match step {
            5..10 => {
                huge_text(0.0, "Release");
            }
            30..40 => {
                huge_text(SPACE_SIZE * 2.0, "Release");
            }
            45..55 => {
                huge_text(SPACE_SIZE, "Release");
            }
            55..120 => {
                huge_text(0.0, "Release\nthe\nHypno\nDrones");
            }
            120.. => return true,
            _ => return false,
        }

        false
    }
}
