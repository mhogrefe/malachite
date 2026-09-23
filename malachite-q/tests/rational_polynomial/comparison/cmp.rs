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
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::common::test_cmp_helper;
use malachite_nz::test_util::generators::integer_polynomial_pair_gen;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_gen, rational_polynomial_pair_gen, rational_polynomial_triple_gen,
};
use malachite_q::test_util::rational_polynomial::comparison::cmp::{
    rational_polynomial_cmp_evaluated, rational_polynomial_cmp_naive,
};

#[test]
fn test_cmp() {
    // In ascending order. As over the `Integer`s, a dominating polynomial with a negative leading
    // coefficient is below everything of lower degree.
    test_cmp_helper::<RationalPolynomial>(&[
        "-x^3", "-x^2", "-x", "-1/2*x", "-1", "-1/2", "-1/3", "0", "1/3", "1/2", "1", "1/3*x",
        "1/2*x", "x", "x^2", "x^3",
    ]);
}

#[test]
fn test_cmp_fractions() {
    let test = |s, t, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = RationalPolynomial::from_str(t).unwrap();
        assert_eq!(p.cmp(&q), out, "{s} vs {t}");
        assert_eq!(q.cmp(&p), out.reverse(), "{t} vs {s}");
    };
    // Constants compare as the rationals they are.
    test("1/2", "1/3", Greater);
    test("1/3", "1/2", Less);
    test("2/4", "1/2", Equal);
    // The shared denominator does not make two polynomials with different denominators
    // incomparable; the comparison cross-multiplies coefficient by coefficient.
    test("1/2*x", "1/3*x", Greater);
    test("1/2*x+1/3", "1/2*x+1/4", Greater);
    test("1/2*x+1/3", "1/2*x+1/2", Less);
    // A denominator of 1 on one side and not the other.
    test("x", "1/2*x", Greater);
    test("1/2*x", "x", Less);
    test("2", "3/2", Greater);
}

#[test]
fn cmp_properties() {
    rational_polynomial_pair_gen().test_properties(|(p, q)| {
        let c = p.cmp(&q);
        // Comparison is antisymmetric, and agrees with `Eq`.
        assert_eq!(q.cmp(&p), c.reverse());
        assert_eq!(p == q, c == Equal);

        // The screens give the same answer as materializing every coefficient as a `Rational`,
        // which reduces each one, and as evaluating the difference past its largest root.
        assert_eq!(rational_polynomial_cmp_naive(&p, &q), c);
        assert_eq!(rational_polynomial_cmp_evaluated(&p, &q), c);

        // Giving one side a denominator of 1 exercises the branches where only one side is scaled,
        // which the equal-denominator screen would otherwise hide.
        let p_int = RationalPolynomial::from(p.numerator_ref().clone());
        assert_eq!(p_int.cmp(&q), rational_polynomial_cmp_naive(&p_int, &q));
        assert_eq!(q.cmp(&p_int), rational_polynomial_cmp_naive(&q, &p_int));

        // The dominating polynomial's leading coefficient decides when the degrees differ.
        match p.degree().cmp(&q.degree()) {
            Equal => {}
            Greater => assert_eq!(c, p.leading_coefficient().sign()),
            Less => assert_eq!(c, q.leading_coefficient().sign().reverse()),
        }
    });

    rational_polynomial_gen().test_properties(|p| {
        assert_eq!(p.cmp(&p), Equal);
        assert_eq!(
            p.cmp(&RationalPolynomial::ZERO),
            p.leading_coefficient().sign()
        );
    });

    rational_polynomial_triple_gen().test_properties(|(p, q, r)| {
        if p < q && q < r {
            assert!(p < r);
        } else if p > q && q > r {
            assert!(p > r);
        }
    });

    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        // Both denominators are 1, so this is the equal-denominator screen, and the answer must
        // match `IntegerPolynomial`'s own ordering.
        assert_eq!(
            RationalPolynomial::from(p.clone()).cmp(&RationalPolynomial::from(q.clone())),
            p.cmp(&q)
        );
    });
}
