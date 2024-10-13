//! Linear feedback shift register for pseudo randomness.
//!
//! This is done in-house to have a minimal footprint on code size, and keep
/// platform independence.
use std::sync::atomic::{AtomicU32, Ordering};

// global seed
static SEED: AtomicU32 = AtomicU32::new(0);

pub(crate) fn next() -> u32 {
    init();
    // Polynomial for 32-bit with maximal LFSR feedback length, as listed among
    // many others on http://users.ece.cmu.edu/~koopman/lfsr/
    // const POLYNOMIAL: u32 = 0x8200_4040;
    // x^31 + x^25 + x^14 + x^6 + 1
    let prev = SEED.load(Ordering::SeqCst);
    let high_bit = ((prev >> 31) ^ (prev >> 25) ^ (prev >> 14) ^ (prev >> 6)) << 31;
    let next = (prev >> 1) | high_bit;
    SEED.store(next, Ordering::SeqCst);

    next
}

fn init() {
    if SEED.load(Ordering::SeqCst) == 0 {
        // unwrap impossible: system clock drift cannot produce time < UNIX_EPOCH
        let t = std::time::UNIX_EPOCH.elapsed().unwrap();
        let seed = (t.as_micros() & u32::MAX as u128) as u32;
        _ = SEED.compare_exchange(0, seed, Ordering::SeqCst, Ordering::SeqCst);
    }
}

/// Weak random id
pub fn random_id() -> String {
    let random_num = next();
    // hex string
    format!("{random_num:08x}")
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    #[test]
    fn test_deterministic_values() {
        // this test is mostly to show the first few values are reasonably
        // distributed at a first glance by human

        super::SEED.store(4230144649, std::sync::atomic::Ordering::SeqCst);
        let nums: Vec<_> = std::iter::repeat_with(super::random_id).take(20).collect();
        expect![[r#"
            [
                "7e117144",
                "bf08b8a2",
                "5f845c51",
                "afc22e28",
                "57e11714",
                "abf08b8a",
                "55f845c5",
                "2afc22e2",
                "157e1171",
                "8abf08b8",
                "455f845c",
                "a2afc22e",
                "d157e117",
                "68abf08b",
                "b455f845",
                "da2afc22",
                "ed157e11",
                "768abf08",
                "bb455f84",
                "dda2afc2",
            ]
        "#]]
        .assert_debug_eq(&nums);
    }
}
