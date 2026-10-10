// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModShl, ModShlAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_vector_unsigned_unsigned_triple_gen_var_4;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use std::panic::catch_unwind;

#[test]
fn test_mod_shl() {
    let test = |s, bits: u32, m: u8, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let w = (&v).mod_shl(bits, m);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().mod_shl(bits, m), w);
        let mut x = v;
        x.mod_shl_assign(bits, m);
        assert_eq!(x, w);
    };
    test("()", 1, 7, "()");
    // Modulo 1 every element is 0.
    test("(0, 0)", 5, 1, "(0, 0)");
    test("(5, 1, 3)", 0, 7, "(5, 1, 3)");
    test("(5, 1, 3)", 1, 7, "(3, 2, 6)");
    // The modulus need not be odd, so elements can become zero, keeping the dimension.
    test("(5, 1, 3)", 3, 8, "(0, 0, 0)");
    test("(1, 2)", 8, 255, "(1, 2)");
    test("(1, 254)", 1000, 255, "(1, 254)");
}

fn mod_shl_fail_helper<T: ModShl<u8, T, Output = T> + PrimitiveUnsigned>() {
    // The modulus is zero, even with no element to reduce.
    assert_panic!(UnsignedVector::<T>::zero(0).mod_shl(1u8, T::ZERO));
    // An element is not reduced.
    let v = UnsignedVector::<T> {
        elements: vec![T::exact_from(7u8)],
    };
    assert_panic!((&v).mod_shl(1u8, T::exact_from(7u8)));
    assert_panic!({
        let mut w = v.clone();
        w.mod_shl_assign(1u8, T::exact_from(7u8));
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
    UnsignedVector<T>: ModShl<U, T, Output = UnsignedVector<T>> + ModShlAssign<U, T>,
    for<'a> &'a UnsignedVector<T>: ModShl<U, T, Output = UnsignedVector<T>>,
{
    unsigned_vector_unsigned_unsigned_triple_gen_var_4::<T, U>().test_properties(|(v, bits, m)| {
        let w = (&v).mod_shl(bits, m);
        assert_eq!(v.clone().mod_shl(bits, m), w);
        let mut x = v.clone();
        x.mod_shl_assign(bits, m);
        assert_eq!(x, w);

        // The result is reduced, and element by element this is the scalar operation.
        assert!(w.mod_is_reduced(&m));
        assert_eq!(w.dimension(), v.dimension());
        for (&x, &y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(y, x.mod_shl(bits, m));
        }
        // Shifting by 0 changes nothing.
        assert_eq!((&v).mod_shl(U::ZERO, m), v);
    });
}

#[test]
fn mod_shl_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(mod_shl_properties_helper);
}
