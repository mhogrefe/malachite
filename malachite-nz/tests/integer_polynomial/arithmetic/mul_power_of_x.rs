// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{Evaluate, MulPowerOfX, MulPowerOfXAssign, Polynomial};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::integer_polynomial_unsigned_pair_gen_var_1;
use malachite_nz::test_util::integer_polynomial::arithmetic::mul_power_of_x::mul_power_of_x_naive;

#[test]
fn test_mul_power_of_x() {
    let test = |s, n, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = (&p).mul_power_of_x(n);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone().mul_power_of_x(n), q);
        let mut r = p.clone();
        r.mul_power_of_x_assign(n);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(mul_power_of_x_naive(&p, n), q);
    };
    // The zero polynomial stays zero.
    test("0", 0, "0");
    test("0", 5, "0");
    test("x^2-3*x+2", 0, "x^2-3*x+2");
    test("x^2-3*x+2", 1, "x^3-3*x^2+2*x");
    test("x^2-3*x+2", 3, "x^5-3*x^4+2*x^3");
    test("-5", 2, "-5*x^2");
    test(
        "-1000000000000000000000*x+1",
        1,
        "-1000000000000000000000*x^2+x",
    );
}

#[test]
fn mul_power_of_x_properties() {
    integer_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, n)| {
        let q = (&p).mul_power_of_x(n);
        assert!(q.is_valid());
        // The forms agree.
        assert_eq!(p.clone().mul_power_of_x(n), q);
        let mut r = p.clone();
        r.mul_power_of_x_assign(n);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(mul_power_of_x_naive(&p, n), q);

        if p == IntegerPolynomial::ZERO {
            assert_eq!(q, IntegerPolynomial::ZERO);
        } else {
            // The degree goes up by n, the low n coefficients are zero, and the rest are the
            // original ones, moved up.
            assert_eq!(q.degree(), Some(p.degree().unwrap() + n));
            assert_eq!(q.truncate(n), IntegerPolynomial::ZERO);
            for i in 0..p.len() {
                assert_eq!(q.coefficient(i + n), p.coefficient(i));
            }
        }
        // Multiplying by x^n twice is multiplying by x^(2n).
        assert_eq!((&q).mul_power_of_x(n), (&p).mul_power_of_x(2 * n));
        // Multiplying by x^0 changes nothing.
        assert_eq!((&p).mul_power_of_x(0), p);

        // Evaluation at 3 is multiplied by 3^n.
        let three = Integer::from(3);
        assert_eq!(
            (&q).evaluate(&three),
            (&p).evaluate(&three) * (&three).pow(n)
        );
        // It commutes with negation.
        assert_eq!((-&p).mul_power_of_x(n), -&q);
    });
}
