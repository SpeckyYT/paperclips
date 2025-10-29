use std::time::{Duration, Instant};

use eframe::{
    egui::{CentralPanel, Context, ScrollArea, TopBottomPanel}, App, Frame
};
use paperclips::PaperClips;

const TEN_MS: Duration = Duration::from_millis(10);
const FRAME_60FPS: Duration = Duration::from_millis(16);

mod groups;
mod blink;

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
            self.draw_top_console(ui);
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Paperclips: {}", self.paperclips.business.clips.round()));

            ScrollArea::vertical().show(ui, |ui| {
                ui.columns_const(|[left, middle, right]| {
                    // LEFT COLUMN
                    left.add_enabled_ui(self.paperclips.wire.count >= 1.0, |ui| {
                        if ui.button("Make Paperclip").clicked() {
                            self.paperclips.clip_click(1.0);
                        }
                    });
                    self.draw_business_group(left);
                    self.draw_manufacturing_group(left);

                    #[cfg(debug_assertions)]
                    {
                        left.add_space(30.0);
                        self.draw_cheat_group(left);
                    }

                    // MIDDLE COLUMN
                    self.draw_computational_group(middle);
                    if self.paperclips.qchips.q_flag {
                        self.draw_quantum_computing_group(middle);
                    }
                    if self.paperclips.projects.flag {
                        self.draw_projects_group(middle);
                    }

                    // RIGHT COLUMN
                    if self.paperclips.investments.engine_flag {
                        self.draw_investments_group(right);
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
