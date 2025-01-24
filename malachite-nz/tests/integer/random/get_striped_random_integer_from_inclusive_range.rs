// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::random::striped::StripedBitSource;
use malachite_base::num::random::VariableRangeGenerator;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::random::get_striped_random_integer_from_inclusive_range;
use malachite_nz::integer::Integer;
use std::str::FromStr;

fn get_striped_random_integer_from_inclusive_range_helper(
    m_numerator: u64,
    m_denominator: u64,
    a: &str,
    b: &str,
    out: &str,
) {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED.fork("bs"), m_numerator, m_denominator);
    let mut vrg = VariableRangeGenerator::new(EXAMPLE_SEED.fork("vrg"));
    let xs = (0..10)
        .map(|_| {
            get_striped_random_integer_from_inclusive_range(
                &mut bit_source,
                &mut vrg,
                Integer::from_str(a).unwrap(),
                Integer::from_str(b).unwrap(),
            )
        })
        .collect_vec();
    assert_eq!(xs.to_debug_string(), out);
}

#[test]
fn test_get_striped_random_integer_from_inclusive_range() {
    get_striped_random_integer_from_inclusive_range_helper(
        2,
        1,
        "0",
        "0",
        "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]",
    );
    get_striped_random_integer_from_inclusive_range_helper(
        2,
        1,
        "1950",
        "2019",
        "[2014, 1964, 2008, 1994, 1999, 1971, 1990, 1984, 2016, 2018]",
    );
    get_striped_random_integer_from_inclusive_range_helper(
        2,
        1,
        "-10000",
        "9000",
        "[8458, 8998, 8818, -8899, -9414, 8703, 8540, 6141, 2042, 6456]",
    );
    get_striped_random_integer_from_inclusive_range_helper(
        2,
        1,
        "-10000",
        "-1000",
        "[-8724, -9372, -8880, -9422, -8448, -9891, -8959, -8479, -1002, -4963]",
    );

    get_striped_random_integer_from_inclusive_range_helper(
        10,
        1,
        "0",
        "0",
        "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]",
    );
    get_striped_random_integer_from_inclusive_range_helper(
        10,
        1,
        "1950",
        "2019",
        "[2016, 1987, 2019, 1951, 2019, 1951, 2019, 1951, 1950, 1950]",
    );
    get_striped_random_integer_from_inclusive_range_helper(
        10,
        1,
        "-10000",
        "9000",
        "[8312, 8992, 8992, -10000, -2046, 8192, 0, 4095, 63, 2047]",
    );
    get_striped_random_integer_from_inclusive_range_helper(
        10,
        1,
        "-10000",
        "-1000",
        "[-8432, -10000, -1023, -9999, -10000, -1016, -1000, -1007, -8192, -4095]",
    );

    get_striped_random_integer_from_inclusive_range_helper(
        11,
        10,
        "0",
        "0",
        "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]",
    );
    get_striped_random_integer_from_inclusive_range_helper(
        11,
        10,
        "1950",
        "2019",
        "[1992, 1962, 2005, 2018, 1962, 2005, 2005, 2005, 1951, 2005]",
    );
    get_striped_random_integer_from_inclusive_range_helper(
        11,
        10,
        "-10000",
        "9000",
        "[8634, 8874, 8362, -9557, -8877, 8874, 8533, 8362, 5461, 5333]",
    );
    get_striped_random_integer_from_inclusive_range_helper(
        11,
        10,
        "-10000",
        "-1000",
        "[-9077, -9558, -8874, -9557, -8853, -9557, -8885, -8874, -5462, -2710]",
    );
}

#[test]
#[should_panic]
fn get_striped_random_integer_from_inclusive_range_fail_1() {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, 2, 1);
    let mut vrg = VariableRangeGenerator::new(EXAMPLE_SEED.fork("vrg"));
    get_striped_random_integer_from_inclusive_range(
        &mut bit_source,
        &mut vrg,
        Integer::from(10u32),
        Integer::from(9u32),
    );
}
