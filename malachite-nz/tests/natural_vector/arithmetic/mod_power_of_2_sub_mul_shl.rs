// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2AddMulShl, ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2Shl, ModPowerOf2SubMul,
    ModPowerOf2SubMulShl, ModPowerOf2SubMulShlAssign,
};
use malachite_base::num::basic::traits::One;
use malachite_base::test_util::generators::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_1,
    natural_vector_natural_vector_natural_unsigned_unsigned_quintuple_gen_var_1,
};

#[test]
fn test_mod_power_of_2_sub_mul_shl() {
    let test = |s, t, c, bits, pow, out| {
        let u = NaturalVector::from_str(s).unwrap();
        let v = NaturalVector::from_str(t).unwrap();
        let c = Natural::from_str(c).unwrap();
        let w = (&u).mod_power_of_2_sub_mul_shl(&v, &c, bits, pow);
        assert_eq!(w.to_string(), out);
        assert_eq!(
            u.clone()
                .mod_power_of_2_sub_mul_shl(v.clone(), c.clone(), bits, pow),
            w
        );
        assert_eq!(
            u.clone()
                .mod_power_of_2_sub_mul_shl(v.clone(), &c, bits, pow),
            w
        );
        assert_eq!(
            u.clone()
                .mod_power_of_2_sub_mul_shl(&v, c.clone(), bits, pow),
            w
        );
        assert_eq!(u.clone().mod_power_of_2_sub_mul_shl(&v, &c, bits, pow), w);
        let mut x = u.clone();
        x.mod_power_of_2_sub_mul_shl_assign(v.clone(), c.clone(), bits, pow);
        assert_eq!(x, w);
        let mut x = u.clone();
        x.mod_power_of_2_sub_mul_shl_assign(v.clone(), &c, bits, pow);
        assert_eq!(x, w);
        let mut x = u.clone();
        x.mod_power_of_2_sub_mul_shl_assign(&v, c.clone(), bits, pow);
        assert_eq!(x, w);
        let mut x = u;
        x.mod_power_of_2_sub_mul_shl_assign(&v, &c, bits, pow);
        assert_eq!(x, w);
    };
    test("()", "()", "5", 1, 3, "()");
    test("(0, 0)", "(0, 0)", "0", 0, 0, "(0, 0)");
    test("(5, 1, 3)", "(2, 7, 4)", "0", 1, 4, "(5, 1, 3)");
    test("(5, 1, 3)", "(2, 7, 4)", "3", 0, 4, "(15, 12, 7)");
    test("(5, 1, 3)", "(2, 7, 4)", "3", 1, 4, "(9, 7, 11)");
    test("(5, 1, 3)", "(2, 7, 4)", "3", 4, 4, "(5, 1, 3)");
    test(
        "(1, 18446744073709551616)",
        "(18446744073709551615, 3)",
        "18446744073709551617",
        3,
        70,
        "(9, 756316507022091616232)",
    );
    test(
        "(1267650600228229401496703205375, 0)",
        "(1267650600228229401496703205375, 1)",
        "633825300114114700748351602688",
        1,
        100,
        "(1267650600228229401496703205375, 0)",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_mul_shl_fail_1() {
    // An element of the first vector is not reduced.
    NaturalVector::from_str("(8)")
        .unwrap()
        .mod_power_of_2_sub_mul_shl(NaturalVector::from_str("(1)").unwrap(), Natural::ONE, 1, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_mul_shl_fail_2() {
    // An element of the second vector is not reduced.
    (&NaturalVector::from_str("(1)").unwrap()).mod_power_of_2_sub_mul_shl(
        &NaturalVector::from_str("(8)").unwrap(),
        &Natural::ONE,
        1,
        3,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_mul_shl_fail_3() {
    // The scalar is not reduced.
    let mut v = NaturalVector::from_str("(1)").unwrap();
    v.mod_power_of_2_sub_mul_shl_assign(
        NaturalVector::from_str("(1)").unwrap(),
        Natural::from(8u32),
        1,
        3,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_mul_shl_fail_4() {
    // The dimensions differ.
    let mut v = NaturalVector::from_str("(1, 2)").unwrap();
    v.mod_power_of_2_sub_mul_shl_assign(
        &NaturalVector::from_str("(1)").unwrap(),
        &Natural::ONE,
        1,
        3,
    );
}

#[test]
fn mod_power_of_2_sub_mul_shl_properties() {
    natural_vector_natural_vector_natural_unsigned_unsigned_quintuple_gen_var_1().test_properties(
        |(u, v, c, bits, pow)| {
            let w = (&u).mod_power_of_2_sub_mul_shl(&v, &c, bits, pow);
            assert!(w.mod_power_of_2_is_reduced(pow));
            // The forms agree.
            assert_eq!(
                u.clone()
                    .mod_power_of_2_sub_mul_shl(v.clone(), c.clone(), bits, pow),
                w
            );
            assert_eq!(
                u.clone()
                    .mod_power_of_2_sub_mul_shl(v.clone(), &c, bits, pow),
                w
            );
            assert_eq!(
                u.clone()
                    .mod_power_of_2_sub_mul_shl(&v, c.clone(), bits, pow),
                w
            );
            assert_eq!(u.clone().mod_power_of_2_sub_mul_shl(&v, &c, bits, pow), w);
            let mut x = u.clone();
            x.mod_power_of_2_sub_mul_shl_assign(v.clone(), c.clone(), bits, pow);
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_power_of_2_sub_mul_shl_assign(v.clone(), &c, bits, pow);
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_power_of_2_sub_mul_shl_assign(&v, c.clone(), bits, pow);
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_power_of_2_sub_mul_shl_assign(&v, &c, bits, pow);
            assert_eq!(x, w);

            // Element by element it is the scalar mod_power_of_2_sub_mul_shl, and it is
            // `mod_power_of_2_sub_mul` by the shifted scalar.
            for ((x, y), z) in u.elements.iter().zip(&v.elements).zip(&w.elements) {
                assert_eq!(*z, x.mod_power_of_2_sub_mul_shl(y, &c, bits, pow));
            }
            assert_eq!(
                (&u).mod_power_of_2_sub_mul(&v, &(&c).mod_power_of_2_shl(bits, pow), pow),
                w
            );
            // Negating the scalar gives the opposite operation, which undoes this one.
            assert_eq!(
                (&u).mod_power_of_2_add_mul_shl(&v, &(&c).mod_power_of_2_neg(pow), bits, pow),
                w
            );
            assert_eq!((&w).mod_power_of_2_add_mul_shl(&v, &c, bits, pow), u);
            // Shifting by at least `pow` changes nothing.
            if bits >= pow {
                assert_eq!(w, u);
            }
        },
    );

    natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_1().test_properties(
        |(u, v, c, pow)| {
            // With no shift, this is `mod_power_of_2_sub_mul`.
            assert_eq!(
                (&u).mod_power_of_2_sub_mul_shl(&v, &c, 0, pow),
                (&u).mod_power_of_2_sub_mul(&v, &c, pow)
            );
        },
    );

    unsigned_vector_unsigned_vector_unsigned_unsigned_unsigned_quintuple_gen_var_1::<Limb>()
        .test_properties(|(u, v, c, bits, pow)| {
            // It agrees with the UnsignedVector implementation.
            assert_eq!(
                NaturalVector::from(u.clone()).mod_power_of_2_sub_mul_shl(
                    NaturalVector::from(v.clone()),
                    Natural::from(c),
                    bits,
                    pow
                ),
                NaturalVector::from(u.mod_power_of_2_sub_mul_shl(v, c, bits, pow))
            );
        });
}
