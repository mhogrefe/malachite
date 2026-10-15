// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Abs, AbsSquared, CheckedAbs, L1Norm, Square};
use malachite_base::num::basic::traits::{Two, Zero};
use malachite_base::vector::Vector;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::{
    rational_gen, rational_pair_gen, rational_vector_gen, rational_vector_rational_pair_gen,
};
use malachite_q::test_util::rational_vector::arithmetic::checked_abs::*;

#[test]
fn test_checked_abs() {
    let test = |s, out: Option<&str>| {
        let v = RationalVector::from_str(s).unwrap();
        let out = out.map(|x| Rational::from_str(x).unwrap());
        assert_eq!(v.clone().checked_abs(), out);
        assert_eq!((&v).checked_abs(), out);
        assert_eq!(rational_vector_checked_abs_naive(&v), out);
    };
    test("()", Some("0"));
    test("(0)", Some("0"));
    test("(-1/2)", Some("1/2"));
    test("(3/5, -4/5)", Some("1"));
    test("(1/3, -2/3, 2/3)", Some("1"));
    test("(1/2, 1/2)", None);
    test("(3/2, 2)", Some("5/2"));
    test("(1/3, 1/4)", Some("5/12"));
    test("(5/13, 12/13)", Some("1"));
    test("(1/2, 1/2, 1/2, 1/2)", Some("1"));
    test("(2/3, 1)", None);
    test(
        "(18446744073709551616/3, 73786976294838206464/9)",
        Some("92233720368547758080/9"),
    );
}

#[test]
fn checked_abs_properties() {
    rational_vector_gen().test_properties(|v| {
        let r = (&v).checked_abs();
        assert_eq!(v.clone().checked_abs(), r);
        assert_eq!(rational_vector_checked_abs_naive(&v), r);
        if let Some(x) = &r {
            // The norm squares to the squared norm.
            assert_eq!(x.square(), (&v).abs_squared());
            // The norm lies between the largest absolute value of an element and the L1 norm.
            let largest = v
                .elements_ref()
                .iter()
                .map(Abs::abs)
                .max()
                .unwrap_or(Rational::ZERO);
            assert!(largest <= *x);
            assert!(*x <= v.to_l1_norm());
        }
        // Negating the vector does not change its norm.
        assert_eq!((-&v).checked_abs(), r);
    });

    rational_vector_rational_pair_gen().test_properties(|(v, c)| {
        // Scaling the vector by c scales its norm by |c|.
        let r = (&v * &c).checked_abs();
        if c == 0u32 {
            assert_eq!(r, Some(Rational::ZERO));
        } else {
            assert_eq!(r, (&v).checked_abs().map(|x| x * c.abs()));
        }
    });

    rational_pair_gen().test_properties(|(a, b)| {
        // (a^2 - b^2, 2ab) has norm a^2 + b^2.
        let v = RationalVector::from_owned_elements(vec![
            (&a).square() - (&b).square(),
            Rational::TWO * &a * &b,
        ]);
        assert_eq!(v.checked_abs(), Some(a.square() + (&b).square()));
    });

    rational_gen().test_properties(|x| {
        // A 1-dimensional vector's norm is the absolute value of its element.
        let abs = (&x).abs();
        assert_eq!(
            RationalVector::from_owned_elements(vec![x]).checked_abs(),
            Some(abs)
        );
    });
}
