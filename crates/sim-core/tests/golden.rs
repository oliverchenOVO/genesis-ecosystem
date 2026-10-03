use sim_core::replay::Replay;

#[test]
fn golden_command_replay_matches_every_checkpoint() {
    let replay: Replay =
        serde_json::from_str(include_str!("../../../fixtures/golden-v5.json")).unwrap();
    assert_eq!(replay.checkpoints.len(), 6);
    let world = replay.verify().unwrap();
    assert_eq!(world.state.tick, 2000);
    assert_eq!(world.hash(), replay.checkpoints.last().unwrap().hash);
}

#[test]
fn historical_v4_fixture_is_preserved_and_explicitly_incompatible() {
    let replay: Replay =
        serde_json::from_str(include_str!("../../../fixtures/golden-v4.json")).unwrap();
    assert_eq!(replay.simulation_version, 4);
    assert_eq!(replay.checkpoints.len(), 6);
    assert!(replay.verify().unwrap_err().contains("Incompatible"));
}
#[test]
fn prerelease_v5_revision1_fixture_is_preserved_and_explicitly_incompatible() {
    let replay: Replay =
        serde_json::from_str(include_str!("../../../fixtures/golden-v5-prefinal-r1.json")).unwrap();
    assert_eq!(replay.rules_revision, 1);
    assert!(replay
        .verify()
        .unwrap_err()
        .contains("prerelease simulation v5 rules revision"));
}

#[test]
fn prerelease_v5_revision2_fixture_is_preserved_and_explicitly_incompatible() {
    let replay: Replay =
        serde_json::from_str(include_str!("../../../fixtures/golden-v5-r2-archive.json")).unwrap();
    assert_eq!(replay.rules_revision, 2);
    assert_eq!(replay.checkpoints.len(), 6);
    assert!(replay.verify().unwrap_err().contains("rules revision 2"));
}

#[test]
fn historical_v1_fixture_is_explicitly_incompatible() {
    let replay: Replay =
        serde_json::from_str(include_str!("../../../fixtures/golden-v1.json")).unwrap();
    assert!(replay.verify().unwrap_err().contains("Incompatible"));
}

#[test]
fn historical_v2_fixture_is_explicitly_incompatible() {
    let replay: Replay =
        serde_json::from_str(include_str!("../../../fixtures/golden-v2.json")).unwrap();
    assert!(replay.verify().unwrap_err().contains("Incompatible"));
}

#[test]
fn historical_v3_fixture_is_explicitly_incompatible() {
    let replay: Replay =
        serde_json::from_str(include_str!("../../../fixtures/golden-v3.json")).unwrap();
    assert!(replay.verify().unwrap_err().contains("Incompatible"));
}
