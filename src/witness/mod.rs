/// The upgrade of this module from wasmer 2 to wasmer 4
/// heavily relied on (read: almost a direct copy/paste)
/// the work of the `circom-scotia` library:
///     https://github.com/lurk-lab/circom-scotia
///
/// We simply preserved the original tests and BigInt types, but
/// used all of their logic for handling the wasmer `Store`
mod witness_calculator;
pub use witness_calculator::WitnessCalculator;

mod memory;
pub(super) use memory::SafeMemory;

mod circom;
pub(super) use circom::{CircomBase, Wasm};

#[cfg(feature = "circom-2")]
pub(super) use circom::Circom2;

pub(super) use circom::Circom;

use fnv::FnvHasher;
use std::hash::Hasher;

pub(crate) fn fnv(inp: &str) -> (u32, u32) {
    let mut hasher = FnvHasher::default();
    hasher.write(inp.as_bytes());
    let h = hasher.finish();

    ((h >> 32) as u32, h as u32)
}
