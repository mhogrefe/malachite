// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModAddMulShl, ModIsReduced, ModNeg, ModShl, ModSubMul, ModSubMulShl, ModSubMulShlAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_vector_natural_vector_natural_natural_quadruple_gen_var_1,
    natural_vector_natural_vector_natural_unsigned_natural_quintuple_gen_var_1,
};

#[test]
fn test_mod_sub_mul_shl() {
    let test = |s, t, c, bits, m, out| {
        let u = NaturalVector::from_str(s).unwrap();
        let v = NaturalVector::from_str(t).unwrap();
        let c = Natural::from_str(c).unwrap();
        let m = Natural::from_str(m).unwrap();
        let w = (&u).mod_sub_mul_shl(&v, &c, bits, &m);
        assert_eq!(w.to_string(), out);
        assert_eq!(
            u.clone()
                .mod_sub_mul_shl(v.clone(), c.clone(), bits, m.clone()),
            w
        );
        assert_eq!(u.clone().mod_sub_mul_shl(v.clone(), c.clone(), bits, &m), w);
        assert_eq!(u.clone().mod_sub_mul_shl(v.clone(), &c, bits, m.clone()), w);
        assert_eq!(u.clone().mod_sub_mul_shl(v.clone(), &c, bits, &m), w);
        assert_eq!(u.clone().mod_sub_mul_shl(&v, c.clone(), bits, m.clone()), w);
        assert_eq!(u.clone().mod_sub_mul_shl(&v, c.clone(), bits, &m), w);
        assert_eq!(u.clone().mod_sub_mul_shl(&v, &c, bits, m.clone()), w);
        assert_eq!(u.clone().mod_sub_mul_shl(&v, &c, bits, &m), w);
        let mut x = u.clone();
        x.mod_sub_mul_shl_assign(v.clone(), c.clone(), bits, m.clone());
        assert_eq!(x, w);
        let mut x = u.clone();
        x.mod_sub_mul_shl_assign(v.clone(), c.clone(), bits, &m);
        assert_eq!(x, w);
        let mut x = u.clone();
        x.mod_sub_mul_shl_assign(v.clone(), &c, bits, m.clone());
        assert_eq!(x, w);
        let mut x = u.clone();
        x.mod_sub_mul_shl_assign(v.clone(), &c, bits, &m);
        assert_eq!(x, w);
        let mut x = u.clone();
        x.mod_sub_mul_shl_assign(&v, c.clone(), bits, m.clone());
        assert_eq!(x, w);
        let mut x = u.clone();
        x.mod_sub_mul_shl_assign(&v, c.clone(), bits, &m);
        assert_eq!(x, w);
        let mut x = u.clone();
        x.mod_sub_mul_shl_assign(&v, &c, bits, m.clone());
        assert_eq!(x, w);
        let mut x = u.clone();
        x.mod_sub_mul_shl_assign(&v, &c, bits, &m);
        assert_eq!(x, w);
    };
    test("()", "()", "5", 1, "7", "()");
    test("(0, 0)", "(0, 0)", "0", 0, "1", "(0, 0)");
    test("(5, 1, 3)", "(2, 6, 4)", "0", 1, "7", "(5, 1, 3)");
    test("(5, 1, 3)", "(2, 6, 4)", "3", 0, "7", "(6, 4, 5)");
    test("(5, 1, 3)", "(2, 6, 4)", "3", 2, "7", "(2, 6, 4)");
    test(
        "(1, 18446744073709551616)",
        "(18446744073709551615, 3)",
        "18446744073709551617",
        3,
        "1000000000000000000000000000057",
        "(632492292293003140701023067713, 999999999575724886304680312865)",
    );
    test(
        "(1267650600228229401496703205374, 0)",
        "(1267650600228229401496703205374, 1)",
        "1267650600228229401496703205374",
        500,
        "1267650600228229401496703205375",
        "(1267650600228229401496703205373, 1)",
    );
}

