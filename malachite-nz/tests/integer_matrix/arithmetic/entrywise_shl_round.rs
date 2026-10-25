// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::{Shl, Shr};
use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    EntrywiseShlRound, EntrywiseShlRoundAssign, EntrywiseShrRound, ShlRound,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{NegativeOne, One};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::{
    integer_matrix_signed_pair_gen_var_1, integer_matrix_signed_rounding_mode_triple_gen_var_1,
};
use std::panic::catch_unwind;

fn test_entrywise_shl_round_signed_helper<T: PrimitiveSigned>()
where
    IntegerMatrix: EntrywiseShlRound<T, Output = IntegerMatrix> + EntrywiseShlRoundAssign<T>,
    for<'a> &'a IntegerMatrix: EntrywiseShlRound<T, Output = IntegerMatrix>,
{
    let test = |s, bits: i8, rm: RoundingMode, out| {
        let bits = T::from(bits);
        let v = IntegerMatrix::from_str(s).unwrap();
        let w = (&v).entrywise_shl_round(bits, rm);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().entrywise_shl_round(bits, rm), w);
        let mut x = v;
        x.entrywise_shl_round_assign(bits, rm);
        assert_eq!(x, w);
    };
    test("[]_{0,0}", 0, Exact, "[]_{0,0}");
    test("[]_{0,0}", -10, Nearest, "[]_{0,0}");
    test("[1, -2, 3]", 0, Exact, "[1, -2, 3]");
    test("[1, -2, 3]", 2, Exact, "[4, -8, 12]");
    test("[-1, 2; -3, 5]", -1, Floor, "[-1, 1; -2, 2]");
    test("[-1, 2; -3, 5]", -1, Down, "[0, 1; -1, 2]");
    test("[-1, 2; -3, 5]", -1, Ceiling, "[0, 1; -1, 3]");
    test("[-1, 2; -3, 5]", -1, Up, "[-1, 1; -2, 3]");
    test("[-1, 2; -3, 5]", -1, Nearest, "[0, 1; -2, 2]");
    test("[4, -8, 12]", -2, Exact, "[1, -2, 3]");
    test("[7, -7]", -3, Nearest, "[1, -1]");
}

#[test]
fn test_entrywise_shl_round() {
    apply_fn_to_signeds!(test_entrywise_shl_round_signed_helper);
}

macro_rules! entrywise_shl_round_signed_fail_helper {
    ($t:ident) => {
        assert_panic!(
            IntegerMatrix::from_str("[1, -2]")
                .unwrap()
                .entrywise_shl_round($t::NEGATIVE_ONE, Exact)
        );
        assert_panic!(
            (&IntegerMatrix::from_str("[1, -2]").unwrap())
                .entrywise_shl_round($t::NEGATIVE_ONE, Exact)
        );
        assert_panic!({
            let mut v = IntegerMatrix::from_str("[1, -2]").unwrap();
            v.entrywise_shl_round_assign($t::NEGATIVE_ONE, Exact);
        });
    };
}

#[test]
fn entrywise_shl_round_signed_fail() {
    apply_to_signeds!(entrywise_shl_round_signed_fail_helper);
}

fn entrywise_shl_round_signed_properties_helper<T: PrimitiveSigned>()
where
    IntegerMatrix: EntrywiseShlRound<T, Output = IntegerMatrix>
        + EntrywiseShlRoundAssign<T>
        + Shr<T, Output = IntegerMatrix>,
    for<'a> &'a IntegerMatrix: EntrywiseShlRound<T, Output = IntegerMatrix>
        + Shl<T, Output = IntegerMatrix>
        + Shr<T, Output = IntegerMatrix>
        + EntrywiseShrRound<T, Output = IntegerMatrix>,
    for<'a> &'a Integer: ShlRound<T, Output = Integer>,
{
    integer_matrix_signed_rounding_mode_triple_gen_var_1::<T>().test_properties(|(v, bits, rm)| {
        let w = (&v).entrywise_shl_round(bits, rm);
        // The forms agree.
        assert_eq!(v.clone().entrywise_shl_round(bits, rm), w);
        let mut x = v.clone();
        x.entrywise_shl_round_assign(bits, rm);
        assert_eq!(x, w);

        // Element by element, this is the scalar rounding shift, and the shape is unchanged.
        assert_eq!((w.height(), w.width()), (v.height(), v.width()));
        for (x, y) in v.elements_ref().iter().zip(w.elements_ref()) {
            assert_eq!(*y, x.shl_round(bits, rm).0);
        }
        // Rounding toward negative infinity is the plain shift.
        if rm == Floor {
            assert_eq!(&v << bits, w);
        }
        // An exact shift can be undone.
        if rm == Exact {
            assert_eq!(&w >> bits, v);
        }
        // Shifting by the negated amount the other way is the same.
        if let Some(neg_bits) = bits.checked_neg() {
            assert_eq!((&v).entrywise_shr_round(neg_bits, rm), w);
        }
    });

    integer_matrix_signed_pair_gen_var_1::<T>().test_properties(|(v, bits)| {
        // Nearest lies between Floor and Ceiling, which differ by at most 1, element by element.
        let f = (&v).entrywise_shl_round(bits, Floor);
        let c = (&v).entrywise_shl_round(bits, Ceiling);
        let n = (&v).entrywise_shl_round(bits, Nearest);
        for ((f, c), n) in f
            .elements_ref()
            .iter()
            .zip(c.elements_ref())
            .zip(n.elements_ref())
        {
            assert!(f <= n && n <= c);
            assert!(*c == *f || *c == f + Integer::ONE);
        }
    });
}

#[test]
fn entrywise_shl_round_properties() {
    apply_fn_to_signeds!(entrywise_shl_round_signed_properties_helper);
}
