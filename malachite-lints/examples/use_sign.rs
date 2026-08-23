use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;

fn main() {
    let x = std::hint::black_box(-5i32);
    // Comparing an integer with zero: flagged.
    let _ = x.cmp(&0);
    // A suffixed zero on an unsigned value: flagged.
    let n = std::hint::black_box(3u64);
    let _ = n.cmp(&0u64);
    // A bignum compared with its ZERO constant: flagged.
    let i = Integer::from(x);
    let _ = i.cmp(&Integer::ZERO);
    // Comparing with a nonzero value: fine.
    let _ = x.cmp(&1);
    // The suggested form: fine.
    let _ = x.sign();
}
