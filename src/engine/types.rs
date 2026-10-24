use crate::arena::ArenaId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CivilizationStage {
    HunterGatherer,
}

impl CivilizationStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HunterGatherer => "Hunter-Gatherer",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Civilization {
    pub name: String,
    pub stage: CivilizationStage,
    pub happiness: u32,
    pub innovation: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingKind {
    CentralCamp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Building {
    pub civilization_id: ArenaId,
    pub settlement_id: ArenaId,
    pub kind: BuildingKind,
    pub name: String,
}

pub struct Village {
	pub civilization_id: ArenaId
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopulationRole {
    Hunter,
    Gatherer,
    Builder,
    Scout,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopulationGroup {
    pub civilization_id: ArenaId,
    pub role: PopulationRole,
    pub count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resources {
    pub wood: u32,
    pub iron: u32,
    pub food: u32,
}

impl Resources {
    pub fn empty() -> Self {
        Self {
            wood: 0,
            iron: 0,
            food: 0,
        }
    }
}

pub struct SettlementType {

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settlement {
    pub civilization_id: ArenaId,
    pub name: String,
    pub resources: Resources,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildStatus {
    Queued,
    Building,
    Completed,
    Failed,
}

impl BuildStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "Queued",
            Self::Building => "Building",
            Self::Completed => "Completed",
            Self::Failed => "Failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildQueueEntry {
    pub civilization_id: ArenaId,
    pub name: String,
    pub status: BuildStatus,
}

pub struct Cell {

}

pub struct Planet {
    cells: Vec<Cell>,
}
