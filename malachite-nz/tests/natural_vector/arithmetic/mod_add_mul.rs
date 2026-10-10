// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModAdd, ModAddMul, ModAddMulAssign, ModIsReduced, ModMul, ModNeg, ModSubMul,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::*;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_vector_natural_natural_triple_gen_var_1,
    natural_vector_natural_vector_natural_natural_quadruple_gen_var_1,
    natural_vector_natural_vector_natural_triple_gen_var_1,
};

#[test]
fn test_mod_add_mul() {
    let test = |s, t, c, m, out| {
        let u = NaturalVector::from_str(s).unwrap();
        let v = NaturalVector::from_str(t).unwrap();
        let c = Natural::from_str(c).unwrap();
        let m = Natural::from_str(m).unwrap();
        let w = (&u).mod_add_mul(&v, &c, &m);
        assert_eq!(w.to_string(), out);
        assert_eq!(u.clone().mod_add_mul(v.clone(), c.clone(), m.clone()), w);
        assert_eq!(u.clone().mod_add_mul(v.clone(), c.clone(), &m), w);
        assert_eq!(u.clone().mod_add_mul(v.clone(), &c, m.clone()), w);
        assert_eq!(u.clone().mod_add_mul(v.clone(), &c, &m), w);
        assert_eq!(u.clone().mod_add_mul(&v, c.clone(), m.clone()), w);
        assert_eq!(u.clone().mod_add_mul(&v, c.clone(), &m), w);
        assert_eq!(u.clone().mod_add_mul(&v, &c, m.clone()), w);
        assert_eq!(u.clone().mod_add_mul(&v, &c, &m), w);
        let mut x = u.clone();
        x.mod_add_mul_assign(v.clone(), c.clone(), m.clone());
        assert_eq!(x, w);
        let mut x = u.clone();
        x.mod_add_mul_assign(v.clone(), c.clone(), &m);
        assert_eq!(x, w);
        let mut x = u.clone();
        x.mod_add_mul_assign(v.clone(), &c, m.clone());
        assert_eq!(x, w);
        let mut x = u.clone();
        x.mod_add_mul_assign(v.clone(), &c, &m);
        assert_eq!(x, w);
        let mut x = u.clone();
        x.mod_add_mul_assign(&v, c.clone(), m.clone());
        assert_eq!(x, w);
        let mut x = u.clone();
        x.mod_add_mul_assign(&v, c.clone(), &m);
        assert_eq!(x, w);
        let mut x = u.clone();
        x.mod_add_mul_assign(&v, &c, m.clone());
        assert_eq!(x, w);
        let mut x = u.clone();
        x.mod_add_mul_assign(&v, &c, &m);
        assert_eq!(x, w);
    };
    test("()", "()", "5", "7", "()");
    test("(0, 0)", "(0, 0)", "0", "1", "(0, 0)");
    test("(5, 1, 3)", "(2, 6, 4)", "0", "7", "(5, 1, 3)");
    test("(5, 1, 3)", "(2, 6, 4)", "1", "7", "(0, 0, 0)");
    test("(5, 1, 3)", "(2, 6, 4)", "6", "7", "(3, 2, 6)");
    test("(5, 1, 3)", "(2, 6, 4)", "3", "7", "(4, 5, 1)");
    test(
        "(1, 18446744073709551616)",
        "(18446744073709551615, 3)",
        "18446744073709551617",
        "1000000000000000000000000000057",
        "(920938463463374607412372116594, 73786976294838206467)",
    );
    test(
        "(1267650600228229401496703205374, 0)",
        "(1267650600228229401496703205374, 1)",
        "1267650600228229401496703205374",
        "1267650600228229401496703205375",
        "(0, 1267650600228229401496703205374)",
    );
}

