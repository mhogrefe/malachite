use malachite_base::num::basic::integers::PrimitiveInt;
use std::hint::black_box;

const W: u64 = 64;

fn demo(x: u64) {
    // derived from a trait associated const: flagged (declare a `const`).
    let a = u64::WIDTH - 5;
    // derived from a module `const`: flagged.
    let b = W - 5;
    // a bare literal is already as clear as it gets: fine.
    let c = 5u64;
    // a bare path to an existing constant would just be a rename: fine.
    let d = W;
    // a literal computation names no constant: fine.
    let e = 3 + 5;
    // `mut` cannot be a `const`: fine.
    let mut f = W - 5;
    f += 1;
    // depends on a runtime value: fine.
    let g = x - 5;
    black_box((a, b, c, d, e, f, g));
}

fn main() {
    demo(black_box(100));
}
