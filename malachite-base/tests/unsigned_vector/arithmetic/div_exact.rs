// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_7;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use std::panic::catch_unwind;

#[test]
fn test_div_exact() {
    let test = |s, c: u8, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let w = (&v).div_exact(c);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().div_exact(c), w);
        let mut x = v;
        x.div_exact_assign(c);
        assert_eq!(x, w);
    };
    test("()", 5, "()");
    test("(0, 6, 255)", 1, "(0, 6, 255)");
    test("(0, 6, 255)", 3, "(0, 2, 85)");
    test("(4, 8, 12)", 4, "(1, 2, 3)");
    test("(250, 0)", 125, "(2, 0)");
    test("(255, 255)", 255, "(1, 1)");
}

fn div_exact_fail_helper<T: PrimitiveUnsigned>() {
    let v = UnsignedVector::<T> {
        elements: vec![T::ONE, T::TWO],
    };
    assert_panic!(v.clone().div_exact(T::ZERO));
    assert_panic!((&v).div_exact(T::ZERO));
    assert_panic!(UnsignedVector::<T> { elements: vec![] }.div_exact(T::ZERO));
    assert_panic!({
        let mut w = v.clone();
        w.div_exact_assign(T::ZERO);
    });
}

#[test]
fn div_exact_fail() {
    apply_fn_to_unsigneds!(div_exact_fail_helper);
}

fn div_exact_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_pair_gen_var_7::<T>().test_properties(|(v, c)| {
        let w = (&v).div_exact(c);
        // The forms agree.
        assert_eq!(v.clone().div_exact(c), w);
        let mut x = v.clone();
        x.div_exact_assign(c);
        assert_eq!(x, w);

        // Element by element, this is the scalar exact quotient, which is also the ordinary
        // quotient, and multiplying by the scalar recovers the element. The dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());
        for (&x, &y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(y, x.div_exact(c));
            assert_eq!(y, x / c);
            assert_eq!(y * c, x);
        }
        // Dividing by 1 changes nothing.
        assert_eq!((&v).div_exact(T::ONE), v);
        // Dividing by a power of 2 is a right shift.
        if c.is_power_of_2() {
            assert_eq!(&v >> TrailingZeros::trailing_zeros(c), w);
        }
    });
}

#[test]
fn div_exact_properties() {
    apply_fn_to_unsigneds!(div_exact_properties_helper);
}
