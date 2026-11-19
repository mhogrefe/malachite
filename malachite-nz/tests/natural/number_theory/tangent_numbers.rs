// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_nz::natural::Natural;
use malachite_nz::natural::number_theory::tangent_numbers::tangent_numbers_prefix;
use malachite_nz::test_util::natural::number_theory::tangent_numbers::*;

#[test]
fn test_tangent_numbers_prefix() {
    // - the empty and one-element prefixes
    assert!(tangent_numbers_prefix(0).is_empty());
    assert_eq!(tangent_numbers_prefix(1), vec![Natural::ONE]);
    // - the two-element prefix: one pass, with no inner step
    assert_eq!(
        tangent_numbers_prefix(2)
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["1", "2"]
    );
    // - the OEIS A000182 prefix (expected values computed independently with python, from tan' = 1
    //   + tan^2)
    assert_eq!(
        tangent_numbers_prefix(12)
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        [
            "1",
            "2",
            "16",
            "272",
            "7936",
            "353792",
            "22368256",
            "1903757312",
            "209865342976",
            "29088885112832",
            "4951498053124096",
            "1015423886506852352"
        ]
    );
    // - larger values
    let prefix = tangent_numbers_prefix(100);
    assert_eq!(prefix.len(), 100);
    assert_eq!(
        prefix[29].to_string(),
        "474090194351342155974522582010891145370303013658973457042263121068032"
    );
    assert_eq!(
        prefix[99].to_string(),
        "4708832410839036775127083656277455369927039641053956502555399993155472733019528453011116691778992363061715184657414377790009176379417288118062966167517347003597981485965853471991857601361417617516852747544156416749236128186966601581747429234102943849652014945729710502948586969216249495899499695789626698086604635236876027635257311232"
    );
}

#[test]
fn tangent_numbers_prefix_properties() {
    unsigned_gen_var_5::<u8>().test_properties(|n| {
        let n = u64::from(n);
        let prefix = tangent_numbers_prefix(n);
        assert_eq!(u64::exact_from(prefix.len()), n);
        assert_eq!(prefix, tangent_numbers_prefix_naive(n));
        // a shorter prefix is a prefix of a longer one
        let longer = tangent_numbers_prefix(n + 1);
        assert_eq!(prefix[..], longer[..prefix.len()]);
        for (i, t) in prefix.iter().enumerate() {
            // T_j is divisible by 2^(j - 1)
            assert!(t.divisible_by_power_of_2(u64::exact_from(i)), "T_{}", i + 1);
        }
    });
}
