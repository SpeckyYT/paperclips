use eframe::egui::{Color32, CornerRadius, CursorIcon, InnerResponse, Rect, RichText, Sense, Ui, Vec2};
use paperclips::{messages::Console, project::{ProjectStatus, PROJECTS}, qchips::QOPS_FADE_TIME, PaperClips};

pub fn business_group(ui: &mut Ui, pc: &mut PaperClips) -> InnerResponse<()> {
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

pub fn manufacturing_group(ui: &mut Ui, pc: &mut PaperClips) -> InnerResponse<()> {
    ui.group(|ui| {
        ui.heading("Manufacturing");
        ui.separator();

        ui.label(format!(
            "Clips per Second: {:.0}",
            pc.business.clip_rate
        ));

        ui.add_space(10.0);

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
                    if ui.button("AutoClippers").clicked() {
                        pc.business.make_mega_clipper();
                    }
                });
                ui.label(format!("{:.0}", pc.business.mega_clipper_level));
            });
            ui.label(format!("Cost: ${:.2}", pc.business.mega_clipper_cost));
        }
    })
}

pub fn computational_group(ui: &mut Ui, pc: &mut PaperClips) {
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

            ui.label(format!("Operations: {}/{}", c.operations, c.max_operations()));
            ui.label(format!("Creativity: {}", c.creativity));
        });
    }
}

pub fn quantum_computing_group(ui: &mut Ui, pc: &mut PaperClips) {
    if pc.qchips.q_flag {
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
}

pub fn projects_group(ui: &mut Ui, pc: &mut PaperClips) {
    ui.group(|ui| {
        ui.heading("Projects");
        ui.separator();

        for (i, ps) in pc.projects.statuses.into_iter().enumerate() {
            if ps == ProjectStatus::Buyable {
                let project = &PROJECTS[i];

                let affordable = (project.cost.1)(pc);

                ui.add_enabled_ui(affordable, |ui| {
                    let mut pj = ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(project.title.to_string(pc));
                            ui.label(project.cost.0.to_string(pc));
                        });
                        ui.label(project.description.to_string(pc));
                    }).response.interact(Sense::click());

                    if affordable {
                        pj = pj.highlight().on_hover_cursor(CursorIcon::PointingHand);
                        if pj.clicked() {
                            pc.buy_project(i);
                        }
                    }
                });
            }
        }
    });
}

pub fn top_console(ui: &mut Ui, pc: &PaperClips) {
    let Console { max_messages, messages } = &pc.console;
    let to_fill = *max_messages - messages.len();
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
            ui.label(RichText::new(format!("{head} {string}")).monospace());
        });
    }
}
