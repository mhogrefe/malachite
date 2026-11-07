// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::BinomialCoefficient;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::number_theory::traits::{Factorial, RisingFactorial};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::arithmetic::traits::Evaluate;
use malachite_base::polynomial::special::traits::{
    RisingFactorialPolynomial, RisingFactorialPolynomialTruncated,
};
use malachite_base::test_util::generators::{unsigned_gen_var_5, unsigned_pair_gen_var_28};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::natural_polynomial::special::rising_factorial_polynomial::*;

fn verify_truncated(n: u64, len: u64) -> NaturalPolynomial {
    let p = NaturalPolynomial::rising_factorial_polynomial_truncated(n, len);
    assert!(p.is_valid());
    assert_eq!(rising_factorial_polynomial_truncated_naive(n, len), p);
    assert_eq!(rising_factorial_polynomial_truncated_sequential(n, len), p);
    p
}

#[test]
fn test_rising_factorial_polynomial() {
    fn test(n: u64, out: &str) {
        let p = NaturalPolynomial::rising_factorial_polynomial(n);
        assert_eq!(p.to_string(), out);
        assert_eq!(verify_truncated(n, n + 1), p);
    }
    // (expected values computed independently with python)
    // - n == 0: only the final loop writes a coefficient
    test(0, "1");
    // - n >= 1 and len == 0 in the vector
    test(1, "x");
    // - a single factor, n == 1, in the basecase
    test(2, "x^2+x");
    // - i + 1 >= len in the basecase
    test(3, "x^3+3*x^2+2*x");
    // - i + 1 < len in the basecase
    test(4, "x^4+6*x^3+11*x^2+6*x");
    test(5, "x^5+10*x^4+35*x^3+50*x^2+24*x");
    test(6, "x^6+15*x^5+85*x^4+225*x^3+274*x^2+120*x");
}

#[test]
fn test_rising_factorial_polynomial_truncated() {
    fn test(n: u64, len: u64, out: &str) {
        assert_eq!(verify_truncated(n, len).to_string(), out);
    }
    // (expected values computed independently with python) klen == 0
    test(0, 0, "0");
    test(5, 0, "0");
    test(0, 1, "1");
    // n >= 1 and len == 0 in the vector: only the constant term, 0, is written
    test(5, 1, "0");
    test(5, 3, "50*x^2+24*x");
    test(5, 8, "x^5+10*x^4+35*x^3+50*x^2+24*x");
    test(2, 10, "x^2+x");
    // the largest product of one-word factors that the basecase expands whole
    test(
        16,
        17,
        "x^16+120*x^15+6580*x^14+218400*x^13+4899622*x^12+78558480*x^11+928095740*x^10+8207628000*x\
        ^9+54631129553*x^8+272803210680*x^7+1009672107080*x^6+2706813345600*x^5+5056995703824*x^4+\
        6165817614720*x^3+4339163001600*x^2+1307674368000*x",
    );
    // a product too large for single words, which the basecase expands whole in double words
    test(
        17,
        18,
        "x^17+136*x^16+8500*x^15+323680*x^14+8394022*x^13+156952432*x^12+2185031420*x^11+2305715984\
        0*x^10+185953177553*x^9+1146901283528*x^8+5374523477960*x^7+18861567058880*x^6+48366009233\
        424*x^5+87077748875904*x^4+102992244837120*x^3+70734282393600*x^2+20922789888000*x",
    );
    // the smallest product with more than MAX_BASECASE coefficients, which is split
    test(
        18,
        19,
        "x^18+153*x^17+10812*x^16+468180*x^15+13896582*x^14+299650806*x^13+4853222764*x^12+60202693\
        980*x^11+577924894833*x^10+4308105301929*x^9+24871845297936*x^8+110228466184200*x^7+369012\
        649234384*x^6+909299905844112*x^5+1583313975727488*x^4+1821602444624640*x^3+12234055905792\
        00*x^2+355687428096000*x",
    );
    // short prefixes of longer rows
    test(
        20,
        4,
        "668609730341153280*x^3+431565146817638400*x^2+121645100408832000*x",
    );
    test(
        60,
        3,
        "646707637948757470329105611083815125943794398174753411670628055488921600000000000*x^2+1386\
        83118545689835737939019720389406345902876772687432540821294940160000000000000*x",
    );
}

#[test]
fn test_rising_factorial_polynomial_truncated_large() {
    // Rows too long to write out: the algorithm and the naive references must agree, and the
    // coefficients of a whole row must sum to n!.
    let test = |n: u64, len: u64| {
        let p = verify_truncated(n, len);
        assert_eq!(p.len(), len.min(n + 1));
        if len > n {
            assert_eq!((&p).evaluate(&Natural::ONE), Natural::factorial(n));
        }
    };
    // - more than MAX_BASECASE coefficients, and more than 64 factors, in a split product
    test(500, 200);
    test(800, 801);
    // - more than 64 factors, but few coefficients, in a split product
    test(1000, 5);
}

#[test]
fn rising_factorial_polynomial_properties() {
    // n is kept below 256: the nth row has n + 1 coefficients of about n log n bits, and the naive
    // reference fills the whole triangle.
    unsigned_gen_var_5::<u8>().test_properties(|n| {
        let n = u64::from(n);
        let p = NaturalPolynomial::rising_factorial_polynomial(n);
        assert_eq!(verify_truncated(n, n + 1), p);
        assert_eq!(p.len(), n + 1);
        assert_eq!(*p.coefficient(n), 1u32);
        assert_eq!(*p.coefficient(0), u32::from(n == 0));
        // The polynomial evaluated at m is the rising factorial m(m + 1)...(m + n - 1); at 1, it is
        // n!.
        assert_eq!((&p).evaluate(&Natural::ONE), Natural::factorial(n));
        for m in [Natural::TWO, Natural::from(10u32)] {
            assert_eq!((&p).evaluate(&m), m.rising_factorial(n));
        }
        // [n, 1] = (n - 1)! and [n, n - 1] = C(n, 2).
        if n >= 1 {
            assert_eq!(*p.coefficient(1), Natural::factorial(n - 1));
            assert_eq!(
                *p.coefficient(n - 1),
                Natural::binomial_coefficient(Natural::from(n), Natural::TWO)
            );
        }
        // x^(n + 1) = x^(n) (x + n).
        assert_eq!(
            NaturalPolynomial::rising_factorial_polynomial(n + 1),
            p * NaturalPolynomial::from_coefficients_asc(vec![Natural::from(n), Natural::ONE])
        );
    });

    unsigned_pair_gen_var_28::<u8, u8>().test_properties(|(n, len)| {
        let (n, len) = (u64::from(n), u64::from(len));
        let p = verify_truncated(n, len);
        // Truncation keeps a prefix of the coefficients.
        let full = NaturalPolynomial::rising_factorial_polynomial(n);
        let prefix: Vec<Natural> = (0..len).map(|k| full.coefficient(k).clone()).collect();
        assert_eq!(p, NaturalPolynomial::from_coefficients_asc(prefix));
        assert!(p.len() <= len);
        if len > n {
            assert_eq!(p, full);
        }
        if len == 0 || n > 0 && len == 1 {
            assert_eq!(p, NaturalPolynomial::ZERO);
        }
    });
}
