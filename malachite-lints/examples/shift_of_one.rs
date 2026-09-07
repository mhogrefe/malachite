use malachite_base::num::basic::traits::One;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

fn g(x: &Natural, k: u64) -> (Natural, bool, Integer, Rational, Natural) {
    // `(Natural::ONE << n) - 1`: a low-bit mask of a bignum: flagged.
    let a = (Natural::ONE << k) - Natural::ONE;
    // `x & (Natural::ONE << n) != 0`: a bit test of a bignum: flagged.
    let b = x & (Natural::ONE << k) != 0u32;
    // `Integer::ONE << n`: a bignum power of two: flagged.
    let c = Integer::ONE << k;
    // `Rational::ONE << n`: also a power of two: flagged (`Rational` has `PowerOf2`).
    let d = Rational::ONE << k;
    // a constant shift amount: fine, as for the primitives.
    let e = Natural::ONE << 100u32;
    (a, b, c, d, e)
}

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
    // a local bound to a constant expression is followed to its initializer, so also fine.
    let amount = 64 - 1;
    let m = 1u64 << amount;
    (a, b, c, d, e, g, h, i, m)
}

fn main() {
    let _ = f(std::hint::black_box(5), std::hint::black_box(3));
    let _ = g(
        &Natural::from(std::hint::black_box(5u32)),
        std::hint::black_box(3),
    );
}
