// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::{Polynomial, RemPrecomputed, RemPrecomputedAssign};
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::rational_polynomial_pair_gen_var_3;

#[test]
fn test_rem_precomputed() {
    let test = |p, q, out| {
        let p = RationalPolynomial::from_str(p).unwrap();
        let q = RationalPolynomial::from_str(q).unwrap();
        let data = RationalPolynomial::precompute_rem_data(&q);
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
    test("x^3+2*x+1", "2/3", "0");
    // - dividend shorter than the divisor
    test("x+1/2", "x^2+1", "x+1/2");
    test("x^5+3*x^4+2*x+1", "2*x^2+x+1", "49/16*x+19/16");
    test("1/2*x^3+2*x", "2*x^2+x+1", "15/8*x+1/8");
    // - numerator of the divisor not primitive
    test("x^5+1", "4*x^2+2", "1/4*x+1");
    // - dividend at least twice as long as the divisor
    test(
        "x^9-1/3*x^4+1",
        "3/2*x^2-x+1/5",
        "-163904/4100625*x+4157581/4100625",
    );
    test(
        "x^11+x^10+x^9+x^8+x^7+x^6+x^5+x^4+x^3+x^2+x+1",
        "-3*x^2+1",
        "364/243*x+364/243",
    );
}

#[test]
#[should_panic]
fn precompute_rem_data_fail() {
    RationalPolynomial::precompute_rem_data(&RationalPolynomial::from_str("0").unwrap());
}

#[test]
#[should_panic]
fn rem_precomputed_fail() {
    let data =
        RationalPolynomial::precompute_rem_data(&RationalPolynomial::from_str("x+1").unwrap());
    RationalPolynomial::from_str("x^3+1")
        .unwrap()
        .rem_precomputed(RationalPolynomial::from_str("x^2+1").unwrap(), &data);
}

#[test]
fn rem_precomputed_properties() {
    rational_polynomial_pair_gen_var_3().test_properties(|(p, q)| {
        let data = RationalPolynomial::precompute_rem_data(&q);
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
    });
}
