// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModShl, ModShlAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_unsigned_triple_gen_var_5;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use std::panic::catch_unwind;

#[test]
fn test_mod_shl() {
    let test = |s, bits: u32, m: u8, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let b = (&a).mod_shl(bits, m);
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone().mod_shl(bits, m), b);
        let mut x = a;
        x.mod_shl_assign(bits, m);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", 1, 7, "[]_{0,0}");
    // Modulo 1 every element is 0.
    test("[0, 0]", 5, 1, "[0, 0]");
    test("[5, 1, 3]", 0, 7, "[5, 1, 3]");
    test("[5, 1, 3]", 1, 7, "[3, 2, 6]");
    test("[5, 1; 3, 7]", 1, 8, "[2, 2; 6, 6]");
    test("[]_{2,0}", 1, 7, "[]_{2,0}");
    // The modulus need not be odd, so elements can become zero, keeping the shape.
    test("[5, 1, 3]", 3, 8, "[0, 0, 0]");
    test("[1, 2]", 8, 255, "[1, 2]");
    test("[1, 254]", 1000, 255, "[1, 254]");
}

fn mod_shl_fail_helper<T: ModShl<u8, T, Output = T> + PrimitiveUnsigned>() {
    // The modulus is zero, even with no element to reduce.
    assert_panic!(UnsignedMatrix::<T>::zero(0, 0).mod_shl(1u8, T::ZERO));
    // An element is not reduced.
    let a = UnsignedMatrix::<T>::from_str("[7]").unwrap();
    assert_panic!((&a).mod_shl(1u8, T::exact_from(7u8)));
    assert_panic!({
        let mut b = a.clone();
        b.mod_shl_assign(1u8, T::exact_from(7u8));
    });
}

#[test]
fn mod_shl_fail() {
    apply_fn_to_unsigneds!(mod_shl_fail_helper);
}

fn mod_shl_properties_helper<
    T: ModShl<U, T, Output = T> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>()
where
    UnsignedMatrix<T>: ModShl<U, T, Output = UnsignedMatrix<T>> + ModShlAssign<U, T>,
    for<'a> &'a UnsignedMatrix<T>: ModShl<U, T, Output = UnsignedMatrix<T>>,
{
    unsigned_matrix_unsigned_unsigned_triple_gen_var_5::<T, U>().test_properties(|(a, bits, m)| {
        let b = (&a).mod_shl(bits, m);
        assert_eq!(a.clone().mod_shl(bits, m), b);
        let mut x = a.clone();
        x.mod_shl_assign(bits, m);
        assert_eq!(x, b);

        // The result is reduced, and element by element this is the scalar operation.
        assert!(b.mod_is_reduced(&m));
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        for (&x, &y) in a.elements_ref().iter().zip(b.elements_ref()) {
            assert_eq!(y, x.mod_shl(bits, m));
        }
        // Shifting by 0 changes nothing.
        assert_eq!((&a).mod_shl(U::ZERO, m), a);
        // It commutes with transposition.
        assert_eq!(a.transpose().mod_shl(bits, m), b.transpose());
    });
}

#[test]
fn mod_shl_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(mod_shl_properties_helper);
}
