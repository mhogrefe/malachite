use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{ExtendedGcd, ModInverse};
use malachite_base::num::conversion::traits::ExactFrom;
use crate::natural::InnerNatural::Small;
use crate::natural::Natural;

fn mod_inverse_simple_helper(x: Natural, m: Natural) -> Option<Natural> {
    let (gcd, _, inverse) = (&m).extended_gcd(x);
    if gcd != 1u32 {
        None
    } else {
        Some(if inverse < 0u32 {
            Natural::exact_from(inverse + Integer::from(m))
        } else {
            Natural::exact_from(inverse)
        })
    }
}

pub fn mod_inverse_simple(n: Natural, m: Natural) -> Option<Natural> {
    assert_ne!(n, 0u32);
    assert!(n < m);
    match (n, m) {
        (x @ natural_one!(), _) => Some(x),
        (Natural(Small(x)), Natural(Small(y))) => x.mod_inverse(y).map(Natural::from),
        (a, b) => mod_inverse_simple_helper(a, b),
    }
}
