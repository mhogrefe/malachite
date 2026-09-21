// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::common::test_custom_cmp_helper;
use malachite_q::rational_polynomial::{
    RationalPolynomial, ShortlexRationalPolynomial, ShortlexRationalPolynomialRef,
};
use malachite_q::test_util::generators::{
    rational_polynomial_gen, rational_polynomial_pair_gen, rational_polynomial_triple_gen,
};
use malachite_q::test_util::rational_polynomial::comparison::cmp::*;

fn shortlex(p: &RationalPolynomial, q: &RationalPolynomial) -> Ordering {
    ShortlexRationalPolynomialRef(p).cmp(&ShortlexRationalPolynomialRef(q))
}

#[test]
fn test_shortlex_cmp() {
    // In ascending order: the zero polynomial first, then degree 0 by coefficient, then degree 1,
    // and so on. A negative leading coefficient does not send a polynomial below the lower degrees,
    // as it does in the asymptotic order.
    test_custom_cmp_helper::<RationalPolynomial, _>(
        &[
            "0", "-1", "-1/2", "-1/3", "1/3", "1/2", "1", "-x", "-1/2*x", "1/2*x", "x", "-x^2",
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
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = RationalPolynomial::from_str(t).unwrap();
        assert_eq!(shortlex(&p, &q), shortlex_out, "shortlex {s} vs {t}");
        assert_eq!(p.cmp(&q), asymptotic_out, "asymptotic {s} vs {t}");
    };
    test("-x^3", "x^2", Greater, Less);
    test("-1/2*x", "0", Greater, Less);
    test("-1/2*x", "1000000", Greater, Less);
    // Where the dominating leading coefficient is positive, the two agree.
    test("1/2*x^3", "x^2", Greater, Greater);
    // At equal degrees they always agree, since both compare from the top down.
    test("1/2*x", "1/3*x", Greater, Greater);
    test("-1/3", "-1/2", Greater, Greater);
}

#[test]
fn shortlex_cmp_properties() {
    rational_polynomial_pair_gen().test_properties(|(p, q)| {
        let c = shortlex(&p, &q);
        // Comparison is antisymmetric, and agrees with `Eq`.
        assert_eq!(shortlex(&q, &p), c.reverse());
        assert_eq!(p == q, c == Equal);

        // The screens give the same answer as materializing every coefficient as a `Rational`.
        assert_eq!(rational_polynomial_shortlex_cmp_naive(&p, &q), c);

        // The owned wrapper and the borrowing one agree, and both agree with `as_ref`.
        let owned = ShortlexRationalPolynomial(p.clone());
        let other_owned = ShortlexRationalPolynomial(q.clone());
        assert_eq!(owned.cmp(&other_owned), c);
        assert_eq!(owned.as_ref().cmp(&other_owned.as_ref()), c);

        // Degree decides outright, whatever the signs.
        match p.degree().cmp(&q.degree()) {
            Equal => {}
            d => assert_eq!(c, d),
        }

        // The two orders differ exactly when the degrees differ and the dominating polynomial's
        // leading coefficient is negative.
        let dominating_is_negative = match p.degree().cmp(&q.degree()) {
            Equal => false,
            Greater => p.leading_coefficient().sign() == Less,
            Less => q.leading_coefficient().sign() == Less,
        };
        assert_eq!(c != p.cmp(&q), dominating_is_negative);
    });

    rational_polynomial_gen().test_properties(|p| {
        // Reflexivity, and the zero polynomial is the least of them all under shortlex, which it is
        // not under the asymptotic order.
        assert_eq!(shortlex(&p, &p), Equal);
        assert!(
            ShortlexRationalPolynomial(p) >= ShortlexRationalPolynomial(RationalPolynomial::ZERO)
        );
    });

    rational_polynomial_triple_gen().test_properties(|(p, q, r)| {
        // Transitivity, in the two forms that make the order a total one.
        if shortlex(&p, &q) == Less && shortlex(&q, &r) == Less {
            assert_eq!(shortlex(&p, &r), Less);
        } else if shortlex(&p, &q) == Greater && shortlex(&q, &r) == Greater {
            assert_eq!(shortlex(&p, &r), Greater);
        }
    });
}
