use crate::{Float, PaperClips};

impl PaperClips {
    pub fn cheat_clips(&mut self) {
        const ADDED_CLIPS: Float = 100000000.0;
        self.business.clips += ADDED_CLIPS;
        self.business.unused_clips += ADDED_CLIPS;
        self.console.push("you just cheated");
    }
    pub fn cheat_money(&mut self) {
        self.business.funds += 10000000.0;
        self.console.push("LIZA just cheated");
    }
    pub fn cheat_trust(&mut self) {
        self.computational.trust += 1;
        self.console.push("Hilary is nice. Also, Liza just cheated");
    }
    pub fn cheat_ops(&mut self) {
        self.computational.standard_ops += 10000.0;
        self.console.push("you just cheated, Liza");
    }
    pub fn cheat_creat(&mut self) {
        self.computational.creativity_flag = true;
        self.computational.creativity += 1000.0;
        self.console.push("Liza just cheated. Very creative!");
    }
    pub fn cheat_yomi(&mut self) {
        self.strategy.yomi += 1000000.0;
        self.console.push("you just cheated");
    }
    pub fn reset_prestige(&mut self) {
        self.computational.prestige_s = 0.0;
    }

    // // This is GUI-only. It has no effect on the core state
    // pub fn cheat_hypno(&mut self) {}
    
    // // Needs `prestige_u`
    // pub fn cheat_prestige_u(&mut self) {
    //     self.computational.prestige_u
    // }
    pub fn cheat_prestige_s(&mut self) {
        self.computational.prestige_s += 1.0;
    }
    // // `setB`? what?
    // pub fn set_b(&mut self) {}

    pub fn zero_matter(&mut self) {
        self.space.available_matter = 0.0;
        self.console.push("you just cheated");
    }
}
