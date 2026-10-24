use darkforest::engine::{BuildingKind, Darkforest, EngineCommand, EngineOutput};

#[test]
fn starter_engine_exposes_authoritative_snapshot() {
    let darkforest = Darkforest::starter();
    let snapshot = darkforest.snapshot();
    let civilizations = snapshot.civilizations().collect::<Vec<_>>();
    let buildings = snapshot.buildings().collect::<Vec<_>>();
    let build_queue = snapshot.build_queue().collect::<Vec<_>>();

    assert_eq!(civilizations.len(), 1);
    assert_eq!(civilizations[0].1.name, "First Camp");
    assert_eq!(civilizations[0].1.stage.as_str(), "Hunter-Gatherer");
    assert_eq!(civilizations[0].1.population, 24);
    assert_eq!(civilizations[0].1.resources.food, 85);
    assert_eq!(civilizations[0].1.resources.wood, 120);
    assert_eq!(buildings.len(), 1);
    assert_eq!(buildings[0].1.kind, BuildingKind::CentralCamp);
    assert_eq!(build_queue.len(), 2);
    assert_eq!(build_queue[0].1.name, "Storage Pit");
}

#[test]
fn repeated_snapshots_are_stable_without_commands() {
    let darkforest = Darkforest::starter();

    let first = darkforest.snapshot();
    let second = darkforest.snapshot();

    assert_eq!(first, second);
}

#[test]
fn run_once_processes_one_actor_command() {
    let mut darkforest = Darkforest::starter();

    let output = darkforest.run_once(EngineCommand::Snapshot);

    match output {
        EngineOutput::Snapshot(snapshot) => {
            let civilizations = snapshot.civilizations().collect::<Vec<_>>();
            assert_eq!(civilizations[0].1.name, "First Camp");
        }
    }
}
