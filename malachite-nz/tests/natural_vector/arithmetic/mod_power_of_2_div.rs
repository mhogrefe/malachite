// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModDiv, ModPowerOf2Div, ModPowerOf2IsReduced, ModPowerOf2Mul, Parity, PowerOf2,
};
use malachite_base::num::basic::traits::{Two, Zero};
use malachite_base::test_util::generators::unsigned_vector_unsigned_unsigned_triple_gen_var_1;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_natural_unsigned_triple_gen_var_1;
use malachite_nz::test_util::natural_vector::arithmetic::mod_power_of_2_div::*;

#[test]
fn test_mod_power_of_2_div() {
    let test = |s, c: Natural, pow, out: Option<&str>| {
        let v = NaturalVector::from_str(s).unwrap();
        let r = (&v).mod_power_of_2_div(&c, pow);
        assert_eq!(r.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!((&v).mod_power_of_2_div(c.clone(), pow), r);
        assert_eq!(v.clone().mod_power_of_2_div(&c, pow), r);
        assert_eq!(v.clone().mod_power_of_2_div(c.clone(), pow), r);
        assert_eq!(natural_vector_mod_power_of_2_div_naive(&v, &c, pow), r);
    };
    test("()", Natural::ZERO, 0, Some("()"));
    test("(0)", Natural::ZERO, 0, Some("(0)"));
    test("(0, 1)", Natural::ZERO, 4, None);
    test("(1, 2, 3)", Natural::from(3u32), 4, Some("(11, 6, 1)"));
    test("(2, 4, 0)", Natural::TWO, 4, Some("(1, 2, 0)"));
    test("(2, 3)", Natural::TWO, 4, None);
    test(
        "(1267650600228229401496703205375, 3, 18446744073709551616)",
        Natural::from(3u32),
        100,
        Some("(422550200076076467165567735125, 1, 845100400158301849022371987456)"),
    );
    test(
        "(18446744073709551616, 36893488147419103232)",
        Natural::from_str("18446744073709551616").unwrap(),
        100,
        Some("(1, 2)"),
    );
    test(
        "(18446744073709551616, 3)",
        Natural::from_str("18446744073709551616").unwrap(),
        100,
        None,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_div_fail_1() {
    // An element is not reduced.
    (&NaturalVector::from_str("(1, 8)").unwrap()).mod_power_of_2_div(&Natural::from(3u32), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_fail_2() {
    // The divisor is not reduced.
    (&NaturalVector::from_str("(1, 2)").unwrap()).mod_power_of_2_div(&Natural::from(8u32), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_fail_3() {
    NaturalVector::from_str("(1, 8)")
        .unwrap()
        .mod_power_of_2_div(Natural::from(3u32), 3);
}

#[test]
fn mod_power_of_2_div_properties() {
    natural_vector_natural_unsigned_triple_gen_var_1().test_properties(|(v, c, pow)| {
        let r = (&v).mod_power_of_2_div(&c, pow);
        assert_eq!((&v).mod_power_of_2_div(c.clone(), pow), r);
        assert_eq!(v.clone().mod_power_of_2_div(&c, pow), r);
        assert_eq!(v.clone().mod_power_of_2_div(c.clone(), pow), r);
        assert_eq!(natural_vector_mod_power_of_2_div_naive(&v, &c, pow), r);
        // There are quotients exactly when every element has one.
        assert_eq!(
            r.is_some(),
            v.elements
                .iter()
                .all(|x| x.mod_power_of_2_div(&c, pow).is_some())
        );
        if let Some(r) = &r {
            // The quotients are reduced, there is one for each element, and multiplying them by c
            // gives back the elements.
            assert!(r.mod_power_of_2_is_reduced(pow));
            assert_eq!(r.dimension(), v.dimension());
            assert_eq!(r.mod_power_of_2_mul(&c, pow), v);
        }
        // Quotients exist exactly when they do for mod_div with modulus 2^pow, and for an odd
        // divisor the two agree.
        let d = (&v).mod_div(&c, Natural::power_of_2(pow));
        assert_eq!(d.is_some(), r.is_some());
        if c.odd() {
            assert_eq!(d, r);
        }
    });

    unsigned_vector_unsigned_unsigned_triple_gen_var_1::<u64>().test_properties(|(v, c, pow)| {
        // The u64 and Natural versions agree.
        assert_eq!(
            (&v).mod_power_of_2_div(c, pow).map(NaturalVector::from),
            NaturalVector::from(v).mod_power_of_2_div(Natural::from(c), pow)
        );
    });
}
