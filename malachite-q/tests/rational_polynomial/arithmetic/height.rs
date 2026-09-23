// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Height;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::Polynomial;
use malachite_nz::test_util::generators::integer_polynomial_pair_gen;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{rational_gen, rational_polynomial_gen};
use malachite_q::test_util::rational_polynomial::arithmetic::height::*;

#[test]
fn test_height() {
    let test = |s, height: u32, bits| {
        let p = RationalPolynomial::from_str(s).unwrap();
        assert_eq!(p.to_height(), height);
        assert_eq!(p.clone().into_height(), height);
        assert_eq!(p.height_significant_bits(), bits);
    };
    // The zero polynomial's height is the height of the rational number 0, which is 1.
    test("0", 1, 1);
    test("1", 1, 1);
    test("x^2-3*x+2", 3, 2);
    // The coefficients are 1/3 and 1/2, whose heights are 3 and 2.
    test("1/2*x+1/3", 3, 2);
    test("22/7", 22, 5);
    test("-1/101*x", 101, 7);
}

#[test]
fn test_height_is_not_the_stored_denominator() {
    // `1/2*x+1/3` is stored as `(3*x+2)/6`, so the largest number it holds is 6; but a
    // coefficient's numerator and the shared denominator may still share a factor, and once each
    // coefficient is reduced the largest height is 3.
    let p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    assert_eq!(p.numerator_ref().to_string(), "3*x+2");
    assert_eq!(*p.denominator_ref(), 6u32);
    assert_eq!(p.to_height(), 3u32);
}

#[test]
fn height_properties() {
    rational_polynomial_gen().test_properties(|p| {
        let height = p.to_height();
        // The three ways of asking agree, and agree with materializing every coefficient.
        assert_eq!(p.clone().into_height(), height);
        assert_eq!(p.height_significant_bits(), height.significant_bits());
        assert_eq!(rational_polynomial_height_naive(&p), height);

        // It is never zero: every coefficient is a `Rational`, whose denominator is at least 1, and
        // the zero polynomial takes the height of the rational number 0.
        assert_ne!(height, 0u32);

        // The height is one of the coefficients' heights, and none of them exceeds it.
        let heights: Vec<_> = p
            .to_coefficients_asc()
            .iter()
            .map(Height::to_height)
            .collect();
        assert!(heights.iter().all(|h| *h <= height));
        if heights.is_empty() {
            assert_eq!(height, 1u32);
        } else {
            assert!(heights.contains(&height));
        }
    });

    rational_gen().test_properties(|x| {
        // The constant embedding preserves height, zero included.
        assert_eq!(
            RationalPolynomial::from(x.clone()).to_height(),
            x.to_height()
        );
    });

    integer_polynomial_pair_gen().test_properties(|(p, _)| {
        // With a denominator of 1 no coefficient reduces, so this is the integer polynomial's own
        // height — except for the zero polynomial, where the rational convention gives 1 and the
        // integer one gives 0.
        let q = RationalPolynomial::from(p.clone());
        if p.degree().is_none() {
            assert_eq!(q.to_height(), 1u32);
            assert_eq!(p.to_height(), 0u32);
        } else {
            assert_eq!(q.to_height(), p.to_height());
        }
    });
}
