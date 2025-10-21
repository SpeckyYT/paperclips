use std::time::{Duration, Instant};

use eframe::{
    egui::{CentralPanel, Context, TopBottomPanel}, App, Frame
};
use paperclips::PaperClips;

use crate::gui::groups::{business_group, manufacturing_group, projects_group, quantum_computing_group, top_console};

const TEN_MS: Duration = Duration::from_millis(10);

mod groups;

pub struct Gui {
    pub paperclips: PaperClips,

    last_main_update: Instant,
    last_stock_shop_update: Instant,
    last_stocks_update: Instant,
    last_wire_price_and_demand_update: Instant,
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            paperclips: PaperClips::default(),
            last_main_update: Instant::now(),
            last_stock_shop_update: Instant::now(),
            last_stocks_update: Instant::now(),
            last_wire_price_and_demand_update: Instant::now(),
        }
    }
}

impl App for Gui {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.update_paperclips(ctx);

        TopBottomPanel::top("console").show(ctx, |ui| {
            top_console(ui, &mut self.paperclips);
        });

        CentralPanel::default().show(ctx, |ui| {
            let pc = &mut self.paperclips;

            ui.heading(format!("Paperclips: {}", pc.business.clips.ceil()));
            ui.add_enabled_ui(pc.wire.count >= 1.0, |ui| {
                if ui.button("Make Paperclip").clicked() {
                    pc.clip_click(1.0);
                }
            });

            business_group(ui, pc);

            manufacturing_group(ui, pc);

            quantum_computing_group(ui, pc);

            if pc.projects.flag {
                projects_group(ui, pc);
            }
        });
    }
}

impl Gui {
    pub fn update_paperclips(&mut self, ctx: &Context) {
        macro_rules! update_time {
            ($($prop:ident($time:expr) $code:block)*) => {
                $(
                    while self.$prop.elapsed() >= $time {
                        self.$prop += $time;
                        $code;
                    }
                )*
            };
        }
        update_time!{
            last_main_update(TEN_MS) {
                self.paperclips.main_loop_tick();
            }
            last_stock_shop_update(Duration::from_millis(1000)) {
                self.paperclips.update_stock_shop_tick();
            }
            last_stocks_update(Duration::from_millis(2500)) {
                self.paperclips.update_stocks_tick();
            }
            last_wire_price_and_demand_update(Duration::from_millis(100)) {
                self.paperclips.update_wire_price_and_demand_tick();
            }
        }

        ctx.request_repaint_after(TEN_MS.saturating_sub(self.last_main_update.elapsed()));
    }
}
