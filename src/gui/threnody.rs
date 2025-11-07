use std::{sync::{Arc, OnceLock}, thread::spawn, time::Instant};

use kittyaudio::{Sound, include_sound};

use crate::gui::Gui;

static SOUND: OnceLock<Arc<Sound>> = OnceLock::new();
#[inline]
pub fn get_threnody() -> Arc<Sound> {
    SOUND.get_or_init(|| {
        Arc::new({
            if cfg!(target_os = "android") || cfg!(target_os = "ios") {
                include_sound!("../../assets/test_mobile.mp3").unwrap()
            } else {
                include_sound!("../../assets/test_web.mp3").unwrap()
            }
        })
    }).clone()
}

impl Gui {
    #[inline]
    pub fn play_threnody(&mut self) {
        self.audio_mixer.play((*get_threnody()).clone());
    }
}

#[inline]
pub fn setup_threnody() {
    spawn(|| {
        let start = Instant::now();
        get_threnody();
        println!("loaded threnody: {:?}", start.elapsed());
    });
}
