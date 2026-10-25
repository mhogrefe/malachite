// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::{Shl, Shr, ShrAssign};
use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::{
    rational_matrix_signed_pair_gen_var_1, rational_matrix_unsigned_pair_gen_var_3,
};

fn test_shr_unsigned_helper<T: PrimitiveUnsigned>()
where
    RationalMatrix: Shr<T, Output = RationalMatrix> + ShrAssign<T>,
    for<'a> &'a RationalMatrix: Shr<T, Output = RationalMatrix>,
{
    let test = |s, bits: u8, out| {
        let bits = T::from(bits);
        let a = RationalMatrix::from_str(s).unwrap();
        let b = &a >> bits;
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone() >> bits, b);
        let mut x = a;
        x >>= bits;
        assert_eq!(x, b);
    };
    test("[]_{0,0}", 0, "[]_{0,0}");
    test("[1/2, -3/4, 3]", 0, "[1/2, -3/4, 3]");
    test("[1/2, -3/4, 3]", 2, "[1/8, -3/16, 3/4]");
    test("[1/2, -3/4; 3, 5/6]", 2, "[1/8, -3/16; 3/4, 5/24]");
    test("[]_{2,0}", 3, "[]_{2,0}");
    test("[12, 0]", 2, "[3, 0]");
    test("[-1/3]", 100, "[-1/3802951800684688204490109616128]");
}

fn test_shr_signed_helper<T: PrimitiveSigned>()
where
    RationalMatrix: Shr<T, Output = RationalMatrix> + ShrAssign<T>,
    for<'a> &'a RationalMatrix: Shr<T, Output = RationalMatrix>,
{
    let test = |s, bits: i8, out| {
        let bits = T::from(bits);
        let a = RationalMatrix::from_str(s).unwrap();
        let b = &a >> bits;
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone() >> bits, b);
        let mut x = a;
        x >>= bits;
        assert_eq!(x, b);
    };
    test("[]_{0,0}", 0, "[]_{0,0}");
    test("[]_{0,0}", -5, "[]_{0,0}");
    test("[1/2, -3/4, 3]", 0, "[1/2, -3/4, 3]");
    test("[1/2, -3/4, 3]", 2, "[1/8, -3/16, 3/4]");
    test("[1/2, -3/4; 3, 5/6]", 2, "[1/8, -3/16; 3/4, 5/24]");
    test("[]_{2,0}", 3, "[]_{2,0}");
    test("[1/2, -3/4, 3]", -2, "[2, -3, 12]");
    test("[12, 0]", 2, "[3, 0]");
    test("[-1/3]", 100, "[-1/3802951800684688204490109616128]");
    test("[-1/3]", -100, "[-1267650600228229401496703205376/3]");
}

#[test]
fn test_shr() {
    apply_fn_to_unsigneds!(test_shr_unsigned_helper);
    apply_fn_to_signeds!(test_shr_signed_helper);
}

fn shr_unsigned_properties_helper<T: PrimitiveUnsigned>()
where
    RationalMatrix: Shr<T, Output = RationalMatrix> + ShrAssign<T>,
    for<'a> &'a RationalMatrix: Shr<T, Output = RationalMatrix> + Shl<T, Output = RationalMatrix>,
    for<'a> &'a Rational: Shr<T, Output = Rational>,
    Rational: Shr<T, Output = Rational>,
    i64: ExactFrom<T>,
{
    rational_matrix_unsigned_pair_gen_var_3::<T>().test_properties(|(a, bits)| {
        let b = &a >> bits;
        // The forms agree.
        assert_eq!(a.clone() >> bits, b);
        let mut x = a.clone();
        x >>= bits;
        assert_eq!(x, b);

        // Element by element, this is the scalar shift, and the shape is unchanged.
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        for (x, y) in a.elements_ref().iter().zip(b.elements_ref()) {
            assert_eq!(*y, x >> bits);
        }
        // It is division by a power of 2, and shifting by the same amount of another type gives the
        // same result.
        assert_eq!(&a * Rational::power_of_2(-i64::exact_from(bits)), b);
        assert_eq!(
            <&RationalMatrix as Shr<i64>>::shr(&a, i64::exact_from(bits)),
            b
        );
        // Shifting by 0 changes nothing, shifting back recovers the matrix, and shifts compose.
        assert_eq!(&a >> T::ZERO, a);
        assert_eq!(&b << bits, a);
        if let Some(double) = bits.checked_add(bits) {
            assert_eq!(&b >> bits, &a >> double);
        }
        // It commutes with transposition.
        assert_eq!(a.transpose() >> bits, b.transpose());
    });
}

fn shr_signed_properties_helper<T: PrimitiveSigned>()
where
    RationalMatrix: Shr<T, Output = RationalMatrix> + ShrAssign<T>,
    for<'a> &'a RationalMatrix: Shr<T, Output = RationalMatrix> + Shl<T, Output = RationalMatrix>,
    for<'a> &'a Rational: Shr<T, Output = Rational>,
    Rational: Shr<T, Output = Rational>,
    i64: ExactFrom<T>,
{
    rational_matrix_signed_pair_gen_var_1::<T>().test_properties(|(a, bits)| {
        let b = &a >> bits;
        // The forms agree.
        assert_eq!(a.clone() >> bits, b);
        let mut x = a.clone();
        x >>= bits;
        assert_eq!(x, b);

        // Element by element, this is the scalar shift, and the shape is unchanged.
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        for (x, y) in a.elements_ref().iter().zip(b.elements_ref()) {
            assert_eq!(*y, x >> bits);
        }
        // It is division by a power of 2, and shifting by the same amount of another type gives the
        // same result.
        assert_eq!(&a * Rational::power_of_2(-i64::exact_from(bits)), b);
        assert_eq!(
            <&RationalMatrix as Shr<i64>>::shr(&a, i64::exact_from(bits)),
            b
        );
        // Shifting back recovers the matrix, and a right shift is a left shift by the negated
        // amount.
        assert_eq!(&b << bits, a);
        if let Some(neg_bits) = bits.checked_neg() {
            assert_eq!(&a << neg_bits, b);
        }
        // It commutes with transposition.
        assert_eq!(a.transpose() >> bits, b.transpose());
    });
}

#[test]
fn shr_properties() {
    apply_fn_to_unsigneds!(shr_unsigned_properties_helper);
    apply_fn_to_signeds!(shr_signed_properties_helper);
}
