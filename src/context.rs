use crate::engine::DarkforestHandle;

#[derive(Debug, Clone)]
pub struct SharedContext {
    pub _engine: DarkforestHandle,
}

impl SharedContext {
    pub fn new(engine: DarkforestHandle) -> Self {
        Self { _engine: engine }
    }
}
