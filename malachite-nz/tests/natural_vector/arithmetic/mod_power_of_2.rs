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
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::unsigned_vector_gen;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_vector_gen, natural_vector_unsigned_pair_gen_var_2,
};

#[test]
fn test_mod_power_of_2() {
    let test = |s, pow, out| {
        let v = NaturalVector::from_str(s).unwrap();
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
}

#[test]
fn test_mod_power_of_2_beyond_a_word() {
    // A `Natural` element can be larger than any power of 2, so there is no width past which this
    // stops doing anything.
    let test = |s, pow, out| {
        assert_eq!(
            NaturalVector::from_str(s)
                .unwrap()
                .mod_power_of_2(pow)
                .to_string(),
            out
        );
    };
    // 2^128 + 1 modulo 2^64 is 1.
    test("(340282366920938463463374607431768211457, 5)", 64, "(1, 5)");
    // 2^128 modulo 2^64 is 0.
    test("(340282366920938463463374607431768211456)", 64, "(0)");
    // 2^128 modulo 2^128 is 0; modulo 2^129 it is unchanged.
    test("(340282366920938463463374607431768211456)", 128, "(0)");
    test(
        "(340282366920938463463374607431768211456)",
        129,
        "(340282366920938463463374607431768211456)",
    );
}

#[test]
fn mod_power_of_2_properties() {
    natural_vector_unsigned_pair_gen_var_2().test_properties(|(v, pow)| {
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

        // Element by element, this is the `Natural` operation.
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x.mod_power_of_2(pow));
        }

        // A vector that is already reduced is left alone.
        assert_eq!(v.mod_power_of_2_is_reduced(pow), v == w);
    });

    natural_vector_gen().test_properties(|v| {
        // Modulo 2^0 every element vanishes, but the dimension is unchanged.
        assert!((&v).mod_power_of_2(0).elements.iter().all(|x| *x == 0u32));
        // Reducing to the width of the largest element leaves the vector alone, and to one bit less
        // does not, unless every element was zero already.
        let bits = v
            .elements
            .iter()
            .map(SignificantBits::significant_bits)
            .max()
            .unwrap_or(0);
        assert_eq!((&v).mod_power_of_2(bits), v);
        if bits != 0 {
            assert_ne!((&v).mod_power_of_2(bits - 1), v);
        }
    });

    unsigned_vector_gen().test_properties(|v| {
        // The `u64` elements reduce as their `Natural` counterparts do.
        let w = NaturalVector::from(v.clone());
        for pow in 0..8 {
            assert_eq!(
                (&w).mod_power_of_2(pow).elements,
                v.elements
                    .iter()
                    .map(|&x| Natural::from(x.mod_power_of_2(pow)))
                    .collect::<Vec<_>>()
            );
        }
    });
}
