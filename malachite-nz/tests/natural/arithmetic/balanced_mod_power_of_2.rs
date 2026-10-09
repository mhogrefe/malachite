// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    BalancedMod, BalancedModPowerOf2, DivisibleByPowerOf2, PowerOf2, UnsignedAbs,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::test_util::generators::unsigned_pair_gen_var_2;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_4;

#[test]
fn test_balanced_mod_power_of_2() {
    let test = |s, pow, out| {
        let x = Natural::from_str(s).unwrap();
        let r = (&x).balanced_mod_power_of_2(pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(x.balanced_mod_power_of_2(pow), r);
    };
    test("0", 0, "0");
    test("0", 10, "0");
    test("123", 0, "0");
    test("19", 3, "3");
    // - 7 is more than half of 8, so the representative closest to zero is negative
    test("23", 3, "-1");
    // - exactly half the modulus stays positive
    test("20", 3, "4");
    test("1", 1, "1");
    test("2", 1, "0");
    test("3", 1, "1");
    test("1000000000000", 10, "0");
    test("1000000000001", 10, "1");
    // - powers around a word
    test("18446744073709551615", 64, "-1");
    test("9223372036854775808", 64, "9223372036854775808");
    test("9223372036854775809", 64, "-9223372036854775807");
    test("18446744073709551616", 64, "0");
    test("18446744073709551617", 65, "-18446744073709551615");
    test("12345", 100, "12345");
}

#[test]
fn balanced_mod_power_of_2_properties() {
    natural_unsigned_pair_gen_var_4::<u64>().test_properties(|(x, pow)| {
        let r = (&x).balanced_mod_power_of_2(pow);
        assert!(r.is_valid());
        assert_eq!(x.clone().balanced_mod_power_of_2(pow), r);
        // It is balanced_mod with modulus 2^k, and the Integer operation.
        assert_eq!((&x).balanced_mod(Natural::power_of_2(pow)), r);
        assert_eq!(Integer::from(&x).balanced_mod_power_of_2(pow), r);
        // The congruence and the range determine it uniquely.
        assert!((Integer::from(&x) - &r).divisible_by_power_of_2(pow));
        if pow == 0 {
            assert_eq!(r, 0u32);
        } else {
            let half = Natural::power_of_2(pow - 1);
            let abs_r = (&r).unsigned_abs();
            assert!(abs_r <= half);
            if abs_r == half {
                assert!(r > 0u32);
            }
        }
    });

    unsigned_pair_gen_var_2::<Limb, u64>().test_properties(|(x, pow)| {
        // Agreement with the primitive operation, wherever its result fits.
        let top = Limb::power_of_2(Limb::WIDTH - 1);
        if pow == Limb::WIDTH && x == top || pow > Limb::WIDTH && x >= top {
            return;
        }
        assert_eq!(
            Natural::from(x).balanced_mod_power_of_2(pow),
            Integer::from(x.balanced_mod_power_of_2(pow))
        );
    });
}
