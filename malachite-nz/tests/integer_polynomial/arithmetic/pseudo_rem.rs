// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::*;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::integer_polynomial_pair_gen_var_3;

#[test]
fn test_pseudo_rem() {
    let test = |p, q, out: &str, out_d| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let q = IntegerPolynomial::from_str(q).unwrap();
        let (s, d) = p.clone().pseudo_rem(q.clone());
        assert!(s.is_valid());
        assert_eq!(s.to_string(), out);
        assert_eq!(d, out_d);
        assert_eq!((&p).pseudo_rem(&q), (s.clone(), d));
        let mut t = p.clone();
        assert_eq!(t.pseudo_rem_assign(&q), d);
        assert_eq!(t, s);
    };
    // - the dividend is 0
    test("0", "x+1", "0", 0);
    // - the dividend is shorter than the divisor
    test("3*x+5", "2*x^2+1", "3*x+5", 0);
    // - a monic divisor
    test("x^3+2*x+1", "x+1", "-2", 0);
    // - a divisor whose leading coefficient divides
    test("6*x^2+3*x+9", "3*x+1", "26", 1);
    // - a nonunit leading coefficient
    test("x^3+2*x+1", "2*x+1", "-1", 3);
    // - a negative leading coefficient
    test("x^2+1", "-3*x+2", "13", 2);
    // - a constant divisor
    test("5*x^2+3", "2", "0", 1);
    // - a leading coefficient of -1
    test("x^2+3*x+2", "-x+5", "42", 0);
    // - several factors of the leading coefficient
    test("x^4+x+1", "2*x^2+x+1", "11*x+9", 3);
    // - the remainder is 0
    test("6*x^2+5*x+1", "2*x+1", "0", 0);
}

#[test]
#[should_panic]
fn pseudo_rem_fail() {
    IntegerPolynomial::from_str("x+1")
        .unwrap()
        .pseudo_rem(IntegerPolynomial::ZERO);
}

#[test]
#[should_panic]
fn pseudo_rem_assign_fail() {
    let mut p = IntegerPolynomial::from_str("x+1").unwrap();
    p.pseudo_rem_assign(IntegerPolynomial::ZERO);
}

#[test]
fn pseudo_rem_properties() {
    integer_polynomial_pair_gen_var_3().test_properties(|(p, q)| {
        let (s, r, d) = (&p).pseudo_div_rem(&q);
        let x = if 1 == 0 { s } else { r };
        assert_eq!((&p).pseudo_rem(&q), (x.clone(), d));
        assert_eq!(p.clone().pseudo_rem(q.clone()), (x.clone(), d));
        let mut t = p.clone();
        assert_eq!(t.pseudo_rem_assign(q), d);
        assert_eq!(t, x);
    });
}
