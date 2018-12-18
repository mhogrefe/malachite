use common::DemoBenchRegistry;
use malachite_nz::integer::Integer;
use std::iter::repeat;
use std::u32;

pub mod and;
pub mod and_i32;
pub mod and_natural;
pub mod and_u32;
pub mod assign_bit;
pub mod checked_count_ones;
pub mod checked_count_zeros;
pub mod checked_hamming_distance;
pub mod checked_hamming_distance_i32;
pub mod checked_hamming_distance_natural;
pub mod checked_hamming_distance_u32;
pub mod clear_bit;
pub mod flip_bit;
pub mod get_bit;
pub mod index_of_next_false_bit;
pub mod index_of_next_true_bit;
pub mod not;
pub mod or;
pub mod or_i32;
pub mod or_natural;
pub mod or_u32;
pub mod set_bit;
pub mod significant_bits;
pub mod trailing_zeros;
pub mod xor;
pub mod xor_i32;
pub mod xor_u32;

pub(crate) fn integer_op_bits(
    bit_fn: &Fn(bool, bool) -> bool,
    x: &Integer,
    y: &Integer,
) -> Integer {
    let x_negative = *x < 0;
    let y_negative = *y < 0;
    let bit_zip: Box<Iterator<Item = (bool, bool)>> =
        if x.twos_complement_bits().count() >= y.twos_complement_bits().count() {
            Box::new(
                x.twos_complement_bits()
                    .zip(y.twos_complement_bits().chain(repeat(y_negative))),
            )
        } else {
            Box::new(
                x.twos_complement_bits()
                    .chain(repeat(x_negative))
                    .zip(y.twos_complement_bits()),
            )
        };
    let mut and_bits = Vec::new();
    for (b, c) in bit_zip {
        and_bits.push(bit_fn(b, c));
    }
    Integer::from_twos_complement_bits_asc(&and_bits)
}

pub(crate) fn integer_op_limbs(limb_fn: &Fn(u32, u32) -> u32, x: &Integer, y: &Integer) -> Integer {
    let x_extension = if *x < 0 { u32::MAX } else { 0 };
    let y_extension = if *y < 0 { u32::MAX } else { 0 };
    let limb_zip: Box<Iterator<Item = (u32, u32)>> =
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
    let mut and_limbs = Vec::new();
    for (x, y) in limb_zip {
        and_limbs.push(limb_fn(x, y));
    }
    Integer::from_owned_twos_complement_limbs_asc(and_limbs)
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    and::register(registry);
    and_i32::register(registry);
    and_natural::register(registry);
    and_u32::register(registry);
    assign_bit::register(registry);
    checked_count_ones::register(registry);
    checked_count_zeros::register(registry);
    checked_hamming_distance::register(registry);
    checked_hamming_distance_i32::register(registry);
    checked_hamming_distance_natural::register(registry);
    checked_hamming_distance_u32::register(registry);
    clear_bit::register(registry);
    flip_bit::register(registry);
    get_bit::register(registry);
    index_of_next_false_bit::register(registry);
    index_of_next_true_bit::register(registry);
    not::register(registry);
    or::register(registry);
    or_natural::register(registry);
    or_i32::register(registry);
    or_u32::register(registry);
    set_bit::register(registry);
    significant_bits::register(registry);
    trailing_zeros::register(registry);
    xor::register(registry);
    xor_i32::register(registry);
    xor_u32::register(registry);
}
