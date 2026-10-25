// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Shl, ModPowerOf2ShlAssign,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_unsigned_triple_gen_var_4;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_shl() {
    let test = |s, bits: u32, pow, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let b = (&a).mod_power_of_2_shl(bits, pow);
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone().mod_power_of_2_shl(bits, pow), b);
        let mut x = a;
        x.mod_power_of_2_shl_assign(bits, pow);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", 1, 3, "[]_{0,0}");
    test("[0, 0]", 5, 0, "[0, 0]");
    test("[5, 1, 3]", 0, 3, "[5, 1, 3]");
    test("[5, 1, 3]", 1, 3, "[2, 2, 6]");
    test("[5, 1; 3, 7]", 1, 3, "[2, 2; 6, 6]");
    test("[]_{2,0}", 1, 3, "[]_{2,0}");
    // Shifting by at least the power zeroes every element, keeping the shape.
    test("[5, 1, 3]", 3, 3, "[0, 0, 0]");
    test("[5, 1, 3]", 200, 3, "[0, 0, 0]");
    test("[1, 128, 1]", 1, 8, "[2, 0, 2]");
    test("[255, 1]", 7, 8, "[128, 128]");
}

fn mod_power_of_2_shl_fail_helper<T: PrimitiveUnsigned>() {
    let a = UnsignedMatrix::<T>::from_str("[8]").unwrap();
    // An element is not reduced.
    assert_panic!((&a).mod_power_of_2_shl(1u8, 3));
    // The power is too large.
    assert_panic!((&a).mod_power_of_2_shl(1u8, T::WIDTH + 1));
    assert_panic!({
        let mut b = a.clone();
        b.mod_power_of_2_shl_assign(1u8, 3);
    });
}

#[test]
fn mod_power_of_2_shl_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_shl_fail_helper);
}

fn mod_power_of_2_shl_properties_helper<
    T: ModPowerOf2Shl<U, Output = T> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>()
where
    UnsignedMatrix<T>: ModPowerOf2Shl<U, Output = UnsignedMatrix<T>> + ModPowerOf2ShlAssign<U>,
    for<'a> &'a UnsignedMatrix<T>: ModPowerOf2Shl<U, Output = UnsignedMatrix<T>>,
{
    unsigned_matrix_unsigned_unsigned_triple_gen_var_4::<T, U>().test_properties(
        |(a, bits, pow)| {
            let b = (&a).mod_power_of_2_shl(bits, pow);
            assert_eq!(a.clone().mod_power_of_2_shl(bits, pow), b);
            let mut x = a.clone();
            x.mod_power_of_2_shl_assign(bits, pow);
            assert_eq!(x, b);

            // The result is reduced, and element by element this is the scalar operation.
            assert!(b.mod_power_of_2_is_reduced(pow));
            assert_eq!((b.height(), b.width()), (a.height(), a.width()));
            for (&x, &y) in a.elements_ref().iter().zip(b.elements_ref()) {
                assert_eq!(y, x.mod_power_of_2_shl(bits, pow));
            }
            // Shifting by 0 changes nothing.
            assert_eq!((&a).mod_power_of_2_shl(U::ZERO, pow), a);
            // It commutes with transposition.
            assert_eq!(a.transpose().mod_power_of_2_shl(bits, pow), b.transpose());
        },
    );
}

#[test]
fn mod_power_of_2_shl_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(mod_power_of_2_shl_properties_helper);
}
