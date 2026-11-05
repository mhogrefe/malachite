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
use malachite_base::num::linear_algebra::traits::ModPowerOf2CharacteristicPolynomial;
use malachite_base::num::modular::traits::{ModPowerOf2IsReduced, ModPowerOf2Mul};
use malachite_base::polynomial::arithmetic::traits::MulPowerOfX;
use malachite_base::polynomial::modular::traits::{ModEvaluate, ModPowerOf2Evaluate};
use malachite_base::test_util::generators::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_matrix_unsigned_pair_gen_var_6,
    natural_polynomial_natural_matrix_unsigned_triple_gen_var_1,
    natural_polynomial_natural_unsigned_triple_gen_var_1,
};
use malachite_nz::test_util::natural_polynomial::modular::mod_evaluate_at_matrix::*;

fn verify(p: &NaturalPolynomial, a: &NaturalMatrix, pow: u64) -> NaturalMatrix {
    let b = p.mod_power_of_2_evaluate(a, pow);
    assert!(b.is_valid());
    assert_eq!(p.mod_power_of_2_evaluate(a.clone(), pow), b);
    assert_eq!(p.clone().mod_power_of_2_evaluate(a, pow), b);
    assert_eq!(p.clone().mod_power_of_2_evaluate(a.clone(), pow), b);
    assert_eq!(mod_power_of_2_evaluate_at_matrix_horner(p, a, pow), b);
    assert_eq!(
        mod_power_of_2_evaluate_at_matrix_paterson_stockmeyer(p, a, pow),
        b
    );
    assert_eq!(mod_power_of_2_evaluate_at_matrix_naive(p, a, pow), b);
    b
}

#[test]
fn test_mod_power_of_2_evaluate_at_matrix() {
    let test = |p, a, pow, out| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let a = NaturalMatrix::from_str(a).unwrap();
        assert_eq!(verify(&p, &a, pow).to_string(), out);
    };
    // - Horner and Paterson–Stockmeyer: len == 0
    // - dispatch: len < 5
    test("0", "[]_{0,0}", 3, "[]_{0,0}");
    // Modulo 2^0 the only reduced matrix is the zero matrix.
    test("0", "[0, 0; 0, 0]", 0, "[0, 0; 0, 0]");
    // - Horner and Paterson–Stockmeyer: len == 1
    test("5", "[1, 2; 3, 4]", 3, "[5, 0; 0, 5]");
    // - Horner and Paterson–Stockmeyer: ring.is_zero_matrix(c)
    // The value is the constant term times the identity matrix.
    test("3*x^2+x+4", "[0, 0; 0, 0]", 3, "[4, 0; 0, 4]");
    // - Horner and Paterson–Stockmeyer: len > 1 && !ring.is_zero_matrix(c)
    // - Paterson–Stockmeyer: lim == 1
    // - Paterson–Stockmeyer: top.is_empty()
    test("x^2+1", "[1, 1; 0, 1]", 3, "[2, 2; 0, 2]");
    // The characteristic polynomial of [1, 2; 3, 4] is x^2 - 5x - 2, which vanishes at the matrix.
    test("x^2+3*x+6", "[1, 2; 3, 4]", 3, "[0, 0; 0, 0]");
    // - Paterson–Stockmeyer: lim >= 2
    test(
        "x^3+x+1",
        "[1, 2, 0; 0, 1, 1; 1, 0, 2]",
        4,
        "[5, 8, 8; 4, 5, 8; 8, 8, 13]",
    );
    test(
        "x^2+1",
        "[18446744073709551615, 1; 0, 18446744073709551615]",
        64,
        "[2, 18446744073709551614; 0, 2]",
    );
    test(
        "x^2+1",
        "[1267650600228229401496703205375, 1; 0, 1267650600228229401496703205375]",
        100,
        "[2, 1267650600228229401496703205374; 0, 2]",
    );
}

#[test]
fn test_mod_power_of_2_evaluate_at_matrix_long() {
    let test = |p, a, pow, out| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let a = NaturalMatrix::from_str(a).unwrap();
        assert_eq!(verify(&p, &a, pow).to_string(), out);
    };
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
    // The matrix is not square.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_power_of_2_evaluate(&NaturalMatrix::from_str("[1, 2]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_at_matrix_fail_2() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_evaluate(&NaturalMatrix::from_str("[1]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_at_matrix_fail_3() {
    // An element of the matrix is not reduced.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_power_of_2_evaluate(NaturalMatrix::from_str("[8]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_at_matrix_fail_4() {
    // By reference, an element of the matrix is not reduced.
    (&NaturalPolynomial::from_str("x+1").unwrap())
        .mod_power_of_2_evaluate(&NaturalMatrix::from_str("[1, 0; 0, 8]").unwrap(), 3);
}

#[test]
fn mod_power_of_2_evaluate_at_matrix_properties() {
    natural_polynomial_natural_matrix_unsigned_triple_gen_var_1().test_properties(|(p, a, pow)| {
        let b = verify(&p, &a, pow);
        assert_eq!(b.height(), a.height());
        assert_eq!(b.width(), a.height());
        assert!(b.mod_power_of_2_is_reduced(pow));

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

        // This agrees with mod_evaluate mod 2^pow.
        assert_eq!((&p).mod_evaluate(&a, Natural::power_of_2(pow)), b);
    });

    natural_polynomial_natural_unsigned_triple_gen_var_1().test_properties(|(p, x, pow)| {
        // At a 1 x 1 matrix, this is evaluation at the matrix's element.
        let y = (&p).mod_power_of_2_evaluate(&x, pow);
        assert_eq!(
            p.mod_power_of_2_evaluate(NaturalMatrix::from_owned_elements(1, 1, vec![x]), pow),
            NaturalMatrix::from_owned_elements(1, 1, vec![y])
        );
    });

    unsigned_polynomial_unsigned_matrix_unsigned_triple_gen_var_2().test_properties(
        |(p, a, pow)| {
            // For a word-sized modulus, this agrees with the `UnsignedPolynomial` version.
            assert_eq!(
                NaturalPolynomial::from(&p).mod_power_of_2_evaluate(NaturalMatrix::from(&a), pow),
                NaturalMatrix::from(p.mod_power_of_2_evaluate(a, pow))
            );
        },
    );
}

#[test]
fn mod_power_of_2_evaluate_at_matrix_cayley_hamilton_properties() {
    natural_matrix_unsigned_pair_gen_var_6().test_properties(|(a, pow)| {
        // Every square matrix is a root of its characteristic polynomial.
        assert!(
            (&a).mod_power_of_2_characteristic_polynomial(pow)
                .mod_power_of_2_evaluate(&a, pow)
                .is_zero()
        );
    });
}
