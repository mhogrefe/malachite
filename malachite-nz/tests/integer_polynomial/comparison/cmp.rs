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
use malachite_base::test_util::common::test_cmp_helper;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_pair_gen, integer_polynomial_triple_gen,
    natural_polynomial_pair_gen,
};
use malachite_nz::test_util::integer_polynomial::comparison::cmp::*;

#[test]
fn test_cmp() {
    // In ascending order. A dominating polynomial with a negative leading coefficient is below
    // everything of lower degree, so the negative-leading ones come first, in the opposite order to
    // their positive-leading mirrors.
    test_cmp_helper::<IntegerPolynomial>(&[
        "-x^3", "-x^2", "-2*x", "-x", "-x+1", "-100", "-2", "-1", "0", "1", "2", "100", "x-1", "x",
        "2*x", "x^2", "x^3",
    ]);
}

#[test]
fn test_cmp_sign_of_dominating_term() {
    let test = |s, t| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = IntegerPolynomial::from_str(t).unwrap();
        assert!(p > q, "{s} should be greater than {t}");
        assert!(q < p, "{t} should be less than {s}");
    };
    // A positive leading coefficient makes the dominating polynomial the greater one.
    test("x^2", "1000000*x");
    // A negative one makes it the smaller, however high its degree.
    test("x", "-x^1000");
    test("-1000000", "-x");
    // The zero polynomial sits between the two signs.
    test("0", "-x");
    test("x", "0");
    // At equal degrees the highest differing coefficient decides, sign and all.
    test("-x^2+5", "-2*x^2+1000000");
    test("-122", "-123");
}

#[test]
fn cmp_properties() {
    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        let c = p.cmp(&q);
        // Comparison is antisymmetric, and agrees with `Eq`.
        assert_eq!(q.cmp(&p), c.reverse());
        assert_eq!(p == q, c == Equal);

        // What the ordering says is what the polynomials eventually do: evaluating both past the
        // largest root of their difference gives the same answer.
        assert_eq!(integer_polynomial_cmp_evaluated(&p, &q), c);

        // Unlike over the `Natural`s, the degrees alone do not decide: the dominating polynomial is
        // the greater one exactly when its leading coefficient is positive.
        match p.degree().cmp(&q.degree()) {
            Equal => {}
            Greater => assert_eq!(c, p.leading_coefficient().sign()),
            Less => assert_eq!(c, q.leading_coefficient().sign().reverse()),
        }
    });

    integer_polynomial_gen().test_properties(|p| {
        // Reflexivity, and the zero polynomial's place is decided by the leading coefficient's sign
        // rather than by being least.
        assert_eq!(p.cmp(&p), Equal);
        assert_eq!(
            p.cmp(&IntegerPolynomial::ZERO),
            p.leading_coefficient().sign()
        );
    });

    integer_polynomial_triple_gen().test_properties(|(p, q, r)| {
        // Transitivity, in the two forms that make the order a total one.
        if p < q && q < r {
            assert!(p < r);
        } else if p > q && q > r {
            assert!(p > r);
        }
    });

    natural_polynomial_pair_gen().test_properties(|(p, q)| {
        // On polynomials with no negative coefficients the two types agree, since there the
        // dominating polynomial's leading coefficient is always positive.
        assert_eq!(
            p.cmp(&q),
            IntegerPolynomial::from(p.clone()).cmp(&IntegerPolynomial::from(q.clone()))
        );
    });
}
