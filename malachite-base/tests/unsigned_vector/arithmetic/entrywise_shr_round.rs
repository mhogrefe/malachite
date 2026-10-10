// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::Shr;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    EntrywiseShrRound, EntrywiseShrRoundAssign, ModPowerOf2Shl, ShrRound,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_vector_unsigned_pair_gen_var_6,
    unsigned_vector_unsigned_rounding_mode_triple_gen_var_1,
};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use std::panic::catch_unwind;

#[test]
fn test_entrywise_shr_round() {
    let test = |s, bits: u32, rm: RoundingMode, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let w = (&v).entrywise_shr_round(bits, rm);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().entrywise_shr_round(bits, rm), w);
        let mut x = v;
        x.entrywise_shr_round_assign(bits, rm);
        assert_eq!(x, w);
    };
    test("()", 0, Exact, "()");
    test("()", 10, Nearest, "()");
    test("(1, 2, 3, 5)", 0, Exact, "(1, 2, 3, 5)");
    test("(1, 2, 3, 5)", 1, Floor, "(0, 1, 1, 2)");
    test("(1, 2, 3, 5)", 1, Down, "(0, 1, 1, 2)");
    test("(1, 2, 3, 5)", 1, Ceiling, "(1, 1, 2, 3)");
    test("(1, 2, 3, 5)", 1, Up, "(1, 1, 2, 3)");
    // Ties round to even.
    test("(1, 2, 3, 5)", 1, Nearest, "(0, 1, 2, 2)");
    test("(4, 8, 12)", 2, Exact, "(1, 2, 3)");
    test("(255, 128)", 7, Nearest, "(2, 1)");
    // Shifting by at least the width rounds every element to 0 or 1.
    test("(255, 128, 127, 0)", 8, Nearest, "(1, 0, 0, 0)");
    test("(255, 128, 127, 0)", 8, Floor, "(0, 0, 0, 0)");
    test("(255, 1, 0)", 100, Ceiling, "(1, 1, 0)");
    test("(255, 1, 0)", 100, Nearest, "(0, 0, 0)");
    test("(0, 0)", 200, Exact, "(0, 0)");
}

fn entrywise_shr_round_fail_helper<T: PrimitiveUnsigned>() {
    let v = UnsignedVector::<T> {
        elements: vec![T::exact_from(4u8), T::ONE],
    };
    // An element is not divisible by the power of 2.
    assert_panic!(v.clone().entrywise_shr_round(1u8, Exact));
    assert_panic!((&v).entrywise_shr_round(1u32, Exact));
    assert_panic!((&v).entrywise_shr_round(200u64, Exact));
    assert_panic!({
        let mut w = v.clone();
        w.entrywise_shr_round_assign(1u16, Exact);
    });
}

#[test]
fn entrywise_shr_round_fail() {
    apply_fn_to_unsigneds!(entrywise_shr_round_fail_helper);
}

fn entrywise_shr_round_properties_helper<
    T: PrimitiveUnsigned + ShrRound<U, Output = T>,
    U: PrimitiveUnsigned,
>()
where
    UnsignedVector<T>:
        EntrywiseShrRound<U, Output = UnsignedVector<T>> + EntrywiseShrRoundAssign<U>,
    for<'a> &'a UnsignedVector<T>: EntrywiseShrRound<U, Output = UnsignedVector<T>>
        + Shr<U, Output = UnsignedVector<T>>
        + ModPowerOf2Shl<U, Output = UnsignedVector<T>>,
{
    unsigned_vector_unsigned_rounding_mode_triple_gen_var_1::<T, U>().test_properties(
        |(v, bits, rm)| {
            let w = (&v).entrywise_shr_round(bits, rm);
            // The forms agree.
            assert_eq!(v.clone().entrywise_shr_round(bits, rm), w);
            let mut x = v.clone();
            x.entrywise_shr_round_assign(bits, rm);
            assert_eq!(x, w);

            // Element by element, this is the scalar rounding shift, and the dimension is
            // unchanged.
            assert_eq!(w.dimension(), v.dimension());
            for (&x, &y) in v.elements.iter().zip(&w.elements) {
                assert_eq!(y, x.shr_round(bits, rm).0);
            }
            // Rounding down is the plain shift.
            if rm == Floor || rm == Down {
                assert_eq!(&v >> bits, w);
            }
            // An exact shift can be undone by a left shift, which cannot overflow.
            if rm == Exact {
                assert_eq!((&w).mod_power_of_2_shl(bits, T::WIDTH), v);
            }
        },
    );

    unsigned_vector_unsigned_pair_gen_var_6::<T, U>().test_properties(|(v, bits)| {
        // Nearest lies between Floor and Ceiling, which differ by at most 1, element by element.
        let f = (&v).entrywise_shr_round(bits, Floor);
        let c = (&v).entrywise_shr_round(bits, Ceiling);
        let n = (&v).entrywise_shr_round(bits, Nearest);
        for ((&f, &c), &n) in f.elements.iter().zip(&c.elements).zip(&n.elements) {
            assert!(f <= n && n <= c);
            assert!(c == f || c == f + T::ONE);
        }
    });
}

#[test]
fn entrywise_shr_round_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(entrywise_shr_round_properties_helper);
}
