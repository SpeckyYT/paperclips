use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::{core::Float, PaperClips};

pub const CREATIVITY_THRESHOLD: Float = 400.0;
pub const MEM_SIZE: u32 = 1000;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Computational {
    pub comp_flag: bool,

    /// # trust
    pub trust: i32,
    /// # processors
    pub processors: u32,
    /// # memory
    pub memory: u32,
    /// # operations
    pub operations: Float,
    /// # creativity
    pub creativity: Float,
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

    /// # creativityCounter
    pub creativity_counter: u32,
    /// # creativitySpeed
    pub creativity_speed: Float,
    /// # creativityOn
    pub creativity_flag: bool,
    /// # prestigeS
    pub prestige_s: Float,
}

impl Default for Computational {
    fn default() -> Self {
        Self {
            comp_flag: false,
            trust: 2,
            processors: 1,
            memory: 1,
            operations: 0.0,
            creativity: 0.0,
            standard_ops: 0.0,
            temp_ops: 0.0,
            op_fade: 0.0,
            op_fade_timer: 0,
            op_fade_delay: 800,
            next_trust: 3000.0,
            fib: [2.0, 3.0],
            creativity_counter: 0,
            creativity_speed: 1.0,
            creativity_flag: false,
            prestige_s: 0.0,
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

        if self.temp_ops + self.standard_ops < self.max_operations() as Float {
            self.standard_ops += self.temp_ops;
            self.temp_ops = 0.0;
        }

        self.operations = (self.standard_ops + self.temp_ops.floor()).floor();

        if self.operations < self.max_operations() as Float {
            let op_cycle = self.processors as Float / 10.0;
            let op_buf = self.max_operations() as Float - self.operations;

            let op_cycle = op_cycle.min(op_buf);

            self.standard_ops += op_cycle;
        }

        self.standard_ops = self.standard_ops.min(self.max_operations() as Float);
    }
    pub fn calculate_creativity(&mut self) {
        self.creativity_counter += 1;

        let s = self.prestige_s / 10.0;
        let ss = self.creativity_speed * (s + 1.0);

        let creativity_check = CREATIVITY_THRESHOLD / ss;

        if self.creativity_counter as Float >= creativity_check {
            self.creativity = match creativity_check.total_cmp(&1.0) {
                Ordering::Greater|Ordering::Equal => self.creativity + 1.0,
                Ordering::Less => self.creativity + ss / CREATIVITY_THRESHOLD,
            };
            self.creativity_counter = 0;
        }
    }
    /// # `memory * 1000`
    #[inline]
    pub fn max_operations(&self) -> u32 {
        self.memory * MEM_SIZE
    }
}

impl PaperClips {
    pub fn calculate_trust(&mut self) {
        if self.business.clips >= self.computational.next_trust {
            self.computational.trust += 1;
            self.console.push("Production target met: TRUST INCREASED, additional processor/memory capacity granted");
            let fib_next = self.computational.fib.iter().sum::<Float>();
            self.computational.next_trust = fib_next * 1000.0;
            self.computational.fib = [self.computational.fib[1], fib_next];
        }
    }
    /// # addProc()
    pub fn add_processors(&mut self) {
        if self.computational.trust > 0 /* || swarmGifts > 0 */ {
            let processors = &mut self.computational.processors;
            *processors += 1;
            let proc_float = *processors as Float;
            self.computational.creativity_speed = proc_float.log10() * proc_float.powf(1.1) + proc_float - 1.0;
            if self.human_flag {
                // swarmGifts -= 1;
            }
            self.console.push(match self.computational.creativity_flag {
                false => "Processor added, operations per sec increased",
                true => "Processor added, operations (or creativity) per sec increased",
            });
        } 
    }
    /// # addMem()
    pub fn add_memory(&mut self) {
        if self.computational.trust > 0 /* || swarmGifts > 0 */ {
            self.computational.memory += 1;
            if self.human_flag {
                // swarmGifts -= 1;
            }
            self.console.push("Memory added, max operations increased");
        }
    }
}
