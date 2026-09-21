// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::*;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::common::test_custom_cmp_helper;
use malachite_nz::integer_polynomial::{
    IntegerPolynomial, ShortlexIntegerPolynomial, ShortlexIntegerPolynomialRef,
};
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_pair_gen, integer_polynomial_triple_gen,
    natural_polynomial_pair_gen,
};
use malachite_nz::test_util::integer_polynomial::comparison::cmp::*;

fn shortlex(p: &IntegerPolynomial, q: &IntegerPolynomial) -> core::cmp::Ordering {
    ShortlexIntegerPolynomialRef(p).cmp(&ShortlexIntegerPolynomialRef(q))
}

#[test]
fn test_shortlex_cmp() {
    // In ascending order: the zero polynomial first, then everything of degree 0 by coefficient,
    // then degree 1, and so on. Unlike the asymptotic order, a negative leading coefficient does
    // not send a polynomial below the lower degrees.
    test_custom_cmp_helper::<IntegerPolynomial, _>(
        &[
            "0", "-100", "-2", "-1", "1", "2", "100", "-x", "-x+1", "x-1", "x", "2*x", "-x^2",
            "x^2", "-x^3", "x^3",
        ],
        shortlex,
    );
}

#[test]
fn test_shortlex_cmp_vs_asymptotic() {
    // The two orders agree except where the degrees differ and the dominating polynomial's leading
    // coefficient is negative.
    let test = |s, t, shortlex_out, asymptotic_out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = IntegerPolynomial::from_str(t).unwrap();
        assert_eq!(shortlex(&p, &q), shortlex_out, "shortlex {s} vs {t}");
        assert_eq!(p.cmp(&q), asymptotic_out, "asymptotic {s} vs {t}");
    };
    // Degree decides under shortlex; the sign of the dominating term decides asymptotically.
    test("-x^3", "x^2", Greater, Less);
    test("-x", "0", Greater, Less);
    test("-x", "1000000", Greater, Less);
    // Where the dominating leading coefficient is positive, the two agree.
    test("x^3", "x^2", Greater, Greater);
    test("x", "1000000", Greater, Greater);
    // At equal degrees they always agree, since both compare from the top down.
    test("-x^2+5", "-2*x^2+1000000", Greater, Greater);
    test("-123", "-122", Less, Less);
}

#[test]
fn shortlex_cmp_properties() {
    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        let c = shortlex(&p, &q);
        // Comparison is antisymmetric, and agrees with `Eq`.
        assert_eq!(shortlex(&q, &p), c.reverse());
        assert_eq!(p == q, c == Equal);

        // The screens give the same answer as an explicit walk down the coefficients.
        assert_eq!(integer_polynomial_shortlex_cmp_naive(&p, &q), c);

        // The owned wrapper and the borrowing one agree, and both agree with `as_ref`.
        let owned = ShortlexIntegerPolynomial(p.clone());
        let other_owned = ShortlexIntegerPolynomial(q.clone());
        assert_eq!(owned.cmp(&other_owned), c);
        assert_eq!(owned.as_ref().cmp(&other_owned.as_ref()), c);

        // Degree decides outright, whatever the signs.
        match p.degree().cmp(&q.degree()) {
            Equal => {}
            d => assert_eq!(c, d),
        }

        // The two orders differ exactly when the degrees differ and the dominating polynomial's
        // leading coefficient is negative.
        let asymptotic = p.cmp(&q);
        let dominating_is_negative = match p.degree().cmp(&q.degree()) {
            Equal => false,
            Greater => p.leading_coefficient().sign() == Less,
            Less => q.leading_coefficient().sign() == Less,
        };
        assert_eq!(c != asymptotic, dominating_is_negative);
    });

    integer_polynomial_gen().test_properties(|p| {
        // Reflexivity, and the zero polynomial is the least of them all under shortlex, which it is
        // not under the asymptotic order.
        assert_eq!(shortlex(&p, &p), Equal);
        assert!(ShortlexIntegerPolynomial(p) >= ShortlexIntegerPolynomial(IntegerPolynomial::ZERO));
    });

    integer_polynomial_triple_gen().test_properties(|(p, q, r)| {
        // Transitivity, in the two forms that make the order a total one.
        if shortlex(&p, &q) == Less && shortlex(&q, &r) == Less {
            assert_eq!(shortlex(&p, &r), Less);
        } else if shortlex(&p, &q) == Greater && shortlex(&q, &r) == Greater {
            assert_eq!(shortlex(&p, &r), Greater);
        }
    });

    natural_polynomial_pair_gen().test_properties(|(p, q)| {
        // On polynomials with no negative coefficients no leading coefficient is negative, so
        // shortlex is the asymptotic order, which there is `NaturalPolynomial`'s own.
        let p = IntegerPolynomial::from(p);
        let q = IntegerPolynomial::from(q);
        assert_eq!(shortlex(&p, &q), p.cmp(&q));
    });
}
