mod intern;
mod public;
mod web_utils;

#[cfg(test)]
mod test_utils;

pub use public::*;

use intern::pose_db::LimbPositionDatabase;
use std::cell::RefCell;

/// Singleton internal state, shared between `Tracker` instances that run in the
/// same JS worker thread.
struct State {
    db: LimbPositionDatabase,
}
thread_local! {
    static STATE: RefCell<State> = State { db: Default::default() }.into();
}
