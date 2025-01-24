// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CoprimeWith, ModInverse, ModIsReduced, ModMul};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::unsigned_pair_gen_var_38;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen_var_1, natural_pair_gen_var_11};
use malachite_nz::test_util::natural::arithmetic::mod_inverse::mod_inverse_simple;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_mod_inverse() {
    let test = |s, t, out| {
        let n = Natural::from_str(s).unwrap();
        let m = Natural::from_str(t).unwrap();

        let result = n.clone().mod_inverse(m.clone());
        assert_eq!(result.to_debug_string(), out);
        assert!(result.map_or(true, |n| n.is_valid()));

        let result = n.clone().mod_inverse(&m);
        assert_eq!(result.to_debug_string(), out);
        assert!(result.map_or(true, |n| n.is_valid()));

        let result = (&n).mod_inverse(m.clone());
        assert_eq!(result.to_debug_string(), out);
        assert!(result.map_or(true, |n| n.is_valid()));

        let result = (&n).mod_inverse(&m);
        assert_eq!(result.to_debug_string(), out);
        assert!(result.map_or(true, |n| n.is_valid()));

        assert_eq!(mod_inverse_simple(n, m).to_debug_string(), out);
    };
    test("1", "6", "Some(1)");
    test("8", "12", "None");
    test("42", "56", "None");
    test("3", "5", "Some(2)");
    test("3", "10", "Some(7)");
    test("12345678987654321", "98765432123456789", "Some(1777777788)");
    test("12345678987654321", "98765432123456827", "None");
}

#[test]
fn mod_inverse_fail() {
    assert_panic!(Natural::ZERO.mod_inverse(Natural::from(5u32)));
    assert_panic!(Natural::from(30u8).mod_inverse(Natural::from(3u32)));
    assert_panic!(Natural::ZERO.mod_inverse(&Natural::from(5u32)));
    assert_panic!(Natural::from(30u8).mod_inverse(&Natural::from(3u32)));
    assert_panic!((&Natural::ZERO).mod_inverse(Natural::from(5u32)));
    assert_panic!((&Natural::from(30u8)).mod_inverse(Natural::from(3u32)));
    assert_panic!((&Natural::ZERO).mod_inverse(&Natural::from(5u32)));
    assert_panic!((&Natural::from(30u8)).mod_inverse(&Natural::from(3u32)));
}

#[test]
fn mod_inverse_properties() {
    natural_pair_gen_var_11().test_properties(|(n, m)| {
        assert!(n.mod_is_reduced(&m));
        let result_val_val = n.clone().mod_inverse(m.clone());
        let result_val_ref = n.clone().mod_inverse(&m);
        let result_ref_val = (&n).mod_inverse(m.clone());
        let result = (&n).mod_inverse(&m);
        assert!(result.as_ref().map_or(true, Natural::is_valid));
        assert!(result_val_val.as_ref().map_or(true, Natural::is_valid));
        assert!(result_val_ref.as_ref().map_or(true, Natural::is_valid));
        assert!(result_ref_val.as_ref().map_or(true, Natural::is_valid));
        assert_eq!(result_val_val, result);
        assert_eq!(result_val_ref, result);
        assert_eq!(result_ref_val, result);

        assert_eq!(mod_inverse_simple(n.clone(), m.clone()), result);
        assert_eq!(result.is_some(), (&n).coprime_with(&m));
        if let Some(inverse) = result {
            assert!(inverse.mod_is_reduced(&m));
            assert_eq!((&inverse).mod_inverse(&m).as_ref(), Some(&n));
            assert_eq!((&n).mod_mul(&inverse, &m), 1u32);
            assert_eq!((&m - n).mod_inverse(&m), Some(m - inverse));
        }
    });

    natural_gen_var_1().test_properties(|m| {
        assert_eq!(Natural::ONE.mod_inverse(&m), Some(Natural::ONE));
        assert_eq!((&m - Natural::ONE).mod_inverse(&m), Some(m - Natural::ONE));
    });

    unsigned_pair_gen_var_38::<Limb>().test_properties(|(n, m)| {
        assert_eq!(
            Natural::from(n).mod_inverse(Natural::from(m)),
            n.mod_inverse(m).map(Natural::from)
        );
    });
}
