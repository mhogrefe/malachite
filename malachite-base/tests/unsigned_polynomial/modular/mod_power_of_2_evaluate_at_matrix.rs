// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::linear_algebra::traits::ModPowerOf2CharacteristicPolynomial;
use malachite_base::num::modular::traits::{ModPowerOf2IsReduced, ModPowerOf2Mul};
use malachite_base::polynomial::arithmetic::traits::MulPowerOfX;
use malachite_base::polynomial::modular::traits::{ModEvaluate, ModPowerOf2Evaluate};
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_pair_gen_var_7,
    unsigned_polynomial_unsigned_matrix_unsigned_triple_gen_var_2,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1,
};
use malachite_base::test_util::unsigned_polynomial::modular::mod_evaluate_at_matrix::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_mod_power_of_2_evaluate_at_matrix() {
    fn test<T: PrimitiveUnsigned>(p: &str, a: &str, pow: u64, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let a = UnsignedMatrix::<T>::from_str(a).unwrap();
        let b = (&p).mod_power_of_2_evaluate(&a, pow);
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);
        assert_eq!((&p).mod_power_of_2_evaluate(a.clone(), pow), b);
        assert_eq!(p.clone().mod_power_of_2_evaluate(&a, pow), b);
        assert_eq!(p.clone().mod_power_of_2_evaluate(a.clone(), pow), b);
        assert_eq!(mod_power_of_2_evaluate_at_matrix_horner(&p, &a, pow), b);
        assert_eq!(
            mod_power_of_2_evaluate_at_matrix_paterson_stockmeyer(&p, &a, pow),
            b
        );
        assert_eq!(mod_power_of_2_evaluate_at_matrix_naive(&p, &a, pow), b);
    }
    // - Horner and Paterson–Stockmeyer: len == 0
    // - dispatch: len < 5
    test::<u8>("0", "[]_{0,0}", 3, "[]_{0,0}");
    // Modulo 2^0 the only reduced matrix is the zero matrix.
    test::<u8>("0", "[0, 0; 0, 0]", 0, "[0, 0; 0, 0]");
    // - Horner and Paterson–Stockmeyer: len == 1
    test::<u8>("5", "[1, 2; 3, 4]", 3, "[5, 0; 0, 5]");
    // - Horner and Paterson–Stockmeyer: ring.is_zero_matrix(c)
    // The value is the constant term times the identity matrix.
    test::<u8>("3*x^2+x+4", "[0, 0; 0, 0]", 3, "[4, 0; 0, 4]");
    // - Horner and Paterson–Stockmeyer: len > 1 && !ring.is_zero_matrix(c)
    // - Paterson–Stockmeyer: lim == 1
    // - Paterson–Stockmeyer: top.is_empty()
    test::<u8>("x^2+1", "[1, 1; 0, 1]", 3, "[2, 2; 0, 2]");
    // The characteristic polynomial of [1, 2; 3, 4] is x^2 - 5x - 2, which vanishes at the matrix.
    test::<u8>("x^2+3*x+6", "[1, 2; 3, 4]", 3, "[0, 0; 0, 0]");
    // The products overflow a `u8`, but are reduced exactly.
    test::<u8>("255*x^2+255", "[255, 1; 0, 255]", 8, "[254, 2; 0, 254]");
    // - Paterson–Stockmeyer: lim >= 2
    test::<u64>(
        "x^3+x+1",
        "[1, 2, 0; 0, 1, 1; 1, 0, 2]",
        4,
        "[5, 8, 8; 4, 5, 8; 8, 8, 13]",
    );
    test::<u64>(
        "x^2+1",
        "[18446744073709551615, 1; 0, 18446744073709551615]",
        64,
        "[2, 18446744073709551614; 0, 2]",
    );
    test::<u128>(
        "x^2+1",
        "[1267650600228229401496703205375, 1; 0, 1267650600228229401496703205375]",
        100,
        "[2, 1267650600228229401496703205374; 0, 2]",
    );
}

