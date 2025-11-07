use eframe::{run_native, NativeOptions};

use crate::{gui::Gui, setup::init_setup};

pub mod gui;
pub mod setup;

fn main() {
    init_setup();

    run_native(
        "paperclips",
        NativeOptions {
            ..Default::default()
        },
        Box::new(|_cc| {
            let mut gui = Gui::default();
            gui.paperclips.console.push("Welcome to Universal Paperclips");
            Ok(Box::new(gui))
        })
    ).unwrap();
}
