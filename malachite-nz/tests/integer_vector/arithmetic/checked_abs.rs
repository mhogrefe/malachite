// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    AbsSquared, CheckedAbs, L1Norm, Square, UnsignedAbs,
};
use malachite_base::num::basic::traits::{Two, Zero};
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    integer_gen, integer_pair_gen, integer_vector_gen, integer_vector_integer_pair_gen,
};
use malachite_nz::test_util::integer_vector::arithmetic::checked_abs::*;

#[test]
fn test_checked_abs() {
    let test = |s, out: Option<&str>| {
        let v = IntegerVector::from_str(s).unwrap();
        let out = out.map(|x| Natural::from_str(x).unwrap());
        assert_eq!(v.clone().checked_abs(), out);
        assert_eq!((&v).checked_abs(), out);
        assert_eq!(integer_vector_checked_abs_naive(&v), out);
    };
    test("()", Some("0"));
    test("(0)", Some("0"));
    test("(-5)", Some("5"));
    test("(3, -4)", Some("5"));
    test("(-3, -4)", Some("5"));
    test("(1, -1)", None);
    test("(-1, 2, -2)", Some("3"));
    test("(2, -3, 6)", Some("7"));
    test("(-7, 24)", Some("25"));
    test("(1, 1, 1, 1)", Some("2"));
    test("(1, -1, 1)", None);
    test(
        "(-55340232221128654848, 73786976294838206464)",
        Some("92233720368547758080"),
    );
    test("(18446744073709551616, -1)", None);
    test(
        "(-1208925819614629174706177, 1208925819614629174706177)",
        None,
    );
}

#[test]
fn checked_abs_properties() {
    integer_vector_gen().test_properties(|v| {
        let r = (&v).checked_abs();
        assert_eq!(v.clone().checked_abs(), r);
        assert_eq!(integer_vector_checked_abs_naive(&v), r);
        if let Some(x) = &r {
            // The norm squares to the squared norm.
            assert_eq!(x.square(), (&v).abs_squared());
            // The norm lies between the largest absolute value of an element and the L1 norm.
            let largest = v
                .elements_ref()
                .iter()
                .map(|x| x.unsigned_abs_ref().clone())
                .max()
                .unwrap_or(Natural::ZERO);
            assert!(largest <= *x);
            assert!(*x <= v.to_l1_norm());
        }
        // Negating the vector does not change its norm.
        assert_eq!((-&v).checked_abs(), r);
    });

    integer_vector_integer_pair_gen().test_properties(|(v, c)| {
        // Scaling the vector by c scales its norm by |c|.
        let r = (&v * &c).checked_abs();
        if c == 0u32 {
            assert_eq!(r, Some(Natural::ZERO));
        } else {
            assert_eq!(r, (&v).checked_abs().map(|x| x * c.unsigned_abs()));
        }
    });

    integer_pair_gen().test_properties(|(a, b)| {
        // (a^2 - b^2, 2ab) has norm a^2 + b^2.
        let v = IntegerVector::from_owned_elements(vec![
            (&a).square() - (&b).square(),
            Integer::TWO * &a * &b,
        ]);
        assert_eq!(
            v.checked_abs(),
            Some((a.square() + (&b).square()).unsigned_abs())
        );
    });

    integer_gen().test_properties(|x| {
        // A 1-dimensional vector's norm is the absolute value of its element.
        let abs = x.unsigned_abs_ref().clone();
        assert_eq!(
            IntegerVector::from_owned_elements(vec![x]).checked_abs(),
            Some(abs)
        );
    });
}
