#[derive(Debug, Clone, Copy, Default)]
pub struct PCRng {
    pub rng_kind: RngKind,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum RngKind {
    #[default]
    ThreadRng,
    SM64Rng(u16),
    Best,
    Worst,
}
use RngKind::*;

use crate::Float;

impl PCRng {
    #[inline]
    pub fn random_bool(&mut self, mut probability: f64, best: bool) -> bool {
        probability = probability.clamp(0.0, 1.0);
        if probability >= 1.0 { return true }
        if probability <= 0.0 { return false }
        match self.rng_kind {
            ThreadRng => rand::random_bool(probability),
            SM64Rng(ref mut number) => (sm64_rng(number) as f64 / (u16::MAX as f64 + 1.0)) < probability,
            Best => best,
            Worst => !best,
        }
    }
    #[inline]
    pub fn random_bool_no_best(&mut self, probability: f64) -> bool {
        match self.rng_kind {
            Best|Worst => rand::random_bool(probability),
            _ => self.random_bool(probability, true),
        }
    }
    // 0.0..1.0
    #[inline]
    pub fn random_float(&mut self, one_is_best: bool) -> Float {
        let mut best_worst = [ 1.0, 0.0 ];
        if !one_is_best {
            best_worst.reverse();
        }
        let [ best, worst ] = best_worst;
        match self.rng_kind {
            ThreadRng => rand::random::<Float>(),
            SM64Rng(ref mut number) => sm64_rng(number) as Float / (u16::MAX as f64 + 1.0),
            Best => best,
            Worst => worst,
        }
    }
    #[inline]
    pub fn random_float_no_best(&mut self) -> Float {
        match self.rng_kind {
            Best|Worst => rand::random::<Float>(),
            _ => self.random_float(true),
        }
    }
}

pub fn sm64_rng(input: &mut u16) -> u16 {
    if *input == 0x560a {
        *input = 0; // prevent a two-number loop
    }

    let mut s0 = *input << 8;
    s0 ^= *input;

    *input = ((s0 & 0x00ff) << 8) | ((s0 & 0xff00) >> 8);
    s0 = ((s0 & 0x00ff) << 1) ^ *input;

    let s1 = (s0 >> 1) ^ 0xff80;

    *input = if (s0 & 1) == 0 {
        if s1 == 0xaa55 {
            0 // reset cycle at 65,114th number
        } else {
            s1 ^ 0x1ff4
        }
    } else {
        s1 ^ 0x8180
    };

    *input
}
