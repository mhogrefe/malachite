use core::cmp::Ordering::*;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

const A: Natural = Natural::const_from(3);
const B: Natural = Natural::const_from(7);
const C: Natural = Natural::const_from(20);
const D: Natural = Natural::const_from(5);
const I: Integer = Integer::const_from_signed(-3);
const J: Integer = Integer::const_from_signed(7);
const K: Integer = Integer::const_from_signed(-20);
const P: Rational = Rational::const_from_unsigned(3);
const Q: Rational = Rational::const_from_unsigned(7);

fn main() {
    // A product compared with a value, by operator and by method: flagged.
    let _ = &A * &B < C;
    let _ = (&A * &B).cmp(&C);
    let _ = (&A * &B).partial_cmp(&C) == Some(Greater);
    // The product on the right: flagged.
    let _ = C.cmp(&(&A * &B));
    // Two products: flagged.
    let _ = (&A * &B).cmp(&(&C * &D));
    // Against a negated value or a negated product: flagged for Integer, which has the `_neg`
    // functions.
    let _ = &I * &J < -&K;
    let _ = (&I * &J).cmp(&-(&J * &K));
    // Rational: flagged.
    let _ = &P * &Q > P;
    // A Natural product against a negated value: not flagged, since Natural has no `_neg`
    // functions (and a negated Natural is an Integer).
    let _ = Integer::from(&A * &B) > -Integer::from(&C);
    // A Natural product compared with an Integer: not flagged, since the types differ.
    let _ = &A * &B > K;
    // Primitive integers: not flagged.
    let _ = 3u32 * 7 < 20;
    // An opted-out site: not flagged.
    #[cfg_attr(dylint_lib = "malachite_lints", allow(use_cmp_product))]
    let _ = &A * &B == C;
}
