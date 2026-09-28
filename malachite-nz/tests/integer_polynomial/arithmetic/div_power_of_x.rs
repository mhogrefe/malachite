// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::{DivPowerOfX, DivPowerOfXAssign, MulPowerOfX, Polynomial};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::integer_polynomial_unsigned_pair_gen_var_1;
use malachite_nz::test_util::integer_polynomial::arithmetic::div_power_of_x::*;

#[test]
fn test_div_power_of_x() {
    let test = |s, n, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = (&p).div_power_of_x(n);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone().div_power_of_x(n), q);
        let mut r = p.clone();
        r.div_power_of_x_assign(n);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(div_power_of_x_naive(&p, n), q);
    };
    // The zero polynomial stays zero.
    test("0", 0, "0");
    test("0", 5, "0");
    // Dividing by x^0 changes nothing.
    test("x^3-3*x^2+2*x-5", 0, "x^3-3*x^2+2*x-5");
    test("x^3-3*x^2+2*x-5", 1, "x^2-3*x+2");
    test("x^3-3*x^2+2*x-5", 3, "1");
    // Dividing by a power at least the length gives zero.
    test("x^3-3*x^2+2*x-5", 4, "0");
    test("x^3-3*x^2+2*x-5", 100, "0");
    test("-x^3+5", 1, "-x^2");
    test(
        "-1000000000000000000000*x^2+x",
        1,
        "-1000000000000000000000*x+1",
    );
}

#[test]
fn div_power_of_x_properties() {
    integer_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, n)| {
        let q = (&p).div_power_of_x(n);
        assert!(q.is_valid());
        // The forms agree.
        assert_eq!(p.clone().div_power_of_x(n), q);
        let mut r = p.clone();
        r.div_power_of_x_assign(n);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(div_power_of_x_naive(&p, n), q);

        // The coefficients move down by n places.
        assert_eq!(q.len(), p.len().saturating_sub(n));
        for i in 0..q.len() {
            assert_eq!(q.coefficient(i), p.coefficient(i + n));
        }
        // Dividing after multiplying by x^n gives the polynomial back.
        assert_eq!((&p).mul_power_of_x(n).div_power_of_x(n), p);
        // Multiplying back by x^n and adding the dropped low part gives the polynomial back.
        assert_eq!((&q).mul_power_of_x(n) + p.truncate(n), p);
        // Dividing by x^n twice is dividing by x^(2n).
        assert_eq!((&q).div_power_of_x(n), (&p).div_power_of_x(2 * n));
        // Dividing by x^0 changes nothing.
        assert_eq!((&p).div_power_of_x(0), p);
        // It commutes with negation.
        assert_eq!((-&p).div_power_of_x(n), -&q);
    });
}
