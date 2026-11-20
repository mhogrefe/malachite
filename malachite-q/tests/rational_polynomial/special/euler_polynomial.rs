// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{One, OneHalf, Two};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::arithmetic::traits::{Evaluate, MulPowerOfX, TaylorShift};
use malachite_base::polynomial::calculus::traits::Derivative;
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_nz::integer::Integer;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::rational_polynomial::special::euler_polynomial::*;

#[test]
fn test_euler_polynomial() {
    let test = |n, out| {
        assert_eq!(RationalPolynomial::euler_polynomial(n).to_string(), out);
    };
    // - n == 0
    test(0, "1");
    // - n != 0
    test(1, "x-1/2");
    test(2, "x^2-x");
    test(3, "x^3-3/2*x^2+1/4");
    test(4, "x^4-2*x^3+x");
    test(5, "x^5-5/2*x^4+5/2*x^2-1/2");
    test(6, "x^6-3*x^5+5*x^3-3*x");
    test(7, "x^7-7/2*x^6+35/4*x^4-21/2*x^2+17/8");
    test(
        20,
        "x^20-10*x^19+285*x^17-7752*x^15+164730*x^13-2603380*x^11+29015090*x^9-211668360*x^7+90075\
        2361*x^5-1825305870*x^3+1109652905*x",
    );
}

#[test]
fn euler_polynomial_properties() {
    unsigned_gen_var_5::<u8>().test_properties(|n| {
        let n = u64::from(n);
        let p = RationalPolynomial::euler_polynomial(n);
        assert!(p.is_valid());
        assert_eq!(p, euler_polynomial_naive(n));
        // monic of degree n
        assert_eq!(p.degree(), Some(n));
        assert_eq!(p.leading_coefficient(), 1u32);
        // E_n = 2^n E_n(1/2)
        assert_eq!(
            (&p).evaluate(Rational::ONE_HALF) << n,
            Rational::from(Integer::euler_number(n))
        );
        // E_n(x + 1) + E_n(x) = 2 x^n
        assert_eq!(
            (&p).taylor_shift(Rational::ONE) + &p,
            RationalPolynomial::from(Rational::TWO).mul_power_of_x(n)
        );
        if n != 0 {
            // E_n' = n E_{n - 1}
            let previous = RationalPolynomial::euler_polynomial(n - 1);
            assert_eq!(
                (&p).derivative(),
                previous * RationalPolynomial::from(Rational::from(n))
            );
        }
    });
}
