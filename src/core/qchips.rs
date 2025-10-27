use std::time::{Duration, Instant};

use crate::core::{Float, PaperClips};

pub const QOPS_FADE_TIME: Duration = Duration::from_secs(10);

#[derive(Debug, Clone, Copy)]
pub struct QChips {
    /// # qFlag
    pub q_flag: bool,
    /// # qChips
    pub chips: [Float; 10],
    /// Doesn't exist in the original code, but it's part of the `qChips`.
    pub activated: u8,
    /// # qFade
    pub fade: Instant,
    /// # #qCompDisplay
    pub qops: Option<Float>,
    /// # qChipCost
    pub qchip_cost: Float,
}

impl Default for QChips {
    fn default() -> Self {
        Self {
            q_flag: false,
            chips: [0.0; 10],
            activated: 0,
            fade: Instant::now() - QOPS_FADE_TIME,
            qops: None,
            qchip_cost: 10000.0,
        }
    }
}

impl PaperClips {
    pub fn quantum_compute_update(&mut self) {
        let qclock = self.session_start.elapsed().as_secs_f64() as Float;
        for (i, value) in self.qchips.chips.iter_mut().enumerate()  {
            let wave_speed = (i + 1) as Float / 10.0;
            *value = (qclock * wave_speed).sin();
        }
    }

    pub fn quantum_compute(&mut self) {
        self.qchips.fade = Instant::now();
        self.qchips.qops = if self.qchips.activated() == 0 {
            None
        } else {
            let q: Float = self.qchips.chips.iter()
                .take(self.qchips.activated() as usize)
                .copied()
                .sum();
    
            let total_qq = (q * 360.0).ceil();
            let mut qq = total_qq;

            let buffer = self.computational.max_operations() as Float - self.computational.standard_ops;
            let damper = (self.computational.temp_ops / 100.0) + 5.0;
    
            if qq > buffer as Float {
                self.computational.temp_ops += (qq/damper).ceil() - buffer;
                qq = buffer;
                self.computational.op_fade = 0.01;
                self.computational.op_fade_timer = 0;
            }
    
            self.computational.standard_ops += qq;

            Some(total_qq)
        };
    }
}

impl QChips {
    #[inline]
    pub fn activated(&self) -> u8 {
        self.activated.min(self.chips.len() as u8)
    }
}
