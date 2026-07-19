use malachite_base::num::basic::integers::PrimitiveInt;
use std::hint::black_box;

// A `const` item and two `const { .. }` blocks that all compute `u64::WIDTH - 1`: all flagged.
const WIDTH_MINUS_1: u64 = u64::WIDTH - 1;

fn a(x: u64) -> u64 {
    x >> const { u64::WIDTH - 1 }
}

fn b(x: u64) -> u64 {
    x & const { u64::WIDTH - 1 }
}

// Same text `LIMIT - 1`, but each `LIMIT` is a different scope-local constant, so the two blocks
// have different values and are NOT flagged.
fn c(x: u64) -> u64 {
    const LIMIT: u64 = 10;
    x + const { LIMIT - 1 }
}

fn d(x: u64) -> u64 {
    const LIMIT: u64 = 20;
    x + const { LIMIT - 1 }
}

fn main() {
    let _ = black_box((WIDTH_MINUS_1, a(1), b(2), c(3), d(4)));
}
