// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ExtendedGcd, Gcd};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_pair_gen, rational_polynomial_pair_gen_var_6,
};

// Checks the extended GCD of `p` and `q` in every way of calling it; checks that the GCD is the one
// `gcd` finds, that the cofactors combine `p` and `q` into it, and, when the GCD has a lower degree
// than both, that the cofactors have the lowest degrees possible; and returns them.
#[allow(clippy::type_complexity)]
fn verify(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
) -> (RationalPolynomial, RationalPolynomial, RationalPolynomial) {
    let r = p.extended_gcd(q);
    assert_eq!(p.clone().extended_gcd(q.clone()), r);
    assert_eq!(p.clone().extended_gcd(q), r);
    assert_eq!(p.extended_gcd(q.clone()), r);
    let (g, s, t) = &r;
    for x in [g, s, t] {
        assert!(x.is_valid());
    }
    assert_eq!(p.gcd(q), *g);
    assert_eq!(s * p + t * q, *g);
    if *p != RationalPolynomial::ZERO
        && *q != RationalPolynomial::ZERO
        && g.len() < p.len().min(q.len())
    {
        assert!(s.len() + g.len() <= q.len());
        assert!(t.len() + g.len() <= p.len());
    }
    if p.len() != q.len() {
        let (g2, t2, s2) = q.extended_gcd(p);
        assert_eq!((&g2, &s2, &t2), (g, s, t));
    }
    r
}

#[test]
fn test_extended_gcd() {
    let test = |p, q, out: Option<(&str, &str, &str)>| {
        let p = RationalPolynomial::from_str(p).unwrap();
        let q = RationalPolynomial::from_str(q).unwrap();
        assert_eq!(
            Some(verify(&p, &q)).map(|(g, s, t)| (g.to_string(), s.to_string(), t.to_string())),
            out.map(|(g, s, t)| (g.to_string(), s.to_string(), t.to_string()))
        );
    };
    // - both 0
    // - *p == RationalPolynomial::ZERO
    test("0", "0", Some(("0", "0", "0")));
    // - the second is 0
    // - p.len() < q.len()
    // - *q == RationalPolynomial::ZERO
    test("2/3*x+4", "0", Some(("x+6", "3/2", "0")));
    // - the first is 0
    test("0", "-1/2*x-1", Some(("x+2", "0", "-2")));
    // - constants
    // - q.len() == 1
    test("2/3", "-4", Some(("1", "0", "-1/4")));
    // - a nontrivial GCD
    // - !(negative)
    // - g.len() > 1
    test("1/2*x^2-1/2", "3*x^2+6*x+3", Some(("x+1", "-1", "1/6")));
    // - coprime
    // - !(g.len() > 1)
    test("x^2+1/3", "x+1/2", Some(("1", "12/7", "-12/7*x+6/7")));
    // - a negative resultant of the cofactors
    // - negative
    test(
        "-x^3-x^2-x-1",
        "-x+1",
        Some(("1", "-1/4", "1/4*x^2+1/2*x+3/4")),
    );
}

#[test]
fn extended_gcd_properties() {
    rational_polynomial_pair_gen().test_properties(|(p, q)| {
        verify(&p, &q);
    });

    rational_polynomial_pair_gen_var_6().test_properties(|(p, q)| {
        verify(&p, &q);
    });
}
