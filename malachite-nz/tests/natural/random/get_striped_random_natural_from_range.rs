// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::random::striped::StripedBitSource;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::random::get_striped_random_natural_from_range;
use malachite_nz::natural::Natural;
use std::str::FromStr;

fn get_striped_random_natural_from_range_helper(
    m_numerator: u64,
    m_denominator: u64,
    a: &str,
    b: &str,
    out: &str,
) {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, m_numerator, m_denominator);
    let xs = (0..10)
        .map(|_| {
            get_striped_random_natural_from_range(
                &mut bit_source,
                Natural::from_str(a).unwrap(),
                Natural::from_str(b).unwrap(),
            )
        })
        .collect_vec();
    assert_eq!(xs.to_debug_string(), out);
}

#[test]
fn test_get_striped_random_natural_from_range() {
    get_striped_random_natural_from_range_helper(2, 1, "0", "1", "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]");
    get_striped_random_natural_from_range_helper(
        2,
        1,
        "1950",
        "2024",
        "[1950, 1971, 1990, 1962, 2018, 1972, 1952, 1999, 1989, 1987]",
    );
    get_striped_random_natural_from_range_helper(
        2,
        1,
        "1000000",
        "2000001",
        "[1002694, 1403247, 1036052, 1001215, 1170335, 1510298, 1661478, 1012673, 1005113, \
        1014065]",
    );

    get_striped_random_natural_from_range_helper(10, 1, "0", "1", "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]");
    get_striped_random_natural_from_range_helper(
        10,
        1,
        "1950",
        "2024",
        "[1950, 1951, 1983, 2016, 1950, 2020, 2016, 1951, 1950, 1983]",
    );
    get_striped_random_natural_from_range_helper(
        10,
        1,
        "1000000",
        "2000001",
        "[1001471, 1056767, 1032199, 1000432, 1998848, 1040384, 1000000, 1574911, 1981967, \
        1048574]",
    );

    get_striped_random_natural_from_range_helper(
        11,
        10,
        "0",
        "1",
        "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]",
    );
    get_striped_random_natural_from_range_helper(
        11,
        10,
        "1950",
        "2024",
        "[1962, 1962, 1972, 2019, 2019, 1962, 1962, 1986, 2005, 2005]",
    );
    get_striped_random_natural_from_range_helper(
        11,
        10,
        "1000000",
        "2000001",
        "[1004885, 1718613, 1027925, 1004874, 1485482, 1397329, 1741994, 1011029, 1004885, \
        1010346]",
    );
}

#[test]
#[should_panic]
fn get_striped_random_natural_from_range_fail_1() {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, 2, 1);
    get_striped_random_natural_from_range(
        &mut bit_source,
        Natural::from(10u32),
        Natural::from(9u32),
    );
}

#[test]
#[should_panic]
fn get_striped_random_natural_from_range_fail_2() {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, 2, 1);
    get_striped_random_natural_from_range(
        &mut bit_source,
        Natural::from(10u32),
        Natural::from(10u32),
    );
}
