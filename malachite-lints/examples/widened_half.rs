use malachite_base::num::conversion::traits::SplitInHalf;

fn main() {
    let a: u128 = 123;
    let b: u128 = 456;
    let e: u128 = 789;
    // A half converted straight back: flagged.
    let _ = u128::from(a.upper_half()) * 3;
    let _: u128 = e.lower_half().into();
    // Both halves only ever widened again: flagged.
    let (a1, a0) = a.split_in_half();
    let _ = u128::from(a1) * u128::from(a0);
    // A single half, only ever widened again: flagged.
    let b1 = b.upper_half();
    let _ = u128::from(b1) + u128::from(b1);
    // One half used narrow: fine.
    let (c1, c0) = a.split_in_half();
    let _ = c1.wrapping_mul(c1);
    let _ = u128::from(c0);
    // Widened to a different type: fine.
    let x: u64 = 7;
    let (x1, x0) = x.split_in_half();
    let _ = u128::from(x1) + u128::from(x0);
    // Used narrow directly: fine.
    let _ = a.upper_half().wrapping_add(1);
}
