// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{DivExact, EntrywiseDivRound, EntrywiseShrRound};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::generators::{
    unsigned_vector_unsigned_pair_gen_var_7, unsigned_vector_unsigned_pair_gen_var_8,
};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use std::panic::catch_unwind;

#[test]
fn test_div() {
    let test = |s, c: u8, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let w = &v / c;
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone() / c, w);
        let mut x = v;
        x /= c;
        assert_eq!(x, w);
    };
    test("()", 5, "()");
    test("(0, 7, 255)", 1, "(0, 7, 255)");
    test("(0, 7, 255)", 2, "(0, 3, 127)");
    test("(0, 7, 255)", 255, "(0, 0, 1)");
    test("(0, 7, 254)", 8, "(0, 0, 31)");
}

fn div_fail_helper<T: PrimitiveUnsigned>() {
    let v = UnsignedVector::<T> {
        elements: vec![T::ONE, T::TWO],
    };
    assert_panic!(v.clone() / T::ZERO);
    assert_panic!(&v / T::ZERO);
    assert_panic!(UnsignedVector::<T> { elements: vec![] } / T::ZERO);
    assert_panic!({
        let mut w = v.clone();
        w /= T::ZERO;
    });
}

#[test]
fn div_fail() {
    apply_fn_to_unsigneds!(div_fail_helper);
}

fn div_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_pair_gen_var_8::<T>().test_properties(|(v, c)| {
        let w = &v / c;
        // The forms agree.
        assert_eq!(v.clone() / c, w);
        let mut x = v.clone();
        x /= c;
        assert_eq!(x, w);

        // Element by element, this is the scalar quotient, rounded down, and the dimension is
        // unchanged.
        assert_eq!(w.dimension(), v.dimension());
        for (&x, &y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(y, x / c);
            assert_eq!(y, x.div_round(c, Down).0);
        }
        // It is entrywise division rounding down.
        assert_eq!((&v).entrywise_div_round(c, Floor), w);
        assert_eq!((&v).entrywise_div_round(c, Down), w);
        // Dividing by 1 changes nothing, and dividing by a power of 2 is a right shift.
        assert_eq!(&v / T::ONE, v);
        if c.is_power_of_2() {
            let bits = TrailingZeros::trailing_zeros(c);
            assert_eq!(&v >> bits, w);
            assert_eq!((&v).entrywise_shr_round(bits, Down), w);
        }
    });

    unsigned_vector_unsigned_pair_gen_var_7::<T>().test_properties(|(v, c)| {
        // When the division is exact, it agrees with exact division.
        assert_eq!(&v / c, (&v).div_exact(c));
    });
}

#[test]
fn div_properties() {
    apply_fn_to_unsigneds!(div_properties_helper);
}
