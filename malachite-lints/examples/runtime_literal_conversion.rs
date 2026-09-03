// `twenty` exists to show a `const_from*` call inside a `const fn`; it is never called.
#![allow(dead_code)]

use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_q::gaussian_rational::GaussianRational;

const TEN: Natural = Natural::const_from(10);

fn main() {
    // Converting a literal at runtime: flagged.
    let _ = Natural::from(100u32);
    let _ = Integer::from(-100i32);
    let _ = Rational::from(7u32);
    // `const_from*` outside a const context still runs at runtime: flagged.
    let _ = Natural::const_from(10);
    // Wrapped in a `const` block or bound to a named `const`: fine.
    let _ = const { Natural::const_from(10) };
    let _ = TEN;
    // A non-literal argument: fine.
    let k = 100u32;
    let _ = Natural::from(k);
    // A literal too large for a 32-bit limb has no portable `const_from*`: fine.
    let _ = Natural::from(5000000000u64);
    // Fraction constructors with literal arguments: flagged, including the `const` version at
    // runtime (whose naive Euclidean gcd is slower than `from_unsigneds`' runtime gcd).
    let _ = Rational::from_unsigneds(3u32, 4u32);
    let _ = Rational::from_signeds(-3i32, 4i32);
    let _ = Rational::const_from_unsigneds(3, 4);
    let _ = const { Rational::const_from_signeds(-3, 4) };
    // The Gaussian types have no `const_from*` yet: not flagged.
    let _ = GaussianInteger::from(100u32);
    let _ = GaussianRational::from(-100i32);
}

// Inside a `const fn`, a `const_from*` call is evaluated at compile time whenever the caller is:
// fine.
const fn twenty() -> Natural {
    Natural::const_from(20)
}
