// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2AddDot, ModPowerOf2Dot, ModPowerOf2SubDot, ModPowerOf2SubDotAssign, ModSubDot,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_vector::UnsignedVector;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_sub_dot() {
    fn test<T: PrimitiveUnsigned>(x: T, v: &str, w: &str, pow: u64, out: T) {
        let v = UnsignedVector::<T>::from_str(v).unwrap();
        let w = UnsignedVector::<T>::from_str(w).unwrap();
        let r = x.mod_power_of_2_sub_dot(&v, &w, pow);
        assert_eq!(r, out);
        assert_eq!(x.mod_power_of_2_sub_dot(v.clone(), w.clone(), pow), r);
        assert_eq!(x.mod_power_of_2_sub_dot(v.clone(), &w, pow), r);
        assert_eq!(x.mod_power_of_2_sub_dot(&v, w.clone(), pow), r);
        let mut y = x;
        y.mod_power_of_2_sub_dot_assign(v.clone(), w.clone(), pow);
        assert_eq!(y, r);
        let mut y = x;
        y.mod_power_of_2_sub_dot_assign(v.clone(), &w, pow);
        assert_eq!(y, r);
        let mut y = x;
        y.mod_power_of_2_sub_dot_assign(&v, w.clone(), pow);
        assert_eq!(y, r);
        let mut y = x;
        y.mod_power_of_2_sub_dot_assign(&v, &w, pow);
        assert_eq!(y, r);
    }
    test::<u8>(0, "()", "()", 0, 0);
    test::<u8>(5, "(1, 2, 3)", "(4, 5, 7)", 4, 2);
    test::<u8>(255, "(255, 255)", "(255, 1)", 8, 255);
    test::<u64>(
        1,
        "(18446744073709551615, 2)",
        "(3, 18446744073709551615)",
        64,
        6,
    );
}

fn mod_power_of_2_sub_dot_fail_helper<T: PrimitiveUnsigned>() {
    let v = UnsignedVector::<T>::from_str("(1, 2)").unwrap();
    assert_panic!(T::ONE.mod_power_of_2_sub_dot(&v, &v, T::WIDTH + 1));
    let w = UnsignedVector::<T>::from_str("(1)").unwrap();
    assert_panic!(T::ONE.mod_power_of_2_sub_dot(&v, &w, 3));
    let w = UnsignedVector::<T>::from_str("(1, 8)").unwrap();
    assert_panic!(T::ONE.mod_power_of_2_sub_dot(&v, &w, 3));
    assert_panic!(T::exact_from(8u8).mod_power_of_2_sub_dot(&v, &v, 3));
}

#[test]
fn mod_power_of_2_sub_dot_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_dot_fail_helper);
}

fn mod_power_of_2_sub_dot_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_1::<T>().test_properties(
        |(v, w, x, pow)| {
            let r = x.mod_power_of_2_sub_dot(&v, &w, pow);
            assert!(r.mod_power_of_2_is_reduced(pow));
            assert_eq!(x.mod_power_of_2_sub_dot(v.clone(), w.clone(), pow), r);
            assert_eq!(x.mod_power_of_2_sub_dot(v.clone(), &w, pow), r);
            assert_eq!(x.mod_power_of_2_sub_dot(&v, w.clone(), pow), r);
            let mut y = x;
            y.mod_power_of_2_sub_dot_assign(&v, &w, pow);
            assert_eq!(y, r);
            // It is the dot product subtracted from `x`.
            assert_eq!(
                x.mod_power_of_2_sub((&v).mod_power_of_2_dot(&w, pow), pow),
                r
            );
            // Adding it back recovers `x`.
            assert_eq!(r.mod_power_of_2_add_dot(&v, &w, pow), x);
        },
    );

    unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_1::<T>().test_properties(
        |(v, w, x, pow)| {
            // For a power of 2 below the width of `T`, this is `mod_sub_dot`.
            if pow < T::WIDTH {
                assert_eq!(
                    x.mod_power_of_2_sub_dot(&v, &w, pow),
                    x.mod_sub_dot(&v, &w, T::power_of_2(pow))
                );
            }
        },
    );
}

#[test]
fn mod_power_of_2_sub_dot_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_dot_properties_helper);
}
