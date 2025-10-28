use std::time::Instant;

use eframe::egui::{Color32, RichText, Ui};

use crate::gui::Gui;

const TEXT_SIZE: f32 = 80.0;
const LONG_BLINK_INTERVAL: u128 = 32;

impl Gui {
    /// Returns if the long blink finished
    pub fn long_blink(&mut self, ui: &mut Ui, start: Instant) -> bool {
        let time = start.elapsed().as_millis();

        let step = time / LONG_BLINK_INTERVAL;

        let enabled = step % 2 == 0;
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
                huge_text(TEXT_SIZE * 4.0, "Release");
            }
            45..55 => {
                huge_text(TEXT_SIZE * 2.0, "Release");
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
