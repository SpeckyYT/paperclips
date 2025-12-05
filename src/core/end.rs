use crate::{PaperClips, project::{PROJECT_148, PROJECT_211, PROJECT_212, PROJECT_213, PROJECT_215, PROJECT_216}};

#[derive(Debug, Clone, Copy, Default)]
pub struct End {
    pub dismantle: Dismantle,
    pub timer1: u8,
    pub timer2: u8,
    pub timer3: u8,
    pub timer4: u8,
    pub timer5: u8,
    pub timer6: u16,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dismantle {
    #[default]
    None = 0,
    Probes = 1,
    Swarm = 2,
    Factories = 3,
    Strategy = 4,
    Quantum = 5,
    Processors = 6,
    Memory = 7,
}

impl PaperClips {
    pub fn ending(&mut self) {
        // if self.end.dismantle >= 1 {
        //     // probeDesignDivElement.style.display = "none";
        //     if (endTimer1 >= 50) {
        //         // increaseProbeTrustDivElement.style.display = "none";
        //     }

        //     if (endTimer1 >= 100) {
        //         // increaseMaxTrustDivElement.style.display = "none";
        //     }

        //     if (endTimer1 >= 150) {
        //         // spaceDivElement.style.display = "none";
        //     }


        //     if (endTimer1 >= 175) {
        //         // battleCanvasDivElement.style.display = "none";
        //     }

        //     if (endTimer1 >= 190) {
        //         // honorDivElement.style.display = "none";
        //     }

        // }

        // if (dismantle >= 2) {

        //     // wireProductionDivElement.style.display = "none";
        //     // wireTransDivElement.style.display = "";

        //     if (endTimer2 >= 50) {
        //         // swarmGiftDivElement.style.display = "none";
        //     }

        //     if (endTimer2 >= 100) {
        //         // swarmEngineElement.style.display = "none";
        //     }

        //     if (endTimer2 >= 150) {
        //         // swarmSliderDivElement.style.display = "none";
        //     }

        // }

        // if (dismantle >= 3) {
        //     // factoryDivSpaceElement.style.display = "none";
        //     // clipsPerSecDivElement.style.display = "none";
        //     // tothDivElement.style.display = "none";

        // }

        // if (dismantle >= 4) {
        //     // strategyEngineElement.style.display = "none";
        //     // tournamentManagementElement.style.display = "none";
        // }

        if self.end.dismantle >= Dismantle::Quantum {
            // TODO handle GUI
            // btnQcomputeElement.style.display = "none";
            self.qchips.chips.iter_mut().for_each(|c| *c = 0.5);
            if matches!(self.end.timer4, 10|60|100|130|150|160|165|169|172|174) {
                self.wire.count += 1.0;
            }
            self.qchips.activated = match self.end.timer4 {
                250.. => { self.qchips.q_flag = false; 0 },
                174.. => 0,
                172.. => 1,
                169.. => 2,
                165.. => 3,
                160.. => 4,
                150.. => 5,
                130.. => 6,
                100.. => 7,
                60.. => 8,
                10.. => 9,
                0.. => 10,
            };
        }

        // if (dismantle >= 6) {
        //     // processorDisplayElement.style.display = "none";
        // }

        // if (dismantle >= 7) {
        //     compDivElement.style.display = "none";
        //     projectsDivElement.style.display = "none";

        // }

        if self.projects.is_active(PROJECT_148) {
            self.end.timer1 = self.end.timer1.saturating_add(1);
        }
        if self.projects.is_active(PROJECT_211) {
            self.end.timer2 = self.end.timer2.saturating_add(1);
        }
        if self.projects.is_active(PROJECT_212) {
            self.end.timer3 = self.end.timer3.saturating_add(1);
        }

        if self.projects.is_active(PROJECT_213) {
            self.end.timer4 = self.end.timer4.saturating_add(1);
        }

        if self.projects.is_active(PROJECT_215) {
            self.end.timer5 = self.end.timer5.saturating_add(1);
        }
        if self.projects.is_active(PROJECT_216) && self.wire.count <= 0.0 {
            self.end.timer6 = self.end.timer6.saturating_add(1);
        }

        if self.end.timer6 >= 250 {
            // TODO
            // creationDivElement.style.display = "none";
        }

        if self.end.timer6 >= 500 && self.milestone_flag == 15 {
            self.threnody.play();
            self.console.push("Universal Paperclips");
            self.milestone_flag += 1;
        }
        if self.end.timer6 >= 600 && self.milestone_flag == 16 {
            self.console.push("a game by Frank Lantz");
            self.milestone_flag += 1;
        }
        if self.end.timer6 >= 700 && self.milestone_flag == 17 {
            self.console.push("combat programming by Bennett Foddy");
            self.milestone_flag += 1;
        }
        if self.end.timer6 >= 800 && self.milestone_flag == 18 {
            self.console.push("'Riversong' by Tonto's Expanding Headband used by kind permission of Malcolm Cecil");
            self.milestone_flag += 1;
        }
        if self.end.timer6 >= 900 && self.milestone_flag == 19 {
            self.console.push("© 2017 Everybody House Games");
            self.milestone_flag += 1;
        }
    }
}