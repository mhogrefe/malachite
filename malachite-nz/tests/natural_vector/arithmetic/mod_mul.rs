// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModAdd, ModIsReduced, ModMul, ModMulAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::unsigned_vector_unsigned_unsigned_triple_gen_var_2;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_natural_natural_triple_gen_var_1;

#[test]
fn test_mod_mul() {
    let test = |s, c, m, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let c = Natural::from_str(c).unwrap();
        let m = Natural::from_str(m).unwrap();
        // All eight combinations of value and reference, and in place with all four.
        let w = (&v).mod_mul(&c, &m);
        assert_eq!(w.to_string(), out);
        assert_eq!((&v).mod_mul(&c, m.clone()), w);
        assert_eq!((&v).mod_mul(c.clone(), &m), w);
        assert_eq!((&v).mod_mul(c.clone(), m.clone()), w);
        assert_eq!(v.clone().mod_mul(&c, &m), w);
        assert_eq!(v.clone().mod_mul(&c, m.clone()), w);
        assert_eq!(v.clone().mod_mul(c.clone(), &m), w);
        assert_eq!(v.clone().mod_mul(c.clone(), m.clone()), w);
        let mut x = v.clone();
        x.mod_mul_assign(&c, &m);
        assert_eq!(x, w);
        let mut x = v.clone();
        x.mod_mul_assign(&c, m.clone());
        assert_eq!(x, w);
        let mut x = v.clone();
        x.mod_mul_assign(c.clone(), &m);
        assert_eq!(x, w);
        let mut x = v;
        x.mod_mul_assign(c, m);
        assert_eq!(x, w);
    };
    test("()", "5", "7", "()");
    test("(0, 0)", "0", "1", "(0, 0)");
    test("(5, 1, 3)", "0", "7", "(0, 0, 0)");
    test("(5, 1, 3)", "1", "7", "(5, 1, 3)");
    test("(5, 1, 3)", "3", "7", "(1, 3, 2)");
    test(
        "(18446744073709551615, 2)",
        "18446744073709551615",
        "18446744073709551616",
        "(1, 18446744073709551614)",
    );
    test(
        "(123456789012345678901234567890, 7)",
        "2",
        "1000000000000000000000000000000",
        "(246913578024691357802469135780, 14)",
    );
}

#[test]
#[should_panic]
fn mod_mul_fail_1() {
    // The modulus is zero, even though there is no element to reduce.
    let _ = NaturalVector::from_str("()")
        .unwrap()
        .mod_mul(Natural::ZERO, Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_mul_fail_2() {
    // An element is not reduced.
    let _ = (&NaturalVector::from_str("(7)").unwrap()).mod_mul(Natural::ONE, Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_mul_fail_3() {
    // The scalar is not reduced.
    let _ = NaturalVector::from_str("(1)")
        .unwrap()
        .mod_mul(&Natural::from(7u32), &Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_mul_assign_fail() {
    let mut v = NaturalVector::from_str("(1)").unwrap();
    v.mod_mul_assign(Natural::from(7u32), Natural::from(7u32));
}

#[test]
fn mod_mul_properties() {
    natural_vector_natural_natural_triple_gen_var_1().test_properties(|(v, c, m)| {
        let w = (&v).mod_mul(&c, &m);
        // The forms agree.
        assert_eq!((&v).mod_mul(&c, m.clone()), w);
        assert_eq!((&v).mod_mul(c.clone(), &m), w);
        assert_eq!((&v).mod_mul(c.clone(), m.clone()), w);
        assert_eq!(v.clone().mod_mul(&c, &m), w);
        assert_eq!(v.clone().mod_mul(c.clone(), m.clone()), w);
        let mut x = v.clone();
        x.mod_mul_assign(&c, &m);
        assert_eq!(x, w);
        let mut x = v.clone();
        x.mod_mul_assign(c.clone(), m.clone());
        assert_eq!(x, w);

        // The result is reduced, and is the ordinary product reduced.
        assert!(w.mod_is_reduced(&m));
        assert_eq!(w.dimension(), v.dimension());
        assert_eq!((&v * &c) % &m, w);
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x.mod_mul(&c, &m));
        }

        // Multiplying by 0 gives the zero vector, by 1 (when 1 is reduced) changes nothing,
        // multiplying twice is multiplying by the square, and it distributes over modular addition.
        assert_eq!(
            (&v).mod_mul(Natural::ZERO, &m),
            NaturalVector::zero(v.dimension())
        );
        if m != 1u32 {
            assert_eq!((&v).mod_mul(Natural::ONE, &m), v);
        }
        assert_eq!((&v).mod_mul((&c).mod_mul(&c, &m), &m), (&w).mod_mul(&c, &m));
        assert_eq!((&v).mod_add(&v, &m).mod_mul(&c, &m), (&w).mod_add(&w, &m));
    });

    unsigned_vector_unsigned_unsigned_triple_gen_var_2::<u64>().test_properties(|(v, c, m)| {
        // The `u64` elements multiply as their `Natural` counterparts do.
        assert_eq!(
            NaturalVector::from(v.clone()).mod_mul(Natural::from(c), Natural::from(m)),
            NaturalVector::from(v.mod_mul(c, m))
        );
    });
}
