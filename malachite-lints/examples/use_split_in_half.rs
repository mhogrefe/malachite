use malachite_base::num::conversion::traits::SplitInHalf;

fn main() {
    let t: u64 = 123;
    // Both halves of the same value: flagged.
    let hi = t.upper_half();
    let lo = t.lower_half();
    let _ = (hi, lo);
    // Only one half: fine.
    let u: u64 = 4;
    let _ = u.lower_half();
    // Halves of different values: fine.
    let a: u64 = 5;
    let b: u64 = 6;
    let _ = (a.upper_half(), b.lower_half());
    // A mutable local may change between the two calls: fine.
    let mut m: u64 = 7;
    let first = m.upper_half();
    m += 1;
    let _ = (first, m.lower_half());
    // Already split: fine.
    let s: u64 = 8;
    let (_, _) = s.split_in_half();
}
