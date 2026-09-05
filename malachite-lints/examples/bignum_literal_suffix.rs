use malachite_base::num::arithmetic::traits::ShrRound;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_float::Float;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

fn main() {
    let n = const { Natural::const_from(100) };
    let i = const { Integer::const_from_signed(-100) };
    let f = Float::from(1.5f64);
    // Comparing with a `u64` literal: flagged, use `u32`.
    if n == 100u64 {
        println!("hundred");
        return;
    }
    // An unsuffixed literal selects the `i32` implementation: flagged.
    if n > 3 {
        println!("big");
        return;
    }
    // A negative literal with an `i64` suffix: flagged, use `i32`.
    if i < -1i64 {
        println!("negative");
        return;
    }
    // A nonnegative literal with a signed suffix: flagged, use `u32`.
    if i > 0i32 {
        println!("positive");
        return;
    }
    // The literal may be on either side: flagged.
    if 5u8 < n {
        println!("more than five");
        return;
    }
    // Comparison methods, including the `*_abs` ones: flagged.
    let _ = n.partial_cmp(&100u64);
    let _ = f.eq(&1usize);
    let _ = n.lt(&5);
    let _ = i.le_abs(&3i64);
    let _ = f.partial_cmp_abs(&2u8);
    // Shifts by a literal: flagged.
    let _ = &n << 1u64;
    let _ = &n >> 2;
    let mut m = n.clone();
    m <<= 3usize;
    let _ = &i << -1i64;
    let _ = (&n).shr_round(1i64, Ceiling).0;
    let _ = (&f).shr_round(4u64, Floor).0;
    // Conforming literals: fine.
    if n == 100u32 && i < -1i32 && f > 1u32 {
        println!("conforming");
        return;
    }
    let _ = &n << 1u32;
    let _ = &i >> -1i32;
    let _ = (&n).shr_round(1u32, Ceiling).0;
    // Primitives are not covered, and neither are variable shift counts.
    let k = 5u64;
    let _ = &n << k;
    if k == 5 && (k << 1) == 10 {
        println!("primitive");
    }
    let _ = m;
}
