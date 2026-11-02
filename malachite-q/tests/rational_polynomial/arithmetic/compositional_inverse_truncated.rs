// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::*;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::*;
use malachite_q::test_util::rational_polynomial::arithmetic::compositional_inverse_truncated::*;

fn verify(p: &RationalPolynomial, len: u64) -> Option<RationalPolynomial> {
    let r = p.compositional_inverse_truncated(len);
    if let Some(r) = &r {
        assert!(r.is_valid());
    }
    assert_eq!(p.clone().compositional_inverse_truncated(len), r);
    assert_eq!(compositional_inverse_truncated_naive(p, len), r);
    assert_eq!(compositional_inverse_truncated_newton_wrapper(p, len), r);
    assert_eq!(compositional_inverse_truncated_lagrange_wrapper(p, len), r);
    assert_eq!(
        compositional_inverse_truncated_lagrange_fast_wrapper(p, len),
        r
    );
    r
}

fn show(p: Option<RationalPolynomial>) -> String {
    p.map_or_else(|| "None".to_string(), |p| p.to_string())
}

#[test]
fn test_compositional_inverse_truncated() {
    let test = |p, len, out| {
        let p = RationalPolynomial::from_str(p).unwrap();
        assert_eq!(show(verify(&p, len)), out);
    };
    // - the length is 0
    test("x", 0, "0");
    // - the length is 1
    test("1/2*x", 1, "0");
    // - the length is 2
    test("x^2+1/3*x", 2, "3*x");
    // - the fast Lagrange algorithm
    test("1/3*x^2+1/2*x", 5, "-640/27*x^4+64/9*x^3-8/3*x^2+2*x");
    // - integer coefficients, but a coefficient of x other than 1 or -1
    test("x^2+2*x", 6, "7/256*x^5-5/128*x^4+1/16*x^3-1/8*x^2+1/2*x");
    // - integer coefficients and a coefficient of x of 1: the integer case
    test("x^2+x", 6, "14*x^5-5*x^4+2*x^3-x^2+x");
    // - the coefficient of x is -1, with a fractional coefficient
    test("1/2*x^3-x", 8, "-3/2*x^7-3/4*x^5-1/2*x^3-x");
    // - a length at which Newton's iteration is used
    test(
        "1/3*x^2+1/2*x",
        12,
        "35223764992/59049*x^11-2549088256/19683*x^10+187432960/6561*x^9-4685824/729*x^8+\
        360448/243*x^7-28672/81*x^6+7168/81*x^5-640/27*x^4+64/9*x^3-8/3*x^2+2*x",
    );
    // - a length beyond those at which Newton's iteration is used
    test(
        "1/2*x^2+x",
        21,
        "-883631595/262144*x^20+119409675/65536*x^19-64822395/65536*x^18+17678835/32768*x^17\
        -9694845/32768*x^16+334305/2048*x^15-185725/2048*x^14+52003/1024*x^13-29393/1024*x^12\
        +4199/256*x^11-2431/256*x^10+715/128*x^9-429/128*x^8+33/16*x^7-21/16*x^6+7/8*x^5-5/8*x^4\
        +1/2*x^3-1/2*x^2+x",
    );
    // - the constant coefficient is not 0
    test("x+1/2", 5, "None");
    // - the coefficient of x is 0
    test("1/2*x^2", 5, "None");
    // - the polynomial is 0
    test("0", 5, "None");
}

#[test]
fn compositional_inverse_truncated_properties() {
    rational_polynomial_unsigned_pair_gen_var_5().test_properties(|(p, len)| {
        let q = (&p).compositional_inverse_truncated(len).unwrap();
        assert!(q.is_valid());
        assert_eq!(
            p.clone().compositional_inverse_truncated(len),
            Some(q.clone())
        );
        assert_eq!(
            compositional_inverse_truncated_newton_wrapper(&p, len),
            Some(q.clone())
        );
        // The naive version and the plain Lagrange algorithm reduce every coefficient after every
        // operation, which dominates the test's running time at the longer lengths.
        if len <= 12 {
            assert_eq!(
                compositional_inverse_truncated_naive(&p, len),
                Some(q.clone())
            );
            assert_eq!(
                compositional_inverse_truncated_lagrange_wrapper(&p, len),
                Some(q.clone())
            );
        }
        assert_eq!(
            compositional_inverse_truncated_lagrange_fast_wrapper(&p, len),
            Some(q.clone())
        );
        if len >= 2 {
            let x = RationalPolynomial::x();
            assert_eq!((&p).compose_truncated(&q, len), x);
            assert_eq!((&q).compose_truncated(&p, len), x);
        }
    });

    rational_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, len)| {
        let q = (&p).compositional_inverse_truncated(len);
        let reversible = p.len() >= 2 && p.coefficient(0) == 0u32 && p.coefficient(1) != 0u32;
        assert_eq!(q.is_some(), reversible);
        assert_eq!(compositional_inverse_truncated_naive(&p, len), q);
    });
}
