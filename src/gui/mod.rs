use std::time::{Duration, Instant};

use eframe::{
    egui::{CentralPanel, Context, ScrollArea, TopBottomPanel}, App, Frame
};
use paperclips::PaperClips;

use crate::gui::{groups::{business_group, computational_group, investments_group, manufacturing_group, projects_group, quantum_computing_group, top_console}};

const TEN_MS: Duration = Duration::from_millis(10);
const FRAME_60FPS: Duration = Duration::from_millis(16);

mod groups;
mod blink;

pub struct Gui {
    pub paperclips: PaperClips,
    gui_draw_state: GuiDrawState,

    last_main_update: Instant,
    last_stock_shop_update: Instant,
    last_stocks_update: Instant,
    last_wire_price_and_demand_update: Instant,
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            paperclips: PaperClips::default(),
            gui_draw_state: GuiDrawState::Normal,

            last_main_update: Instant::now(),
            last_stock_shop_update: Instant::now(),
            last_stocks_update: Instant::now(),
            last_wire_price_and_demand_update: Instant::now(),
        }
    }
}

enum GuiDrawState {
    Normal,
    /// "Release the Hypno Drones" animation
    LongBlink(Instant),
}

impl App for Gui {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.update_paperclips(ctx);

        TopBottomPanel::top("console").show(ctx, |ui| {
            top_console(ui, &self.paperclips);
        });

        CentralPanel::default().show(ctx, |ui| {
            let pc = &mut self.paperclips;
            ui.heading(format!("Paperclips: {}", pc.business.clips.round()));

            ScrollArea::vertical().show(ui, |ui| {
                ui.columns_const(|[left, middle, right]| {
                    // LEFT COLUMN
                    left.add_enabled_ui(pc.wire.count >= 1.0, |ui| {
                        if ui.button("Make Paperclip").clicked() {
                            pc.clip_click(1.0);
                        }
                    });
                    business_group(left, pc);
                    manufacturing_group(left, pc);

                    // MIDDLE COLUMN
                    computational_group(middle, pc);
                    if pc.qchips.q_flag {
                        quantum_computing_group(middle, pc);
                    }
                    if pc.projects.flag {
                        projects_group(middle, pc);
                    }

                    // RIGHT COLUMN
                    if pc.investments.engine_flag {
                        investments_group(right, pc);
                    }
                });
            });
        });
    }
}

impl Gui {
    pub fn update_paperclips(&mut self, ctx: &Context) {
        macro_rules! update_time {
            ($($prop:ident($time:expr) $code:block)*) => {
                const TOTAL_LOOPS: usize = [$(stringify!($prop)),*].len();
                $(
                    let start = Instant::now();
                    while start.elapsed() < FRAME_60FPS / TOTAL_LOOPS as u32 {
                        if self.$prop.elapsed() >= $time {
                            self.$prop += $time;
                            $code;
                        } else {
                            break
                        }
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