#[test]
#[should_panic]
fn mod_add_mul_fail_1() {
    // An element of the first vector is not reduced.
    NaturalVector::from_str("(7)").unwrap().mod_add_mul(
        NaturalVector::from_str("(1)").unwrap(),
        Natural::ONE,
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_add_mul_fail_2() {
    // An element of the second vector is not reduced.
    (&NaturalVector::from_str("(1)").unwrap()).mod_add_mul(
        &NaturalVector::from_str("(7)").unwrap(),
        &Natural::ONE,
        &Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_add_mul_fail_3() {
    // The scalar is not reduced.
    let mut v = NaturalVector::from_str("(1)").unwrap();
    v.mod_add_mul_assign(
        NaturalVector::from_str("(1)").unwrap(),
        Natural::from(7u32),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_add_mul_fail_4() {
    // The dimensions differ.
    let mut v = NaturalVector::from_str("(1, 2)").unwrap();
    v.mod_add_mul_assign(
        &NaturalVector::from_str("(1)").unwrap(),
        &Natural::ONE,
        &Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_add_mul_fail_5() {
    // The modulus is zero.
    let mut v = NaturalVector::from_str("()").unwrap();
    v.mod_add_mul_assign(
        &NaturalVector::from_str("()").unwrap(),
        &Natural::ZERO,
        &Natural::ZERO,
    );
}

#[test]
fn mod_add_mul_properties() {
    natural_vector_natural_vector_natural_natural_quadruple_gen_var_1().test_properties(
        |(u, v, c, m)| {
            let w = (&u).mod_add_mul(&v, &c, &m);
            assert!(w.mod_is_reduced(&m));
            // The forms agree.
            assert_eq!(u.clone().mod_add_mul(v.clone(), c.clone(), m.clone()), w);
            assert_eq!(u.clone().mod_add_mul(v.clone(), c.clone(), &m), w);
            assert_eq!(u.clone().mod_add_mul(v.clone(), &c, m.clone()), w);
            assert_eq!(u.clone().mod_add_mul(v.clone(), &c, &m), w);
            assert_eq!(u.clone().mod_add_mul(&v, c.clone(), m.clone()), w);
            assert_eq!(u.clone().mod_add_mul(&v, c.clone(), &m), w);
            assert_eq!(u.clone().mod_add_mul(&v, &c, m.clone()), w);
            assert_eq!(u.clone().mod_add_mul(&v, &c, &m), w);
            let mut x = u.clone();
            x.mod_add_mul_assign(v.clone(), c.clone(), m.clone());
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_add_mul_assign(v.clone(), c.clone(), &m);
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_add_mul_assign(v.clone(), &c, m.clone());
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_add_mul_assign(v.clone(), &c, &m);
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_add_mul_assign(&v, c.clone(), m.clone());
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_add_mul_assign(&v, c.clone(), &m);
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_add_mul_assign(&v, &c, m.clone());
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_add_mul_assign(&v, &c, &m);
            assert_eq!(x, w);

            // It is the unfused combination, element by element the scalar mod_add_mul, and the
            // dimension is unchanged.
            assert_eq!(w, (&u).mod_add(&(&v).mod_mul(&c, &m), &m));
            assert_eq!(w.dimension(), u.dimension());
            for ((x, y), z) in u.elements.iter().zip(&v.elements).zip(&w.elements) {
                assert_eq!(*z, x.mod_add_mul(y, &c, &m));
            }
            // Negating the scalar gives the opposite operation, which undoes this one.
            assert_eq!((&u).mod_sub_mul(&v, &(&c).mod_neg(&m), &m), w);
            assert_eq!((&w).mod_sub_mul(&v, &c, &m), u);
            // Negating everything negates the result.
            assert_eq!(
                (&u).mod_neg(&m).mod_add_mul((&v).mod_neg(&m), &c, &m),
                (&w).mod_neg(&m)
            );
        },
    );

    natural_vector_natural_vector_natural_triple_gen_var_1().test_properties(|(u, v, m)| {
        // A zero scalar changes nothing, and a scalar of 1 leaves the sum.
        assert_eq!((&u).mod_add_mul(&v, &Natural::ZERO, &m), u);
        if m != 1u32 {
            assert_eq!(
                (&u).mod_add_mul(&v, &Natural::ONE, &m),
                (&u).mod_add(&v, &m)
            );
        }
    });

    natural_vector_natural_natural_triple_gen_var_1().test_properties(|(v, c, m)| {
        // Starting from the zero vector gives the scalar product.
        assert_eq!(
            NaturalVector::zero(v.dimension()).mod_add_mul(&v, &c, &m),
            (&v).mod_mul(&c, &m)
        );
    });

    unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_2::<Limb>()
        .test_properties(|(u, v, c, m)| {
            // It agrees with the UnsignedVector implementation.
            assert_eq!(
                NaturalVector::from(u.clone()).mod_add_mul(
                    NaturalVector::from(v.clone()),
                    Natural::from(c),
                    Natural::from(m)
                ),
                NaturalVector::from(u.mod_add_mul(v, c, m))
            );
        });
}
