// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::test_util::generators::unsigned_gen;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_gen;
#[cfg(feature = "32_bit_limbs")]
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limb_count() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().limb_count(), out);
    };
    test("0", 0);
    test("123", 1);
    test("1000000000000", 2);
    test("4294967295", 1);
    test("4294967296", 2);
    test("18446744073709551615", 2);
    test("18446744073709551616", 3);
}

#[test]
fn limb_count_properties() {
    natural_gen().test_properties(|x| {
        let n = x.limb_count();
        assert_eq!(x <= Limb::MAX, n <= 1);
        if x != 0 {
            assert!(Natural::power_of_2((n - 1) << Limb::LOG_WIDTH) <= x);
            assert!(x < Natural::power_of_2(n << Limb::LOG_WIDTH));
        }
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert!(Natural::from(u).limb_count() <= 1);
    });
}
