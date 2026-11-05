// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::linear_algebra::traits::ModCharacteristicPolynomial;
use malachite_base::num::modular::traits::{ModIsReduced, ModMul};
use malachite_base::polynomial::arithmetic::traits::MulPowerOfX;
use malachite_base::polynomial::modular::traits::{ModEvaluate, ModPowerOf2Evaluate};
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_pair_gen_var_11,
    unsigned_polynomial_unsigned_matrix_unsigned_triple_gen_var_1,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2,
};
use malachite_base::test_util::unsigned_polynomial::modular::mod_evaluate_at_matrix::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_mod_evaluate_at_matrix() {
    fn test<T: PrimitiveUnsigned>(p: &str, a: &str, m: T, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let a = UnsignedMatrix::<T>::from_str(a).unwrap();
        let b = (&p).mod_evaluate(&a, m);
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);
        assert_eq!((&p).mod_evaluate(a.clone(), m), b);
        assert_eq!(p.clone().mod_evaluate(&a, m), b);
        assert_eq!(p.clone().mod_evaluate(a.clone(), m), b);
        assert_eq!(mod_evaluate_at_matrix_horner(&p, &a, m), b);
        assert_eq!(mod_evaluate_at_matrix_paterson_stockmeyer(&p, &a, m), b);
        assert_eq!(mod_evaluate_at_matrix_naive(&p, &a, m), b);
    }
    // - Horner and Paterson–Stockmeyer: len == 0
    // - dispatch: len < 5
    test::<u8>("0", "[]_{0,0}", 7, "[]_{0,0}");
    // - Horner and Paterson–Stockmeyer: ring.is_zero_matrix(c)
    test::<u8>("3*x+1", "[]_{0,0}", 7, "[]_{0,0}");
    test::<u8>("0", "[1, 2; 3, 4]", 7, "[0, 0; 0, 0]");
    // - Horner and Paterson–Stockmeyer: len == 1
    test::<u8>("5", "[1, 2; 3, 4]", 7, "[5, 0; 0, 5]");
    // The value is the constant term times the identity matrix.
    test::<u8>("3*x^2+x+4", "[0, 0; 0, 0]", 7, "[4, 0; 0, 4]");
    // - Horner and Paterson–Stockmeyer: len > 1 && !ring.is_zero_matrix(c)
    // - Paterson–Stockmeyer: lim == 1
    // - Paterson–Stockmeyer: top.is_empty()
    test::<u8>("x", "[1, 2; 3, 4]", 7, "[1, 2; 3, 4]");
    // The characteristic polynomial of [1, 2; 3, 4] is x^2 - 5x - 2, which vanishes at the matrix.
    test::<u8>("x^2+2*x+5", "[1, 2; 3, 4]", 7, "[0, 0; 0, 0]");
    // Modulo 1 the only reduced matrix is the zero matrix.
    test::<u8>("0", "[0, 0; 0, 0]", 1, "[0, 0; 0, 0]");
    // A 1 x 1 matrix holds the value of the polynomial at its element.
    test::<u8>("5*x^2+3*x+7", "[6]", 13, "[10]");
    // The products overflow a `u8`, but are reduced exactly.
    test::<u8>("200*x^2+100", "[150, 1; 0, 150]", 251, "[172, 11; 0, 172]");
    // - Paterson–Stockmeyer: lim >= 2
    // - dispatch: len >= 5 && n * len < 425
    // The powers of a nilpotent matrix vanish from the cube on.
    test::<u8>(
        "x^5+x^4+x^3+x^2+x+1",
        "[0, 1, 0; 0, 0, 1; 0, 0, 0]",
        7,
        "[1, 1, 1; 0, 1, 1; 0, 0, 1]",
    );
    test::<u64>(
        "x^3+x+1",
        "[1, 2, 0; 0, 1, 1; 1, 0, 2]",
        11,
        "[5, 8, 8; 4, 5, 8; 8, 8, 2]",
    );
    test::<u64>(
        "x^2+1",
        "[18446744073709551555, 1; 0, 18446744073709551555]",
        18446744073709551557,
        "[5, 18446744073709551553; 0, 5]",
    );
    test::<u128>(
        "x^2+x",
        "[1267650600228229401496703205376, 1; 1, 0]",
        170141183460469231731687303715884105727,
        "[1267650609672962367235993632769, 1267650600228229401496703205377; \
        1267650600228229401496703205377, 1]",
    );
}

