// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::{Shl, Shr};
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    EntrywiseShlRound, EntrywiseShrRound, EntrywiseShrRoundAssign, ShrRound,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::One;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::{
    integer_vector_signed_pair_gen_var_1, integer_vector_signed_rounding_mode_triple_gen_var_2,
    integer_vector_unsigned_pair_gen_var_3, integer_vector_unsigned_rounding_mode_triple_gen_var_1,
};
use std::panic::catch_unwind;

fn test_entrywise_shr_round_unsigned_helper<T: PrimitiveUnsigned>()
where
    IntegerVector: EntrywiseShrRound<T, Output = IntegerVector> + EntrywiseShrRoundAssign<T>,
    for<'a> &'a IntegerVector: EntrywiseShrRound<T, Output = IntegerVector>,
{
    let test = |s, bits: u8, rm: RoundingMode, out| {
        let bits = T::from(bits);
        let v = IntegerVector::from_str(s).unwrap();
        let w = (&v).entrywise_shr_round(bits, rm);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().entrywise_shr_round(bits, rm), w);
        let mut x = v;
        x.entrywise_shr_round_assign(bits, rm);
        assert_eq!(x, w);
    };
    test("()", 0, Exact, "()");
    test("()", 10, Nearest, "()");
    test("(1, -2, 3)", 0, Exact, "(1, -2, 3)");
    test("(-1, 2, -3, 5)", 1, Floor, "(-1, 1, -2, 2)");
    test("(-1, 2, -3, 5)", 1, Down, "(0, 1, -1, 2)");
    test("(-1, 2, -3, 5)", 1, Ceiling, "(0, 1, -1, 3)");
    test("(-1, 2, -3, 5)", 1, Up, "(-1, 1, -2, 3)");
    test("(-1, 2, -3, 5)", 1, Nearest, "(0, 1, -2, 2)");
    test("(4, -8, 12)", 2, Exact, "(1, -2, 3)");
    test("(7, -7)", 3, Nearest, "(1, -1)");
    test("(7, -1)", 100, Up, "(1, -1)");
    test("(7, -1)", 100, Down, "(0, 0)");
}

fn test_entrywise_shr_round_signed_helper<T: PrimitiveSigned>()
where
    IntegerVector: EntrywiseShrRound<T, Output = IntegerVector> + EntrywiseShrRoundAssign<T>,
    for<'a> &'a IntegerVector: EntrywiseShrRound<T, Output = IntegerVector>,
{
    let test = |s, bits: i8, rm: RoundingMode, out| {
        let bits = T::from(bits);
        let v = IntegerVector::from_str(s).unwrap();
        let w = (&v).entrywise_shr_round(bits, rm);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().entrywise_shr_round(bits, rm), w);
        let mut x = v;
        x.entrywise_shr_round_assign(bits, rm);
        assert_eq!(x, w);
    };
    test("()", 0, Exact, "()");
    test("()", 10, Nearest, "()");
    test("(1, -2, 3)", 0, Exact, "(1, -2, 3)");
    test("(-1, 2, -3, 5)", 1, Floor, "(-1, 1, -2, 2)");
    test("(-1, 2, -3, 5)", 1, Down, "(0, 1, -1, 2)");
    test("(-1, 2, -3, 5)", 1, Ceiling, "(0, 1, -1, 3)");
    test("(-1, 2, -3, 5)", 1, Up, "(-1, 1, -2, 3)");
    test("(-1, 2, -3, 5)", 1, Nearest, "(0, 1, -2, 2)");
    test("(4, -8, 12)", 2, Exact, "(1, -2, 3)");
    test("(7, -7)", 3, Nearest, "(1, -1)");
    test("(7, -1)", 100, Up, "(1, -1)");
    test("(7, -1)", 100, Down, "(0, 0)");
    test("(1, -2, 3)", -2, Exact, "(4, -8, 12)");
}

#[test]
fn test_entrywise_shr_round() {
    apply_fn_to_unsigneds!(test_entrywise_shr_round_unsigned_helper);
    apply_fn_to_signeds!(test_entrywise_shr_round_signed_helper);
}

macro_rules! entrywise_shr_round_unsigned_fail_helper {
    ($t:ident) => {
        assert_panic!(
            IntegerVector::from_str("(1, -2)")
                .unwrap()
                .entrywise_shr_round($t::ONE, Exact)
        );
        assert_panic!(
            (&IntegerVector::from_str("(1, -2)").unwrap()).entrywise_shr_round($t::ONE, Exact)
        );
        assert_panic!({
            let mut v = IntegerVector::from_str("(1, -2)").unwrap();
            v.entrywise_shr_round_assign($t::ONE, Exact);
        });
    };
}

#[test]
fn entrywise_shr_round_unsigned_fail() {
    apply_to_unsigneds!(entrywise_shr_round_unsigned_fail_helper);
}

