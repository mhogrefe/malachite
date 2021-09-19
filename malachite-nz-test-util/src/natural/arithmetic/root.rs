use crate::natural::arithmetic::sqrt::floor_inverse_binary;
use malachite_base::num::arithmetic::traits::{DivRound, Pow, PowerOf2};
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::Natural;

pub fn _floor_root_binary(x: &Natural, exp: u64) -> Natural {
    if exp == 0 {
        panic!("Cannot take 0th root");
    } else if exp == 1 || x < &Natural::TWO {
        x.clone()
    } else {
        let p = Natural::power_of_2(x.significant_bits().div_round(exp, RoundingMode::Ceiling));
        floor_inverse_binary(|x| x.pow(exp), x, &p >> 1, p)
    }
}

pub fn _ceiling_root_binary(x: &Natural, exp: u64) -> Natural {
    let floor_root = _floor_root_binary(x, exp);
    if &(&floor_root).pow(exp) == x {
        floor_root
    } else {
        floor_root + Natural::ONE
    }
}

pub fn _checked_root_binary(x: &Natural, exp: u64) -> Option<Natural> {
    let floor_root = _floor_root_binary(x, exp);
    if &(&floor_root).pow(exp) == x {
        Some(floor_root)
    } else {
        None
    }
}

pub fn _root_rem_binary(x: &Natural, exp: u64) -> (Natural, Natural) {
    let floor_root = _floor_root_binary(x, exp);
    let rem = x - (&floor_root).pow(exp);
    (floor_root, rem)
}
