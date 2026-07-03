use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

fn main() {
    let n = const { Natural::const_from(100) };
    let i = const { Integer::const_from_signed(-100) };
    let q = const { Rational::const_from_unsigneds(1, 3) };
    // Operator comparisons with power_of_2, on either side: flagged, with type-specific advice.
    let _ = n < Natural::power_of_2(5);
    let _ = Natural::power_of_2(5) <= n;
    let _ = i == Integer::power_of_2(5);
    let _ = q >= Rational::power_of_2(-5i64);
    // Comparison methods: flagged.
    let _ = q.lt_abs(&Rational::power_of_2(-5i64));
    let _ = n.cmp(&Natural::power_of_2(5)) == Ordering::Less;
    // Not comparisons: fine.
    let _ = &n & Natural::power_of_2(5);
    // Comparing with something that is not power_of_2: fine.
    let m = const { Natural::const_from(32) };
    let _ = n < m;
}
