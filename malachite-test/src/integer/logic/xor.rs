use malachite_nz::integer::Integer;
use std::iter::repeat;
use std::u32;

pub fn integer_xor_alt_1(x: &Integer, y: &Integer) -> Integer {
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
    let mut or_bits = Vec::new();
    for (b, c) in bit_zip {
        or_bits.push(b ^ c);
    }
    Integer::from_twos_complement_bits_asc(&or_bits)
}

pub fn integer_xor_alt_2(x: &Integer, y: &Integer) -> Integer {
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
    let mut or_limbs = Vec::new();
    for (x, y) in limb_zip {
        or_limbs.push(x ^ y);
    }
    Integer::from_owned_twos_complement_limbs_asc(or_limbs)
}
