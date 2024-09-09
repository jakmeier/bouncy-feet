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

    let expect = expect!["Criss-Cross,Criss-Cross,Diagonal Running Man,Diagonal Running Man,Double RM,Double RM,Gangsta Hop,Gangsta Hop,Gangsta RM,Happy Feet,Idle,Idle Side,Kicking Reverse RM,Kicking Reverse RM,Knee Up,Open RM on Heels,Open RM on Heels,Open Running Man,Open Running Man,Pendulum,Pendulum,RM on Heels,RM on Heels,Reverse RM,Reverse RM,Running Man,Running Man,Simple Heels,Simple Heels,T-Step,T-Step,V-Step,V-Step,Yeah!"];
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
