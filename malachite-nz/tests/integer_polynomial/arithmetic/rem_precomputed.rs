// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::{Polynomial, RemPrecomputed, RemPrecomputedAssign};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::integer_polynomial_pair_gen_var_6;

#[test]
fn test_rem_precomputed() {
    let test = |p, q, out| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let q = IntegerPolynomial::from_str(q).unwrap();
        let data = IntegerPolynomial::precompute_rem_data(&q);
        let r = p.clone().rem_precomputed(q.clone(), &data);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().rem_precomputed(&q, &data), r);
        assert_eq!((&p).rem_precomputed(q.clone(), &data), r);
        assert_eq!((&p).rem_precomputed(&q, &data), r);
        let mut t = p.clone();
        t.rem_precomputed_assign(q.clone(), &data);
        assert_eq!(t, r);
        let mut t = p.clone();
        t.rem_precomputed_assign(&q, &data);
        assert_eq!(t, r);
        assert_eq!(&p % &q, r);
    };
    test("0", "x+1", "0");
    test("x^3+2*x+1", "1", "0");
    test("x^3+2*x+1", "-1", "0");
    // - dividend shorter than the divisor
    test("x+1", "x^2+1", "x+1");
    test("x^5+3*x^4+2*x+1", "x^2+x-1", "-2*x+4");
    test("x^3+2*x+1", "x^2+x-1", "4*x");
    // - leading coefficient -1, where FLINT's precomputed powers are wrong
    test("5*x^4+4*x^3+3*x^2+2*x+1", "-x^2+1", "6*x+9");
    // - dividend at least twice as long as the divisor
    test("x^10+1", "x^2+x+1", "x+1");
    test("x^9+2*x^7-x+3", "-x^3+2*x+5", "100*x^2+231*x+208");
}

#[test]
#[should_panic]
fn precompute_rem_data_fail_1() {
    IntegerPolynomial::precompute_rem_data(&IntegerPolynomial::from_str("0").unwrap());
}

#[test]
#[should_panic]
fn precompute_rem_data_fail_2() {
    IntegerPolynomial::precompute_rem_data(&IntegerPolynomial::from_str("2*x+1").unwrap());
}

#[test]
#[should_panic]
fn rem_precomputed_fail() {
    let data = IntegerPolynomial::precompute_rem_data(&IntegerPolynomial::from_str("x+1").unwrap());
    IntegerPolynomial::from_str("x^3+1")
        .unwrap()
        .rem_precomputed(IntegerPolynomial::from_str("x^2+1").unwrap(), &data);
}

#[test]
fn rem_precomputed_properties() {
    integer_polynomial_pair_gen_var_6().test_properties(|(p, q)| {
        let data = IntegerPolynomial::precompute_rem_data(&q);
        let r = (&p).rem_precomputed(&q, &data);
        assert!(r.is_valid());
        assert_eq!(p.clone().rem_precomputed(q.clone(), &data), r);
        assert_eq!(p.clone().rem_precomputed(&q, &data), r);
        assert_eq!((&p).rem_precomputed(q.clone(), &data), r);
        let mut t = p.clone();
        t.rem_precomputed_assign(&q, &data);
        assert_eq!(t, r);
        assert!(r.len() < q.len());
        assert_eq!(&p % &q, r);
        assert_eq!((-&p).rem_precomputed(&q, &data), -&r);
        assert_eq!(
            p.rem_precomputed(-&q, &IntegerPolynomial::precompute_rem_data(&-&q)),
            r
        );
    });
}
