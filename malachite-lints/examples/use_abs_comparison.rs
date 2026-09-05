use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_float::Float;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

fn main() {
    let i = const { Integer::const_from_signed(-100) };
    let j = const { Integer::const_from_signed(7) };
    let k = const { Integer::const_from_signed(9) };
    let n = const { Natural::const_from(3) };
    let f = Float::from(1.5f64);
    let q = const { Rational::const_from_signeds(-22, 7) };
    // `abs()` compared with a nonnegative literal: flagged, use the `*_abs` comparison.
    if (&i).abs() <= 100u32 {
        println!("small");
        return;
    }
    if (&f).abs() < 2u32 {
        println!("small float");
        return;
    }
    if (&q).abs() > 3u32 {
        println!("big rational");
        return;
    }
    // The `abs()` may be on the right, which flips the comparison: flagged.
    if 5u32 < (&i).abs() {
        println!("more than five");
        return;
    }
    // Both sides `abs()`: flagged.
    if (&i).abs() >= (&j).abs() {
        println!("larger");
        return;
    }
    // Against a `Natural`, which is never negative: flagged.
    if (&i).abs() == n {
        println!("equal");
        return;
    }
    // The method spellings: flagged.
    let _ = (&i).abs().partial_cmp(&5u32);
    let _ = (&i).abs().cmp(&(&j).abs());
    let _ = (&i).abs().lt(&5u32);
    // A by-value `abs()`: flagged.
    if k.abs() != 9u32 {
        println!("not nine");
        return;
    }
    // Against a signed value, which may be negative: not flagged, since `le_abs` would compare
    // the magnitudes of both sides.
    let m = const { Integer::const_from_signed(-5) };
    if (&i).abs() < m {
        println!("less than m");
        return;
    }
    // Already an `*_abs` comparison: fine.
    if i.le_abs(&100u32) {
        println!("abs comparison");
    }
}
