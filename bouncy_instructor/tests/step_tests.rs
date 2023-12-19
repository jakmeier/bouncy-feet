use expect_test::expect;

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

    let expect = expect!["Idle,Idle Side,Pendulum,Running Man,Running Man on Heels,V-Step"];
    expect.assert_eq(&step_names.join(","));
}
