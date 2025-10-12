use std::arch::naked_asm;

use crate::{paperclips_core::Float, PaperClips};

pub struct Computational {
    pub comp_flag: bool,

    /// # trust
    pub trust: u32,
    /// # processors
    pub processors: u32,
    /// # memory
    pub memory: u32,
    /// # operations
    pub operations: Float,
    /// # standardOps
    pub standard_ops: Float,
    /// # tempOps
    pub temp_ops: Float,
    /// # opFade
    pub op_fade: Float,
    /// # opFadeTimer
    pub op_fade_timer: u32,
    /// # opFadeDelay
    pub op_fade_delay: u32,
    
    /// # nextTrust
    pub next_trust: Float,
    /// # \[fib2, fib1\]
    pub fib: [Float; 2],
}

impl Default for Computational {
    fn default() -> Self {
        Self {
            comp_flag: false,
            trust: 2,
            processors: 1,
            memory: 1,
            operations: 0.0,
            standard_ops: 0.0,
            temp_ops: 0.0,
            op_fade: 0.0,
            op_fade_timer: 0,
            op_fade_delay: 800,
            next_trust: 3000.0,
            fib: [2.0, 3.0],
        }
    }
}

impl Computational {
    pub fn calculate_operations(&mut self) {
        self.temp_ops = if self.temp_ops > 0.0 {
            self.op_fade_timer += 1;

            if self.op_fade_timer > self.op_fade_delay {
                self.op_fade += 3f64.powf(3.5) as Float / 1000.0;
            }

            (self.temp_ops - self.op_fade).round()
        } else {
            0.0
        };

        if self.temp_ops + self.standard_ops < (self.memory * 1000) as Float {
            self.standard_ops += self.temp_ops;
            self.temp_ops = 0.0;
        }

        self.operations = (self.standard_ops + self.temp_ops.floor()).floor();

        if self.operations < (self.memory * 1000) as Float {
            let op_cycle = self.processors as Float / 10.0;
            let op_buf = (self.memory * 1000) as Float - self.operations;

            let op_cycle = op_cycle.min(op_buf);

            self.standard_ops += op_cycle;
        }

        self.standard_ops = self.standard_ops.min((self.memory * 1000) as Float);
    }
}

impl PaperClips {
    pub fn calculate_trust(&mut self) {
        if self.business.clips >= self.computational.next_trust {
            self.computational.trust += 1;
            self.messages.push("Production target met: TRUST INCREASED, additional processor/memory capacity granted");
            let fib_next = self.computational.fib.iter().sum::<Float>();
            self.computational.next_trust = fib_next * 1000.0;
            self.computational.fib = [self.computational.fib[1], fib_next];
        }
    }
}
