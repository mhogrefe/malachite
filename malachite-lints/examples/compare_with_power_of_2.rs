use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;

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
    let g = GaussianInteger::from(3u32);
    let h = GaussianRational::from(3u32);
    if g == GaussianInteger::power_of_2(10) {
        println!("gaussian");
    }
    if h != GaussianRational::power_of_2(10u64) {
        println!("gaussian rational");
    }
}
