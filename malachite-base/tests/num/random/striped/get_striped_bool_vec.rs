// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::random::striped::{get_striped_bool_vec, StripedBitSource};
use malachite_base::random::EXAMPLE_SEED;

pub(crate) fn bool_slice_to_string(bs: &[bool]) -> String {
    bs.iter().map(|&b| if b { '1' } else { '0' }).collect()
}

fn get_striped_bool_vec_helper(m_numerator: u64, m_denominator: u64, len: u64, out: &str) {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, m_numerator, m_denominator);
    let bits = bool_slice_to_string(&get_striped_bool_vec(&mut bit_source, len));
    assert_eq!(bits, out);
}

#[test]
fn test_get_striped_bool_vec() {
    get_striped_bool_vec_helper(2, 1, 0, "");
    get_striped_bool_vec_helper(
        2,
        1,
        50,
        "00110011000110101010010110100100000110000111010111",
    );
    get_striped_bool_vec_helper(
        10,
        1,
        50,
        "00011111111111000000011111111111111000000000001111",
    );
    get_striped_bool_vec_helper(
        11,
        10,
        50,
        "01010101010101101000111001010101010101111010101010",
    );
}
