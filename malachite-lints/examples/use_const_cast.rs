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
    // a bare named constant, which is a compile-time constant even without a block: flagged.
    let e = i128::from(u64::WIDTH);
    // a constant expression that was never wrapped: flagged.
    let f = u32::exact_from(u64::WIDTH - 5);
    // an `as` cast of a bare named constant: flagged.
    let g = u64::WIDTH as f64;
    // a conversion of a non-constant argument: fine.
    let h = u64::wrapping_from(black_box(5u32));
    // a literal needs no block; the compiler folds it anyway: fine.
    let i = i128::from(64u64);
    black_box((a, b, c, d, e, f, g, h, i));
}

fn main() {
    demo();
}
