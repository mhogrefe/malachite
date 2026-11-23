// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::oracle_row;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::arithmetic::traits::{Evaluate, MulPowerOfX, TaylorShift};
use malachite_base::polynomial::calculus::traits::Derivative;
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::rational_polynomial::special::bernoulli_polynomial::*;

#[test]
fn test_bernoulli_polynomial() {
    let test = |n, out| {
        oracle_row!(
            "flint",
            "arith_bernoulli_polynomial",
            "bernoulli_polynomial({n}) = {out}"
        );

        assert_eq!(RationalPolynomial::bernoulli_polynomial(n).to_string(), out);
    };
    // - n == 0
    test(0, "1");
    // - n != 0
    test(1, "x-1/2");
    test(2, "x^2-x+1/6");
    test(3, "x^3-3/2*x^2+1/2*x");
    test(4, "x^4-2*x^3+x^2-1/30");
    test(5, "x^5-5/2*x^4+5/3*x^3-1/6*x");
    test(6, "x^6-3*x^5+5/2*x^4-1/2*x^2+1/42");
    test(7, "x^7-7/2*x^6+7/2*x^5-7/6*x^3+1/6*x");
    test(
        20,
        "x^20-10*x^19+95/3*x^18-323/2*x^16+6460/7*x^14-4199*x^12+41990/3*x^10-223193/7*x^8+45220*x\
        ^6-68723/2*x^4+219335/21*x^2-174611/330",
    );
}

#[test]
fn bernoulli_polynomial_properties() {
    unsigned_gen_var_5::<u8>().test_properties(|n| {
        let n = u64::from(n);
        let p = RationalPolynomial::bernoulli_polynomial(n);
        assert!(p.is_valid());
        assert_eq!(p, bernoulli_polynomial_naive(n));
        // monic of degree n
        assert_eq!(p.degree(), Some(n));
        assert_eq!(p.leading_coefficient(), 1u32);
        // B_n(0) = B_n
        assert_eq!((&p).evaluate(Rational::ZERO), Rational::bernoulli_number(n));
        if n != 0 {
            // B_n' = n B_{n - 1}
            let previous = RationalPolynomial::bernoulli_polynomial(n - 1);
            assert_eq!(
                (&p).derivative(),
                previous * RationalPolynomial::from(Rational::from(n))
            );
            // B_n(x + 1) - B_n(x) = n x^(n - 1)
            assert_eq!(
                (&p).taylor_shift(Rational::ONE) - &p,
                RationalPolynomial::from(Rational::from(n)).mul_power_of_x(n - 1)
            );
        }
    });
}
