use std::iter::repeat;

use malachite_base::comparison::Max;
use malachite_base::num::logic::traits::{BitConvertible, BitIterable};
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;

use common::DemoBenchRegistry;

pub mod and;
pub mod assign_bit;
pub mod assign_bits;
pub mod bits;
pub mod checked_count_ones;
pub mod checked_count_zeros;
pub mod checked_hamming_distance;
pub mod clear_bit;
pub mod flip_bit;
pub mod from_bits;
pub mod get_bit;
pub mod get_bits;
pub mod index_of_next_false_bit;
pub mod index_of_next_true_bit;
pub mod not;
pub mod or;
pub mod set_bit;
pub mod significant_bits;
pub mod to_bits;
pub mod trailing_zeros;
pub mod xor;

fn integer_op_bits(bit_fn: &dyn Fn(bool, bool) -> bool, x: &Integer, y: &Integer) -> Integer {
    let x_negative = *x < 0;
    let y_negative = *y < 0;
    let bit_zip: Box<dyn Iterator<Item = (bool, bool)>> = if x.bits().count() >= y.bits().count() {
        Box::new(x.bits().zip(y.bits().chain(repeat(y_negative))))
    } else {
        Box::new(x.bits().chain(repeat(x_negative)).zip(y.bits()))
    };
    let mut bits = Vec::new();
    for (b, c) in bit_zip {
        bits.push(bit_fn(b, c));
    }
    Integer::from_bits_asc(&bits)
}

fn integer_op_limbs(limb_fn: &dyn Fn(Limb, Limb) -> Limb, x: &Integer, y: &Integer) -> Integer {
    let x_extension = if *x < 0 { Limb::MAX } else { 0 };
    let y_extension = if *y < 0 { Limb::MAX } else { 0 };
    let limb_zip: Box<dyn Iterator<Item = (Limb, Limb)>> =
        if x.twos_complement_limbs().count() >= y.twos_complement_limbs().count() {
            Box::new(
                x.twos_complement_limbs()
                    .zip(y.twos_complement_limbs().chain(repeat(y_extension))),
            )
        } else {
            Box::new(
                x.twos_complement_limbs()
                    .chain(repeat(x_extension))
                    .zip(y.twos_complement_limbs()),
            )
        };
    let mut limbs = Vec::new();
    for (x, y) in limb_zip {
        limbs.push(limb_fn(x, y));
    }
    Integer::from_owned_twos_complement_limbs_asc(limbs)
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    and::register(registry);
    assign_bit::register(registry);
    assign_bits::register(registry);
    bits::register(registry);
    checked_count_ones::register(registry);
    checked_count_zeros::register(registry);
    checked_hamming_distance::register(registry);
    clear_bit::register(registry);
    flip_bit::register(registry);
    from_bits::register(registry);
    get_bit::register(registry);
    get_bits::register(registry);
    index_of_next_false_bit::register(registry);
    index_of_next_true_bit::register(registry);
    not::register(registry);
    or::register(registry);
    set_bit::register(registry);
    to_bits::register(registry);
    significant_bits::register(registry);
    trailing_zeros::register(registry);
    xor::register(registry);
}
