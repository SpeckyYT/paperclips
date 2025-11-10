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
            let gui = Gui::default();
            Ok(Box::new(gui))
        })
    ).unwrap();
}
