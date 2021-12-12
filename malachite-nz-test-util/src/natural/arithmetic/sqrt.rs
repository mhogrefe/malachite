use malachite_base::num::arithmetic::traits::{PowerOf2, ShrRound, Square};
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;

pub(crate) fn floor_inverse_binary<F: Fn(&Natural) -> Natural>(
    f: F,
    x: &Natural,
    mut low: Natural,
    mut high: Natural,
) -> Natural {
    loop {
        if high <= low {
            return low;
        }
        let mid = (&low + &high).shr_round(1, RoundingMode::Ceiling);
        match f(&mid).cmp(x) {
            Ordering::Equal => return mid,
            Ordering::Less => low = mid,
            Ordering::Greater => high = mid - Natural::ONE,
        }
    }
}

pub fn floor_sqrt_binary(x: &Natural) -> Natural {
    if x < &Natural::TWO {
        x.clone()
    } else {
        let p = Natural::power_of_2(x.significant_bits().shr_round(1, RoundingMode::Ceiling));
        floor_inverse_binary(|x| x.square(), x, &p >> 1, p)
    }
}

pub fn ceiling_sqrt_binary(x: &Natural) -> Natural {
    let floor_sqrt = floor_sqrt_binary(x);
    if &(&floor_sqrt).square() == x {
        floor_sqrt
    } else {
        floor_sqrt + Natural::ONE
    }
}

pub fn checked_sqrt_binary(x: &Natural) -> Option<Natural> {
    let floor_sqrt = floor_sqrt_binary(x);
    if &(&floor_sqrt).square() == x {
        Some(floor_sqrt)
    } else {
        None
    }
}

pub fn sqrt_rem_binary(x: &Natural) -> (Natural, Natural) {
    let floor_sqrt = floor_sqrt_binary(x);
    let rem = x - (&floor_sqrt).square();
    (floor_sqrt, rem)
}
