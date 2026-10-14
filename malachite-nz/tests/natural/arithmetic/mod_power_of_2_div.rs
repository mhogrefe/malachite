// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModDiv, ModPowerOf2Div, ModPowerOf2Inverse, ModPowerOf2IsReduced, ModPowerOf2Mul, Parity,
    PowerOf2,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::unsigned_triple_gen_var_11;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_natural_unsigned_triple_gen_var_4;

#[test]
fn test_mod_power_of_2_div() {
    let test = |s, t, pow, out: Option<&str>| {
        let x = Natural::from_str(s).unwrap();
        let y = Natural::from_str(t).unwrap();
        let q = (&x).mod_power_of_2_div(&y, pow);
        assert_eq!(q.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!((&x).mod_power_of_2_div(y.clone(), pow), q);
        assert_eq!(x.clone().mod_power_of_2_div(&y, pow), q);
        assert_eq!(x.mod_power_of_2_div(y, pow), q);
    };
    test("0", "0", 0, Some("0"));
    test("0", "0", 4, Some("0"));
    test("1", "0", 4, None);
    test("6", "3", 4, Some("2"));
    test("1", "3", 4, Some("11"));
    test("6", "4", 4, None);
    test("12", "4", 4, Some("3"));
    test(
        "1267650600228229401496703205375",
        "3",
        100,
        Some("422550200076076467165567735125"),
    );
    test("110680464442257309696", "73786976294838206464", 100, None);
    test(
        "110680464442257309696",
        "36893488147419103232",
        100,
        Some("3"),
    );
    test("5", "55340232221128654848", 70, None);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_fail_1() {
    Natural::from(8u32).mod_power_of_2_div(Natural::from(3u32), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_fail_2() {
    Natural::from(3u32).mod_power_of_2_div(Natural::from(8u32), 3);
}

#[test]
fn mod_power_of_2_div_properties() {
    natural_natural_unsigned_triple_gen_var_4().test_properties(|(x, y, pow)| {
        let q = (&x).mod_power_of_2_div(&y, pow);
        assert_eq!((&x).mod_power_of_2_div(y.clone(), pow), q);
        assert_eq!(x.clone().mod_power_of_2_div(&y, pow), q);
        assert_eq!(x.clone().mod_power_of_2_div(y.clone(), pow), q);
        // A quotient exists exactly when x is 0, or y is nonzero and has no more trailing zeros
        // than x.
        assert_eq!(
            q.is_some(),
            x == 0u32 || (y != 0u32 && x.trailing_zeros() >= y.trailing_zeros())
        );
        if let Some(q) = &q {
            // The quotient is reduced, times y it is x, and it is the smallest, less than 2^(pow -
            // s).
            assert!(q.mod_power_of_2_is_reduced(pow));
            assert_eq!(q.mod_power_of_2_mul(&y, pow), x);
            match y.trailing_zeros() {
                None => assert_eq!(*q, Natural::ZERO),
                Some(s) => assert!(q.mod_power_of_2_is_reduced(pow - s)),
            }
        }
        // For an odd divisor, this is multiplication by the inverse.
        if y.odd() {
            assert_eq!(
                q,
                Some((&x).mod_power_of_2_mul((&y).mod_power_of_2_inverse(pow).unwrap(), pow))
            );
        }
        // A quotient exists exactly when one exists for mod_div with modulus 2^pow, and for an odd
        // divisor the two agree.
        let r = (&x).mod_div(&y, Natural::power_of_2(pow));
        assert_eq!(r.is_some(), q.is_some());
        if y.odd() {
            assert_eq!(r, q);
        }
    });

    unsigned_triple_gen_var_11::<u64>().test_properties(|(x, y, pow)| {
        // The u64 and Natural versions agree.
        assert_eq!(
            x.mod_power_of_2_div(y, pow).map(Natural::from),
            Natural::from(x).mod_power_of_2_div(Natural::from(y), pow)
        );
    });
}
