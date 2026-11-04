// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Gcd, Lcm, LcmAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::modular::traits::DivisibleBy;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::arithmetic::traits::MakeMonic;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_gen, rational_polynomial_pair_gen_var_6,
};

// Checks the LCM of `p` and `q` in every way of calling it; checks that it is 0 or monic, that it
// is a multiple of both, and that its product with the GCD is the monic multiple of `p * q`; and
// returns it.
fn verify(p: &RationalPolynomial, q: &RationalPolynomial) -> RationalPolynomial {
    let l = p.lcm(q);
    assert_eq!(p.clone().lcm(q.clone()), l);
    assert_eq!(p.clone().lcm(q), l);
    assert_eq!(p.lcm(q.clone()), l);
    let mut u = p.clone();
    u.lcm_assign(q);
    assert_eq!(u, l);
    let mut u = p.clone();
    u.lcm_assign(q.clone());
    assert_eq!(u, l);
    assert_eq!(q.lcm(p), l);
    assert!(l.is_valid());
    if *p == RationalPolynomial::ZERO || *q == RationalPolynomial::ZERO {
        assert_eq!(l, RationalPolynomial::ZERO);
    } else {
        assert_eq!(l.leading_coefficient(), Rational::ONE);
        assert!((&l).divisible_by(p));
        assert!((&l).divisible_by(q));
        assert_eq!(&l * p.gcd(q), (p * q).make_monic());
    }
    l
}

#[test]
fn test_lcm() {
    let test = |p, q, out: Option<&str>| {
        let p = RationalPolynomial::from_str(p).unwrap();
        let q = RationalPolynomial::from_str(q).unwrap();
        assert_eq!(
            Some(verify(&p, &q)).map(|g| g.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - both 0
    // - *p == RationalPolynomial::ZERO || *q == RationalPolynomial::ZERO
    test("0", "0", Some("0"));
    // - the second is 0
    test("2/3*x+4", "0", Some("0"));
    // - the first is 0
    test("0", "-1/2*x-1", Some("0"));
    // - constants
    // - p.len() == 1 && q.len() == 1
    test("2/3", "-4", Some("1"));
    // - a nontrivial GCD
    // - !(*p == RationalPolynomial::ZERO || *q == RationalPolynomial::ZERO) && !(p.len() == 1 &&
    //   q.len() == 1)
    test("1/2*x^2-1/2", "3*x^2+6*x+3", Some("x^3+x^2-x-1"));
    // - coprime
    test("x^2+1/3", "x+1/2", Some("x^3+1/2*x^2+1/3*x+1/6"));
    // - a negative resultant of the cofactors
    test("-x^3-x^2-x-1", "-x+1", Some("x^4-1"));
}

#[test]
fn lcm_properties() {
    rational_polynomial_pair_gen_var_6().test_properties(|(p, q)| {
        verify(&p, &q);
    });

    rational_polynomial_gen().test_properties(|p| {
        assert_eq!((&p).lcm(RationalPolynomial::ZERO), RationalPolynomial::ZERO);
        assert_eq!((&p).lcm(&p), (&p).make_monic());
    });
}
