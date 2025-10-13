use eframe::{run_native, NativeOptions};

use crate::gui::Gui;

mod gui;

fn main() {
    run_native(
        "paperclips",
        NativeOptions::default(),
        Box::new(|_cc| {
            let gui = Gui::default();
            Ok(Box::new(gui))
        })
    ).unwrap();
}
