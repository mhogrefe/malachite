use malachite_base::num::arithmetic::traits::{PowerOf2, ShrRound, Square};
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::arithmetic::root::floor_inverse_binary;
use malachite_nz::natural::Natural;

#[doc(hidden)]
pub fn _floor_sqrt_binary(x: &Natural) -> Natural {
    if x < &Natural::TWO {
        x.clone()
    } else {
        let p = Natural::power_of_2(x.significant_bits().shr_round(1, RoundingMode::Ceiling));
        floor_inverse_binary(|x| x.square(), x, &p >> 1, p)
    }
}

#[doc(hidden)]
pub fn _ceiling_sqrt_binary(x: &Natural) -> Natural {
    let floor_sqrt = _floor_sqrt_binary(x);
    if &(&floor_sqrt).square() == x {
        floor_sqrt
    } else {
        floor_sqrt + Natural::ONE
    }
}

#[doc(hidden)]
pub fn _checked_sqrt_binary(x: &Natural) -> Option<Natural> {
    let floor_sqrt = _floor_sqrt_binary(x);
    if &(&floor_sqrt).square() == x {
        Some(floor_sqrt)
    } else {
        None
    }
}

#[doc(hidden)]
pub fn _sqrt_rem_binary(x: &Natural) -> (Natural, Natural) {
    let floor_sqrt = _floor_sqrt_binary(x);
    let rem = x - (&floor_sqrt).square();
    (floor_sqrt, rem)
}
