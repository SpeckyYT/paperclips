use eframe::egui::{Align2, Color32, CornerRadius, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};
use paperclips::combat::{BATTLE_HEIGHT, BATTLE_WIDTH, ship::Team};

use crate::gui::Gui;

impl Gui {
    pub fn draw_combat_group(&mut self, ui: &mut Ui) {
        let pc = &mut self.paperclips;

        let size = ui.available_size();
        let width = size.x;
        let unit = width / BATTLE_WIDTH as f32;
        let height = unit * BATTLE_HEIGHT as f32;

        let (resp, painter) = ui.allocate_painter(Vec2::new(width, height), Sense::HOVER);
        let rect = resp.rect;

        for ship in &pc.combat.ships {
            let x = rect.left() + ship.x as f32 / BATTLE_WIDTH as f32 * width;
            let y = rect.top() + ship.y as f32 / BATTLE_HEIGHT as f32 * height;
            let ship_rect = Rect::from_center_size(
                (x, y).into(),
                (unit, unit).into(),
            );
            painter.rect(
                ship_rect,
                CornerRadius::same(1),
                match ship.team {
                    Team::Left => Color32::WHITE,
                    Team::Right => Color32::BLACK,
                },
                Stroke::NONE,
                StrokeKind::Middle,
            );
        }

        painter.rect_filled(rect, CornerRadius::ZERO, Color32::DARK_GRAY);
        painter.text(
            Pos2::new(rect.left() + 10.0, rect.top() + 10.0),
            Align2::LEFT_TOP, "no way!",
            FontId::monospace(8.0),
            Color32::BLACK,
        );
    }
}
