#![doc = include_str!("../README.md")]
extern crate near_o11y;
extern crate once_cell;

use near_o11y::metrics::{try_create_int_counter, try_create_int_gauge, IntCounter, IntGauge};
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU64, Ordering};

/// An indicate for dynamic config changes
pub static DYN_CONFIG_CHANGE: Lazy<IntCounter> = Lazy::new(|| {
    try_create_int_counter(
        "near_dynamic_config_changes",
        "Total number of changes on dynamic configure",
    )
    .unwrap()
});
/// An indicate for expected shutdown
pub static EXPECTED_SHUTDOWN_BLOCK: Lazy<IntGauge> = Lazy::new(|| {
    try_create_int_gauge("near_block_expected_shutdown", "The block height expected shutdown")
        .unwrap()
});

// NOTE: AtomicU64 is the same unit as BlockHeight, and use to store the expected blockheight to
// shutdown
pub static EXPECTED_SHUTDOWN_AT: AtomicU64 = AtomicU64::new(0);

pub fn reload(expected_shutdown: Option<u64>) {
    if let Some(expected_shutdown) = expected_shutdown {
        EXPECTED_SHUTDOWN_AT.store(expected_shutdown, Ordering::Relaxed);
        EXPECTED_SHUTDOWN_BLOCK.set(expected_shutdown as i64);
    } else {
        EXPECTED_SHUTDOWN_AT.store(0, Ordering::Relaxed);
        EXPECTED_SHUTDOWN_BLOCK.set(0i64);
    }
    DYN_CONFIG_CHANGE.inc();
}
