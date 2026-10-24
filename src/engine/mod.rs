mod types;

pub use types::*;

use crate::arena::{Arena, ArenaId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkForest {
    build_queue: Vec<BuildQueueEntry>,
    buildings: Arena<Building>,
    civilizations: Arena<Civilization>,
    population: Vec<PopulationGroup>,
    settlements: Arena<Settlement>,
}
