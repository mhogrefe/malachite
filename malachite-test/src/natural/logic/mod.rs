use std::iter::repeat;

use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use common::DemoBenchRegistry;

pub mod and;
pub mod assign_bit;
pub mod clear_bit;
pub mod count_ones;
pub mod flip_bit;
pub mod get_bit;
pub mod get_bits;
pub mod hamming_distance;
pub mod index_of_next_false_bit;
pub mod index_of_next_true_bit;
pub mod limb_count;
pub mod not;
pub mod or;
pub mod set_bit;
pub mod significant_bits;
pub mod trailing_zeros;
pub mod xor;

fn natural_op_bits(bit_fn: &dyn Fn(bool, bool) -> bool, x: &Natural, y: &Natural) -> Natural {
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

fn natural_op_limbs(limb_fn: &dyn Fn(Limb, Limb) -> Limb, x: &Natural, y: &Natural) -> Natural {
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

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    and::register(registry);
    assign_bit::register(registry);
    clear_bit::register(registry);
    count_ones::register(registry);
    flip_bit::register(registry);
    get_bit::register(registry);
    get_bits::register(registry);
    hamming_distance::register(registry);
    index_of_next_false_bit::register(registry);
    index_of_next_true_bit::register(registry);
    limb_count::register(registry);
    not::register(registry);
    or::register(registry);
    set_bit::register(registry);
    significant_bits::register(registry);
    trailing_zeros::register(registry);
    xor::register(registry);
}
