use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::basic::traits::Two;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

fn main() {
    let x = const { Natural::const_from(100) };
    let i = const { Integer::const_from_signed(-101) };
    // Parity via `% 2` compared with 0 or 1: flagged.
    let _ = &x % Natural::TWO == 0u32;
    let _ = &i % Integer::TWO != 0i32;
    // divisible_by(2): flagged.
    let _ = (&x).divisible_by(Natural::TWO);
    // Other moduli or comparands: fine.
    let three = const { Natural::const_from(3) };
    let _ = &x % &three == 0u32;
    let _ = (&x).divisible_by(&three);
}
