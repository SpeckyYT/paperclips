use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Threnody {
    pub should_play: bool,
}

impl Threnody {
    #[inline]
    pub fn play(&mut self) {
        self.should_play = true;
    }
    /// Returns if it should play and will set the variable to false
    #[inline]
    pub fn check(&mut self) -> bool {
        std::mem::take(&mut self.should_play)
    }
}