#[test]
fn test_mod_evaluate_at_matrix_long() {
    fn test(p: &str, a: &str, m: u64, out: &str) {
        let p = UnsignedPolynomial::<u64>::from_str(p).unwrap();
        let a = UnsignedMatrix::<u64>::from_str(a).unwrap();
        let b = (&p).mod_evaluate(&a, m);
        assert_eq!(b.to_string(), out);
        assert_eq!(mod_evaluate_at_matrix_horner(&p, &a, m), b);
        assert_eq!(mod_evaluate_at_matrix_paterson_stockmeyer(&p, &a, m), b);
        assert_eq!(mod_evaluate_at_matrix_naive(&p, &a, m), b);
    }
    // - Paterson–Stockmeyer: !top.is_empty()
    test(
        "15*x^69+80*x^68+46*x^67+14*x^66+85*x^65+57*x^64+31*x^63+7*x^62+86*x^61+66*x^60+48*x^59+32\
        *x^58+18*x^57+6*x^56+97*x^55+89*x^54+83*x^53+79*x^52+77*x^51+77*x^50+79*x^49+83*x^48+89*x^\
        47+97*x^46+6*x^45+18*x^44+32*x^43+48*x^42+66*x^41+86*x^40+7*x^39+31*x^38+57*x^37+85*x^36+1\
        4*x^35+46*x^34+80*x^33+15*x^32+53*x^31+93*x^30+34*x^29+78*x^28+23*x^27+71*x^26+20*x^25+72*\
        x^24+25*x^23+81*x^22+38*x^21+98*x^20+59*x^19+22*x^18+88*x^17+55*x^16+24*x^15+96*x^14+69*x^\
        13+44*x^12+21*x^11+82*x^9+65*x^8+50*x^7+37*x^6+26*x^5+17*x^4+10*x^3+5*x^2+2*x+1",
        "[1, 8, 15, 22, 29, 36; 4, 11, 18, 25, 32, 39; 7, 14, 21, 28, 35, 42; 10, 17, 24, 31, 38, \
        45; 13, 20, 27, 34, 41, 48; 16, 23, 30, 37, 44, 51]",
        101,
        "[93, 55, 18, 82, 45, 8; 6, 9, 10, 12, 14, 16; 21, 62, 3, 43, 84, 24; 36, 15, 95, 75, 53, \
        32; 51, 69, 87, 4, 23, 40; 66, 22, 79, 35, 92, 49]",
    );
    // - dispatch: n * len >= 425
    test(
        "53*x^70+15*x^69+80*x^68+46*x^67+14*x^66+85*x^65+57*x^64+31*x^63+7*x^62+86*x^61+66*x^60+48\
        *x^59+32*x^58+18*x^57+6*x^56+97*x^55+89*x^54+83*x^53+79*x^52+77*x^51+77*x^50+79*x^49+83*x^\
        48+89*x^47+97*x^46+6*x^45+18*x^44+32*x^43+48*x^42+66*x^41+86*x^40+7*x^39+31*x^38+57*x^37+8\
        5*x^36+14*x^35+46*x^34+80*x^33+15*x^32+53*x^31+93*x^30+34*x^29+78*x^28+23*x^27+71*x^26+20*\
        x^25+72*x^24+25*x^23+81*x^22+38*x^21+98*x^20+59*x^19+22*x^18+88*x^17+55*x^16+24*x^15+96*x^\
        14+69*x^13+44*x^12+21*x^11+82*x^9+65*x^8+50*x^7+37*x^6+26*x^5+17*x^4+10*x^3+5*x^2+2*x+1",
        "[1, 8, 15, 22, 29, 36; 4, 11, 18, 25, 32, 39; 7, 14, 21, 28, 35, 42; 10, 17, 24, 31, 38, \
        45; 13, 20, 27, 34, 41, 48; 16, 23, 30, 37, 44, 51]",
        101,
        "[69, 100, 31, 63, 95, 26; 52, 25, 97, 69, 41, 13; 36, 49, 63, 75, 88, 0; 20, 74, 27, 82, \
        34, 88; 4, 99, 93, 87, 82, 75; 89, 23, 58, 93, 27, 63]",
    );
}

