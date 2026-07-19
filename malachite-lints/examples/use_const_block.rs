use malachite_base::num::basic::integers::PrimitiveInt;
use std::hint::black_box;

const FLAG: bool = true;

fn demo(x: u64) {
    // an integer constant island inside a runtime shift: flagged.
    let a = x >> (u64::WIDTH - 1);
    // a pure-`bool` constant island inside a runtime condition: flagged.
    let b = !FLAG || x > 5;
    // references a runtime local, so not a compile-time constant: fine.
    let c = x - 1;
    // a bare named constant is atomic: fine.
    let d = x + u64::WIDTH;
    // a literal computation names no constant (the compiler folds it): fine.
    let e = x + (3 + 5);
    // already inside a `const { .. }` block: fine.
    let f = x >> const { u64::WIDTH - 1 };
    black_box((a, b, c, d, e, f));
}

fn main() {
    demo(black_box(100));
}
