use core::cmp::Ordering::*;
use malachite_base::num::basic::traits::{I, NegativeOne, One, Two, Zero};
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_q::gaussian_rational::GaussianRational;

const X: Rational = Rational::const_from_signeds(22, 7);
const N: Natural = Natural::const_from(10);

fn main() {
    let x = X;
    let n = N;
    let k = 10u32;
    // Operator comparison with a named constant: flagged. (The bodies are kept distinct so that
    // `collapse_adjacent_ifs` does not also fire.)
    if x == Rational::ONE {
        println!("one");
        return;
    }
    if x > Rational::TWO {
        println!("big");
        return;
    }
    if x < Rational::NEGATIVE_ONE {
        println!("small");
        return;
    }
    if n == Natural::ZERO {
        println!("zero");
        return;
    }
    // Comparison methods with a named constant: flagged.
    match x.cmp(&Rational::ONE) {
        Equal => (),
        Greater => (),
        Less => (),
    }
    let _ = x.partial_cmp(&Rational::TWO);
    // Comparison methods with `from(primitive)`: flagged.
    let _ = n.cmp(&Natural::from(k));
    // Comparing with the primitive directly: fine.
    if x == 1u32 {
        return;
    }
    let _ = x.partial_cmp(&1u32);
    // Comparing two general bignums: fine.
    let y = X;
    let _ = x.cmp(&y);
    if x == y {}
    let g = GaussianInteger::from(3u32);
    let h = GaussianRational::from(3u32);
    // The Gaussian types compare directly with primitives too.
    if g == GaussianInteger::ONE {
        println!("gaussian one");
        return;
    }
    if h != GaussianRational::ZERO {
        println!("gaussian nonzero");
        return;
    }
    // No primitive equals `I`, so this is not flagged.
    if g == GaussianInteger::I {
        println!("i");
    }
}
