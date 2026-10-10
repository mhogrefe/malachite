// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::{Shr, ShrAssign};
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{EntrywiseShrRound, ShrRound};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_6;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_shr() {
    let test = |s, bits: u32, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let w = &v >> bits;
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone() >> bits, w);
        let mut x = v;
        x >>= bits;
        assert_eq!(x, w);
    };
    test("()", 0, "()");
    test("()", 10, "()");
    test("(1, 2, 3, 5)", 0, "(1, 2, 3, 5)");
    test("(1, 2, 3, 5)", 1, "(0, 1, 1, 2)");
    test("(4, 8, 12)", 2, "(1, 2, 3)");
    test("(255, 128)", 7, "(1, 1)");
    // Shifting by at least the width zeroes every element, keeping the dimension.
    test("(255, 128)", 8, "(0, 0)");
    test("(255, 128)", 200, "(0, 0)");
}

fn shr_properties_helper<T: PrimitiveUnsigned + ShrRound<U, Output = T>, U: PrimitiveUnsigned>()
where
    UnsignedVector<T>: Shr<U, Output = UnsignedVector<T>> + ShrAssign<U>,
    for<'a> &'a UnsignedVector<T>:
        Shr<U, Output = UnsignedVector<T>> + EntrywiseShrRound<U, Output = UnsignedVector<T>>,
{
    unsigned_vector_unsigned_pair_gen_var_6::<T, U>().test_properties(|(v, bits)| {
        let w = &v >> bits;
        // The forms agree.
        assert_eq!(v.clone() >> bits, w);
        let mut x = v.clone();
        x >>= bits;
        assert_eq!(x, w);

        // Element by element, this is the scalar shift (which takes the floor even when `bits` is
        // at least `T::WIDTH`), and the dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());
        for (&x, &y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(y, x.shr_round(bits, Floor).0);
            assert!(y <= x);
        }
        // It takes the floor.
        assert_eq!((&v).entrywise_shr_round(bits, Floor), w);
        assert_eq!((&v).entrywise_shr_round(bits, Down), w);
        // Shifting by at least the width zeroes every element.
        if bits >= U::exact_from(T::WIDTH) {
            assert!(w.is_zero());
        }
        // Shifting by 0 changes nothing, and shifts compose.
        assert_eq!(&v >> U::ZERO, v);
        if let Some(double) = bits.checked_add(bits) {
            assert_eq!(&w >> bits, &v >> double);
        }
    });
}

#[test]
fn shr_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(shr_properties_helper);
}
