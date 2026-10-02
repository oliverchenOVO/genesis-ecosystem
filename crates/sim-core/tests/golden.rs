use sim_core::replay::Replay;

#[test]
fn golden_command_replay_matches_every_checkpoint() {
    let replay: Replay =
        serde_json::from_str(include_str!("../../../fixtures/golden-v1.json")).unwrap();
    assert_eq!(replay.checkpoints.len(), 6);
    let world = replay.verify().unwrap();
    assert_eq!(world.state.tick, 2000);
    assert_eq!(world.hash(), replay.checkpoints.last().unwrap().hash);
}
