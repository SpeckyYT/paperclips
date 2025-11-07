use std::{borrow::Cow, time::Instant};

use eframe::egui::{Color32, ComboBox, CornerRadius, CursorIcon, Frame, InnerResponse, Rect, RichText, Sense, Ui, Vec2};
use egui_extras::{Column, TableBuilder};
use paperclips::{investments::Riskiness, messages::Console, qchips::QOPS_FADE_TIME, strategy::TourneyDisplay, util::blink};
use strum::IntoEnumIterator;

use crate::gui::Gui;

mod strategy;

impl Gui {
    pub fn draw_business_group(&mut self, ui: &mut Ui) -> InnerResponse<()> {
        let pc = &mut self.paperclips;
        
        ui.group(|ui| {
            ui.heading("Business");
            ui.separator();
            
            ui.label(format!(
                "Available Funds: ${:.2}",
                pc.business.funds
            ));
    
            if pc.business.rev_per_sec_flag {
                ui.label(format!("Avg. Rev. per sec: ${:.2}", pc.business.avg_rev));
                ui.label(format!("Avg. Clips Sold per sec: {:.0}", pc.business.avg_sales));
            }
    
            ui.label(format!(
                "Unsold Inventory: {:.0}",
                pc.business.unsold_clips
            ));
            ui.horizontal(|ui| {
                ui.add_enabled_ui(pc.business.margin > 0.01, |ui| {
                    if ui.button("lower").clicked() {
                        pc.business.lower_price();
                    }
                });
                if ui.button("raise").clicked() {
                    pc.business.raise_price();
                }
                ui.label(format!(
                    "Price per Clip: ${:.2}",
                    pc.business.margin
                ));
            });
            ui.label(format!(
                "Public Demand: {:.0}%",
                pc.business.demand * 10.0
            )); // `* 10.0` is intentional
    
            ui.add_space(10.0);
    
            ui.horizontal(|ui| {
                ui.add_enabled_ui(
                    pc.business.funds >= pc.business.ad_cost,
                    |ui| {
                        if ui.button("Marketing").clicked() {
                            pc.business.buy_ads();
                        }
                    },
                );
                ui.label(format!("Level: {}", pc.business.marketing_lvl));
            });
            ui.label(format!("Cost: ${}", pc.business.ad_cost));
        })
    }
    
