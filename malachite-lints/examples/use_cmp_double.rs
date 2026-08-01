use core::ops::Shl;
use malachite_base::num::basic::traits::One;
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

const X: Natural = Natural::const_from(5);
const Y: Natural = Natural::const_from(3);
const A: Integer = Integer::const_from_signed(-5);
const B: Integer = Integer::const_from_unsigned(3);

// A type with `Shl` and `Ord` but no `*_double` implementation.
#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Meters(u32);

impl Shl<u32> for Meters {
    type Output = Meters;

    fn shl(self, other: u32) -> Meters {
        Meters(self.0 << other)
    }
}

fn main() {
    // `cmp` and `partial_cmp`, and their `_abs` counterparts: flagged.
    let _ = X.cmp(&(&Y << 1));
    let _ = X.partial_cmp(&(&Y << 1));
    let _ = A.cmp_abs(&(&B << 1));
    let _ = A.partial_cmp_abs(&(&B << 1));
    // The comparison operators: flagged.
    let _ = X < &Y << 1;
    let _ = X >= &Y << 1;
    let _ = X == &Y << 1;
    // The method spellings of those operators: flagged.
    let _ = X.lt(&(&Y << 1));
    let _ = X.ge(&(&Y << 1));
    // The `_abs` predicates: flagged.
    let _ = A.lt_abs(&(&B << 1));
    let _ = A.ge_abs(&(&B << 1));
    // A shift by `T::ONE` rather than a literal: flagged.
    let _ = X.cmp(&(&Y << u64::ONE));
    // Comparing against a halved value, for the two operators that survive the flooring: flagged.
    let _ = X <= &Y >> 1;
    let _ = X > &Y >> 1;
    let _ = X.le(&(&Y >> 1));
    let _ = &Y >> 1 >= X;
    let _ = &Y >> 1 < X;
    // The doubled value on the left: flagged, with the operands swapped.
    let _ = (&X << 1u64).cmp(&Y);
    let _ = &X << 1 < Y;

    // `a < b >> 1` is not `2a < b` (b = 5, a = 2), so the strict form is left alone: fine.
    let _ = X < &Y >> 1;
    let _ = X >= &Y >> 1;
    let _ = X == &Y >> 1;
    // `|floor(b/2)|` is not `floor(|b|/2)` for negative odd `b`, so no `_abs` halving: fine.
    let _ = A.le_abs(&(&B >> 1));
    // Shifting by something other than 1 is a different comparison: fine.
    let _ = X.cmp(&(&Y << 2));
    // Doubling that is not compared against anything: fine.
    let _ = &Y << 1;
    // A comparison with no doubling at all: fine.
    let _ = X.cmp(&Y);
    // A type with no `*_double` implementation: fine.
    let _ = Meters(5).cmp(&(Meters(3) << 1));
    // A primitive, where `<< 1` is a single instruction: fine.
    let _ = 5u32.cmp(&(3u32 << 1));
}
