use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::basic::traits::Two;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

fn main() {
    let x = const { Natural::const_from(100) };
    let i = const { Integer::const_from_signed(-101) };
    // Parity of a bignum via `% 2` compared with 0 or 1: flagged.
    let _ = &x % Natural::TWO == 0u32;
    let _ = &i % Integer::TWO != 0i32;
    // `divisible_by(2)`: flagged.
    let _ = (&x).divisible_by(Natural::TWO);
    // Primitives via `% 2`: flagged.
    let n = std::hint::black_box(7u64);
    let _ = n % 2 == 0;
    let _ = n % 2 != 0;
    // `& 1` for any integer type: flagged.
    let _ = (n & 1) == 0;
    let _ = (n & 1) == 1;
    let s = std::hint::black_box(-7i64);
    let _ = (s & 1) != 0;
    // A signed primitive `% 2 == 1` does not test oddness (the remainder can be -1): fine.
    let _ = s % 2 == 1;
}
