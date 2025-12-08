use std::time::{Duration, Instant};

use eframe::{
    App, Frame, egui::{CentralPanel, Context, ScrollArea, TopBottomPanel}
};
use kittyaudio::Mixer;
use paperclips::{PaperClips, util::number_cruncher};

const TEN_MS: Duration = Duration::from_millis(10);
const FRAME_60FPS: Duration = Duration::from_millis(16);

pub mod groups;
pub mod blink;
pub mod threnody;
pub mod secrets;

pub struct Gui {
    pub paperclips: PaperClips,

    audio_mixer: Mixer,

    last_main_update: Instant,
    last_combat_update: Instant,
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            paperclips: PaperClips::default(),

            audio_mixer: {
                let mixer = Mixer::new();
                mixer.init();
                mixer
            },

            last_main_update: Instant::now(),
            last_combat_update: Instant::now(),
        }
    }
}

impl App for Gui {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.update_paperclips(ctx);

        TopBottomPanel::top("console").show(ctx, |ui| {
            // #consoleDiv
            self.draw_top_console(ui);
            // #topDiv / #prestigeDiv
            self.draw_prestige(ui);
        });

        // #topDiv
        CentralPanel::default().show(ctx, |ui| {
            let resp = ui.heading(format!("Paperclips: {}", self.paperclips.business.clips.round()))
                .on_hover_text(number_cruncher(self.paperclips.business.clips, Some(1)));

            self.paperclips_button_secret(resp);

            ScrollArea::vertical().show(ui, |ui| {
                ui.columns_const(|[left, middle, right]| {
                    // LEFT COLUMN
                    self.draw_make_paperclip(left);

                    match self.paperclips.human_flag {
                        false => {
                            self.draw_creation_group(left);
                            self.draw_wire_production_group(left);
                            if self.paperclips.space.space_flag {
                                self.draw_space_group(left);
                            }
                        },
                        true => {
                            self.draw_business_group(left);
                            self.draw_manufacturing_group(left);
                        }
                    }

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
                    if self.paperclips.strategy.engine_flag {
                        self.draw_strategy_group(right);
                    }
                    self.draw_combat_group(right); // TODO
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
                self.paperclips.main_tick();
                self.check_threnody();
            }
            last_combat_update(FRAME_60FPS) {
                self.paperclips.update_combat();
            }
        }

        ctx.request_repaint_after(TEN_MS.saturating_sub(self.last_main_update.elapsed()));
    }
    pub fn check_threnody(&mut self) {
        if self.paperclips.threnody.check() {
            self.play_threnody();
        }
    }
}
