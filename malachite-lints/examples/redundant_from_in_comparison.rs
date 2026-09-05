use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;

fn main() {
    let n = const { Natural::const_from(100) };
    let prec = 5u64;
    let r = -3i32;
    let three = 3u32;
    // Not comparisons: fine.
    let big = const { Integer::const_from_unsigned(30) };
    // Comparisons with a conversion from a primitive: flagged.
    let _ = big >= Integer::from(prec);
    let _ = Integer::from(prec) < big;
    let _ = n == Natural::from(prec);
    let q = const { Rational::const_from_unsigneds(1, 3) };
    let _ = q <= Rational::from(r);
    let _ = Rational::from(three) != q;
    // The argument is not a primitive: fine.
    let _ = big > Integer::from(Natural::from(three));
    let g = GaussianInteger::from(3u32);
    let h = GaussianRational::from(3u32);
    let k = 5u32;
    if g == GaussianInteger::from(k) {
        println!("gaussian");
    }
    if h != GaussianRational::from(k) {
        println!("gaussian rational");
    }
}
