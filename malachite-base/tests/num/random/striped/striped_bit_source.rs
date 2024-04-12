// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::random::striped::StripedBitSource;
use malachite_base::random::EXAMPLE_SEED;

const BIT_STRING_LENGTH: usize = 80;

fn generate_string(bit_source: &mut StripedBitSource) -> String {
    let mut string = String::with_capacity(BIT_STRING_LENGTH);
    for bit in bit_source.take(BIT_STRING_LENGTH) {
        if bit {
            string.push('1');
        } else {
            string.push('0');
        }
    }
    string
}

fn striped_bit_source_helper(m_numerator: u64, m_denominator: u64, bit_string: &str) {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, m_numerator, m_denominator);
    assert_eq!(generate_string(&mut bit_source), bit_string);
}

#[test]
pub fn test_striped_bit_source() {
    striped_bit_source_helper(
        4,
        1,
        "00000001011001100000000111100000000011111111110000111011000000000000111111111111",
    );
    striped_bit_source_helper(
        10,
        1,
        "00011111111111000000011111111111111000000000001111111111000000000000000000011111",
    );
    striped_bit_source_helper(
        1000000,
        1,
        "00000000000000000000000000000000000000000000000000000000000000000000000000000000",
    );

    striped_bit_source_helper(
        2,
        1,
        "00110011000110101010010110100100000110000111010111100100000000111000100100011000",
    );

    striped_bit_source_helper(
        5,
        4,
        "01010010110101001100101101011010101010001010101011010010101010010001101000010000",
    );
    striped_bit_source_helper(
        11,
        10,
        "01010101010101101000111001010101010101111010101010101010110101011010101010101010",
    );
}

#[test]
#[should_panic]
fn new_fail_1() {
    StripedBitSource::new(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn new_fail_2() {
    StripedBitSource::new(EXAMPLE_SEED, 1, 1);
}

#[test]
#[should_panic]
fn new_fail_3() {
    StripedBitSource::new(EXAMPLE_SEED, 2, 3);
}

#[test]
pub fn test_end_block() {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, 1000000, 1);
    let mut strings = Vec::with_capacity(5);
    for _ in 0..5 {
        strings.push(generate_string(&mut bit_source));
        bit_source.end_block();
    }
    assert_eq!(
        strings,
        &[
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "11111111111111111111111111111111111111111111111111111111111111111111111111111111",
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000"
        ]
    );
}