#[test]
fn test_mod_power_of_2_evaluate_at_matrix_long() {
    fn test(p: &str, a: &str, pow: u64, out: &str) {
        let p = UnsignedPolynomial::<u64>::from_str(p).unwrap();
        let a = UnsignedMatrix::<u64>::from_str(a).unwrap();
        let b = (&p).mod_power_of_2_evaluate(&a, pow);
        assert_eq!(b.to_string(), out);
        assert_eq!(mod_power_of_2_evaluate_at_matrix_horner(&p, &a, pow), b);
        assert_eq!(
            mod_power_of_2_evaluate_at_matrix_paterson_stockmeyer(&p, &a, pow),
            b
        );
        assert_eq!(mod_power_of_2_evaluate_at_matrix_naive(&p, &a, pow), b);
    }
    // - Paterson–Stockmeyer: !top.is_empty()
    // - dispatch: len >= 5 && n * len < 425
    test(
        "26*x^69+17*x^68+10*x^67+5*x^66+2*x^65+x^64+2*x^63+5*x^62+10*x^61+17*x^60+26*x^59+37*x^58+\
        50*x^57+65*x^56+82*x^55+101*x^54+122*x^53+17*x^52+42*x^51+69*x^50+98*x^49+x^48+34*x^47+69*\
        x^46+106*x^45+17*x^44+58*x^43+101*x^42+18*x^41+65*x^40+114*x^39+37*x^38+90*x^37+17*x^36+74\
        *x^35+5*x^34+66*x^33+x^32+66*x^31+5*x^30+74*x^29+17*x^28+90*x^27+37*x^26+114*x^25+65*x^24+\
        18*x^23+101*x^22+58*x^21+17*x^20+106*x^19+69*x^18+34*x^17+x^16+98*x^15+69*x^14+42*x^13+17*\
        x^12+122*x^11+101*x^10+82*x^9+65*x^8+50*x^7+37*x^6+26*x^5+17*x^4+10*x^3+5*x^2+2*x+1",
        "[1, 8, 15, 22, 29, 36; 4, 11, 18, 25, 32, 39; 7, 14, 21, 28, 35, 42; 10, 17, 24, 31, 38, \
        45; 13, 20, 27, 34, 41, 48; 16, 23, 30, 37, 44, 51]",
        7,
        "[49, 96, 16, 64, 112, 32; 24, 117, 80, 44, 8, 100; 0, 8, 17, 24, 32, 40; 104, 28, 80, 5, \
        56, 108; 80, 48, 16, 112, 81, 48; 56, 68, 80, 92, 104, 117]",
    );
    // - dispatch: n * len >= 425
    test(
        "37*x^70+26*x^69+17*x^68+10*x^67+5*x^66+2*x^65+x^64+2*x^63+5*x^62+10*x^61+17*x^60+26*x^59+\
        37*x^58+50*x^57+65*x^56+82*x^55+101*x^54+122*x^53+17*x^52+42*x^51+69*x^50+98*x^49+x^48+34*\
        x^47+69*x^46+106*x^45+17*x^44+58*x^43+101*x^42+18*x^41+65*x^40+114*x^39+37*x^38+90*x^37+17\
        *x^36+74*x^35+5*x^34+66*x^33+x^32+66*x^31+5*x^30+74*x^29+17*x^28+90*x^27+37*x^26+114*x^25+\
        65*x^24+18*x^23+101*x^22+58*x^21+17*x^20+106*x^19+69*x^18+34*x^17+x^16+98*x^15+69*x^14+42*\
        x^13+17*x^12+122*x^11+101*x^10+82*x^9+65*x^8+50*x^7+37*x^6+26*x^5+17*x^4+10*x^3+5*x^2+2*x+\
        1",
        "[1, 8, 15, 22, 29, 36; 4, 11, 18, 25, 32, 39; 7, 14, 21, 28, 35, 42; 10, 17, 24, 31, 38, \
        45; 13, 20, 27, 34, 41, 48; 16, 23, 30, 37, 44, 51]",
        7,
        "[44, 40, 37, 34, 31, 28; 80, 0, 46, 93, 12, 59; 117, 86, 56, 24, 121, 90; 26, 45, 64, 84\
        , 102, 121; 63, 4, 73, 14, 84, 24; 100, 91, 82, 73, 64, 56]",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_at_matrix_fail_1() {
    // pow is greater than the width of the type.
    UnsignedPolynomial::<u8>::from_str("0")
        .unwrap()
        .mod_power_of_2_evaluate(&UnsignedMatrix::<u8>::from_str("[0]").unwrap(), 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_at_matrix_fail_2() {
    // The matrix is not square.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_evaluate(&UnsignedMatrix::<u8>::from_str("[1, 2]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_at_matrix_fail_3() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_evaluate(&UnsignedMatrix::<u8>::from_str("[1]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_at_matrix_fail_4() {
    // An element of the matrix is not reduced.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_evaluate(UnsignedMatrix::<u8>::from_str("[8]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_at_matrix_fail_5() {
    // By reference, an element of the matrix is not reduced.
    (&UnsignedPolynomial::<u64>::from_str("x+1").unwrap())
        .mod_power_of_2_evaluate(&UnsignedMatrix::<u64>::from_str("[1, 0; 0, 8]").unwrap(), 3);
}

#[test]
fn mod_power_of_2_evaluate_at_matrix_properties() {
    unsigned_polynomial_unsigned_matrix_unsigned_triple_gen_var_2().test_properties(
        |(p, a, pow)| {
            let b = (&p).mod_power_of_2_evaluate(&a, pow);
            assert!(b.is_valid());
            assert_eq!(b.height(), a.height());
            assert_eq!(b.width(), a.height());
            assert!(b.mod_power_of_2_is_reduced(pow));
            assert_eq!((&p).mod_power_of_2_evaluate(a.clone(), pow), b);
            assert_eq!(p.clone().mod_power_of_2_evaluate(&a, pow), b);
            assert_eq!(p.clone().mod_power_of_2_evaluate(a.clone(), pow), b);
            assert_eq!(mod_power_of_2_evaluate_at_matrix_horner(&p, &a, pow), b);
            assert_eq!(
                mod_power_of_2_evaluate_at_matrix_paterson_stockmeyer(&p, &a, pow),
                b
            );
            assert_eq!(mod_power_of_2_evaluate_at_matrix_naive(&p, &a, pow), b);

            // p(A) commutes with A.
            assert_eq!(
                (&b).mod_power_of_2_mul(&a, pow),
                (&a).mod_power_of_2_mul(&b, pow)
            );
            // (x * p)(A) = A * p(A).
            assert_eq!(
                (&p).mul_power_of_x(1).mod_power_of_2_evaluate(&a, pow),
                (&a).mod_power_of_2_mul(&b, pow)
            );

            // Below the width of the type, this agrees with mod_evaluate mod 2^pow.
            if pow < u64::WIDTH {
                assert_eq!((&p).mod_evaluate(&a, u64::power_of_2(pow)), b);
            }
        },
    );

    fn scalar_helper<T: PrimitiveUnsigned>() {
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(
            |(p, x, pow)| {
                // At a 1 x 1 matrix, this is evaluation at the matrix's element.
                assert_eq!(
                    (&p).mod_power_of_2_evaluate(&UnsignedMatrix::from_elements(1, 1, &[x]), pow),
                    UnsignedMatrix::from_elements(1, 1, &[p.mod_power_of_2_evaluate(x, pow)])
                );
            },
        );
    }
    apply_fn_to_unsigneds!(scalar_helper);
}

#[test]
fn mod_power_of_2_evaluate_at_matrix_cayley_hamilton_properties() {
    unsigned_matrix_unsigned_pair_gen_var_7().test_properties(|(a, pow)| {
        // Every square matrix is a root of its characteristic polynomial.
        assert!(
            (&a).mod_power_of_2_characteristic_polynomial(pow)
                .mod_power_of_2_evaluate(&a, pow)
                .is_zero()
        );
    });
}