    pub fn draw_manufacturing_group(&mut self, ui: &mut Ui) -> InnerResponse<()> {
        let pc = &mut self.paperclips;

        ui.group(|ui| {
            ui.heading("Manufacturing");
            ui.separator();
    
            ui.label(format!(
                "Clips per Second: {:.0}",
                pc.business.clip_rate
            ));
    
            ui.add_space(10.0);
    
            if pc.wire.buyer_flag {
                ui.horizontal(|ui| {
                    if ui.button("WireBuyer").clicked() {
                        pc.wire.buyer_status ^= true;
                    }
                    ui.label(match pc.wire.buyer_status {
                        true => "ON",
                        false => "OFF",
                    });
                });
            }
            ui.horizontal(|ui| {
                ui.add_enabled_ui(pc.business.funds >= pc.wire.cost, |ui| {
                    if ui.button("Wire").clicked() {
                        pc.buy_wire();
                    }
                });
                ui.label(format!("{:.0} inches", pc.wire.count));
            });
            ui.label(format!("Cost: ${:.0}", pc.wire.cost));
    
            if pc.business.clipper_flag {
                ui.add_space(10.0);
    
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(pc.business.funds >= pc.business.clipper_cost, |ui| {
                        if ui.button("AutoClippers").clicked() {
                            pc.business.make_clipper();
                        }
                    });
                    ui.label(format!("{:.0}", pc.business.clipper_level));
                });
                ui.label(format!("Cost: ${:.2}", pc.business.clipper_cost));
            }
    
            if pc.business.mega_clipper_flag {
                ui.add_space(10.0);
    
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(pc.business.funds >= pc.business.mega_clipper_cost, |ui| {
                        if ui.button("MegaClippers").clicked() {
                            pc.business.make_mega_clipper();
                        }
                    });
                    ui.label(format!("{:.0}", pc.business.mega_clipper_level));
                });
                ui.label(format!("Cost: ${:.2}", pc.business.mega_clipper_cost));
            }
        })
    }
    
    pub fn draw_computational_group(&mut self, ui: &mut Ui) {
        let pc = &mut self.paperclips;

        if pc.computational.comp_flag {
            ui.group(|ui| {
                let c = &pc.computational;
    
                ui.heading("Computational Resources");
                ui.separator();
    
                ui.label(format!("Trust: {}", c.trust));
                ui.label(format!("+1 Trust at: {} clips", c.next_trust));
    
                ui.add_space(10.0);
    
                let enable_buttons = c.trust > c.processors as i32 + c.memory as i32 /* && swarmGifts > 0 */;
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(enable_buttons, |ui| {
                        if ui.button("Processors").clicked() {
                            pc.add_processors();
                        }
                    });
                    ui.label(pc.computational.processors.to_string());
                });
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(enable_buttons, |ui| {
                        if ui.button("Memory").clicked() {
                            pc.add_memory();
                        }
                    });
                    ui.label(pc.computational.memory.to_string());
                });
                let c = &pc.computational;
    
                ui.add_space(10.0);
    
                ui.label(format!("Operations: {:.0}/{}", c.operations, c.max_operations()));
                ui.label(format!("Creativity: {:.0}", c.creativity));
            });
        }
    }
    
    pub fn draw_quantum_computing_group(&mut self, ui: &mut Ui) {
        let pc = &mut self.paperclips;

        ui.group(|ui| {
            const SIZE: f32 = 20.0;
            const SPACING: f32 = 2.0;
    
            ui.heading("Quantum Computing");
            let activated = pc.qchips.activated();
            let size = Vec2::new((SIZE + SPACING) * activated as f32, SIZE);
            let (resp, painter) = ui.allocate_painter(size, Sense::HOVER);
            let base = resp.rect.min;
            for i in 0..activated {
                let x_off = (SIZE + SPACING) * i as f32;
                let pos = base + Vec2::new(x_off, 0.0);
                let rect = Rect::from_min_size(pos, Vec2::splat(SIZE));
                let chip = pc.qchips.chips[i as usize];
                let color = Color32::WHITE.gamma_multiply(chip.clamp(0.0, 1.0) as f32);
                painter.rect_filled(rect, CornerRadius::ZERO, color);
            }
            ui.horizontal(|ui| {
                if ui.button("Compute").clicked() {
                    pc.quantum_compute();
                }
                let text = match pc.qchips.qops {
                    Some(qops) => format!("qOps: {qops:.0}"),
                    None => "Need Photonic Chips".to_string(),
                };
                let text_color = ui.style().visuals.text_color();
                let transparency = QOPS_FADE_TIME
                    .saturating_sub(pc.qchips.fade.elapsed())
                    .as_secs_f32()
                    / QOPS_FADE_TIME.as_secs_f32();
                let color = text_color.gamma_multiply(transparency);
                ui.label(RichText::new(text).color(color));
            });
        });
    }
    
    pub fn draw_projects_group(&mut self, ui: &mut Ui) {
        let pc = &mut self.paperclips;

        ui.group(|ui| {
            ui.heading("Projects");
            ui.separator();
    
            let buyable_projects = pc.projects.buyable_projects.clone().into_iter().enumerate();
            for (bpi, (instant, project)) in buyable_projects {
                let affordable = (project.cost.1)(pc);
    
                ui.add_enabled_ui(affordable, |ui| {
                    let mut frame = Frame::group(ui.style()).begin(ui);
    
                    {
                        let ui = &mut frame.content_ui;
                        ui.horizontal(|ui| {
                            ui.label(project.title.to_string(pc));
                            ui.label(project.cost.0.to_string(pc));
                        });
                        ui.label(project.description.to_string(pc));
                    }
    
                    let pj = frame.allocate_space(ui);
                    if pj.hovered() && affordable && pj.enabled() {
                        frame.frame.stroke.color = Color32::GRAY;
                    }
                    if !blink(instant) {
                        frame.frame.stroke.color = Color32::WHITE;
                    }
                    frame.paint(ui);
    
                    if affordable {
                        let pj = pj.interact(Sense::CLICK).on_hover_cursor(CursorIcon::PointingHand);
                        if pj.clicked() {
                            pc.buy_project(bpi);
                        }
                    }
                });
            }
        });
    }
    
    pub fn draw_investments_group(&mut self, ui: &mut Ui) {
        let pc = &mut self.paperclips;

        ui.group(|ui| {
            ui.heading("Investments");
            ui.separator();
    
            let riskiness = &mut pc.investments.riskiness;
        
            ComboBox::from_label("Riskiness")
                .selected_text(riskiness.name())
                .show_ui(ui, |ui| {
                    for risk in Riskiness::iter() {
                        ui.selectable_value(riskiness, risk, risk.name());
                    }
                });
    
            ui.columns_const(|[left, right]| {
                if left.button("Deposit").clicked() {
                    pc.invest_deposit();
                }
                if left.button("Withdraw").clicked() {
                    pc.invest_withdraw();
                }
                
                right.label(format!("Cash: ${:.2}", pc.investments.bankroll));
                right.label(format!("Stocks: ${:.2}", pc.investments.sec_total()));
                right.label(format!("Total: ${:.2}", pc.investments.port_total()));
            });
    
            const TABLE_HEADINGS: &[&str] = &["Stock", "Amt.", "Price", "Total", "P/L"];
    
            ui.group(|ui| {
                TableBuilder::new(ui)
                    .columns(Column::remainder(), TABLE_HEADINGS.len())
                    .striped(true)
                    .header(15.0, |mut row| {
                        for col in TABLE_HEADINGS {
                            row.col(|ui| { ui.label(*col); });
                        }
                    })
                    .body(|mut body| {
                        let to_fill = pc.investments.max_port - pc.investments.stocks.len();
                        for stock in &pc.investments.stocks {
                            body.row(15.0, |mut row| {
                                row.col(|ui| { ui.label(&*stock.symbol); });
                                row.col(|ui| { ui.label(format!("{}", &stock.amount)); });
                                row.col(|ui| { ui.label(format!("{:.0}", &stock.price)); });
                                row.col(|ui| { ui.label(format!("{:.0}", &stock.total())); });
                                row.col(|ui| { ui.label(format!("{:.0}", &stock.profit)); });
                            });
                        }
                        body.rows(15.0, to_fill, |mut row| {
                            for _ in 0..pc.investments.max_port {
                                row.col(|_| {});
                            }
                        });
                    });
            });
            ui.horizontal(|ui| {
                ui.add_enabled_ui(pc.strategy.yomi >= pc.investments.invest_upgrade_cost, |ui| {
                    if ui.button("Upgrade Investment Engine").clicked() {
                        pc.invest_upgrade();
                    }
                });
                ui.label(format!("Level: {}", pc.investments.invest_level));
            });
            ui.label(format!("Cost: {:.0} Yomi", pc.investments.invest_upgrade_cost));
        });
    }
    
    pub fn draw_strategy_group(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.heading("Strategic Modeling");
            ui.separator();

            ui.add_enabled_ui(self.paperclips.strategy.tourney_in_prog && !self.paperclips.strategy.disable_run_button, |ui| {
                ComboBox::from_label("Pick a Strat")
                .selected_text(self.paperclips.strategy.pick.name)
                .show_ui(ui, |ui| {
                    for (strat, _) in &self.paperclips.strategy.strats {
                        ui.selectable_value(&mut self.paperclips.strategy.pick, strat, strat.name);
                    }
                });
                if ui.button("Run").clicked() {
                    self.paperclips.run_tourney();
                }
            });

            let display_text: Cow<'static, str> = match self.paperclips.strategy.tourney_report_display {
                TourneyDisplay::RunTournament => "Pick strategy, run tournament, gain yomi".into(),
                TourneyDisplay::Round => format!("Round: {}", self.paperclips.strategy.current_round + 1).into(),
                TourneyDisplay::Results(false) => "TOURNAMENT RESULTS (roll over for grid)".into(),
                TourneyDisplay::Results(true) => "TOURNAMENT RESULTS (roll over for payoff grid)".into(),
            };
            ui.label(display_text);

            // tournamentStuff

            ui.group(|ui| {
                let mut frame = Frame::NONE.begin(ui);
                let f_ui = &mut frame.content_ui;

                static mut HOVERED: bool = false;
                if self.paperclips.strategy.results_flag && !unsafe { HOVERED } {
                    self.draw_strats_results(f_ui);
                } else {
                    self.draw_payoff_grid(f_ui);
                }
                let resp = frame.end(ui);
                unsafe { HOVERED = resp.hovered(); }
            });

            ui.label(format!("Yomi: {:.0}", self.paperclips.strategy.yomi));
            
            ui.add_enabled_ui(!self.paperclips.strategy.tourney_in_prog && self.paperclips.computational.operations >= self.paperclips.strategy.tourney_cost, |ui| {
                if ui.button("New Tournament").clicked() {
                    self.paperclips.new_tourney();
                }
            });
            ui.label(format!("Cost: {:.0} ops", self.paperclips.strategy.tourney_cost));
        });
    }

    pub fn draw_top_console(&mut self, ui: &mut Ui) {
        if let Some(start) = self.paperclips.space.hypno_drone_event {
            if self.long_blink(ui, start) {
                self.paperclips.space.hypno_drone_event = None;
            }
        }

        let pc = &mut self.paperclips;

        let Console { max_messages, messages } = &pc.console;
        let to_fill = *max_messages - messages.len();

        // TODO: make this into a black background
        for _ in 0..to_fill {
            ui.add_enabled_ui(false, |ui| ui.label(RichText::new(".").monospace()));
        }
        for (i, string) in messages.iter().enumerate() {
            let is_last = i >= messages.len() - 1;
            ui.add_enabled_ui(is_last, |ui| {
                let head = match is_last {
                    false => '.',
                    true => '>',
                };
                let tail = match is_last && (pc.ticks / 25).is_multiple_of(2) {
                    true => '|',
                    false => ' ',
                };
                ui.label(RichText::new(format!("{head} {string}{tail}")).monospace());
            });
        }
    }

    pub fn draw_cheat_group(&mut self, ui: &mut Ui) {
        if ui.button("Free Clips").clicked() {
            self.paperclips.cheat_clips();
        }
        if ui.button("Free Money").clicked() {
            self.paperclips.cheat_money();
        }
        if ui.button("Free Trust").clicked() {
            self.paperclips.cheat_trust();
        }
        if ui.button("Free Ops").clicked() {
            self.paperclips.cheat_ops();
        }
        if ui.button("Free Creativity").clicked() {
            self.paperclips.cheat_creat();
        }
        if ui.button("Free Yomi").clicked() {
            self.paperclips.cheat_yomi();
        }
        if ui.button("Reset Prestige").clicked() {
            self.paperclips.reset_prestige();
        }

        if ui.button("Destroy all Humans").clicked() {
            self.paperclips.space.hypno_drone_event = Some(Instant::now());
        }
        if ui.button("Free Prestige U").clicked() {
            // TODO:
            // self.paperclips.cheat_prestige_u();
        }
        if ui.button("Free Prestige S").clicked() {
            self.paperclips.cheat_prestige_s();
        }
        if ui.button("Set Battle Number 1 to 7").clicked() {
            // TODO:
            // self.set_b()
        }
        if ui.button("Set Avail Matter to 0").clicked() {
            self.paperclips.zero_matter();
        }
    }
}
