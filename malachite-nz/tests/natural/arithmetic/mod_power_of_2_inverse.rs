// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModInverse, ModPowerOf2Inverse, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2Neg, Parity,
    PowerOf2,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::LowMask;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::{unsigned_gen_var_11, unsigned_pair_gen_var_39};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_14;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_mod_power_of_2_inverse() {
    let test = |s, pow, out| {
        let n = Natural::from_str(s).unwrap();

        let result = n.clone().mod_power_of_2_inverse(pow);
        assert_eq!(result.to_debug_string(), out);
        assert!(result.map_or(true, |n| n.is_valid()));

        let result = (&n).mod_power_of_2_inverse(pow);
        assert_eq!(result.to_debug_string(), out);
        assert!(result.map_or(true, |n| n.is_valid()));

        assert_eq!(
            n.mod_inverse(Natural::power_of_2(pow)).to_debug_string(),
            out
        );
    };
    test("1", 6, "Some(1)");
    test("8", 12, "None");
    test("3", 5, "Some(11)");
    test("3", 10, "Some(683)");
    test("12345678987654321", 60, "Some(454333680368735313)");
    test("12345678987654322", 60, "None");
}

#[test]
fn mod_power_of_2_inverse() {
    assert_panic!(Natural::ZERO.mod_power_of_2_inverse(5));
    assert_panic!(Natural::from(30u8).mod_power_of_2_inverse(3));
    assert_panic!((&Natural::ZERO).mod_power_of_2_inverse(5));
    assert_panic!((&Natural::from(30u8)).mod_power_of_2_inverse(3));
}

#[test]
fn mod_power_of_2_inverse_properties() {
    natural_unsigned_pair_gen_var_14().test_properties(|(n, pow)| {
        assert!(n.mod_power_of_2_is_reduced(pow));
        let result = n.clone().mod_power_of_2_inverse(pow);
        let result_ref = (&n).mod_power_of_2_inverse(pow);
        assert!(result.as_ref().map_or(true, Natural::is_valid));
        assert!(result_ref.as_ref().map_or(true, Natural::is_valid));
        assert_eq!(result_ref, result);

        assert_eq!((&n).mod_inverse(Natural::power_of_2(pow)), result);
        assert_eq!(result.is_some(), n.odd());
        if let Some(inverse) = result {
            assert!(inverse.mod_power_of_2_is_reduced(pow));
            assert_eq!((&inverse).mod_power_of_2_inverse(pow).as_ref(), Some(&n));
            assert_eq!((&n).mod_power_of_2_mul(&inverse, pow), 1u32);
            assert_eq!(
                n.mod_power_of_2_neg(pow).mod_power_of_2_inverse(pow),
                Some(inverse.mod_power_of_2_neg(pow))
            );
        }
    });

    unsigned_gen_var_11().test_properties(|pow| {
        assert_eq!(Natural::ONE.mod_power_of_2_inverse(pow), Some(Natural::ONE));
        assert_eq!(
            Natural::low_mask(pow).mod_power_of_2_inverse(pow),
            Some(Natural::low_mask(pow))
        );
    });

    unsigned_pair_gen_var_39::<Limb>().test_properties(|(n, pow)| {
        assert_eq!(
            Natural::from(n).mod_power_of_2_inverse(pow),
            n.mod_power_of_2_inverse(pow).map(Natural::from)
        );
    });
}
