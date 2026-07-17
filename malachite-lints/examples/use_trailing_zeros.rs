use malachite_base::num::arithmetic::traits::Parity;

// Strips trailing zeros one at a time while decrementing a counter: flagged.
fn flagged_with_counter() -> (u64, i64) {
    let mut x = std::hint::black_box(48u64);
    let mut k = std::hint::black_box(10i64);
    while x.even() {
        x >>= 1;
        k -= 1;
    }
    (x, k)
}

// Strips trailing zeros with no counter: flagged.
fn flagged_no_counter() -> u64 {
    let mut x = std::hint::black_box(48u64);
    while x.even() {
        x >>= 1;
    }
    x
}

// Shifts by 2 rather than 1, so it is not a trailing-zeros strip: fine.
fn shift_by_two() -> u64 {
    let mut x = std::hint::black_box(48u64);
    while x.even() {
        x >>= 2;
    }
    x
}

// Does extra work each iteration, so it is more than a bit-strip: fine.
fn extra_work() -> (u64, u64) {
    let mut x = std::hint::black_box(48u64);
    let mut acc = std::hint::black_box(0u64);
    while x.even() {
        x >>= 1;
        acc += x;
    }
    (x, acc)
}

// The second statement mutates the tested integer itself, not a separate counter: fine.
fn mutates_x() -> u64 {
    let mut x = std::hint::black_box(48u64);
    while x.even() {
        x >>= 1;
        x += 1;
    }
    x
}

// The condition is not a parity test: fine.
fn not_parity() -> u64 {
    let mut x = std::hint::black_box(48u64);
    while x > 1 {
        x >>= 1;
    }
    x
}

fn main() {
    let _ = flagged_with_counter();
    let _ = flagged_no_counter();
    let _ = shift_by_two();
    let _ = extra_work();
    let _ = mutates_x();
    let _ = not_parity();
}
