// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    BalancedMod, BalancedModPowerOf2, BalancedModPowerOf2Assign, DivisibleByPowerOf2, ModPowerOf2,
    PowerOf2, UnsignedAbs,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::test_util::generators::signed_unsigned_pair_gen_var_1;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::integer_unsigned_pair_gen_var_2;

#[test]
fn test_balanced_mod_power_of_2() {
    let test = |s, pow, out| {
        let x = Integer::from_str(s).unwrap();
        let r = (&x).balanced_mod_power_of_2(pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(x.clone().balanced_mod_power_of_2(pow), r);
        let mut y = x;
        y.balanced_mod_power_of_2_assign(pow);
        assert!(y.is_valid());
        assert_eq!(y, r);
    };
    test("0", 0, "0");
    test("0", 10, "0");
    test("-123", 0, "0");
    test("23", 3, "-1");
    test("20", 3, "4");
    // - a negative value is reduced into the same range
    test("-19", 3, "-3");
    // - exactly half the modulus stays positive, whichever side it comes from
    test("-20", 3, "4");
    test("-1", 1, "1");
    test("-2", 1, "0");
    // - powers around a word
    test("-1", 64, "-1");
    test("-9223372036854775808", 64, "9223372036854775808");
    test("-18446744073709551615", 64, "1");
    test("-18446744073709551616", 64, "0");
    test("-12345", 100, "-12345");
}

#[test]
fn balanced_mod_power_of_2_properties() {
    integer_unsigned_pair_gen_var_2::<u64>().test_properties(|(x, pow)| {
        let r = (&x).balanced_mod_power_of_2(pow);
        assert!(r.is_valid());
        assert_eq!(x.clone().balanced_mod_power_of_2(pow), r);
        let mut y = x.clone();
        y.balanced_mod_power_of_2_assign(pow);
        assert_eq!(y, r);

        // It is balanced_mod with modulus 2^k, and depends only on the ordinary remainder.
        assert_eq!((&x).balanced_mod(Integer::power_of_2(pow)), r);
        assert_eq!((&x).mod_power_of_2(pow).balanced_mod_power_of_2(pow), r);
        // The congruence and the range determine it uniquely.
        assert!((&x - &r).divisible_by_power_of_2(pow));
        if pow == 0 {
            assert_eq!(r, 0u32);
        } else {
            let half = Natural::power_of_2(pow - 1);
            let abs_r = (&r).unsigned_abs();
            assert!(abs_r <= half);
            if abs_r == half {
                assert!(r > 0u32);
            } else {
                // Away from the tie, negating the input negates the result.
                assert_eq!((-&x).balanced_mod_power_of_2(pow), -&r);
            }
        }
    });

    signed_unsigned_pair_gen_var_1::<SignedLimb, u64>().test_properties(|(x, pow)| {
        // Agreement with the primitive operation, wherever its result fits.
        if pow == SignedLimb::WIDTH && x == SignedLimb::MIN {
            return;
        }
        assert_eq!(
            Integer::from(x).balanced_mod_power_of_2(pow),
            Integer::from(x.balanced_mod_power_of_2(pow))
        );
    });
}