macro_rules! entrywise_shr_round_signed_fail_helper {
    ($t:ident) => {
        assert_panic!(
            IntegerVector::from_str("(1, -2)")
                .unwrap()
                .entrywise_shr_round($t::ONE, Exact)
        );
        assert_panic!(
            (&IntegerVector::from_str("(1, -2)").unwrap()).entrywise_shr_round($t::ONE, Exact)
        );
        assert_panic!({
            let mut v = IntegerVector::from_str("(1, -2)").unwrap();
            v.entrywise_shr_round_assign($t::ONE, Exact);
        });
    };
}

#[test]
fn entrywise_shr_round_signed_fail() {
    apply_to_signeds!(entrywise_shr_round_signed_fail_helper);
}

fn entrywise_shr_round_unsigned_properties_helper<T: PrimitiveUnsigned>()
where
    IntegerVector: EntrywiseShrRound<T, Output = IntegerVector>
        + EntrywiseShrRoundAssign<T>
        + Shl<T, Output = IntegerVector>,
    for<'a> &'a IntegerVector: EntrywiseShrRound<T, Output = IntegerVector>
        + Shr<T, Output = IntegerVector>
        + Shl<T, Output = IntegerVector>,
    for<'a> &'a Integer: ShrRound<T, Output = Integer>,
{
    integer_vector_unsigned_rounding_mode_triple_gen_var_1::<T>().test_properties(
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
            for (x, y) in v.elements.iter().zip(&w.elements) {
                assert_eq!(*y, x.shr_round(bits, rm).0);
            }
            // Rounding toward negative infinity is the plain shift.
            if rm == Floor {
                assert_eq!(&v >> bits, w);
            }
            // An exact shift can be undone.
            if rm == Exact {
                assert_eq!(&w << bits, v);
            }
        },
    );

    integer_vector_unsigned_pair_gen_var_3::<T>().test_properties(|(v, bits)| {
        // Nearest lies between Floor and Ceiling, which differ by at most 1, element by element.
        let f = (&v).entrywise_shr_round(bits, Floor);
        let c = (&v).entrywise_shr_round(bits, Ceiling);
        let n = (&v).entrywise_shr_round(bits, Nearest);
        for ((f, c), n) in f.elements.iter().zip(&c.elements).zip(&n.elements) {
            assert!(f <= n && n <= c);
            assert!(*c == *f || *c == f + Integer::ONE);
        }
    });
}

fn entrywise_shr_round_signed_properties_helper<T: PrimitiveSigned>()
where
    IntegerVector: EntrywiseShrRound<T, Output = IntegerVector>
        + EntrywiseShrRoundAssign<T>
        + Shl<T, Output = IntegerVector>,
    for<'a> &'a IntegerVector: EntrywiseShrRound<T, Output = IntegerVector>
        + Shr<T, Output = IntegerVector>
        + Shl<T, Output = IntegerVector>
        + EntrywiseShlRound<T, Output = IntegerVector>,
    for<'a> &'a Integer: ShrRound<T, Output = Integer>,
{
    integer_vector_signed_rounding_mode_triple_gen_var_2::<T>().test_properties(|(v, bits, rm)| {
        let w = (&v).entrywise_shr_round(bits, rm);
        // The forms agree.
        assert_eq!(v.clone().entrywise_shr_round(bits, rm), w);
        let mut x = v.clone();
        x.entrywise_shr_round_assign(bits, rm);
        assert_eq!(x, w);

        // Element by element, this is the scalar rounding shift, and the dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x.shr_round(bits, rm).0);
        }
        // Rounding toward negative infinity is the plain shift.
        if rm == Floor {
            assert_eq!(&v >> bits, w);
        }
        // An exact shift can be undone.
        if rm == Exact {
            assert_eq!(&w << bits, v);
        }
        // Shifting by the negated amount the other way is the same.
        if let Some(neg_bits) = bits.checked_neg() {
            assert_eq!((&v).entrywise_shl_round(neg_bits, rm), w);
        }
    });

    integer_vector_signed_pair_gen_var_1::<T>().test_properties(|(v, bits)| {
        // Nearest lies between Floor and Ceiling, which differ by at most 1, element by element.
        let f = (&v).entrywise_shr_round(bits, Floor);
        let c = (&v).entrywise_shr_round(bits, Ceiling);
        let n = (&v).entrywise_shr_round(bits, Nearest);
        for ((f, c), n) in f.elements.iter().zip(&c.elements).zip(&n.elements) {
            assert!(f <= n && n <= c);
            assert!(*c == *f || *c == f + Integer::ONE);
        }
    });
}

#[test]
fn entrywise_shr_round_properties() {
    apply_fn_to_unsigneds!(entrywise_shr_round_unsigned_properties_helper);
    apply_fn_to_signeds!(entrywise_shr_round_signed_properties_helper);
}
