// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{AbsSquared, CheckedAbs, L1Norm, Square};
use malachite_base::num::basic::traits::{Two, Zero};
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_gen, natural_pair_gen, natural_vector_gen, natural_vector_natural_pair_gen,
};
use malachite_nz::test_util::natural_vector::arithmetic::checked_abs::*;

#[test]
fn test_checked_abs() {
    let test = |s, out: Option<&str>| {
        let v = NaturalVector::from_str(s).unwrap();
        let out = out.map(|x| Natural::from_str(x).unwrap());
        assert_eq!(v.clone().checked_abs(), out);
        assert_eq!((&v).checked_abs(), out);
        assert_eq!(natural_vector_checked_abs_naive(&v), out);
    };
    test("()", Some("0"));
    test("(0)", Some("0"));
    test("(5)", Some("5"));
    test("(3, 4)", Some("5"));
    test("(4, 3)", Some("5"));
    test("(1, 1)", None);
    test("(1, 2, 2)", Some("3"));
    test("(2, 3, 6)", Some("7"));
    test("(1, 4, 8)", Some("9"));
    test("(0, 0, 0)", Some("0"));
    test("(1, 1, 1, 1)", Some("2"));
    test("(1, 1, 1)", None);
    test("(5, 12)", Some("13"));
    test("(8, 15)", Some("17"));
    test("(7, 24, 60)", Some("65"));
    test("(18446744073709551616, 0)", Some("18446744073709551616"));
    test(
        "(55340232221128654848, 73786976294838206464)",
        Some("92233720368547758080"),
    );
    test("(18446744073709551615, 18446744073709551615)", None);
    test(
        "(1267650600228229401496703205376, 1267650600228229401496703205376)",
        None,
    );
}

#[test]
fn checked_abs_properties() {
    natural_vector_gen().test_properties(|v| {
        let r = (&v).checked_abs();
        assert_eq!(v.clone().checked_abs(), r);
        assert_eq!(natural_vector_checked_abs_naive(&v), r);
        if let Some(x) = &r {
            // The norm squares to the squared norm.
            assert_eq!(x.square(), (&v).abs_squared());
            // The norm lies between the largest absolute value of an element and the L1 norm.
            let largest = v
                .elements_ref()
                .iter()
                .max()
                .cloned()
                .unwrap_or(Natural::ZERO);
            assert!(largest <= *x);
            assert!(*x <= v.to_l1_norm());
        }
    });

    natural_vector_natural_pair_gen().test_properties(|(v, c)| {
        // Scaling the vector by c scales its norm by |c|.
        let r = (&v * &c).checked_abs();
        if c == 0u32 {
            assert_eq!(r, Some(Natural::ZERO));
        } else {
            assert_eq!(r, (&v).checked_abs().map(|x| x * c.clone()));
        }
    });

    natural_pair_gen().test_properties(|(a, b)| {
        // (a^2 - b^2, 2ab) has norm a^2 + b^2.
        let v = NaturalVector::from_owned_elements(vec![
            if a >= b {
                (&a).square() - (&b).square()
            } else {
                (&b).square() - (&a).square()
            },
            Natural::TWO * &a * &b,
        ]);
        assert_eq!(v.checked_abs(), Some(a.square() + (&b).square()));
    });

    natural_gen().test_properties(|x| {
        // A 1-dimensional vector's norm is the absolute value of its element.
        let abs = x.clone();
        assert_eq!(
            NaturalVector::from_owned_elements(vec![x]).checked_abs(),
            Some(abs)
        );
    });
}
