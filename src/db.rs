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
    pub fn new() -> Self {
        let db = Db::<DarkforestDb>::new();
        Self {
            civilizations: db.table_with_ids(vec![Civilization {
                id: 1,
                name: "First Camp".to_string(),
                stage: "Primitive".to_string(),
                population: 24,
                happiness: 72,
                innovation: 3,
                wood: 120,
                iron: 0,
                food: 85,
            }]),
            build_queue: db.table_with_ids(vec![
                BuildQueueItem {
                    id: 1,
                    civilization_id: 1,
                    name: "Storage Pit".to_string(),
                    status: "Ready to place".to_string(),
                },
                BuildQueueItem {
                    id: 2,
                    civilization_id: 1,
                    name: "Scout Camp".to_string(),
                    status: "Requires 80 wood".to_string(),
                },
            ]),
            reports: db.table_with_ids(vec![Report {
                id: 1,
                civilization_id: 1,
                title: "Settlement founded".to_string(),
                body: "The first camp controls only its immediate surroundings.".to_string(),
            }]),
        }
    }
}
