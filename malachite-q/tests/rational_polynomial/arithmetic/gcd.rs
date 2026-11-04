// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{DivExact, Gcd, GcdAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::modular::traits::DivisibleBy;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::arithmetic::traits::MakeMonic;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_gen, rational_polynomial_pair_gen, rational_polynomial_pair_gen_var_6,
};

// Checks the GCD of `p` and `q` in every way of calling it; checks that it is 0 or monic, that it
// divides both, and that it is symmetric; and returns it.
fn verify(p: &RationalPolynomial, q: &RationalPolynomial) -> RationalPolynomial {
    let g = p.gcd(q);
    assert_eq!(p.clone().gcd(q.clone()), g);
    assert_eq!(p.clone().gcd(q), g);
    assert_eq!(p.gcd(q.clone()), g);
    let mut u = p.clone();
    u.gcd_assign(q);
    assert_eq!(u, g);
    let mut u = p.clone();
    u.gcd_assign(q.clone());
    assert_eq!(u, g);
    assert_eq!(q.gcd(p), g);
    assert!(g.is_valid());
    if g != RationalPolynomial::ZERO {
        assert_eq!(g.leading_coefficient(), Rational::ONE);
        assert!(p.divisible_by(&g));
        assert!(q.divisible_by(&g));
    }
    g
}

#[test]
fn test_gcd() {
    let test = |p, q, out: Option<&str>| {
        let p = RationalPolynomial::from_str(p).unwrap();
        let q = RationalPolynomial::from_str(q).unwrap();
        assert_eq!(
            Some(verify(&p, &q)).map(|g| g.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - both 0
    // - !(p.len() < q.len())
    // - *p == RationalPolynomial::ZERO
    test("0", "0", Some("0"));
    // - the second is 0
    // - p.len() < q.len()
    // - *q == RationalPolynomial::ZERO
    test("2/3*x+4", "0", Some("x+6"));
    // - the first is 0
    test("0", "-1/2*x-1", Some("x+2"));
    // - constants
    // - p.len() == 1
    test("2/3", "-4", Some("1"));
    // - a nontrivial GCD
    // - !(*p == RationalPolynomial::ZERO) && !(*q == RationalPolynomial::ZERO) && !(p.len() == 1)
    test("1/2*x^2-1/2", "3*x^2+6*x+3", Some("x+1"));
    // - coprime
    test("x^2+1/3", "x+1/2", Some("1"));
    // - a negative resultant of the cofactors
    test("-x^3-x^2-x-1", "-x+1", Some("1"));
}

#[test]
fn gcd_properties() {
    rational_polynomial_pair_gen().test_properties(|(p, q)| {
        verify(&p, &q);
    });

    rational_polynomial_pair_gen_var_6().test_properties(|(p, q)| {
        let g = verify(&p, &q);
        if g != RationalPolynomial::ZERO {
            // The cofactors are coprime.
            assert_eq!(
                (&p).div_exact(&g).gcd((&q).div_exact(&g)),
                RationalPolynomial::one()
            );
        }
    });

    rational_polynomial_gen().test_properties(|p| {
        let monic = (&p).make_monic();
        assert_eq!((&p).gcd(&p), monic);
        assert_eq!((&p).gcd(RationalPolynomial::ZERO), monic);
    });
}
