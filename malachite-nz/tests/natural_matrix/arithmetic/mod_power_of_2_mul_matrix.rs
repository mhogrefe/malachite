// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Mul, ModPowerOf2MulAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_matrix_unsigned_triple_gen_var_2, natural_matrix_unsigned_pair_gen_var_4,
};
use malachite_nz::test_util::natural_matrix::arithmetic::mod_power_of_2_mul_matrix::*;

#[test]
fn test_mod_power_of_2_mul_matrix() {
    let test = |s, t, pow, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let b = NaturalMatrix::from_str(t).unwrap();
        let c = a.clone().mod_power_of_2_mul(b.clone(), pow);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(a.clone().mod_power_of_2_mul(&b, pow), c);
        assert_eq!((&a).mod_power_of_2_mul(b.clone(), pow), c);
        assert_eq!((&a).mod_power_of_2_mul(&b, pow), c);
        let mut d = a.clone();
        d.mod_power_of_2_mul_assign(b.clone(), pow);
        assert!(d.is_valid());
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_power_of_2_mul_assign(&b, pow);
        assert_eq!(d, c);
        assert_eq!(
            natural_matrix_mod_power_of_2_mul_matrix_naive(&a, &b, pow),
            c
        );
    };
    test("[]_{0,0}", "[]_{0,0}", 3, "[]_{0,0}");
    test("[]_{0,3}", "[1, 2; 3, 4; 0, 1]", 3, "[]_{0,2}");
    test("[]_{2,0}", "[]_{0,3}", 3, "[0, 0, 0; 0, 0, 0]");
    test("[1, 2, 3; 4, 5, 6]", "[]_{3,0}", 3, "[]_{2,0}");
    test("[3]", "[4]", 3, "[4]");
    test("[1; 2; 3]", "[4, 5]", 3, "[4, 5; 0, 2; 4, 7]");
    test("[1, 2; 3, 4]", "[5, 6; 7, 8]", 4, "[3, 6; 11, 2]");
    test(
        "[1, 2, 3; 4, 5, 6]",
        "[6, 5; 4, 3; 2, 1]",
        5,
        "[20, 14; 24, 9]",
    );
    test("[0, 0; 0, 0]", "[0, 0; 0, 0]", 0, "[0, 0; 0, 0]");
    test(
        "[200, 250; 100, 7]",
        "[254, 253; 3, 128]",
        8,
        "[94, 168; 77, 84]",
    );
    test(
        "[127, 99; 64, 1]",
        "[3, 100; 127, 126]",
        7,
        "[26, 86; 63, 126]",
    );
    test("[1, 1; 1, 0]", "[1, 1; 1, 0]", 1, "[0, 1; 1, 1]");
    test(
        "[18446744073709551615, 9223372036854775808; 5, 18446744073709551616]",
        "[18446744073709551619, 7; 36893488147419103231, 1]",
        66,
        "[27670116110564327421, 64563604257983430649; 15, 18446744073709551651]",
    );
    test(
        "[1267650600228229401496703205375, 3; 633825300114114700748351602688, 18446744073709551616]\
        ",
        "[1267650600228229401496703205375, 633825300114114700748351602693; 17, 12676506002282294014\
        96703205373]",
        100,
        "[52, 633825300114114700748351602674; 633825300427709350001413980160, 633825300058774468527\
        222947840]",
    );
    test(
        "[170141183460469231731687303715884105728, 1; 3, 170141183460469231731687303715884105739]",
        "[340282366920938463463374607431768211455, 2; 170141183460469231731687303715884105728, 9]",
        128,
        "[0, 9; 170141183460469231731687303715884105725, 170141183460469231731687303715884105833]",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_matrix_fail_1() {
    // The shapes are incompatible.
    let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    let _ = (&a).mod_power_of_2_mul(&a, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_matrix_fail_2() {
    // The first matrix is not reduced.
    let a = NaturalMatrix::from_str("[8]").unwrap();
    let b = NaturalMatrix::from_str("[1]").unwrap();
    let _ = a.mod_power_of_2_mul(b, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_matrix_fail_3() {
    // The second matrix is not reduced.
    let a = NaturalMatrix::from_str("[1]").unwrap();
    let b = NaturalMatrix::from_str("[8]").unwrap();
    let _ = a.mod_power_of_2_mul(b, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_matrix_assign_fail() {
    // The shapes are incompatible.
    let mut a = NaturalMatrix::from_str("[1; 2]").unwrap();
    a.mod_power_of_2_mul_assign(NaturalMatrix::from_str("[1; 2]").unwrap(), 100);
}

#[test]
fn mod_power_of_2_mul_matrix_properties() {
    natural_matrix_natural_matrix_unsigned_triple_gen_var_2().test_properties(|(a, b, pow)| {
        let c = (&a).mod_power_of_2_mul(&b, pow);
        assert!(c.is_valid());
        assert_eq!((c.height(), c.width()), (a.height(), b.width()));
        assert_eq!(a.clone().mod_power_of_2_mul(b.clone(), pow), c);
        assert_eq!(a.clone().mod_power_of_2_mul(&b, pow), c);
        assert_eq!((&a).mod_power_of_2_mul(b.clone(), pow), c);
        let mut d = a.clone();
        d.mod_power_of_2_mul_assign(b.clone(), pow);
        assert!(d.is_valid());
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_power_of_2_mul_assign(&b, pow);
        assert_eq!(d, c);
        assert_eq!(
            natural_matrix_mod_power_of_2_mul_matrix_naive(&a, &b, pow),
            c
        );
        // The product modulo 2^k is the exact product, reduced.
        assert_eq!((&a * &b).mod_power_of_2(pow), c);
        // When 2^k fits in a limb, the product agrees with the product of `UnsignedMatrix`es.
        if pow <= Limb::WIDTH {
            let a_limbs = UnsignedMatrix::<Limb>::try_from(&a).unwrap();
            let b_limbs = UnsignedMatrix::<Limb>::try_from(&b).unwrap();
            assert_eq!(
                NaturalMatrix::from(a_limbs.mod_power_of_2_mul(b_limbs, pow)),
                c
            );
        }
        // The transpose of a product is the product of the transposes in the other order.
        assert_eq!(
            b.transpose().mod_power_of_2_mul(a.transpose(), pow),
            c.transpose()
        );
    });

    natural_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        let (h, w) = (a.height(), a.width());
        // Multiplying by the identity matrix leaves a matrix unchanged, unless 2^k is 1.
        if pow != 0 {
            assert_eq!(NaturalMatrix::identity(h).mod_power_of_2_mul(&a, pow), a);
            assert_eq!((&a).mod_power_of_2_mul(NaturalMatrix::identity(w), pow), a);
        }
        // Zero matrices give zero products.
        assert_eq!(
            (&a).mod_power_of_2_mul(NaturalMatrix::zero(w, 3), pow),
            NaturalMatrix::zero(h, 3)
        );
    });
}
