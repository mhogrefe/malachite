// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{DivisibleBy, Gcd, ModDiv, ModIsReduced, ModMul};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::test_util::generators::unsigned_vector_unsigned_unsigned_triple_gen_var_2;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_natural_natural_triple_gen_var_1;
use malachite_nz::test_util::natural_vector::arithmetic::mod_div::*;

#[test]
fn test_mod_div() {
    let test = |s, c: Natural, m: Natural, out: Option<&str>| {
        let v = NaturalVector::from_str(s).unwrap();
        let r = (&v).mod_div(&c, &m);
        assert_eq!(r.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!((&v).mod_div(&c, m.clone()), r);
        assert_eq!((&v).mod_div(c.clone(), &m), r);
        assert_eq!((&v).mod_div(c.clone(), m.clone()), r);
        assert_eq!(v.clone().mod_div(&c, &m), r);
        assert_eq!(v.clone().mod_div(&c, m.clone()), r);
        assert_eq!(v.clone().mod_div(c.clone(), &m), r);
        assert_eq!(v.clone().mod_div(c.clone(), m.clone()), r);
        assert_eq!(natural_vector_mod_div_naive(&v, &c, &m), r);
    };
    test("()", Natural::ZERO, Natural::ONE, Some("()"));
    test("(0)", Natural::ZERO, Natural::ONE, Some("(0)"));
    test(
        "(1, 2, 3)",
        Natural::from(3u32),
        Natural::from(7u32),
        Some("(5, 3, 1)"),
    );
    test(
        "(2, 4, 0)",
        Natural::TWO,
        Natural::from(6u32),
        Some("(1, 2, 0)"),
    );
    test("(2, 3)", Natural::TWO, Natural::from(6u32), None);
    test("(0, 1)", Natural::ZERO, Natural::from(5u32), None);
    test(
        "(18446744073709551616, 3, 170141183460469231731687303715884105726)",
        Natural::from_str("18446744073709551617").unwrap(),
        Natural::from_str("170141183460469231731687303715884105727").unwrap(),
        Some(
            "(170141183460469231713240559642174554113, 55340232221128654845, \
            170141183460469231713240559642174554112)",
        ),
    );
    test(
        "(18446744073709551616, 6)",
        Natural::from_str("18446744073709551616").unwrap(),
        Natural::from_str("36893488147419103232").unwrap(),
        None,
    );
    test(
        "(18446744073709551616, 3)",
        Natural::from_str("18446744073709551616").unwrap(),
        Natural::from_str("36893488147419103232").unwrap(),
        None,
    );
}

#[test]
#[should_panic]
fn mod_div_fail_1() {
    // An element is not reduced.
    (&NaturalVector::from_str("(1, 7)").unwrap()).mod_div(&Natural::ONE, &Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_div_fail_2() {
    // The divisor is not reduced.
    (&NaturalVector::from_str("(1, 2)").unwrap())
        .mod_div(&Natural::from(7u32), &Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_div_fail_3() {
    // The modulus is zero.
    (&NaturalVector::from_str("()").unwrap()).mod_div(&Natural::ZERO, &Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_div_fail_4() {
    NaturalVector::from_str("(1, 7)")
        .unwrap()
        .mod_div(Natural::ONE, Natural::from(7u32));
}

#[test]
fn mod_div_properties() {
    natural_vector_natural_natural_triple_gen_var_1().test_properties(|(v, c, m)| {
        let r = (&v).mod_div(&c, &m);
        assert_eq!((&v).mod_div(&c, m.clone()), r);
        assert_eq!((&v).mod_div(c.clone(), &m), r);
        assert_eq!((&v).mod_div(c.clone(), m.clone()), r);
        assert_eq!(v.clone().mod_div(&c, &m), r);
        assert_eq!(v.clone().mod_div(&c, m.clone()), r);
        assert_eq!(v.clone().mod_div(c.clone(), &m), r);
        assert_eq!(v.clone().mod_div(c.clone(), m.clone()), r);
        assert_eq!(natural_vector_mod_div_naive(&v, &c, &m), r);
        // There are quotients exactly when gcd(c, m) divides every element.
        let g = (&c).gcd(&m);
        assert_eq!(r.is_some(), v.elements.iter().all(|x| x.divisible_by(&g)));
        if let Some(r) = r {
            // The quotients are reduced, there is one for each element, and multiplying them by c
            // gives back the elements.
            assert!(r.mod_is_reduced(&m));
            assert_eq!(r.dimension(), v.dimension());
            assert_eq!((&r).mod_mul(&c, &m), v);
        }
        // Multiplying by c and dividing by it again gives back the vector when c is invertible.
        if g == 1u32 {
            assert_eq!((&v).mod_mul(&c, &m).mod_div(&c, &m), Some(v));
        }
    });

    unsigned_vector_unsigned_unsigned_triple_gen_var_2::<u64>().test_properties(|(v, c, m)| {
        // The u64 and Natural versions agree.
        assert_eq!(
            (&v).mod_div(c, m).map(NaturalVector::from),
            NaturalVector::from(v).mod_div(Natural::from(c), Natural::from(m))
        );
    });
}