#[test]
#[should_panic]
fn mod_sub_mul_shl_fail_1() {
    // An element of the first vector is not reduced.
    NaturalVector::from_str("(7)").unwrap().mod_sub_mul_shl(
        NaturalVector::from_str("(1)").unwrap(),
        Natural::ONE,
        1,
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_sub_mul_shl_fail_2() {
    // The scalar is not reduced.
    let mut v = NaturalVector::from_str("(1)").unwrap();
    v.mod_sub_mul_shl_assign(
        NaturalVector::from_str("(1)").unwrap(),
        Natural::from(7u32),
        1,
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_sub_mul_shl_fail_3() {
    // The dimensions differ.
    let mut v = NaturalVector::from_str("(1, 2)").unwrap();
    v.mod_sub_mul_shl_assign(
        &NaturalVector::from_str("(1)").unwrap(),
        &Natural::ONE,
        1,
        &Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_sub_mul_shl_fail_4() {
    // The modulus is zero.
    let mut v = NaturalVector::from_str("()").unwrap();
    v.mod_sub_mul_shl_assign(
        &NaturalVector::from_str("()").unwrap(),
        &Natural::ZERO,
        1,
        &Natural::ZERO,
    );
}

#[test]
fn mod_sub_mul_shl_properties() {
    natural_vector_natural_vector_natural_unsigned_natural_quintuple_gen_var_1().test_properties(
        |(u, v, c, bits, m)| {
            let w = (&u).mod_sub_mul_shl(&v, &c, bits, &m);
            assert!(w.mod_is_reduced(&m));
            // The forms agree.
            assert_eq!(
                u.clone()
                    .mod_sub_mul_shl(v.clone(), c.clone(), bits, m.clone()),
                w
            );
            assert_eq!(u.clone().mod_sub_mul_shl(v.clone(), c.clone(), bits, &m), w);
            assert_eq!(u.clone().mod_sub_mul_shl(v.clone(), &c, bits, m.clone()), w);
            assert_eq!(u.clone().mod_sub_mul_shl(v.clone(), &c, bits, &m), w);
            assert_eq!(u.clone().mod_sub_mul_shl(&v, c.clone(), bits, m.clone()), w);
            assert_eq!(u.clone().mod_sub_mul_shl(&v, c.clone(), bits, &m), w);
            assert_eq!(u.clone().mod_sub_mul_shl(&v, &c, bits, m.clone()), w);
            assert_eq!(u.clone().mod_sub_mul_shl(&v, &c, bits, &m), w);
            let mut x = u.clone();
            x.mod_sub_mul_shl_assign(v.clone(), c.clone(), bits, m.clone());
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_sub_mul_shl_assign(v.clone(), c.clone(), bits, &m);
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_sub_mul_shl_assign(v.clone(), &c, bits, m.clone());
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_sub_mul_shl_assign(v.clone(), &c, bits, &m);
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_sub_mul_shl_assign(&v, c.clone(), bits, m.clone());
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_sub_mul_shl_assign(&v, c.clone(), bits, &m);
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_sub_mul_shl_assign(&v, &c, bits, m.clone());
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_sub_mul_shl_assign(&v, &c, bits, &m);
            assert_eq!(x, w);

            // Element by element it is the scalar mod_sub_mul_shl, and it is `mod_sub_mul` by the
            // shifted scalar.
            for ((x, y), z) in u.elements.iter().zip(&v.elements).zip(&w.elements) {
                assert_eq!(*z, x.mod_sub_mul_shl(y, &c, bits, &m));
            }
            assert_eq!((&u).mod_sub_mul(&v, &(&c).mod_shl(bits, &m), &m), w);
            // Negating the scalar gives the opposite operation, which undoes this one.
            assert_eq!((&u).mod_add_mul_shl(&v, &(&c).mod_neg(&m), bits, &m), w);
            assert_eq!((&w).mod_add_mul_shl(&v, &c, bits, &m), u);
        },
    );

    natural_vector_natural_vector_natural_natural_quadruple_gen_var_1().test_properties(
        |(u, v, c, m)| {
            // With no shift, this is `mod_sub_mul`.
            assert_eq!(
                (&u).mod_sub_mul_shl(&v, &c, 0, &m),
                (&u).mod_sub_mul(&v, &c, &m)
            );
        },
    );

    unsigned_vector_unsigned_vector_unsigned_unsigned_unsigned_quintuple_gen_var_2::<Limb>()
        .test_properties(|(u, v, c, bits, m)| {
            // It agrees with the UnsignedVector implementation.
            assert_eq!(
                NaturalVector::from(u.clone()).mod_sub_mul_shl(
                    NaturalVector::from(v.clone()),
                    Natural::from(c),
                    bits,
                    Natural::from(m)
                ),
                NaturalVector::from(u.mod_sub_mul_shl(v, c, bits, m))
            );
        });
}
