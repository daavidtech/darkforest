use crate::engine::GameState;
use wgui::{Db, DbTable, HasId, Wdb, WguiModel};

#[derive(Debug, Clone, WguiModel)]
pub struct Civilization {
    pub id: u32,
    pub name: String,
    pub stage: String,
    pub population: u32,
    pub happiness: u32,
    pub innovation: u32,
    pub wood: u32,
    pub iron: u32,
    pub food: u32,
}

impl HasId for Civilization {
    fn id(&self) -> u32 {
        self.id
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}

#[derive(Debug, Clone, WguiModel)]
pub struct BuildQueueItem {
    pub id: u32,
    pub civilization_id: u32,
    pub name: String,
    pub status: String,
}

impl HasId for BuildQueueItem {
    fn id(&self) -> u32 {
        self.id
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}

#[derive(Debug, Clone, WguiModel)]
pub struct Report {
    pub id: u32,
    pub civilization_id: u32,
    pub title: String,
    pub body: String,
}

impl HasId for Report {
    fn id(&self) -> u32 {
        self.id
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}

#[derive(Debug, Wdb)]
pub struct DarkforestDb {
    pub civilizations: DbTable<Civilization>,
    pub build_queue: DbTable<BuildQueueItem>,
    pub reports: DbTable<Report>,
}

impl DarkforestDb {
    pub fn from_game_state(state: &GameState) -> Self {
        let db = Db::<DarkforestDb>::new();
        Self {
            civilizations: db.table_with_ids(
                state
                    .civilizations()
                    .map(|(id, civilization)| Civilization {
                        id: id.raw(),
                        name: civilization.name.clone(),
                        stage: civilization.stage.as_str().to_string(),
                        population: civilization.population,
                        happiness: civilization.happiness,
                        innovation: civilization.innovation,
                        wood: civilization.resources.wood,
                        iron: civilization.resources.iron,
                        food: civilization.resources.food,
                    })
                    .collect(),
            ),
            build_queue: db.table_with_ids(
                state
                    .build_queue()
                    .map(|(id, item)| BuildQueueItem {
                        id: id.raw(),
                        civilization_id: item.civilization_id.raw(),
                        name: item.name.clone(),
                        status: item.status.clone(),
                    })
                    .collect(),
            ),
            reports: db.table_with_ids(starter_reports()),
        }
    }
}

fn starter_reports() -> Vec<Report> {
    vec![Report {
        id: 1,
        civilization_id: 1,
        title: "Settlement founded".to_string(),
        body: "The first camp controls only its immediate surroundings.".to_string(),
    }]
}
