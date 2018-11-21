use std::sync::Arc;

use crate::hosting::{ExternalFunc, ExternalMemory, ExternalModule};

pub struct SpecTest {}

impl SpecTest {
    pub fn new() -> SpecTest {
        SpecTest {}
    }
}

impl ExternalModule for SpecTest {
    fn name(&self) -> &str {
        "spectest"
    }

    fn funcs(&self) -> &[Arc<ExternalFunc>] {
        &[]
    }

    fn mems(&self) -> &[ExternalMemory] {
        &[]
    }
}
