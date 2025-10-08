use crate::paperclips_core::Float;

pub struct Computational {
    /// # processors
    pub processors: u32,
    /// # memory
    pub memory: u32,
    /// # standardOps
    pub standard_ops: Float,
    /// # tempOps
    pub temp_ops: Float,
}

impl Default for Computational {
    fn default() -> Self {
        Self {
            processors: 1,
            memory: 1,
            standard_ops: 0.0,
            temp_ops: 0.0,
        }
    }
}
