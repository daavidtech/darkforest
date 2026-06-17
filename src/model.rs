use crate::db::DarkforestDb;

#[derive(Debug, Clone, wgui::WguiModel)]
pub struct ResourceSnapshot {
    pub wood: u32,
    pub iron: u32,
    pub food: u32,
}

#[derive(Debug, Clone, wgui::WguiModel)]
pub struct QueueItem {
    pub name: String,
    pub status: String,
}

#[derive(Debug, Clone, wgui::WguiModel)]
pub struct ReportItem {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, wgui::WguiModel)]
pub struct GameSnapshot {
    pub civilization_name: String,
    pub stage: String,
    pub population: u32,
    pub happiness: u32,
    pub innovation: u32,
    pub resources: ResourceSnapshot,
    pub queue: Vec<QueueItem>,
    pub reports: Vec<ReportItem>,
}

impl GameSnapshot {
    pub fn from_db(db: &DarkforestDb) -> Self {
        let civilization = db
            .civilizations
            .snapshot()
            .into_iter()
            .next()
            .expect("starter civilization missing");
        let queue = db
            .build_queue
            .snapshot()
            .into_iter()
            .filter(|item| item.civilization_id == civilization.id)
            .map(|item| QueueItem {
                name: item.name,
                status: item.status,
            })
            .collect();
        let reports = db
            .reports
            .snapshot()
            .into_iter()
            .filter(|report| report.civilization_id == civilization.id)
            .map(|report| ReportItem {
                title: report.title,
                body: report.body,
            })
            .collect();

        Self {
            civilization_name: civilization.name,
            stage: civilization.stage,
            population: civilization.population,
            happiness: civilization.happiness,
            innovation: civilization.innovation,
            resources: ResourceSnapshot {
                wood: civilization.wood,
                iron: civilization.iron,
                food: civilization.food,
            },
            queue,
            reports,
        }
    }
}
