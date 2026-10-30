// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::*;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::integer_polynomial_integer_pair_gen;
use malachite_nz::test_util::integer_polynomial::arithmetic::pseudo_div_rem::div_root_naive;

// Checks the quotient and remainder of `p` by $x - c$ in every way of calling them, against the
// naive version; checks that the remainder is the value of `p` at `c` and that $(x - c)q + r$ is
// `p`; and returns them.
fn verify(p: &IntegerPolynomial, c: &Integer) -> (IntegerPolynomial, Integer) {
    let (q, r) = p.div_root(c);
    assert!(q.is_valid());
    assert_eq!(p.clone().div_root(c.clone()), (q.clone(), r.clone()));
    assert_eq!(p.clone().div_root(c), (q.clone(), r.clone()));
    assert_eq!(p.div_root(c.clone()), (q.clone(), r.clone()));
    let mut s = p.clone();
    assert_eq!(s.div_root_assign(c), r);
    assert_eq!(s, q);
    let mut s = p.clone();
    assert_eq!(s.div_root_assign(c.clone()), r);
    assert_eq!(s, q);
    assert_eq!(div_root_naive(p, c), (q.clone(), r.clone()));
    assert_eq!(p.evaluate(c), r);
    let divisor = IntegerPolynomial::from_coefficients_asc(vec![-c, Integer::ONE]);
    assert_eq!(divisor * &q + IntegerPolynomial::from(r.clone()), *p);
    (q, r)
}

#[test]
fn test_div_root() {
    let test = |p, c: i32, out: &str, out_r: i32| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let (q, r) = verify(&p, &Integer::from(c));
        assert_eq!(q.to_string(), out);
        assert_eq!(r, out_r);
    };
    // - the polynomial is 0
    test("0", 3, "0", 0);
    // - a constant
    test("5", 3, "0", 5);
    // - c is 0
    test("x^2+3*x+2", 0, "x+3", 2);
    // - a root
    test("x^2+3*x+2", -1, "x+2", 0);
    // - not a root
    test("x^3+2*x+1", 3, "x^2+3*x+11", 34);
    // - a negative c
    test("2*x^3-x+7", -5, "2*x^2-10*x+49", -238);
}

#[test]
fn div_root_properties() {
    integer_polynomial_integer_pair_gen().test_properties(|(p, c)| {
        verify(&p, &c);
    });

    integer_polynomial_integer_pair_gen().test_properties(|(p, _)| {
        // Dividing by x shifts the coefficients down.
        let (q, r) = verify(&p, &Integer::ZERO);
        assert_eq!(q, (&p).div_power_of_x(1));
        assert_eq!(r, *p.coefficient(0));
    });
}
