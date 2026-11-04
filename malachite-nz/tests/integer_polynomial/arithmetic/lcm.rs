// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Gcd, Lcm, LcmAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::modular::traits::DivisibleBy;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_pair_gen_var_7,
};

// Checks the LCM of `p` and `q` in every way of calling it; checks that it is 0 or has a positive
// leading coefficient, that it is a multiple of both, and that its product with the GCD is `p * q`
// up to sign; and returns it.
fn verify(p: &IntegerPolynomial, q: &IntegerPolynomial) -> IntegerPolynomial {
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
    if *p == IntegerPolynomial::ZERO || *q == IntegerPolynomial::ZERO {
        assert_eq!(l, IntegerPolynomial::ZERO);
    } else {
        assert!(*l.leading_coefficient() > 0u32);
        assert!((&l).divisible_by(p));
        assert!((&l).divisible_by(q));
        let product = p * q;
        let lg = &l * p.gcd(q);
        assert!(lg == product || lg == -product);
    }
    l
}

#[test]
fn test_lcm() {
    let test = |p, q, out: Option<&str>| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let q = IntegerPolynomial::from_str(q).unwrap();
        assert_eq!(
            Some(verify(&p, &q)).map(|g| g.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - both 0
    // - p.coefficients.is_empty() || q.coefficients.is_empty()
    test("0", "0", Some("0"));
    // - the first is 0
    test("0", "2*x-4", Some("0"));
    // - constants
    // - !(*l.coefficients.last().unwrap() > 0u32)
    test("6", "-4", Some("12"));
    // - coprime
    // - *l.coefficients.last().unwrap() > 0u32
    test("x^2+1", "x+1", Some("x^3+x^2+x+1"));
    // - a common factor
    test("x^2-1", "x^2+2*x+1", Some("x^3+x^2-x-1"));
    // - a common content
    test("6*x+6", "4*x^2-4", Some("12*x^2-12"));
    // - a negative leading coefficient
    test("-x^2+1", "x+1", Some("x^2-1"));
}

#[test]
fn lcm_properties() {
    integer_polynomial_pair_gen_var_7().test_properties(|(p, q)| {
        verify(&p, &q);
    });

    integer_polynomial_gen().test_properties(|p| {
        assert_eq!((&p).lcm(IntegerPolynomial::ZERO), IntegerPolynomial::ZERO);
        assert_eq!((&p).lcm(&p), (&p).gcd(IntegerPolynomial::ZERO));
    });
}
