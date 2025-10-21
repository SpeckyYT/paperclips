use eframe::{run_native, NativeOptions};

use crate::gui::Gui;

mod gui;

fn main() {
    run_native(
        "paperclips",
        NativeOptions {
            ..Default::default()
        },
        Box::new(|_cc| {
            let mut gui = Gui::default();
            gui.paperclips.messages.push("Welcome to Universal Paperclips");
            Ok(Box::new(gui))
        })
    ).unwrap();
}
