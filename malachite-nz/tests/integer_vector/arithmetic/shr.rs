// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::{Shl, Shr, ShrAssign};
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::EntrywiseShrRound;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::{
    integer_vector_signed_pair_gen_var_1, integer_vector_unsigned_pair_gen_var_3,
};

fn test_shr_unsigned_helper<T: PrimitiveUnsigned>()
where
    IntegerVector: Shr<T, Output = IntegerVector> + ShrAssign<T>,
    for<'a> &'a IntegerVector: Shr<T, Output = IntegerVector>,
{
    let test = |s, bits: u8, out| {
        let bits = T::from(bits);
        let v = IntegerVector::from_str(s).unwrap();
        let w = &v >> bits;
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone() >> bits, w);
        let mut x = v;
        x >>= bits;
        assert_eq!(x, w);
    };
    test("()", 0, "()");
    test("()", 10, "()");
    test("(1, -2, 3)", 0, "(1, -2, 3)");
    test("(4, -8, 12)", 2, "(1, -2, 3)");
    test("(-1, 2, -3, 5)", 1, "(-1, 1, -2, 2)");
    test("(7, -1)", 3, "(0, -1)");
    test(
        "(8873554201597605810476922437632, -1267650600228229401496703205376)",
        100,
        "(7, -1)",
    );
}

fn test_shr_signed_helper<T: PrimitiveSigned>()
where
    IntegerVector: Shr<T, Output = IntegerVector> + ShrAssign<T>,
    for<'a> &'a IntegerVector: Shr<T, Output = IntegerVector>,
{
    let test = |s, bits: i8, out| {
        let bits = T::from(bits);
        let v = IntegerVector::from_str(s).unwrap();
        let w = &v >> bits;
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone() >> bits, w);
        let mut x = v;
        x >>= bits;
        assert_eq!(x, w);
    };
    test("()", 0, "()");
    test("()", -5, "()");
    test("(1, -2, 3)", 0, "(1, -2, 3)");
    test("(4, -8, 12)", 2, "(1, -2, 3)");
    test("(-1, 2, -3, 5)", 1, "(-1, 1, -2, 2)");
    test("(1, -2, 3)", -2, "(4, -8, 12)");
    test("(7, -1)", 3, "(0, -1)");
}

#[test]
fn test_shr() {
    apply_fn_to_unsigneds!(test_shr_unsigned_helper);
    apply_fn_to_signeds!(test_shr_signed_helper);
}

fn shr_unsigned_properties_helper<T: PrimitiveUnsigned>()
where
    IntegerVector: Shr<T, Output = IntegerVector> + ShrAssign<T>,
    for<'a> &'a IntegerVector: Shr<T, Output = IntegerVector>
        + Shl<T, Output = IntegerVector>
        + EntrywiseShrRound<T, Output = IntegerVector>,
    for<'a> &'a Integer: Shr<T, Output = Integer>,
    u64: ExactFrom<T>,
{
    integer_vector_unsigned_pair_gen_var_3::<T>().test_properties(|(v, bits)| {
        let w = &v >> bits;
        // The forms agree.
        assert_eq!(v.clone() >> bits, w);
        let mut x = v.clone();
        x >>= bits;
        assert_eq!(x, w);

        // Element by element, this is the scalar shift, and the dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x >> bits);
        }
        // Shifting by the same amount of another type gives the same result.
        assert_eq!(
            <&IntegerVector as Shr<u64>>::shr(&v, u64::exact_from(bits)),
            w
        );
        // It takes the floor.
        assert_eq!((&v).entrywise_shr_round(bits, Floor), w);
        // Shifting by 0 changes nothing, shifting left and then back recovers the vector, and
        // shifts compose.
        assert_eq!(&v >> T::ZERO, v);
        assert_eq!(&(&v << bits) >> bits, v);
        if let Some(double) = bits.checked_add(bits) {
            assert_eq!(&w >> bits, &v >> double);
        }
    });
}

fn shr_signed_properties_helper<T: PrimitiveSigned>()
where
    IntegerVector: Shr<T, Output = IntegerVector> + ShrAssign<T>,
    for<'a> &'a IntegerVector: Shr<T, Output = IntegerVector>
        + Shl<T, Output = IntegerVector>
        + EntrywiseShrRound<T, Output = IntegerVector>,
    for<'a> &'a Integer: Shr<T, Output = Integer>,
    i64: ExactFrom<T>,
{
    integer_vector_signed_pair_gen_var_1::<T>().test_properties(|(v, bits)| {
        let w = &v >> bits;
        // The forms agree.
        assert_eq!(v.clone() >> bits, w);
        let mut x = v.clone();
        x >>= bits;
        assert_eq!(x, w);

        // Element by element, this is the scalar shift, and the dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x >> bits);
        }
        // Shifting by the same amount of another type gives the same result.
        assert_eq!(
            <&IntegerVector as Shr<i64>>::shr(&v, i64::exact_from(bits)),
            w
        );
        // It takes the floor.
        assert_eq!((&v).entrywise_shr_round(bits, Floor), w);
        // A right shift is a left shift by the negated amount, and shifting left and then back
        // recovers the vector.
        if let Some(neg_bits) = bits.checked_neg() {
            assert_eq!(&v << neg_bits, w);
        }
        if bits <= T::ZERO {
            assert_eq!(&w << bits, v);
        }
    });
}

#[test]
fn shr_properties() {
    apply_fn_to_unsigneds!(shr_unsigned_properties_helper);
    apply_fn_to_signeds!(shr_signed_properties_helper);
}
