// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2IsReduced};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::unsigned_vector_gen;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_vector_gen, natural_vector_unsigned_pair_gen_var_2,
};

#[test]
fn test_mod_power_of_2_is_reduced() {
    let test = |s, pow, out| {
        assert_eq!(
            NaturalVector::from_str(s)
                .unwrap()
                .mod_power_of_2_is_reduced(pow),
            out
        );
    };
    // The 0-dimensional vector is reduced modulo every power of 2, including 2^0.
    test("()", 0, true);
    test("()", 8, true);
    // So is a vector of zeros.
    test("(0, 0, 0)", 0, true);
    test("(1, 3, 2)", 2, true);
    test("(1, 3, 2)", 1, false);
    test("(1, 3, 2)", 0, false);
    // One unreduced element is enough, wherever it is.
    test("(4, 3)", 2, false);
    test("(3, 4)", 2, false);
    test("(3, 3)", 2, true);
    test("(340282366920938463463374607431768211455)", 128, true);
    test("(340282366920938463463374607431768211456)", 128, false);
}

#[test]
fn mod_power_of_2_is_reduced_properties() {
    natural_vector_unsigned_pair_gen_var_2().test_properties(|(v, pow)| {
        let reduced = v.mod_power_of_2_is_reduced(pow);
        // The defining property: every element is reduced.
        assert_eq!(
            reduced,
            v.elements.iter().all(|x| x.mod_power_of_2_is_reduced(pow))
        );
        // A vector is reduced exactly when reducing it changes nothing.
        assert_eq!(reduced, (&v).mod_power_of_2(pow) == v);
        // Reducing always gives a reduced vector.
        assert!((&v).mod_power_of_2(pow).mod_power_of_2_is_reduced(pow));
        // Being reduced modulo 2^k means being reduced modulo every larger power.
        if reduced {
            assert!(v.mod_power_of_2_is_reduced(pow + 1));
        }
    });

    natural_vector_gen().test_properties(|v| {
        // The bit length of the largest element decides it.
        let bits = v
            .elements
            .iter()
            .map(SignificantBits::significant_bits)
            .max()
            .unwrap_or(0);
        assert!(v.mod_power_of_2_is_reduced(bits));
        if bits != 0 {
            assert!(!v.mod_power_of_2_is_reduced(bits - 1));
        }
    });

    unsigned_vector_gen().test_properties(|v| {
        // A vector of `u64`s is reduced modulo 2^64, and the conversion changes nothing.
        let w = NaturalVector::from(v.clone());
        assert!(w.mod_power_of_2_is_reduced(64));
        for pow in 0..8 {
            assert_eq!(
                w.mod_power_of_2_is_reduced(pow),
                v.elements.iter().all(|x| x.significant_bits() <= pow)
            );
        }
    });
}
