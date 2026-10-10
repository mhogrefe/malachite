// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::{Shl, ShlAssign, Shr};
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{L1Norm, PowerOf2};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vector_unsigned_pair_gen_var_3;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::{
    rational_vector_signed_pair_gen_var_1, rational_vector_unsigned_pair_gen_var_2,
};

fn test_shl_helper<T: PrimitiveUnsigned>()
where
    RationalVector: Shl<T, Output = RationalVector> + ShlAssign<T>,
    for<'a> &'a RationalVector: Shl<T, Output = RationalVector>,
{
    let test = |s, bits: u8, out| {
        let bits = T::from(bits);
        let v = RationalVector::from_str(s).unwrap();
        let w = &v << bits;
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone() << bits, w);
        let mut x = v;
        x <<= bits;
        assert_eq!(x, w);
    };
    test("()", 0, "()");
    test("(1/2, -3/4, 3)", 0, "(1/2, -3/4, 3)");
    test("(1/2, -3/4, 3)", 2, "(2, -3, 12)");
    test("(1/8, 0)", 2, "(1/2, 0)");
    test("(-1/3)", 100, "(-1267650600228229401496703205376/3)");
}

fn test_shl_signed_helper<T: PrimitiveSigned>()
where
    RationalVector: Shl<T, Output = RationalVector> + ShlAssign<T>,
    for<'a> &'a RationalVector: Shl<T, Output = RationalVector>,
{
    let test = |s, bits: i8, out| {
        let bits = T::from(bits);
        let v = RationalVector::from_str(s).unwrap();
        let w = &v << bits;
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone() << bits, w);
        let mut x = v;
        x <<= bits;
        assert_eq!(x, w);
    };
    test("()", 0, "()");
    test("()", -5, "()");
    test("(1/2, -3/4, 3)", 0, "(1/2, -3/4, 3)");
    test("(1/2, -3/4, 3)", 2, "(2, -3, 12)");
    test("(1/2, -3/4, 3)", -2, "(1/8, -3/16, 3/4)");
    test("(12, 0)", -2, "(3, 0)");
    test("(-1/3)", 100, "(-1267650600228229401496703205376/3)");
    test("(-1/3)", -100, "(-1/3802951800684688204490109616128)");
}

#[test]
fn test_shl() {
    apply_fn_to_unsigneds!(test_shl_helper);
    apply_fn_to_signeds!(test_shl_signed_helper);
}

fn shl_properties_helper<T: PrimitiveUnsigned>()
where
    RationalVector: Shl<T, Output = RationalVector> + ShlAssign<T>,
    for<'a> &'a RationalVector: Shl<T, Output = RationalVector>,
    Rational: Shl<T, Output = Rational>,
    for<'a> &'a Rational: Shl<T, Output = Rational>,
    u64: ExactFrom<T>,
    for<'a> &'a IntegerVector: Shl<T, Output = IntegerVector>,
{
    rational_vector_unsigned_pair_gen_var_2::<T>().test_properties(|(v, bits)| {
        let w = &v << bits;
        // The forms agree.
        assert_eq!(v.clone() << bits, w);
        let mut x = v.clone();
        x <<= bits;
        assert_eq!(x, w);

        // Element by element, this is the scalar shift, and the dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x << bits);
        }
        // It is multiplication by a power of 2, and shifting by the same amount of another type
        // gives the same result.
        assert_eq!(&v * Rational::power_of_2(u64::exact_from(bits)), w);
        assert_eq!(
            <&RationalVector as Shl<u64>>::shl(&v, u64::exact_from(bits)),
            w
        );
        // Shifting by 0 changes nothing, and shifts compose.
        assert_eq!(&v << T::ZERO, v);
        if let Some(double) = bits.checked_add(bits) {
            assert_eq!(&w << bits, &v << double);
        }
        // The l^1 norm is shifted too.
        assert_eq!(w.to_l1_norm(), v.to_l1_norm() << bits);
    });

    integer_vector_unsigned_pair_gen_var_3::<T>().test_properties(|(v, bits)| {
        // As a RationalVector, the shift is the same.
        assert_eq!(
            RationalVector::from(&v << bits),
            RationalVector::from(v) << bits
        );
    });
}

#[test]
fn shl_properties() {
    apply_fn_to_unsigneds!(shl_properties_helper);
}

fn shl_signed_properties_helper<T: PrimitiveSigned>()
where
    RationalVector: Shl<T, Output = RationalVector> + ShlAssign<T>,
    for<'a> &'a RationalVector: Shl<T, Output = RationalVector> + Shr<T, Output = RationalVector>,
    for<'a> &'a Rational: Shl<T, Output = Rational>,
    Rational: Shl<T, Output = Rational>,
    i64: ExactFrom<T>,
{
    rational_vector_signed_pair_gen_var_1::<T>().test_properties(|(v, bits)| {
        let w = &v << bits;
        // The forms agree.
        assert_eq!(v.clone() << bits, w);
        let mut x = v.clone();
        x <<= bits;
        assert_eq!(x, w);

        // Element by element, this is the scalar shift, and the dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x << bits);
        }
        // It is multiplication by a power of 2, and shifting by the same amount of another type
        // gives the same result.
        assert_eq!(&v * Rational::power_of_2(i64::exact_from(bits)), w);
        assert_eq!(
            <&RationalVector as Shl<i64>>::shl(&v, i64::exact_from(bits)),
            w
        );
        // A left shift is a right shift by the negated amount, and shifting back recovers the
        // vector.
        if let Some(neg_bits) = bits.checked_neg() {
            assert_eq!(&v >> neg_bits, w);
            assert_eq!(&w << neg_bits, v);
        }
        // The l^1 norm is shifted too.
        assert_eq!(w.to_l1_norm(), v.to_l1_norm() << bits);
    });
}

#[test]
fn shl_signed_properties() {
    apply_fn_to_signeds!(shl_signed_properties_helper);
}
