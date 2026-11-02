// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::*;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::integer_polynomial::arithmetic::compositional_inverse_truncated::*;

fn verify(p: &IntegerPolynomial, len: u64) -> Option<IntegerPolynomial> {
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

fn show(p: Option<IntegerPolynomial>) -> String {
    p.map_or_else(|| "None".to_string(), |p| p.to_string())
}

#[test]
fn test_compositional_inverse_truncated() {
    let test = |p, len, out| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        assert_eq!(show(verify(&p, len)), out);
    };
    // - the length is 0
    test("x", 0, "0");
    // - the length is 1
    test("x", 1, "0");
    // - the length is 2
    test("x^2-x", 2, "-x");
    // - the fast Lagrange algorithm
    test("x^2+x", 6, "14*x^5-5*x^4+2*x^3-x^2+x");
    // - the coefficient of x is -1
    test("3*x^3+2*x^2-x", 7, "3864*x^6-503*x^5+70*x^4-11*x^3+2*x^2-x");
    // - a longer result
    test(
        "x^3-x^2+x",
        10,
        "55*x^9-33*x^8-30*x^7-14*x^6-4*x^5+x^3+x^2+x",
    );
    // - a sparse polynomial
    test("x^5+x", 12, "5*x^9-x^5+x");
    // - the coefficient of x is not 1 or -1
    test("x^2+2*x", 5, "None");
    // - the constant coefficient is not 0
    test("x+1", 5, "None");
    // - the coefficient of x is 0
    test("x^2", 5, "None");
    // - the polynomial is 0
    test("0", 5, "None");
}

#[test]
fn compositional_inverse_truncated_properties() {
    integer_polynomial_unsigned_pair_gen_var_8().test_properties(|(p, len)| {
        let q = (&p).compositional_inverse_truncated(len).unwrap();
        assert!(q.is_valid());
        assert_eq!(
            p.clone().compositional_inverse_truncated(len),
            Some(q.clone())
        );
        assert_eq!(
            compositional_inverse_truncated_naive(&p, len),
            Some(q.clone())
        );
        assert_eq!(
            compositional_inverse_truncated_newton_wrapper(&p, len),
            Some(q.clone())
        );
        assert_eq!(
            compositional_inverse_truncated_lagrange_wrapper(&p, len),
            Some(q.clone())
        );
        assert_eq!(
            compositional_inverse_truncated_lagrange_fast_wrapper(&p, len),
            Some(q.clone())
        );
        if len >= 2 {
            let x = IntegerPolynomial::x();
            assert_eq!((&p).compose_truncated(&q, len), x);
            assert_eq!((&q).compose_truncated(&p, len), x);
        }
    });

    integer_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, len)| {
        let q = (&p).compositional_inverse_truncated(len);
        let xs = p.coefficients_asc();
        let reversible = xs.len() >= 2 && xs[0] == 0u32 && xs[1].unsigned_abs_ref() == &1u32;
        assert_eq!(q.is_some(), reversible);
        assert_eq!(compositional_inverse_truncated_naive(&p, len), q);
    });
}
