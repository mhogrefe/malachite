// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::striped::{get_striped_unsigned_vec, StripedBitSource};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToBinaryString;

fn get_striped_unsigned_vec_helper<T: PrimitiveUnsigned>(
    m_numerator: u64,
    m_denominator: u64,
    len: u64,
    out: &[&str],
) {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, m_numerator, m_denominator);
    let xs = get_striped_unsigned_vec::<T>(&mut bit_source, len)
        .iter()
        .map(T::to_binary_string)
        .collect_vec();
    assert_eq!(xs, out);
}

#[test]
fn test_get_striped_unsigned_vec() {
    get_striped_unsigned_vec_helper::<u8>(2, 1, 0, &[]);
    get_striped_unsigned_vec_helper::<u8>(
        2,
        1,
        100,
        &[
            "11001100", "1011000", "10100101", "100101", "11000", "10101110", "100111", "11000000",
            "10010001", "11000", "11000100", "10001011", "1001",
        ],
    );
    get_striped_unsigned_vec_helper::<u8>(
        10,
        1,
        100,
        &[
            "11111000", "111111", "11100000", "11111111", "111", "11000000", "11111111", "0", "0",
            "11111000", "11111111", "11111111", "11",
        ],
    );
    get_striped_unsigned_vec_helper::<u8>(
        11,
        10,
        100,
        &[
            "10101010", "1101010", "1110001", "10101010", "11101010", "1010101", "1010101",
            "10101011", "1010101", "1010101", "1010101", "1101101", "1101",
        ],
    );
    get_striped_unsigned_vec_helper::<u64>(
        2,
        1,
        130,
        &[
            "1100000000100111101011100001100000100101101001010101100011001100",
            "1100110010001011010100110100110001011110001000001100010010001",
            "10",
        ],
    );
    get_striped_unsigned_vec_helper::<u64>(
        10,
        1,
        130,
        &[
            "11111111110000000000011111111111111000000011111111111000",
            "1100000111111111111111000000001111111111111111111111100000000000",
            "11",
        ],
    );
    get_striped_unsigned_vec_helper::<u64>(
        11,
        10,
        130,
        &[
            "1010101101010101010101011110101010101010011100010110101010101010",
            "101101010101011010110101001110101101101010101010101010101010101",
            "1",
        ],
    );
}
