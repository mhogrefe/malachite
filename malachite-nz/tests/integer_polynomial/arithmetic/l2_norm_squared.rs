// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Height, Square};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{L2NormSquared, Polynomial};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::integer_polynomial_gen;
use malachite_nz::test_util::integer_polynomial::arithmetic::l2_norm_squared::*;

#[test]
fn test_l2_norm_squared() {
    let test = |s, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let n = (&p).l2_norm_squared();
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
        assert_eq!(l2_norm_squared_naive(&p), n);
    };
    test("0", "0");
    test("1", "1");
    test("-5", "25");
    test("x", "1");
    test("x^2-3*x+2", "14");
    test("-x^3+x", "2");
    // Coefficients of more than one limb.
    test(
        "1000000000000000000000*x-1",
        "1000000000000000000000000000000000000000001",
    );
}

#[test]
fn l2_norm_squared_properties() {
    integer_polynomial_gen().test_properties(|p| {
        let n = (&p).l2_norm_squared();
        assert!(n.is_valid());
        assert_eq!(l2_norm_squared_naive(&p), n);
        // Negating the polynomial changes nothing.
        assert_eq!((-&p).l2_norm_squared(), n);
        // It is zero exactly for the zero polynomial.
        assert_eq!(n == 0u32, p == IntegerPolynomial::ZERO);
        // It is between the square of the height and that times the number of coefficients.
        let h = p.to_height();
        assert!((&h).square() <= n);
        assert!(n <= Natural::from(p.len()) * h.square());
        // It is the same as for the polynomial of absolute values of the coefficients.
        let abs = NaturalPolynomial::from_coefficients_asc(
            p.coefficients_asc()
                .iter()
                .map(|c| c.unsigned_abs_ref().clone())
                .collect::<Vec<_>>(),
        );
        assert_eq!((&abs).l2_norm_squared(), n);
    });
}
