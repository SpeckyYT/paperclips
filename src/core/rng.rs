#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct PCRng {
    pub rng_kind: RngKind,
}

const PREV_BELOW_ONE: Float = Float::from_bits((1.0 as Float).to_bits() - 1);

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum RngKind {
    #[default]
    ThreadRng,
    SM64Rng(u16),
    Best,
    Worst,
}
use RngKind::*;
use serde::{Deserialize, Serialize};

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
        let [ best, worst ] = match one_is_best {
            true => [ PREV_BELOW_ONE, 0.0 ],
            false => [ 0.0, PREV_BELOW_ONE ],
        };
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
    #[inline]
    pub fn is_best(&mut self) -> bool {
        matches!(self.rng_kind, RngKind::Best)
    }
    #[inline]
    pub fn is_worst(&mut self) -> bool {
        matches!(self.rng_kind, RngKind::Worst)
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
