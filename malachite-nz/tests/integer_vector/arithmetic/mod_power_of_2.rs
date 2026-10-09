// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    Mod, ModPowerOf2, ModPowerOf2IsReduced, PowerOf2, RemPowerOf2, RemPowerOf2Assign,
};
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    integer_vector_gen, integer_vector_unsigned_pair_gen_var_2,
};

#[test]
fn test_mod_power_of_2() {
    let test = |s, pow, out| {
        let v = IntegerVector::from_str(s).unwrap();
        let w = (&v).mod_power_of_2(pow);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.mod_power_of_2(pow), w);
    };
    test("()", 0, "()");
    test("()", 8, "()");
    test("(0)", 3, "(0)");
    // Negative elements become non-negative.
    test("(1, -3, -2)", 2, "(1, 1, 2)");
    // Modulo 2^0 every element is zero, but the dimension is unchanged.
    test("(1, -3, -2)", 0, "(0, 0, 0)");
    // A negative multiple of the modulus becomes zero, and stays.
    test("(-4, 3)", 2, "(0, 3)");
    // Powers at and beyond a word.
    test("(-1)", 64, "(18446744073709551615)");
    test("(-1)", 100, "(1267650600228229401496703205375)");
    test("(-18446744073709551616)", 64, "(0)");
}

#[test]
fn test_rem_power_of_2() {
    let test = |s, pow, out| {
        let v = IntegerVector::from_str(s).unwrap();
        let w = (&v).rem_power_of_2(pow);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().rem_power_of_2(pow), w);
        let mut x = v;
        x.rem_power_of_2_assign(pow);
        assert_eq!(x, w);
    };
    test("()", 0, "()");
    test("()", 8, "()");
    test("(0)", 3, "(0)");
    // Every element keeps its sign.
    test("(1, -7, -2)", 2, "(1, -3, -2)");
    // Modulo 2^0 every element is zero, but the dimension is unchanged.
    test("(1, -3)", 0, "(0, 0)");
    test("(-4, -3)", 2, "(0, -3)");
    // Powers at and beyond a word.
    test("(-1)", 64, "(-1)");
    test("(-18446744073709551617)", 64, "(-1)");
    test("(-18446744073709551617)", 100, "(-18446744073709551617)");
}

#[test]
fn mod_power_of_2_properties() {
    integer_vector_unsigned_pair_gen_var_2().test_properties(|(v, pow)| {
        let w = (&v).mod_power_of_2(pow);
        assert_eq!(v.clone().mod_power_of_2(pow), w);
        // The result is reduced, and the dimension is unchanged.
        assert!(w.mod_power_of_2_is_reduced(pow));
        assert_eq!(w.dimension(), v.dimension());
        // Element by element, this is the `Integer` operation.
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x.mod_power_of_2(pow));
        }
        // It is `mod_op` with a power-of-2 modulus.
        assert_eq!((&v).mod_op(Natural::power_of_2(pow)), w);
        // On a vector with no negative elements, this is the `NaturalVector` operation.
        if let Ok(n) = NaturalVector::try_from(&v) {
            assert_eq!(n.mod_power_of_2(pow), w);
        }
        // Taking the signed remainder first changes nothing.
        assert_eq!((&v).rem_power_of_2(pow).mod_power_of_2(pow), w);
    });

    integer_vector_gen().test_properties(|v| {
        // Modulo 2^0 every element vanishes, but the dimension is unchanged.
        assert_eq!((&v).mod_power_of_2(0), NaturalVector::zero(v.dimension()));
        for pow in [64, 100] {
            assert_eq!(
                (&v).mod_power_of_2(pow),
                (&v).mod_op(Natural::power_of_2(pow))
            );
        }
    });
}

#[test]
fn rem_power_of_2_properties() {
    integer_vector_unsigned_pair_gen_var_2().test_properties(|(v, pow)| {
        let w = (&v).rem_power_of_2(pow);
        assert_eq!(v.clone().rem_power_of_2(pow), w);
        let mut x = v.clone();
        x.rem_power_of_2_assign(pow);
        assert_eq!(x, w);
        assert_eq!(w.dimension(), v.dimension());
        // Element by element, this is the `Integer` operation.
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x.rem_power_of_2(pow));
        }
        // It is `%` with a power-of-2 modulus, and it commutes with negation.
        assert_eq!(&v % Integer::power_of_2(pow), w);
        assert_eq!((-&v).rem_power_of_2(pow), -&w);
        // Reducing again changes nothing.
        assert_eq!((&w).rem_power_of_2(pow), w);
    });
}
