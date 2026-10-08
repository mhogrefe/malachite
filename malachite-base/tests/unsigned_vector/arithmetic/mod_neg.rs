// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModNeg, ModNegAssign, ModPowerOf2Neg};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_vector_unsigned_pair_gen_var_4, unsigned_vector_unsigned_pair_gen_var_5,
};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_mod_neg() {
    fn test<T: PrimitiveUnsigned>(s: &str, m: T, out: &str) {
        let v = UnsignedVector::<T>::from_str(s).unwrap();
        let w = (&v).mod_neg(m);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().mod_neg(m), w);
        let mut x = v;
        x.mod_neg_assign(m);
        assert_eq!(x, w);
    }
    test::<u8>("()", 1, "()");
    test::<u8>("()", 7, "()");
    test::<u8>("(0)", 1, "(0)");
    test::<u8>("(5, 1, 0)", 7, "(2, 6, 0)");
    // A zero element stays zero, wherever it is.
    test::<u8>("(0, 1)", 255, "(0, 254)");
    test::<u8>("(1, 0)", 255, "(254, 0)");
    test::<u8>("(3, 3)", 6, "(3, 3)");
    test::<u64>(
        "(1, 18446744073709551614)",
        18446744073709551615,
        "(18446744073709551614, 1)",
    );
}

#[test]
#[should_panic]
fn mod_neg_fail_1() {
    // An element is not reduced.
    (&UnsignedVector::<u8>::from_str("(7, 1)").unwrap()).mod_neg(7);
}

#[test]
#[should_panic]
fn mod_neg_fail_2() {
    // The modulus is zero.
    (&UnsignedVector::<u8>::from_str("()").unwrap()).mod_neg(0);
}

#[test]
#[should_panic]
fn mod_neg_assign_fail() {
    // An element is not reduced.
    let mut v = UnsignedVector::<u8>::from_str("(7, 1)").unwrap();
    v.mod_neg_assign(7);
}

fn mod_neg_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_pair_gen_var_5::<T>().test_properties(|(v, m)| {
        let w = (&v).mod_neg(m);
        assert_eq!(v.clone().mod_neg(m), w);
        let mut x = v.clone();
        x.mod_neg_assign(m);
        assert_eq!(x, w);

        // The result is reduced, has the same dimension, and is the elementwise negation.
        assert!(w.mod_is_reduced(&m));
        assert_eq!(w.dimension(), v.dimension());
        for (&x, &y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(y, x.mod_neg(m));
        }
        // Negating twice gives the vector back.
        assert_eq!((&w).mod_neg(m), v);
    });

    unsigned_vector_unsigned_pair_gen_var_4::<T>().test_properties(|(v, pow)| {
        // For a power-of-2 modulus that fits in a `T`, this is `mod_power_of_2_neg`.
        if pow < T::WIDTH {
            assert_eq!(
                (&v).mod_neg(T::power_of_2(pow)),
                (&v).mod_power_of_2_neg(pow)
            );
        }
    });
}

#[test]
fn mod_neg_properties() {
    apply_fn_to_unsigneds!(mod_neg_properties_helper);
}
