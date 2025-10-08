use std::time::Instant;

use crate::paperclips_core::{Float, PaperClips};

pub struct QChips {
    pub chips: [Float; 10],
    pub activated: u8,
    pub fade: Instant,
}

impl Default for QChips {
    fn default() -> Self {
        Self { chips: [0.0; 10], activated: 0, fade: Instant::now() }
    }
}

impl PaperClips {
    pub fn quantum_compute(&mut self) {
        let qclock = self.session_start.elapsed().as_secs_f64() as Float / 10.0;
        for (i, value) in self.qchips.chips.iter_mut().enumerate()  {
            let wave_speed = (10 - i) as Float / 10.0;
            *value = (qclock * wave_speed).sin();
            // update qchip opacity
        }
    }

    pub fn quantum_compute_button(&mut self) {
        self.qchips.fade = Instant::now();

        let q: Float = self.qchips.chips.iter()
            .take(self.qchips.activated as usize)
            .copied()
            .sum();

        let mut qq = (q * 360.0).ceil();
        
        let buffer = (self.computational.memory as Float * 1000.0) - self.computational.standard_ops;
        let damper = (self.computational.temp_ops / 100.0) + 5.0;

        if qq > buffer as Float {
            self.computational.temp_ops += (qq/damper).ceil() - buffer;
            qq = buffer;
            // opFade = 0.01;
            // opFadeTimer = 0;
        }

        self.computational.standard_ops += qq;

        // update "qops: {q * 360}"
    }
}
