// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModAddDot, ModAddDotAssign, ModDot, ModSubDot};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_vector::UnsignedVector;
use std::panic::catch_unwind;

#[test]
fn test_mod_add_dot() {
    fn test<T: PrimitiveUnsigned>(x: T, v: &str, w: &str, m: T, out: T) {
        let v = UnsignedVector::<T>::from_str(v).unwrap();
        let w = UnsignedVector::<T>::from_str(w).unwrap();
        let r = x.mod_add_dot(&v, &w, m);
        assert_eq!(r, out);
        assert_eq!(x.mod_add_dot(v.clone(), w.clone(), m), r);
        assert_eq!(x.mod_add_dot(v.clone(), &w, m), r);
        assert_eq!(x.mod_add_dot(&v, w.clone(), m), r);
        let mut y = x;
        y.mod_add_dot_assign(v.clone(), w.clone(), m);
        assert_eq!(y, r);
        let mut y = x;
        y.mod_add_dot_assign(v.clone(), &w, m);
        assert_eq!(y, r);
        let mut y = x;
        y.mod_add_dot_assign(&v, w.clone(), m);
        assert_eq!(y, r);
        let mut y = x;
        y.mod_add_dot_assign(&v, &w, m);
        assert_eq!(y, r);
    }
    test::<u8>(0, "()", "()", 1, 0);
    test::<u8>(3, "(1, 2, 3)", "(4, 5, 7)", 11, 5);
    test::<u8>(254, "(254, 254)", "(254, 254)", 255, 1);
    test::<u64>(
        5,
        "(18446744073709551614, 18446744073709551614, 18446744073709551614)",
        "(18446744073709551614, 18446744073709551614, 18446744073709551613)",
        18446744073709551615,
        9,
    );
}

fn mod_add_dot_fail_helper<T: PrimitiveUnsigned>() {
    let v = UnsignedVector::<T>::from_str("(1, 2)").unwrap();
    assert_panic!(T::ONE.mod_add_dot(&v, &v, T::ZERO));
    let w = UnsignedVector::<T>::from_str("(1)").unwrap();
    assert_panic!(T::ONE.mod_add_dot(&v, &w, T::exact_from(7u8)));
    let w = UnsignedVector::<T>::from_str("(1, 7)").unwrap();
    assert_panic!(T::ONE.mod_add_dot(&v, &w, T::exact_from(7u8)));
    assert_panic!(T::exact_from(7u8).mod_add_dot(&v, &v, T::exact_from(7u8)));
}

#[test]
fn mod_add_dot_fail() {
    apply_fn_to_unsigneds!(mod_add_dot_fail_helper);
}

fn mod_add_dot_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_2::<T>().test_properties(
        |(v, w, x, m)| {
            let r = x.mod_add_dot(&v, &w, m);
            assert!(r < m);
            assert_eq!(x.mod_add_dot(v.clone(), w.clone(), m), r);
            assert_eq!(x.mod_add_dot(v.clone(), &w, m), r);
            assert_eq!(x.mod_add_dot(&v, w.clone(), m), r);
            let mut y = x;
            y.mod_add_dot_assign(&v, &w, m);
            assert_eq!(y, r);
            // It is the dot product added to `x`.
            assert_eq!(x.mod_add((&v).mod_dot(&w, m), m), r);
            // Subtracting it back recovers `x`.
            assert_eq!(r.mod_sub_dot(&v, &w, m), x);
        },
    );
}

#[test]
fn mod_add_dot_properties() {
    apply_fn_to_unsigneds!(mod_add_dot_properties_helper);
}
