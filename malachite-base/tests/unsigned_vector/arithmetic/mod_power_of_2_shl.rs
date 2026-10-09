// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Shl, ModPowerOf2ShlAssign,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_vector_unsigned_unsigned_triple_gen_var_3;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_shl() {
    let test = |s, bits: u32, pow, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let w = (&v).mod_power_of_2_shl(bits, pow);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().mod_power_of_2_shl(bits, pow), w);
        let mut x = v;
        x.mod_power_of_2_shl_assign(bits, pow);
        assert_eq!(x, w);
    };
    test("()", 1, 3, "()");
    test("(0, 0)", 5, 0, "(0, 0)");
    test("(5, 1, 3)", 0, 3, "(5, 1, 3)");
    test("(5, 1, 3)", 1, 3, "(2, 2, 6)");
    // Shifting by at least the power zeroes every element, keeping the dimension.
    test("(5, 1, 3)", 3, 3, "(0, 0, 0)");
    test("(5, 1, 3)", 200, 3, "(0, 0, 0)");
    test("(1, 128, 1)", 1, 8, "(2, 0, 2)");
    test("(255, 1)", 7, 8, "(128, 128)");
}

fn mod_power_of_2_shl_fail_helper<T: PrimitiveUnsigned>() {
    let v = UnsignedVector::<T> {
        elements: vec![T::exact_from(8u8)],
    };
    // An element is not reduced.
    assert_panic!((&v).mod_power_of_2_shl(1u8, 3));
    // The power is too large.
    assert_panic!((&v).mod_power_of_2_shl(1u8, T::WIDTH + 1));
    assert_panic!({
        let mut w = v.clone();
        w.mod_power_of_2_shl_assign(1u8, 3);
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
    UnsignedVector<T>: ModPowerOf2Shl<U, Output = UnsignedVector<T>> + ModPowerOf2ShlAssign<U>,
    for<'a> &'a UnsignedVector<T>: ModPowerOf2Shl<U, Output = UnsignedVector<T>>,
{
    unsigned_vector_unsigned_unsigned_triple_gen_var_3::<T, U>().test_properties(
        |(v, bits, pow)| {
            let w = (&v).mod_power_of_2_shl(bits, pow);
            assert_eq!(v.clone().mod_power_of_2_shl(bits, pow), w);
            let mut x = v.clone();
            x.mod_power_of_2_shl_assign(bits, pow);
            assert_eq!(x, w);

            // The result is reduced, and element by element this is the scalar operation.
            assert!(w.mod_power_of_2_is_reduced(pow));
            assert_eq!(w.dimension(), v.dimension());
            for (&x, &y) in v.elements.iter().zip(&w.elements) {
                assert_eq!(y, x.mod_power_of_2_shl(bits, pow));
            }
            // Shifting by 0 changes nothing.
            assert_eq!((&v).mod_power_of_2_shl(U::ZERO, pow), v);
        },
    );
}

#[test]
fn mod_power_of_2_shl_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(mod_power_of_2_shl_properties_helper);
}
