// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Evaluate;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::fft::mul_middle_to_out_fft;
use malachite_nz::integer_polynomial::arithmetic::square::classical::square_to_out_classical;
use malachite_nz::integer_polynomial::arithmetic::square::karatsuba::square_to_out_karatsuba;
use malachite_nz::integer_polynomial::arithmetic::square::kronecker::square_to_out_kronecker;
use malachite_nz::integer_polynomial::arithmetic::square::schonhage_strassen::*;
use malachite_nz::integer_polynomial::arithmetic::square::square_to_out;
use malachite_nz::integer_polynomial::arithmetic::square::tiny::{
    square_to_out_tiny_1, square_to_out_tiny_2,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_polynomial_gen, natural_polynomial_pair_gen};
use malachite_nz::test_util::natural_polynomial::arithmetic::mul::{
    natural_generated_coefficients, naturals_mul_naive,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::square::square_naive;

#[test]
fn test_square() {
    let test = |s, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let r = (&p).square();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().square(), r);
        let mut s = p.clone();
        s.square_assign();
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(square_naive(&p), r);
    };
    test("0", "0");
    test("1", "1");
    // A constant, which `square_assign` squares in place.
    test("3", "9");
    test("x+1", "x^2+2*x+1");
    test("x^2+3*x+2", "x^4+6*x^3+13*x^2+12*x+4");
    test(
        "18446744073709551616*x+1",
        "340282366920938463463374607431768211456*x^2+36893488147419103232*x+1",
    );
}

// Every squaring kernel and every branch of the dispatcher, on polynomials with non-negative
// coefficients of chosen sizes.
#[test]
fn test_square_algorithms() {
    let test = |len: usize, bits: u64| {
        let xs = natural_generated_coefficients(len, bits);
        let expected = naturals_mul_naive(&xs, &xs);
        let n = (len << 1) - 1;
        let mut out = vec![Natural::ZERO; n];
        square_to_out(&mut out, &xs);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        square_to_out_classical(&mut out, &xs);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        square_to_out_karatsuba(&mut out, &xs);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        square_to_out_kronecker(&mut out, &xs);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        square_to_out_schonhage_strassen(&mut out, &xs);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        if mul_middle_to_out_fft(&mut out, &xs, &xs, 0, n) {
            assert_eq!(out, expected);
        }
        let small = bits <= Limb::WIDTH - 2;
        let rbits = (bits << 1) + u64::from(usize::BITS - len.leading_zeros());
        if small && rbits <= Limb::WIDTH - 2 {
            let mut out = vec![Natural::ZERO; n];
            square_to_out_tiny_1(&mut out, &xs);
            assert_eq!(out, expected);
        }
        if small && rbits < Limb::WIDTH << 1 {
            let mut out = vec![Natural::ZERO; n];
            square_to_out_tiny_2(&mut out, &xs);
            assert_eq!(out, expected);
        }
    };
    // - one-word tiny kernel
    test(5, 4);
    // - two-word tiny kernel
    test(6, 40);
    // - classical
    test(3, 500);
    // - Karatsuba
    test(8, 1000);
    // - Kronecker substitution
    test(30, 100);
    // - Schönhage–Strassen
    test(40, 600);
    test(150, 600);
    // - small-prime FFT
    test(170, 20);
    test(300, 64);
}

#[test]
fn square_properties() {
    natural_polynomial_gen().test_properties(|p| {
        let r = (&p).square();
        assert!(r.is_valid());
        // The forms agree.
        assert_eq!(p.clone().square(), r);
        let mut s = p.clone();
        s.square_assign();
        assert!(s.is_valid());
        assert_eq!(s, r);

        assert_eq!(square_naive(&p), r);
        assert_eq!(&p * &p, r);
        // The square agrees with the square of the same polynomial as an `IntegerPolynomial`.
        assert_eq!(
            IntegerPolynomial::from(p.clone()).square(),
            IntegerPolynomial::from(r.clone())
        );
        for x in [Natural::from(3u32), Natural::from(10u32)] {
            assert_eq!((&r).evaluate(&x), (&p).evaluate(&x).square());
        }
    });

    natural_polynomial_pair_gen().test_properties(|(p, q)| {
        // (p + q)^2 = p^2 + 2pq + q^2.
        let pq = &p * &q;
        assert_eq!(
            (&p + &q).square(),
            (&p).square() + &pq + &pq + (&q).square()
        );
    });
}
