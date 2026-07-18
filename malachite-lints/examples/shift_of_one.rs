use malachite_base::num::basic::traits::One;

fn f(x: u64, k: u64) -> (u64, bool, bool, u64, u64, u64, u64, u64, u64) {
    // `(1 << n) - 1`: a low-bit mask (both the literal 1 and `u64::ONE`): flagged.
    let a = (1u64 << k) - 1;
    // `x & (1 << n) != 0`: a bit test: flagged.
    let b = x & (u64::ONE << k) != 0;
    // `x & (1 << n) == 0`: the negated bit test: flagged.
    let c = x & (1u64 << k) == 0;
    // a bare `1 << n`: a power of two: flagged.
    let d = 1u64 << k;
    // `u64::ONE << n`: also a power of two: flagged.
    let e = u64::ONE << k;
    // `(1 << n) - 2` is not a mask, so the shift is still a power of two: flagged.
    let g = (1u64 << k) - 2;
    // shifting something other than one: fine.
    let h = x << k;
    // a constant shift amount folds at compile time, so the raw shift is preferable: fine.
    let i = 1u64 << 5;
    // a local bound to a const expression is followed to its initializer, so also fine.
    const W: u64 = 64;
    let amount = W - 1;
    let m = 1u64 << amount;
    (a, b, c, d, e, g, h, i, m)
}

fn main() {
    let _ = f(std::hint::black_box(5), std::hint::black_box(3));
}
