use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use std::hint::black_box;

fn demo() {
    // `from` of a `const { .. }` block: flagged.
    let a = i128::from(const { u64::WIDTH - 1 });
    // `exact_from` of a `const { .. }` block: flagged.
    let b = u32::exact_from(const { u64::WIDTH - 2 });
    // `wrapping_from` of a `const { .. }` block: flagged.
    let c = usize::wrapping_from(const { u64::WIDTH - 3 });
    // an `as` cast of a `const { .. }` block, including to a float: flagged.
    let d = const { u64::WIDTH - 4 } as f64;
    // a conversion of a non-constant argument: fine.
    let e = u64::wrapping_from(black_box(5u32));
    // a conversion of a bare constant (not a `const { .. }` block): fine.
    let f = i128::from(u64::WIDTH);
    black_box((a, b, c, d, e, f));
}

fn main() {
    demo();
}
