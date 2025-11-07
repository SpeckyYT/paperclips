use eframe::egui::{RichText, Ui};
use egui_extras::{Column, TableBuilder};
use paperclips::strategy::StrategyGrid;

use crate::gui::Gui;

impl Gui {
    pub fn draw_payoff_grid(&mut self, ui: &mut Ui) {
        // this is a shit way to detect it
        let has_labels = self.paperclips.strategy.disable_run_button && !self.paperclips.strategy.results_flag;

        let (strat_h, strat_v) = self.paperclips.strategy.fight;
        let StrategyGrid { aa, ab, ba, bb, choice_names: (choice_a, choice_b), .. } = self.paperclips.strategy.grid;

        if has_labels {
            ui.label(strat_h.name);
            ui.label(strat_v.name);
        }

        TableBuilder::new(ui)
        .id_salt("strat_grid")
        .column(Column::remainder().at_least(75.0).at_most(125.0))
        .columns(Column::auto().at_least(75.0).at_most(125.0), 3)
        .body(|mut body| {
            body.row(20.0, |mut row| {
                row.col(|ui| {
                    ui.label("");
                    ui.label(choice_a);
                    ui.label(choice_b);
                });
                row.col(|ui| {
                    ui.label(choice_a);
                    ui.label(format!("{aa},{aa}"));
                    ui.label(format!("{ba},{ab}"));
                });
                row.col(|ui| {
                    ui.label(choice_b);
                    ui.label(format!("{ab},{ba}"));
                    ui.label(format!("{bb},{bb}"));
                });
            });
        });
    }
    pub fn draw_strats_results(&mut self, ui: &mut Ui) {
        for (i, &(strat, points)) in self.paperclips.strategy.strats.iter().enumerate() {
            let mut line = RichText::new(format!("{}. {}: {points}", i + 1, strat.name));
            if strat == self.paperclips.strategy.pick {
                line = line.strong();
            }
            ui.label(line);
        }
    }
}
