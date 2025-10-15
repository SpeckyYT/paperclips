use std::time::{Duration, Instant};

use eframe::{egui::{CentralPanel, Color32, Context, CornerRadius, Rect, Sense, Vec2}, App, Frame};
use paperclips::PaperClips;

const TEN_MS: Duration = Duration::from_millis(10);

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

        CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Paperclips: {}", self.paperclips.business.clips));
            if ui.button("Make Paperclip").clicked() {
                self.paperclips.clip_click(1.0);
            }

            ui.group(|ui| {
                ui.heading("Business");
                ui.separator();

                ui.label(format!("Available Funds: ${:.2}", self.paperclips.business.funds));
                ui.label(format!("Unsold Inventory: {:.0}", self.paperclips.business.unsold_clips));
                ui.horizontal(|ui| {
                    if ui.button("lower").clicked() { self.paperclips.business.lower_price(); }
                    if ui.button("raise").clicked() { self.paperclips.business.raise_price(); }
                    ui.label(format!("Price per Clip: ${:.2}", self.paperclips.business.margin));
                });
                ui.label(format!("Public Demand: {:.0}%", self.paperclips.business.demand * 10.0)); // `* 10.0` is intentional

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("Marketing").clicked() {
                        self.paperclips.business.buy_ads();
                    }
                    ui.label(format!("Level: {}", self.paperclips.business.marketing_lvl));
                });
                ui.label(format!("Cost: ${}", self.paperclips.business.ad_cost));
            });

            ui.group(|ui| {
                ui.heading("Manufacturing");
                ui.separator();

                ui.label(format!("Clips per Second: {:.0}", self.paperclips.business.clip_rate));

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("Wire").clicked() {
                        self.paperclips.buy_wire();
                    }
                    ui.label(format!("{:.0} inches", self.paperclips.wire.count));
                });
                ui.label(format!("Cost: ${:.0}", self.paperclips.wire.cost));
            });

            if self.paperclips.qchips.q_flag {
                ui.group(|ui | {
                    const SIZE: f32 = 20.0;
                    const SPACING: f32 = 2.0;
    
                    ui.heading("Quantum Computing");
                    let activated = self.paperclips.qchips.activated;
                    let size = Vec2::new(
                        (SIZE + SPACING) * activated as f32,
                        SIZE,
                    );
                    let (resp, painter) = ui.allocate_painter(size, Sense::HOVER);
                    let base = resp.rect.min;
                    for i in 0..activated {
                        let x_off = (SIZE + SPACING) * i as f32;
                        let pos = base + Vec2::new(x_off, 0.0);
                        let r = Rect::from_min_size(pos, Vec2::splat(SIZE));
                        let chip = self.paperclips.qchips.chips[i as usize];
                        let color = Color32::WHITE.gamma_multiply(chip.max(0.0).min(1.0) as f32);
                        painter.rect_filled(r, CornerRadius::ZERO, color);
                    }
                    ui.horizontal(|ui| {
                        if ui.button("Compute").clicked() {
                            self.paperclips.quantum_compute();
                        }
                        ui.label(format!("qOps: {}", 69420));
                    });
                });
            }
        });
    }
}

impl Gui {
    pub fn update_paperclips(&mut self, ctx: &Context) {
        macro_rules! update_time {
            ($($prop:ident($time:expr) $code:block)*) => {
                $(
                    if self.$prop.elapsed() >= $time {
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
