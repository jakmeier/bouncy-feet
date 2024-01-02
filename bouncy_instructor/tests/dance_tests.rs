use std::collections::HashSet;

mod common;

#[test]
fn test_unique_dance_id() {
    common::load_static_files();
    let steps = bouncy_instructor::dances();
    let mut unique_ids = HashSet::<String>::new();
    let mut duplicates = vec![];
    for step in &steps {
        if !unique_ids.insert(step.id()) {
            duplicates.push(step.id());
        }
    }

    assert!(
        unique_ids.len() > 0,
        "No static dances found after loading."
    );
    assert!(
        steps.len() == unique_ids.len(),
        "Duplicate step ID detected. {duplicates:?}"
    )
}
