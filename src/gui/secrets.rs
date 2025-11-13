use eframe::egui::{CursorIcon, Response, Sense};

use crate::gui::Gui;

impl Gui {
    pub fn video_serio_button(&mut self, resp: Response) {
        if self.paperclips.business.clips.floor() == 151.0 && resp.on_hover_cursor(CursorIcon::PointingHand).interact(Sense::click()).clicked() {
            self.play_video_serio();
        }
    }
}
