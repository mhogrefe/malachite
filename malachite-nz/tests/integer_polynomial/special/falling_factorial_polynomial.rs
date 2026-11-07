// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::number_theory::traits::FallingFactorial;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::arithmetic::traits::Evaluate;
use malachite_base::polynomial::special::traits::{
    FallingFactorialPolynomial, FallingFactorialPolynomialTruncated,
    RisingFactorialPolynomialTruncated,
};
use malachite_base::test_util::generators::{unsigned_gen_var_5, unsigned_pair_gen_var_28};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::integer_polynomial::special::falling_factorial_polynomial::*;

fn verify_truncated(n: u64, len: u64) -> IntegerPolynomial {
    let p = IntegerPolynomial::falling_factorial_polynomial_truncated(n, len);
    assert!(p.is_valid());
    assert_eq!(falling_factorial_polynomial_truncated_sequential(n, len), p);
    p
}

#[test]
fn test_falling_factorial_polynomial() {
    fn test(n: u64, out: &str) {
        let p = IntegerPolynomial::falling_factorial_polynomial(n);
        assert_eq!(p.to_string(), out);
        assert_eq!(verify_truncated(n, n + 1), p);
    }
    // (expected values computed independently with python)
    test(0, "1");
    test(1, "x");
    test(2, "x^2-x");
    test(3, "x^3-3*x^2+2*x");
    test(4, "x^4-6*x^3+11*x^2-6*x");
    test(5, "x^5-10*x^4+35*x^3-50*x^2+24*x");
    test(6, "x^6-15*x^5+85*x^4-225*x^3+274*x^2-120*x");
}

#[test]
fn test_falling_factorial_polynomial_truncated() {
    fn test(n: u64, len: u64, out: &str) {
        assert_eq!(verify_truncated(n, len).to_string(), out);
    }
    // (expected values computed independently with python)
    // - len == 0
    test(0, 0, "0");
    test(5, 0, "0");
    test(0, 1, "1");
    // - n >= 1 and len == 1: only the constant term, 0
    test(5, 1, "0");
    test(5, 3, "-50*x^2+24*x");
    test(5, 8, "x^5-10*x^4+35*x^3-50*x^2+24*x");
    test(2, 10, "x^2-x");
    // - short prefixes of longer rows
    test(
        20,
        4,
        "-668609730341153280*x^3+431565146817638400*x^2-121645100408832000*x",
    );
    test(
        60,
        3,
        "646707637948757470329105611083815125943794398174753411670628055488921600000000000*x^2-1386\
        83118545689835737939019720389406345902876772687432540821294940160000000000000*x",
    );
}

#[test]
fn test_falling_factorial_polynomial_truncated_large() {
    // Rows too long to write out: the algorithm and the sequential reference must agree, and a
    // whole row must vanish at 1 once n >= 2.
    let test = |n: u64, len: u64| {
        let p = verify_truncated(n, len);
        assert_eq!(p.len(), len.min(n + 1));
        if len > n {
            assert_eq!((&p).evaluate(&Integer::ONE), Integer::ZERO);
        }
    };
    test(500, 200);
    test(800, 801);
    test(1000, 5);
}

#[test]
fn falling_factorial_polynomial_properties() {
    // n is kept below 256: the nth row has n + 1 coefficients of about n log n bits.
    unsigned_gen_var_5::<u8>().test_properties(|n| {
        let n = u64::from(n);
        let p = IntegerPolynomial::falling_factorial_polynomial(n);
        assert_eq!(verify_truncated(n, n + 1), p);
        assert_eq!(p.len(), n + 1);
        assert_eq!(*p.coefficient(n), 1u32);
        assert_eq!(*p.coefficient(0), u32::from(n == 0));
        // The polynomial evaluated at m is the falling factorial m(m - 1)...(m - n + 1), which is 0
        // when 0 <= m < n.
        for m in [Integer::ONE, Integer::from(10u32), Integer::from(-3)] {
            assert_eq!((&p).evaluate(&m), m.falling_factorial(n));
        }
        // x^(n + 1) falling = x^(n) falling (x - n).
        assert_eq!(
            IntegerPolynomial::falling_factorial_polynomial(n + 1),
            p.clone()
                * IntegerPolynomial::from_coefficients_asc(vec![-Integer::from(n), Integer::ONE])
        );
        // The coefficients are those of the rising factorial polynomial, with signs (-1)^(n + k).
        let rising = NaturalPolynomial::rising_factorial_polynomial_truncated(n, n + 1);
        for k in 0..=n {
            let c = Integer::from(rising.coefficient(k).clone());
            assert_eq!(*p.coefficient(k), if n.even() == k.even() { c } else { -c });
        }
    });

    unsigned_pair_gen_var_28::<u8, u8>().test_properties(|(n, len)| {
        let (n, len) = (u64::from(n), u64::from(len));
        let p = verify_truncated(n, len);
        // Truncation keeps a prefix of the coefficients.
        let full = IntegerPolynomial::falling_factorial_polynomial(n);
        let prefix: Vec<Integer> = (0..len).map(|k| full.coefficient(k).clone()).collect();
        assert_eq!(p, IntegerPolynomial::from_coefficients_asc(prefix));
        assert!(p.len() <= len);
        if len > n {
            assert_eq!(p, full);
        }
        if len == 0 || n > 0 && len == 1 {
            assert_eq!(p, IntegerPolynomial::ZERO);
        }
    });
}
