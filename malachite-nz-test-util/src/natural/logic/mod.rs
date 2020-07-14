use std::iter::repeat;

use malachite_base::num::logic::traits::{BitConvertible, BitIterable, SignificantBits};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

pub fn natural_op_bits(bit_fn: &dyn Fn(bool, bool) -> bool, x: &Natural, y: &Natural) -> Natural {
    let bit_zip: Box<dyn Iterator<Item = (bool, bool)>> =
        if x.significant_bits() >= y.significant_bits() {
            Box::new(x.bits().zip(y.bits().chain(repeat(false))))
        } else {
            Box::new(x.bits().chain(repeat(false)).zip(y.bits()))
        };
    let mut and_bits = Vec::new();
    for (b, c) in bit_zip {
        and_bits.push(bit_fn(b, c));
    }
    Natural::from_bits_asc(&and_bits)
}

pub fn natural_op_limbs(limb_fn: &dyn Fn(Limb, Limb) -> Limb, x: &Natural, y: &Natural) -> Natural {
    let limb_zip: Box<dyn Iterator<Item = (Limb, Limb)>> = if x.limb_count() >= y.limb_count() {
        Box::new(x.limbs().zip(y.limbs().chain(repeat(0))))
    } else {
        Box::new(x.limbs().chain(repeat(0)).zip(y.limbs()))
    };
    let mut and_limbs = Vec::new();
    for (x, y) in limb_zip {
        and_limbs.push(limb_fn(x, y));
    }
    Natural::from_owned_limbs_asc(and_limbs)
}

pub mod and;
pub mod count_ones;
pub mod from_bits;
pub mod get_bit;
pub mod hamming_distance;
pub mod index_of_next_false_bit;
pub mod index_of_next_true_bit;
pub mod or;
pub mod set_bit;
pub mod to_bits;
pub mod trailing_zeros;
pub mod xor;
