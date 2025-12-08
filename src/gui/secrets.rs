use eframe::egui::{CursorIcon, Response, Sense};

use crate::gui::Gui;

impl Gui {
    pub fn paperclips_button_secret(&mut self, resp: Response) {
        let hover = || {
            resp.clone().on_hover_cursor(CursorIcon::PointingHand).interact(Sense::click()).clicked()
        };
        match self.paperclips.business.clips.floor() {
            105.0 if hover() => self.play_video_serio(),
            151.0 if hover() => self.play_la_zucca(),
            _ => {}
        }
    }
}
