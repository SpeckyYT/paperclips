use eframe::egui::{Color32, CornerRadius, InnerResponse, Rect, RichText, Sense, Ui, Vec2};
use paperclips::{qchips::QOPS_FADE_TIME, PaperClips};

pub fn business_group(ui: &mut Ui, paperclips: &mut PaperClips) -> InnerResponse<()> {
    ui.group(|ui| {
        ui.heading("Business");
        ui.separator();

        ui.label(format!("Available Funds: ${:.2}", paperclips.business.funds));
        ui.label(format!("Unsold Inventory: {:.0}", paperclips.business.unsold_clips));
        ui.horizontal(|ui| {
            if ui.button("lower").clicked() { paperclips.business.lower_price(); }
            if ui.button("raise").clicked() { paperclips.business.raise_price(); }
            ui.label(format!("Price per Clip: ${:.2}", paperclips.business.margin));
        });
        ui.label(format!("Public Demand: {:.0}%", paperclips.business.demand * 10.0)); // `* 10.0` is intentional

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.add_enabled_ui(paperclips.business.funds >= paperclips.business.ad_cost, |ui| {
                if ui.button("Marketing").clicked() {
                    paperclips.business.buy_ads();
                }
            });
            ui.label(format!("Level: {}", paperclips.business.marketing_lvl));
        });
        ui.label(format!("Cost: ${}", paperclips.business.ad_cost));
    })
}

pub fn manufacturing_group(ui: &mut Ui, paperclips: &mut PaperClips) -> InnerResponse<()> {
    ui.group(|ui| {
        ui.heading("Manufacturing");
        ui.separator();

        ui.label(format!("Clips per Second: {:.0}", paperclips.business.clip_rate));

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.add_enabled_ui(paperclips.business.funds >= paperclips.wire.cost, |ui| {
                if ui.button("Wire").clicked() {
                    paperclips.buy_wire();
                }
            });
            ui.label(format!("{:.0} inches", paperclips.wire.count));
        });
        ui.label(format!("Cost: ${:.0}", paperclips.wire.cost));
    })
}

pub fn quantum_computing_group(ui: &mut Ui, paperclips: &mut PaperClips) {
    if paperclips.qchips.q_flag {
        ui.group(|ui | {
            const SIZE: f32 = 20.0;
            const SPACING: f32 = 2.0;

            ui.heading("Quantum Computing");
            let activated = paperclips.qchips.activated;
            let size = Vec2::new(
                (SIZE + SPACING) * activated as f32,
                SIZE,
            );
            let (resp, painter) = ui.allocate_painter(size, Sense::HOVER);
            let base = resp.rect.min;
            for i in 0..activated {
                let x_off = (SIZE + SPACING) * i as f32;
                let pos = base + Vec2::new(x_off, 0.0);
                let rect = Rect::from_min_size(pos, Vec2::splat(SIZE));
                let chip = paperclips.qchips.chips[i as usize];
                let color = Color32::WHITE.gamma_multiply(chip.clamp(0.0, 1.0) as f32);
                painter.rect_filled(rect, CornerRadius::ZERO, color);
            }
            ui.horizontal(|ui| {
                if ui.button("Compute").clicked() {
                    paperclips.quantum_compute();
                }
                let text = match paperclips.qchips.qops {
                    Some(qops) => format!("qOps: {qops:.0}"),
                    None => "Need Photonic Chips".to_string(),
                };
                let text_color = ui.style().visuals.text_color();
                let transparency = QOPS_FADE_TIME.saturating_sub(paperclips.qchips.fade.elapsed()).as_secs_f32() / QOPS_FADE_TIME.as_secs_f32();
                let color = text_color.gamma_multiply(transparency);
                ui.label(RichText::new(text).color(color));
            });
        });
    }
}
