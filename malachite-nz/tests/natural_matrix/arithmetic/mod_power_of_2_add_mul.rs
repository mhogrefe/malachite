// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Add, ModPowerOf2AddMul, ModPowerOf2AddMulAssign, ModPowerOf2IsReduced,
    ModPowerOf2Mul, ModPowerOf2Neg, ModPowerOf2SubMul,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_matrix_natural_unsigned_quadruple_gen_var_1,
    natural_matrix_natural_matrix_unsigned_triple_gen_var_1,
    natural_matrix_natural_unsigned_triple_gen_var_1,
};

#[test]
fn test_mod_power_of_2_add_mul() {
    let test = |s, t, c, pow, out| {
        let u = NaturalMatrix::from_str(s).unwrap();
        let v = NaturalMatrix::from_str(t).unwrap();
        let c = Natural::from_str(c).unwrap();
        let b = (&u).mod_power_of_2_add_mul(&v, &c, pow);
        assert_eq!(b.to_string(), out);
        assert_eq!(
            u.clone().mod_power_of_2_add_mul(v.clone(), c.clone(), pow),
            b
        );
        assert_eq!(u.clone().mod_power_of_2_add_mul(v.clone(), &c, pow), b);
        assert_eq!(u.clone().mod_power_of_2_add_mul(&v, c.clone(), pow), b);
        assert_eq!(u.clone().mod_power_of_2_add_mul(&v, &c, pow), b);
        let mut x = u.clone();
        x.mod_power_of_2_add_mul_assign(v.clone(), c.clone(), pow);
        assert_eq!(x, b);
        let mut x = u.clone();
        x.mod_power_of_2_add_mul_assign(v.clone(), &c, pow);
        assert_eq!(x, b);
        let mut x = u.clone();
        x.mod_power_of_2_add_mul_assign(&v, c.clone(), pow);
        assert_eq!(x, b);
        let mut x = u;
        x.mod_power_of_2_add_mul_assign(&v, &c, pow);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", "[]_{0,0}", "5", 3, "[]_{0,0}");
    test("[0, 0]", "[0, 0]", "0", 0, "[0, 0]");
    test("[5, 1, 3]", "[2, 7, 4]", "0", 3, "[5, 1, 3]");
    test("[5, 1, 3]", "[2, 7, 4]", "1", 3, "[7, 0, 7]");
    test("[5, 1, 3]", "[2, 7, 4]", "3", 3, "[3, 6, 7]");
    test(
        "[1, 18446744073709551616]",
        "[18446744073709551615, 3]",
        "18446744073709551617",
        70,
        "[0, 73786976294838206467]",
    );
    test(
        "[1267650600228229401496703205375, 0]",
        "[1267650600228229401496703205375, 1]",
        "633825300114114700748351602688",
        100,
        "[633825300114114700748351602687, 633825300114114700748351602688]",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_add_mul_fail_1() {
    // An element of the first matrix is not reduced.
    NaturalMatrix::from_str("[8]")
        .unwrap()
        .mod_power_of_2_add_mul(NaturalMatrix::from_str("[1]").unwrap(), Natural::ONE, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_mul_fail_2() {
    // An element of the second matrix is not reduced.
    (&NaturalMatrix::from_str("[1]").unwrap()).mod_power_of_2_add_mul(
        &NaturalMatrix::from_str("[8]").unwrap(),
        &Natural::ONE,
        3,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_add_mul_fail_3() {
    // The scalar is not reduced.
    let mut v = NaturalMatrix::from_str("[1]").unwrap();
    v.mod_power_of_2_add_mul_assign(
        NaturalMatrix::from_str("[1]").unwrap(),
        Natural::from(8u32),
        3,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_add_mul_fail_4() {
    // The dimensions differ.
    let mut v = NaturalMatrix::from_str("[1, 2]").unwrap();
    v.mod_power_of_2_add_mul_assign(&NaturalMatrix::from_str("[1]").unwrap(), &Natural::ONE, 3);
}

#[test]
fn mod_power_of_2_add_mul_properties() {
    natural_matrix_natural_matrix_natural_unsigned_quadruple_gen_var_1().test_properties(
        |(u, v, c, pow)| {
            let b = (&u).mod_power_of_2_add_mul(&v, &c, pow);
            assert!(b.mod_power_of_2_is_reduced(pow));
            // The forms agree.
            assert_eq!(
                u.clone().mod_power_of_2_add_mul(v.clone(), c.clone(), pow),
                b
            );
            assert_eq!(u.clone().mod_power_of_2_add_mul(v.clone(), &c, pow), b);
            assert_eq!(u.clone().mod_power_of_2_add_mul(&v, c.clone(), pow), b);
            assert_eq!(u.clone().mod_power_of_2_add_mul(&v, &c, pow), b);
            let mut x = u.clone();
            x.mod_power_of_2_add_mul_assign(v.clone(), c.clone(), pow);
            assert_eq!(x, b);
            let mut x = u.clone();
            x.mod_power_of_2_add_mul_assign(v.clone(), &c, pow);
            assert_eq!(x, b);
            let mut x = u.clone();
            x.mod_power_of_2_add_mul_assign(&v, c.clone(), pow);
            assert_eq!(x, b);
            let mut x = u.clone();
            x.mod_power_of_2_add_mul_assign(&v, &c, pow);
            assert_eq!(x, b);

            // It is the unfused combination, element by element the scalar mod_power_of_2_add_mul,
            // and the shape is unchanged.
            assert_eq!(
                b,
                (&u).mod_power_of_2_add(&(&v).mod_power_of_2_mul(&c, pow), pow)
            );
            assert_eq!((b.height(), b.width()), (u.height(), u.width()));
            for ((x, y), z) in u
                .elements_ref()
                .iter()
                .zip(v.elements_ref())
                .zip(b.elements_ref())
            {
                assert_eq!(*z, x.mod_power_of_2_add_mul(y, &c, pow));
            }
            // Negating the scalar gives the opposite operation, which undoes this one.
            assert_eq!(
                (&u).mod_power_of_2_sub_mul(&v, &(&c).mod_power_of_2_neg(pow), pow),
                b
            );
            assert_eq!((&b).mod_power_of_2_sub_mul(&v, &c, pow), u);
            // Negating everything negates the result.
            assert_eq!(
                (&u).mod_power_of_2_neg(pow).mod_power_of_2_add_mul(
                    (&v).mod_power_of_2_neg(pow),
                    &c,
                    pow
                ),
                (&b).mod_power_of_2_neg(pow)
            );
        },
    );

    natural_matrix_natural_matrix_unsigned_triple_gen_var_1().test_properties(|(u, v, pow)| {
        // A zero scalar changes nothing, and a scalar of 1 leaves the sum.
        assert_eq!((&u).mod_power_of_2_add_mul(&v, &Natural::ZERO, pow), u);
        if pow != 0 {
            assert_eq!(
                (&u).mod_power_of_2_add_mul(&v, &Natural::ONE, pow),
                (&u).mod_power_of_2_add(&v, pow)
            );
        }
    });

    natural_matrix_natural_unsigned_triple_gen_var_1().test_properties(|(v, c, pow)| {
        // Starting from the zero matrix gives the scalar product.
        assert_eq!(
            NaturalMatrix::zero(v.height(), v.width()).mod_power_of_2_add_mul(&v, &c, pow),
            (&v).mod_power_of_2_mul(&c, pow)
        );
    });

    unsigned_matrix_unsigned_matrix_unsigned_unsigned_quadruple_gen_var_2::<Limb>()
        .test_properties(|(u, v, c, pow)| {
            // It agrees with the UnsignedMatrix implementation.
            assert_eq!(
                NaturalMatrix::from(u.clone()).mod_power_of_2_add_mul(
                    NaturalMatrix::from(v.clone()),
                    Natural::from(c),
                    pow
                ),
                NaturalMatrix::from(u.mod_power_of_2_add_mul(v, c, pow))
            );
        });
}
