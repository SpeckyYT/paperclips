use std::{sync::{Arc, OnceLock}, thread::spawn};

use kittyaudio::{Sound, include_sound};

use crate::gui::Gui;

pub const IS_MOBILE: bool = cfg!(target_os = "android") || cfg!(target_os = "ios");

macro_rules! sounds {
    ($($static:ident = $func:ident $code:expr)*) => {
        $(
            static $static: OnceLock<Arc<Sound>> = OnceLock::new();
            #[inline]
            fn $func() -> Arc<Sound> {
                $static.get_or_init(|| {
                    Arc::new($code)
                }).clone()
            }
        )*

        impl Gui {
            $(
                #[inline]
                pub fn $func(&mut self) {
                    self.audio_mixer.play((*$func()).clone());
                }
            )*
        }

        pub fn load_sounds() {
            $( spawn(|| { $func(); }); )*
        }
    };
}

sounds!{
    THRENODY_SOUND = play_threnody {
        if IS_MOBILE {
            include_sound!("../../assets/test_mobile.mp3").unwrap()
        } else {
            include_sound!("../../assets/test_web.mp3").unwrap()
        }
    }
    VIDEO_SERIO_SOUND = play_video_serio include_sound!("../../assets/video_serio.mp3").unwrap()
}
