use bouncy_instructor::parse_challenge_str;

#[test]
fn jumping_jacks_example_parses() {
    let set = parse_challenge_str(
        include_str!("../../bouncy_frontend/src/lib/assets/challenges/jumping-jacks.ron"),
        "en",
    )
    .expect("the jumping-jacks challenge definition must parse");
    assert_eq!(set.challenges().len(), 1);
    assert_eq!(set.challenges()[0].id(), "three-jumping-jacks");
    assert!(set.tracker(0).is_some());
}