#[test]
#[should_panic]
fn mod_evaluate_at_matrix_fail_1() {
    // m is 0.
    UnsignedPolynomial::<u8>::from_str("0")
        .unwrap()
        .mod_evaluate(&UnsignedMatrix::<u8>::from_str("[0]").unwrap(), 0);
}

#[test]
#[should_panic]
fn mod_evaluate_at_matrix_fail_2() {
    // The matrix is not square.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_evaluate(&UnsignedMatrix::<u8>::from_str("[1, 2]").unwrap(), 5);
}

#[test]
#[should_panic]
fn mod_evaluate_at_matrix_fail_3() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("5*x+1")
        .unwrap()
        .mod_evaluate(&UnsignedMatrix::<u8>::from_str("[1]").unwrap(), 5);
}

#[test]
#[should_panic]
fn mod_evaluate_at_matrix_fail_4() {
    // An element of the matrix is not reduced.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_evaluate(UnsignedMatrix::<u8>::from_str("[5]").unwrap(), 5);
}

#[test]
#[should_panic]
fn mod_evaluate_at_matrix_fail_5() {
    // By reference, an element of the matrix is not reduced.
    (&UnsignedPolynomial::<u64>::from_str("x+1").unwrap())
        .mod_evaluate(&UnsignedMatrix::<u64>::from_str("[1, 0; 0, 7]").unwrap(), 7);
}

#[test]
fn mod_evaluate_at_matrix_properties() {
    unsigned_polynomial_unsigned_matrix_unsigned_triple_gen_var_1().test_properties(|(p, a, m)| {
        let b = (&p).mod_evaluate(&a, m);
        assert!(b.is_valid());
        assert_eq!(b.height(), a.height());
        assert_eq!(b.width(), a.height());
        assert!(b.mod_is_reduced(&m));
        assert_eq!((&p).mod_evaluate(a.clone(), m), b);
        assert_eq!(p.clone().mod_evaluate(&a, m), b);
        assert_eq!(p.clone().mod_evaluate(a.clone(), m), b);
        assert_eq!(mod_evaluate_at_matrix_horner(&p, &a, m), b);
        assert_eq!(mod_evaluate_at_matrix_paterson_stockmeyer(&p, &a, m), b);
        assert_eq!(mod_evaluate_at_matrix_naive(&p, &a, m), b);

        // p(A) commutes with A.
        assert_eq!((&b).mod_mul(&a, m), (&a).mod_mul(&b, m));
        // (x * p)(A) = A * p(A).
        assert_eq!(
            (&p).mul_power_of_x(1).mod_evaluate(&a, m),
            (&a).mod_mul(&b, m)
        );

        // Modulo a power of 2, this agrees with mod_power_of_2_evaluate.
        if m.is_power_of_2() {
            assert_eq!(
                (&p).mod_power_of_2_evaluate(&a, u64::from(m.trailing_zeros())),
                b
            );
        }
    });

    fn scalar_helper<T: PrimitiveUnsigned>() {
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>().test_properties(
            |(p, x, m)| {
                // At a 1 x 1 matrix, this is evaluation at the matrix's element.
                assert_eq!(
                    (&p).mod_evaluate(&UnsignedMatrix::from_elements(1, 1, &[x]), m),
                    UnsignedMatrix::from_elements(1, 1, &[p.mod_evaluate(x, m)])
                );
            },
        );
    }
    apply_fn_to_unsigneds!(scalar_helper);
}

#[test]
fn mod_evaluate_at_matrix_cayley_hamilton_properties() {
    unsigned_matrix_unsigned_pair_gen_var_11().test_properties(|(a, m)| {
        // Every square matrix is a root of its characteristic polynomial.
        assert!(
            (&a).mod_characteristic_polynomial(m)
                .mod_evaluate(&a, m)
                .is_zero()
        );
    });
}
