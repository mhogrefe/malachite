use malachite_base::num::arithmetic::traits::{Parity, WrappingMulAssign};
use malachite_base::num::logic::traits::BitAccess;

// Reads the low bit with `& 1` and shifts right: flagged.
fn flagged_and_one() -> u64 {
    let mut x = std::hint::black_box(13u64);
    let mut count = 0;
    while x != 0 {
        count += x & 1;
        x >>= 1;
    }
    count
}

// Tests the low bit with `odd()`, with the condition written `exp > 0`: flagged.
fn flagged_odd() -> u64 {
    let mut exp = std::hint::black_box(13u64);
    let mut result = 1u64;
    let mut base = 3u64;
    while exp > 0 {
        if exp.odd() {
            result.wrapping_mul_assign(base);
        }
        base.wrapping_mul_assign(base);
        exp >>= 1;
    }
    result
}

// Tests the low bit with `get_bit(0)`, with the condition written `0 != x`: flagged.
fn flagged_get_bit() -> u32 {
    let mut x = std::hint::black_box(13u32);
    let mut count = 0;
    while 0 != x {
        if x.get_bit(0) {
            count += 1;
        }
        x >>= 1;
    }
    count
}

// Uses `x` for more than its low bit: fine.
fn uses_x_otherwise() -> u64 {
    let mut x = std::hint::black_box(13u64);
    let mut sum = 0;
    while x != 0 {
        if x.odd() {
            sum += x;
        }
        x >>= 1;
    }
    sum
}

// Shifts by 2: fine.
fn shifts_by_two() -> u64 {
    let mut x = std::hint::black_box(13u64);
    let mut count = 0;
    while x != 0 {
        if x.even() {
            count += 1;
        }
        x >>= 2;
    }
    count
}

// Signed, where an arithmetic shift never takes a negative value to zero: fine.
fn signed() -> i64 {
    let mut x = std::hint::black_box(13i64);
    let mut count = 0;
    while x != 0 {
        if x.odd() {
            count += 1;
        }
        x >>= 1;
    }
    count
}

// Never reads the low bit: fine.
fn no_low_bit_read() -> u64 {
    let mut x = std::hint::black_box(13u64);
    let mut steps = 0;
    while x != 0 {
        steps += 1;
        x >>= 1;
    }
    steps
}

// The shift is not the last statement: fine.
fn shift_not_last() -> u64 {
    let mut x = std::hint::black_box(13u64);
    let mut count = 0;
    while x != 0 {
        x >>= 1;
        if x.odd() {
            count += 1;
        }
    }
    count
}

fn main() {
    let _ = flagged_and_one();
    let _ = flagged_odd();
    let _ = flagged_get_bit();
    let _ = uses_x_otherwise();
    let _ = shifts_by_two();
    let _ = signed();
    let _ = no_low_bit_read();
    let _ = shift_not_last();
}
