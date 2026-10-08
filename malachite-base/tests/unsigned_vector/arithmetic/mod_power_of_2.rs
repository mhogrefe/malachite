// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Assign, ModPowerOf2IsReduced,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::{
    unsigned_vector_gen, unsigned_vector_unsigned_pair_gen_var_2,
};
use malachite_base::unsigned_vector::UnsignedVector;

#[test]
fn test_mod_power_of_2() {
    let test = |s, pow, out| {
        let v = UnsignedVector::<u64>::from_str(s).unwrap();
        // by reference
        assert_eq!((&v).mod_power_of_2(pow).to_string(), out);
        // by value
        assert_eq!(v.clone().mod_power_of_2(pow).to_string(), out);
        // in place
        let mut w = v;
        w.mod_power_of_2_assign(pow);
        assert_eq!(w.to_string(), out);
    };
    test("()", 0, "()");
    test("()", 8, "()");
    test("(0)", 3, "(0)");
    test("(1, 3, 2)", 1, "(1, 1, 0)");
    test("(1, 3, 2)", 2, "(1, 3, 2)");
    // Modulo 2^0 every element is zero, but the dimension is unchanged.
    test("(1, 3, 2)", 0, "(0, 0, 0)");
    // Elements that reduce to zero stay, wherever they are.
    test("(4, 3)", 2, "(0, 3)");
    test("(3, 4)", 2, "(3, 0)");
    test("(4, 4, 4)", 2, "(0, 0, 0)");
    test("(8, 9, 10, 1)", 3, "(0, 1, 2, 1)");
    // A power at least as wide as a `u64` leaves the vector alone.
    test("(18446744073709551615, 1)", 64, "(18446744073709551615, 1)");
    test(
        "(18446744073709551615, 1)",
        100,
        "(18446744073709551615, 1)",
    );
    test("(18446744073709551615, 1)", 63, "(9223372036854775807, 1)");
}

#[test]
fn test_mod_power_of_2_u8() {
    let test = |s, pow, out| {
        assert_eq!(
            UnsignedVector::<u8>::from_str(s)
                .unwrap()
                .mod_power_of_2(pow)
                .to_string(),
            out
        );
    };
    test("(255, 128)", 7, "(127, 0)");
    test("(255, 128)", 8, "(255, 128)");
    test("(255, 128)", 9, "(255, 128)");
}

#[test]
fn mod_power_of_2_properties() {
    unsigned_vector_unsigned_pair_gen_var_2().test_properties(|(v, pow)| {
        let w = (&v).mod_power_of_2(pow);
        // The three forms agree.
        assert_eq!(v.clone().mod_power_of_2(pow), w);
        let mut x = v.clone();
        x.mod_power_of_2_assign(pow);
        assert_eq!(x, w);

        // The dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());
        // The result is reduced, and reducing it again changes nothing.
        assert!(w.mod_power_of_2_is_reduced(pow));
        assert_eq!((&w).mod_power_of_2(pow), w);

        // Element by element, this is the `u64` operation.
        for (&x, &y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(y, x.mod_power_of_2(pow));
        }

        // A vector that is already reduced is left alone.
        assert_eq!(v.mod_power_of_2_is_reduced(pow), v == w);
    });

    unsigned_vector_gen().test_properties(|v| {
        // A power at least as wide as a `u64` changes nothing, since no element reaches it.
        assert_eq!((&v).mod_power_of_2(u64::WIDTH), v);
        // Modulo 2^0 every element vanishes, but the dimension is unchanged.
        assert!((&v).mod_power_of_2(0).elements.iter().all(|&x| x == 0));
        // Reducing to the width of the largest element leaves the vector alone, and to one bit less
        // does not, unless every element was zero already.
        let bits = v
            .elements
            .iter()
            .map(|x| x.significant_bits())
            .max()
            .unwrap_or(0);
        assert_eq!((&v).mod_power_of_2(bits), v);
        if bits != 0 {
            assert_ne!((&v).mod_power_of_2(bits - 1), v);
        }
    });
}
