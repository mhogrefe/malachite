// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    DivExact, EntrywiseDivRound, EntrywiseDivRoundAssign, EntrywiseShrRound,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_vector_unsigned_pair_gen_var_8,
    unsigned_vector_unsigned_rounding_mode_triple_gen_var_2,
};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use std::panic::catch_unwind;

#[test]
fn test_entrywise_div_round() {
    let test = |s, c: u8, rm: RoundingMode, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let w = (&v).entrywise_div_round(c, rm);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().entrywise_div_round(c, rm), w);
        let mut x = v;
        x.entrywise_div_round_assign(c, rm);
        assert_eq!(x, w);
    };
    test("()", 5, Exact, "()");
    test("(1, 2, 3, 5)", 1, Exact, "(1, 2, 3, 5)");
    test("(1, 2, 3, 5)", 2, Floor, "(0, 1, 1, 2)");
    test("(1, 2, 3, 5)", 2, Down, "(0, 1, 1, 2)");
    test("(1, 2, 3, 5)", 2, Ceiling, "(1, 1, 2, 3)");
    test("(1, 2, 3, 5)", 2, Up, "(1, 1, 2, 3)");
    // Ties round to even.
    test("(1, 2, 3, 5)", 2, Nearest, "(0, 1, 2, 2)");
    test("(3, 6, 255)", 3, Exact, "(1, 2, 85)");
    test("(255, 127, 128)", 255, Nearest, "(1, 0, 1)");
}

fn entrywise_div_round_fail_helper<T: PrimitiveUnsigned>() {
    let v = UnsignedVector::<T> {
        elements: vec![T::ONE, T::TWO],
    };
    // The divisor is zero.
    assert_panic!(v.clone().entrywise_div_round(T::ZERO, Floor));
    assert_panic!((&v).entrywise_div_round(T::ZERO, Floor));
    assert_panic!(UnsignedVector::<T> { elements: vec![] }.entrywise_div_round(T::ZERO, Floor));
    assert_panic!({
        let mut w = v.clone();
        w.entrywise_div_round_assign(T::ZERO, Floor);
    });
    // The division is not exact.
    assert_panic!(v.clone().entrywise_div_round(T::TWO, Exact));
    assert_panic!((&v).entrywise_div_round(T::TWO, Exact));
    assert_panic!({
        let mut w = v.clone();
        w.entrywise_div_round_assign(T::TWO, Exact);
    });
}

#[test]
fn entrywise_div_round_fail() {
    apply_fn_to_unsigneds!(entrywise_div_round_fail_helper);
}

fn entrywise_div_round_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_rounding_mode_triple_gen_var_2::<T>().test_properties(|(v, c, rm)| {
        let w = (&v).entrywise_div_round(c, rm);
        // The forms agree.
        assert_eq!(v.clone().entrywise_div_round(c, rm), w);
        let mut x = v.clone();
        x.entrywise_div_round_assign(c, rm);
        assert_eq!(x, w);

        // Element by element, this is the scalar rounding division, and the dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());
        for (&x, &y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(y, x.div_round(c, rm).0);
        }
        // Rounding down is the plain quotient.
        if rm == Floor || rm == Down {
            assert_eq!(&v / c, w);
        }
        // An exact division is exact division.
        if rm == Exact {
            assert_eq!((&v).div_exact(c), w);
        }
        // Dividing by a power of 2 is a rounding right shift.
        if c.is_power_of_2() {
            assert_eq!(
                (&v).entrywise_shr_round(TrailingZeros::trailing_zeros(c), rm),
                w
            );
        }
    });

    unsigned_vector_unsigned_pair_gen_var_8::<T>().test_properties(|(v, c)| {
        // Nearest lies between Floor and Ceiling, which differ by at most 1, element by element.
        let f = (&v).entrywise_div_round(c, Floor);
        let g = (&v).entrywise_div_round(c, Ceiling);
        let n = (&v).entrywise_div_round(c, Nearest);
        for ((&f, &g), &n) in f.elements.iter().zip(&g.elements).zip(&n.elements) {
            assert!(f <= n && n <= g);
            assert!(g == f || g == f + T::ONE);
        }
    });
}

#[test]
fn entrywise_div_round_properties() {
    apply_fn_to_unsigneds!(entrywise_div_round_properties_helper);
}
