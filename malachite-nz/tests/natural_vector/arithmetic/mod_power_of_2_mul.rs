// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2MulAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::unsigned_vector_unsigned_unsigned_triple_gen_var_1;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_natural_unsigned_triple_gen_var_1;

#[test]
fn test_mod_power_of_2_mul() {
    let test = |s, c, pow, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let c = Natural::from_str(c).unwrap();
        // All four combinations of value and reference, and in place with both.
        let w = (&v).mod_power_of_2_mul(&c, pow);
        assert_eq!(w.to_string(), out);
        assert_eq!((&v).mod_power_of_2_mul(c.clone(), pow), w);
        assert_eq!(v.clone().mod_power_of_2_mul(&c, pow), w);
        assert_eq!(v.clone().mod_power_of_2_mul(c.clone(), pow), w);
        let mut x = v.clone();
        x.mod_power_of_2_mul_assign(&c, pow);
        assert_eq!(x, w);
        let mut x = v;
        x.mod_power_of_2_mul_assign(c, pow);
        assert_eq!(x, w);
    };
    test("()", "5", 3, "()");
    test("(0, 0)", "0", 0, "(0, 0)");
    test("(5, 1, 3)", "0", 3, "(0, 0, 0)");
    test("(5, 1, 3)", "1", 3, "(5, 1, 3)");
    test("(5, 1, 3)", "3", 3, "(7, 3, 1)");
    test(
        "(18446744073709551615, 1)",
        "18446744073709551615",
        64,
        "(1, 18446744073709551615)",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_fail_1() {
    // An element is not reduced.
    let _ = NaturalVector::from_str("(8)")
        .unwrap()
        .mod_power_of_2_mul(Natural::ONE, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_fail_2() {
    // The scalar is not reduced.
    let _ = (&NaturalVector::from_str("(1)").unwrap()).mod_power_of_2_mul(&Natural::from(8u32), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_assign_fail() {
    let mut v = NaturalVector::from_str("(1)").unwrap();
    v.mod_power_of_2_mul_assign(Natural::from(8u32), 3);
}

// The empty vector has no elements to check, so only the scalar check makes this panic.
#[test]
#[should_panic]
fn mod_power_of_2_mul_empty_vector_fail() {
    let _ = NaturalVector::from_str("()")
        .unwrap()
        .mod_power_of_2_mul(Natural::from(8u32), 3);
}

#[test]
fn mod_power_of_2_mul_properties() {
    natural_vector_natural_unsigned_triple_gen_var_1().test_properties(|(v, c, pow)| {
        let w = (&v).mod_power_of_2_mul(&c, pow);
        // The forms agree.
        assert_eq!((&v).mod_power_of_2_mul(c.clone(), pow), w);
        assert_eq!(v.clone().mod_power_of_2_mul(&c, pow), w);
        assert_eq!(v.clone().mod_power_of_2_mul(c.clone(), pow), w);
        let mut x = v.clone();
        x.mod_power_of_2_mul_assign(&c, pow);
        assert_eq!(x, w);
        let mut x = v.clone();
        x.mod_power_of_2_mul_assign(c.clone(), pow);
        assert_eq!(x, w);

        // The result is reduced, and is the ordinary product reduced.
        assert!(w.mod_power_of_2_is_reduced(pow));
        assert_eq!(w.dimension(), v.dimension());
        assert_eq!((&v * &c).mod_power_of_2(pow), w);
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x.mod_power_of_2_mul(&c, pow));
        }

        // Multiplying by 0 gives the zero vector, by 1 (when 1 is reduced) changes nothing,
        // multiplying twice is multiplying by the square, and it distributes over modular addition.
        assert_eq!(
            (&v).mod_power_of_2_mul(Natural::ZERO, pow),
            NaturalVector::zero(v.dimension())
        );
        if pow != 0 {
            assert_eq!((&v).mod_power_of_2_mul(Natural::ONE, pow), v);
        }
        assert_eq!(
            (&v).mod_power_of_2_mul((&c).mod_power_of_2_mul(&c, pow), pow),
            (&w).mod_power_of_2_mul(&c, pow)
        );
        assert_eq!(
            (&v).mod_power_of_2_add(&v, pow).mod_power_of_2_mul(&c, pow),
            (&w).mod_power_of_2_add(&w, pow)
        );
    });

    unsigned_vector_unsigned_unsigned_triple_gen_var_1::<u64>().test_properties(|(v, c, pow)| {
        // The `u64` elements multiply as their `Natural` counterparts do.
        assert_eq!(
            NaturalVector::from(v.clone()).mod_power_of_2_mul(Natural::from(c), pow),
            NaturalVector::from(v.mod_power_of_2_mul(c, pow))
        );
    });
}
