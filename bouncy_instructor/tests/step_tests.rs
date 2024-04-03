use expect_test::expect;
use std::collections::HashSet;

mod common;

/// This test checks that the list of defined steps doesn't change unexpectedly.
/// Anytime a static step is added, this test needs to be updated.
#[test]
fn test_listing_static_steps() {
    common::load_static_files();
    let mut step_names: Vec<_> = bouncy_instructor::steps()
        .iter()
        .map(|s| s.name())
        .collect();

    step_names.sort();

    let expect = expect!["Criss-Cross,Criss-Cross,Diagonal Running Man,Diagonal Running Man,Double Running Man,Double Running Man,Happy Feet,Idle,Idle Side,Knee Up,Open Running Man,Open Running Man,Open Running Man on Heels,Open Running Man on Heels,Pendulum,Pendulum,Running Man,Running Man,Running Man on Heels,Running Man on Heels,Simple Heels,Simple Heels,T-Step,T-Step,V-Step,V-Step"];
    expect.assert_eq(&step_names.join(","));
}

#[test]
fn test_unique_step_id() {
    common::load_static_files();
    let steps = bouncy_instructor::steps();
    let mut unique_ids = HashSet::<String>::new();
    let mut duplicates = vec![];
    for step in &steps {
        if !unique_ids.insert(step.id()) {
            duplicates.push(step.id());
        }
    }

    assert!(
        steps.len() == unique_ids.len(),
        "Duplicate step ID detected. {duplicates:?}"
    )
}
