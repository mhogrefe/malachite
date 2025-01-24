// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::kronecker_symbol::{
    jacobi_symbol_unsigned_double_fast_2, jacobi_symbol_unsigned_simple,
};
use malachite_base::num::arithmetic::traits::{ModPowerOf2, UnsignedAbs};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{HasHalf, JoinHalves, WrappingFrom};
use malachite_base::test_util::generators::{
    signed_gen, signed_gen_var_13, signed_pair_gen, signed_pair_gen_var_10, signed_pair_gen_var_8,
    signed_pair_gen_var_9, signed_triple_gen, signed_triple_gen_var_6, signed_triple_gen_var_7,
    unsigned_gen, unsigned_gen_var_22, unsigned_pair_gen_var_27, unsigned_pair_gen_var_40,
    unsigned_pair_gen_var_41, unsigned_pair_gen_var_42, unsigned_quadruple_gen_var_12,
    unsigned_triple_gen_var_19, unsigned_triple_gen_var_22, unsigned_triple_gen_var_23,
};
use malachite_base::test_util::num::arithmetic::kronecker_symbol::{
    jacobi_symbol_unsigned_double_fast_1, jacobi_symbol_unsigned_double_simple,
    jacobi_symbol_unsigned_fast_1, jacobi_symbol_unsigned_fast_2_1,
    jacobi_symbol_unsigned_fast_2_2, jacobi_symbol_unsigned_fast_2_3,
    jacobi_symbol_unsigned_fast_2_4,
};
use std::panic::catch_unwind;

#[test]
fn test_jacobi_symbol() {
    fn test_u<T: PrimitiveUnsigned>(a: T, n: T, s: i8) {
        assert_eq!(a.legendre_symbol(n), s);
        assert_eq!(a.jacobi_symbol(n), s);
        assert_eq!(a.kronecker_symbol(n), s);
        assert_eq!(jacobi_symbol_unsigned_simple(a, n), s);
        assert_eq!(jacobi_symbol_unsigned_fast_1(a, n), s);
        assert_eq!(jacobi_symbol_unsigned_fast_2_1(a, n), s);
        assert_eq!(jacobi_symbol_unsigned_fast_2_2(a, n), s);
        assert_eq!(jacobi_symbol_unsigned_fast_2_3(a, n), s);
        assert_eq!(jacobi_symbol_unsigned_fast_2_4(a, n), s);
    }
    test_u::<u8>(0, 1, 1);
    test_u::<u8>(0, 3, 0);
    test_u::<u8>(1, 3, 1);
    test_u::<u8>(2, 3, -1);
    test_u::<u8>(0, 5, 0);
    test_u::<u8>(1, 5, 1);
    test_u::<u8>(2, 5, -1);
    test_u::<u8>(3, 5, -1);
    test_u::<u8>(4, 5, 1);
    test_u::<u8>(0, 7, 0);
    test_u::<u8>(1, 7, 1);
    test_u::<u8>(2, 7, 1);
    test_u::<u8>(3, 7, -1);
    test_u::<u8>(4, 7, 1);
    test_u::<u8>(5, 7, -1);
    test_u::<u8>(6, 7, -1);
    test_u::<u8>(0, 9, 0);
    test_u::<u8>(1, 9, 1);
    test_u::<u8>(2, 9, 1);
    test_u::<u8>(3, 9, 0);
    test_u::<u8>(4, 9, 1);
    test_u::<u8>(5, 9, 1);
    test_u::<u8>(6, 9, 0);
    test_u::<u8>(7, 9, 1);
    test_u::<u8>(8, 9, 1);

    test_u::<u8>(7, 7, 0);
    test_u::<u8>(8, 7, 1);
    test_u::<u8>(9, 7, 1);
    test_u::<u8>(10, 7, -1);
    test_u::<u8>(11, 7, 1);
    test_u::<u8>(12, 7, -1);
    test_u::<u8>(13, 7, -1);
    test_u::<u8>(9, 9, 0);
    test_u::<u8>(10, 9, 1);
    test_u::<u8>(11, 9, 1);
    test_u::<u8>(12, 9, 0);
    test_u::<u8>(13, 9, 1);
    test_u::<u8>(14, 9, 1);
    test_u::<u8>(15, 9, 0);
    test_u::<u8>(16, 9, 1);
    test_u::<u8>(17, 9, 1);

    test_u::<u16>(1001, 9907, -1);
    test_u::<u16>(10908, 9907, -1);

    fn test_s<U: PrimitiveUnsigned, S: ModPowerOf2<Output = U> + PrimitiveSigned>(
        a: S,
        n: S,
        s: i8,
    ) {
        assert_eq!(a.legendre_symbol(n), s);
        assert_eq!(a.jacobi_symbol(n), s);
        assert_eq!(a.kronecker_symbol(n), s);
    }
    test_s::<u8, i8>(0, 1, 1);
    test_s::<u8, i8>(0, 3, 0);
    test_s::<u8, i8>(1, 3, 1);
    test_s::<u8, i8>(2, 3, -1);
    test_s::<u8, i8>(0, 5, 0);
    test_s::<u8, i8>(1, 5, 1);
    test_s::<u8, i8>(2, 5, -1);
    test_s::<u8, i8>(3, 5, -1);
    test_s::<u8, i8>(4, 5, 1);
    test_s::<u8, i8>(0, 7, 0);
    test_s::<u8, i8>(1, 7, 1);
    test_s::<u8, i8>(2, 7, 1);
    test_s::<u8, i8>(3, 7, -1);
    test_s::<u8, i8>(4, 7, 1);
    test_s::<u8, i8>(5, 7, -1);
    test_s::<u8, i8>(6, 7, -1);
    test_s::<u8, i8>(0, 9, 0);
    test_s::<u8, i8>(1, 9, 1);
    test_s::<u8, i8>(2, 9, 1);
    test_s::<u8, i8>(3, 9, 0);
    test_s::<u8, i8>(4, 9, 1);
    test_s::<u8, i8>(5, 9, 1);
    test_s::<u8, i8>(6, 9, 0);
    test_s::<u8, i8>(7, 9, 1);
    test_s::<u8, i8>(8, 9, 1);

    test_s::<u8, i8>(7, 7, 0);
    test_s::<u8, i8>(8, 7, 1);
    test_s::<u8, i8>(9, 7, 1);
    test_s::<u8, i8>(10, 7, -1);
    test_s::<u8, i8>(11, 7, 1);
    test_s::<u8, i8>(12, 7, -1);
    test_s::<u8, i8>(13, 7, -1);
    test_s::<u8, i8>(9, 9, 0);
    test_s::<u8, i8>(10, 9, 1);
    test_s::<u8, i8>(11, 9, 1);
    test_s::<u8, i8>(12, 9, 0);
    test_s::<u8, i8>(13, 9, 1);
    test_s::<u8, i8>(14, 9, 1);
    test_s::<u8, i8>(15, 9, 0);
    test_s::<u8, i8>(16, 9, 1);
    test_s::<u8, i8>(17, 9, 1);

    test_s::<u8, i8>(-7, 7, 0);
    test_s::<u8, i8>(-6, 7, 1);
    test_s::<u8, i8>(-5, 7, 1);
    test_s::<u8, i8>(-4, 7, -1);
    test_s::<u8, i8>(-3, 7, 1);
    test_s::<u8, i8>(-2, 7, -1);
    test_s::<u8, i8>(-1, 7, -1);
    test_s::<u8, i8>(-9, 9, 0);
    test_s::<u8, i8>(-8, 9, 1);
    test_s::<u8, i8>(-7, 9, 1);
    test_s::<u8, i8>(-6, 9, 0);
    test_s::<u8, i8>(-5, 9, 1);
    test_s::<u8, i8>(-4, 9, 1);
    test_s::<u8, i8>(-3, 9, 0);
    test_s::<u8, i8>(-2, 9, 1);
    test_s::<u8, i8>(-1, 9, 1);

    test_s::<u16, i16>(1001, 9907, -1);
    test_s::<u16, i16>(10908, 9907, -1);
    test_s::<u16, i16>(-8906, 9907, -1);
}

fn jacobi_symbol_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::ONE.jacobi_symbol(T::TWO));
}

fn jacobi_symbol_fail_helper_signed<T: PrimitiveSigned>() {
    assert_panic!(T::ONE.jacobi_symbol(T::NEGATIVE_ONE));
}

#[test]
fn jacobi_symbol_fail() {
    apply_fn_to_primitive_ints!(jacobi_symbol_fail_helper);
    apply_fn_to_signeds!(jacobi_symbol_fail_helper_signed);
}

#[test]
fn test_jacobi_symbol_unsigned_double() {
    fn test<T: PrimitiveUnsigned, D: HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned>(
        x_1: T,
        x_0: T,
        y_1: T,
        y_0: T,
        s: i8,
    ) {
        assert_eq!(
            jacobi_symbol_unsigned_double_simple::<T, D>(x_1, x_0, y_1, y_0),
            s
        );
        assert_eq!(jacobi_symbol_unsigned_double_fast_1(x_1, x_0, y_1, y_0), s);
        assert_eq!(jacobi_symbol_unsigned_double_fast_2(x_1, x_0, y_1, y_0), s);
    }
    // - fast_1: y_1 == T::ZERO || y_0 == T::ZERO first time
    // - fast_2: y_1 == T::ZERO && y_0 == T::ONE
    test::<u64, u128>(0, 0, 0, 1, 1);
    // - fast_1: y_1 != T::ZERO && y_0 != T::ZERO first time
    // - fast_1: x_1 != T::ZERO && x_0 != T::ZERO
    // - fast_1: x_0 != T::ZERO first time
    // - fast_1: c != T::WIDTH first time
    // - fast_1: diff_0 != T::ZERO && diff_1 != T::ZERO
    // - fast_1: diff_1.get_highest_bit()
    // - fast_1: y_1 != T::ZERO && y_0 != T::ZERO second time
    // - fast_1: x_0 == T::ZERO second time
    // - fast_1: c != T::WIDTH second time
    // - fast_1: bit.even()
    // - fast_2: y_1 != T::ZERO || y_0 != T::ONE
    // - fast_2: x_0 != T::ZERO first time
    // - fast_2: x_0.odd()
    // - fast_2: x_1 == T::ZERO second time
    // - fast_2: y_1 != T::ZERO
    // - fast_2: skip_loop
    // - fast_2: y_0 != T::ONE second time
    // - fast_2: x_0 == T::ZERO fourth time
    // - fast_2: x_1 != T::ZERO fourth time
    // - fast_2: !bit.get_bit(1)
    test::<u64, u128>(0, 3, 2, 3, 1);
    // - fast_1: x_1 == T::ZERO || x_0 == T::ZERO
    // - fast_2: x_0 == T::ZERO first time
    // - fast_2: x_1 == T::ZERO first time
    test::<u64, u128>(0, 0, 0, 3, 0);
    // - fast_1: t != T::ZERO
    // - fast_1: c == T::WIDTH third time
    // - fast_2: y_0 == T::ONE second time
    test::<u64, u128>(0, 1, 1, 1, 1);
    // - fast_1: c != T::WIDTH third time
    test::<u64, u128>(0, 1, 1, 3, 1);
    // - fast_1: x_0 == T::ZERO first time
    // - fast_2: x_1 != T::ZERO first time
    // - fast_2: y_0 == T::ONE first time
    test::<u64, u128>(1, 0, 0, 3, 1);
    // - fast_1: bit.odd()
    // - fast_2: x_1 != T::ZERO second time
    // - fast_2: !skip_loop
    // - fast_2: x_0 != T::ZERO fourth time
    // - fast_2: bit.get_bit(1)
    test::<u64, u128>(1, 1, 0, 3, -1);
    // - fast_1: t == T::ZERO
    // - fast_2: x_1 == y_1 first time
    // - fast_2: x_1 == y_1 second time
    // - fast_2: x_0 >= y_0
    // - fast_2: x_0 == T::ZERO third time
    test::<u64, u128>(1, 1, 1, 1, 0);
    // - fast_1: y_1 == T::ZERO || y_0 == T::ZERO second time
    test::<u64, u128>(0, 1, 2, 1, 1);
    // - fast_1: x_0 != T::ZERO second time
    // - fast_1: c == T::WIDTH second time
    // - fast_2: x_1 != y_1 first time
    // - fast_2: x_1 != T::ZERO third time
    // - fast_2: y_0 == T::ZERO
    test::<u64, u128>(1, 1, 2, 1, 1);
    // - fast_1: !diff_1.get_highest_bit()
    test::<u64, u128>(2, 1, 0, 3, 0);
    // - fast_1: diff_0 == T::ZERO || diff_1 == T::ZERO
    test::<u64, u128>(2, 1, 2, 1, 0);
    // - fast_1: c == T::WIDTH first time
    // - fast_2: x_0.even()
    // - fast_2: y_0 != T::ZERO
    test::<u8, u16>(242, 128, 173, 173, -1);
    // - fast_2: y_1 == T::ZERO
    test::<u8, u16>(0, 1, 0, 3, 1);
    // - fast_2: x_0 < y_0
    // - fast_2: x_0 != T::ZERO third time
    // - fast_2: x_0 == T::ONE
    test::<u8, u16>(1, 1, 1, 3, 1);
    // - fast_2: x_0 != T::ONE
    test::<u8, u16>(1, 1, 1, 7, -1);
    // - fast_2: x_1 != y_1 second time
    test::<u8, u16>(1, 1, 2, 3, 1);
    // - fast_2: x_0 == T::ZERO second time
    test::<u8, u16>(2, 1, 1, 1, 1);
    // - fast_2: x_0 != T::ZERO second time
    // - fast_2: x_1 == T::ZERO third time
    test::<u8, u16>(2, 1, 1, 3, -1);
    // - fast_2: y_0 != T::ONE first time
    test::<u8, u16>(3, 0, 0, 3, 0);
}

// Odd n is already tested in test_jacobi_symbol, so here we just test even n
#[test]
fn test_kronecker_symbol() {
    fn test_u<T: PrimitiveUnsigned>(a: T, n: T, s: i8) {
        assert_eq!(a.kronecker_symbol(n), s);
    }
    test_u::<u8>(0, 2, 0);
    test_u::<u8>(1, 2, 1);
    test_u::<u8>(2, 2, 0);
    test_u::<u8>(3, 2, -1);
    test_u::<u8>(4, 2, 0);
    test_u::<u8>(5, 2, -1);
    test_u::<u8>(6, 2, 0);
    test_u::<u8>(7, 2, 1);
    test_u::<u8>(0, 4, 0);
    test_u::<u8>(1, 4, 1);
    test_u::<u8>(2, 4, 0);
    test_u::<u8>(3, 4, 1);
    test_u::<u8>(0, 6, 0);
    test_u::<u8>(1, 6, 1);
    test_u::<u8>(2, 6, 0);
    test_u::<u8>(3, 6, 0);
    test_u::<u8>(4, 6, 0);
    test_u::<u8>(5, 6, 1);
    test_u::<u8>(6, 6, 0);
    test_u::<u8>(7, 6, 1);
    test_u::<u8>(8, 6, 0);
    test_u::<u8>(9, 6, 0);
    test_u::<u8>(10, 6, 0);
    test_u::<u8>(11, 6, 1);
    test_u::<u8>(12, 6, 0);
    test_u::<u8>(13, 6, -1);
    test_u::<u8>(14, 6, 0);
    test_u::<u8>(15, 6, 0);
    test_u::<u8>(16, 6, 0);
    test_u::<u8>(17, 6, -1);
    test_u::<u8>(18, 6, 0);
    test_u::<u8>(19, 6, -1);
    test_u::<u8>(20, 6, 0);
    test_u::<u8>(21, 6, 0);
    test_u::<u8>(22, 6, 0);
    test_u::<u8>(23, 6, -1);

    test_u::<u16>(1001, 9908, -1);
    test_u::<u16>(10909, 9908, -1);

    fn test_s<U: PrimitiveUnsigned, S: ModPowerOf2<Output = U> + PrimitiveSigned>(
        a: S,
        n: S,
        s: i8,
    ) {
        assert_eq!(a.kronecker_symbol(n), s);
    }
    test_s::<u8, i8>(0, 2, 0);
    test_s::<u8, i8>(1, 2, 1);
    test_s::<u8, i8>(2, 2, 0);
    test_s::<u8, i8>(3, 2, -1);
    test_s::<u8, i8>(4, 2, 0);
    test_s::<u8, i8>(5, 2, -1);
    test_s::<u8, i8>(6, 2, 0);
    test_s::<u8, i8>(7, 2, 1);
    test_s::<u8, i8>(0, 4, 0);
    test_s::<u8, i8>(1, 4, 1);
    test_s::<u8, i8>(2, 4, 0);
    test_s::<u8, i8>(3, 4, 1);
    test_s::<u8, i8>(0, 6, 0);
    test_s::<u8, i8>(1, 6, 1);
    test_s::<u8, i8>(2, 6, 0);
    test_s::<u8, i8>(3, 6, 0);
    test_s::<u8, i8>(4, 6, 0);
    test_s::<u8, i8>(5, 6, 1);
    test_s::<u8, i8>(6, 6, 0);
    test_s::<u8, i8>(7, 6, 1);
    test_s::<u8, i8>(8, 6, 0);
    test_s::<u8, i8>(9, 6, 0);
    test_s::<u8, i8>(10, 6, 0);
    test_s::<u8, i8>(11, 6, 1);
    test_s::<u8, i8>(12, 6, 0);
    test_s::<u8, i8>(13, 6, -1);
    test_s::<u8, i8>(14, 6, 0);
    test_s::<u8, i8>(15, 6, 0);
    test_s::<u8, i8>(16, 6, 0);
    test_s::<u8, i8>(17, 6, -1);
    test_s::<u8, i8>(18, 6, 0);
    test_s::<u8, i8>(19, 6, -1);
    test_s::<u8, i8>(20, 6, 0);
    test_s::<u8, i8>(21, 6, 0);
    test_s::<u8, i8>(22, 6, 0);
    test_s::<u8, i8>(23, 6, -1);

    test_s::<u8, i8>(-1, 2, 1);
    test_s::<u8, i8>(-2, 2, 0);
    test_s::<u8, i8>(-3, 2, -1);
    test_s::<u8, i8>(-4, 2, 0);
    test_s::<u8, i8>(-5, 2, -1);
    test_s::<u8, i8>(-6, 2, 0);
    test_s::<u8, i8>(-7, 2, 1);
    test_s::<u8, i8>(-1, 4, 1);
    test_s::<u8, i8>(-2, 4, 0);
    test_s::<u8, i8>(-3, 4, 1);
    test_s::<u8, i8>(-1, 6, -1);
    test_s::<u8, i8>(-2, 6, 0);
    test_s::<u8, i8>(-3, 6, 0);
    test_s::<u8, i8>(-4, 6, 0);
    test_s::<u8, i8>(-5, 6, -1);
    test_s::<u8, i8>(-6, 6, 0);
    test_s::<u8, i8>(-7, 6, -1);
    test_s::<u8, i8>(-8, 6, 0);
    test_s::<u8, i8>(-9, 6, 0);
    test_s::<u8, i8>(-10, 6, 0);
    test_s::<u8, i8>(-11, 6, -1);
    test_s::<u8, i8>(-12, 6, 0);
    test_s::<u8, i8>(-13, 6, 1);
    test_s::<u8, i8>(-14, 6, 0);
    test_s::<u8, i8>(-15, 6, 0);
    test_s::<u8, i8>(-16, 6, 0);
    test_s::<u8, i8>(-17, 6, 1);
    test_s::<u8, i8>(-18, 6, 0);
    test_s::<u8, i8>(-19, 6, 1);
    test_s::<u8, i8>(-20, 6, 0);
    test_s::<u8, i8>(-21, 6, 0);
    test_s::<u8, i8>(-22, 6, 0);
    test_s::<u8, i8>(-23, 6, 1);

    test_s::<u8, i8>(0, -2, 0);
    test_s::<u8, i8>(1, -2, 1);
    test_s::<u8, i8>(2, -2, 0);
    test_s::<u8, i8>(3, -2, -1);
    test_s::<u8, i8>(4, -2, 0);
    test_s::<u8, i8>(5, -2, -1);
    test_s::<u8, i8>(6, -2, 0);
    test_s::<u8, i8>(7, -2, 1);
    test_s::<u8, i8>(0, -4, 0);
    test_s::<u8, i8>(1, -4, 1);
    test_s::<u8, i8>(2, -4, 0);
    test_s::<u8, i8>(3, -4, 1);
    test_s::<u8, i8>(0, -6, 0);
    test_s::<u8, i8>(1, -6, 1);
    test_s::<u8, i8>(2, -6, 0);
    test_s::<u8, i8>(3, -6, 0);
    test_s::<u8, i8>(4, -6, 0);
    test_s::<u8, i8>(5, -6, 1);
    test_s::<u8, i8>(6, -6, 0);
    test_s::<u8, i8>(7, -6, 1);
    test_s::<u8, i8>(8, -6, 0);
    test_s::<u8, i8>(9, -6, 0);
    test_s::<u8, i8>(10, -6, 0);
    test_s::<u8, i8>(11, -6, 1);
    test_s::<u8, i8>(12, -6, 0);
    test_s::<u8, i8>(13, -6, -1);
    test_s::<u8, i8>(14, -6, 0);
    test_s::<u8, i8>(15, -6, 0);
    test_s::<u8, i8>(16, -6, 0);
    test_s::<u8, i8>(17, -6, -1);
    test_s::<u8, i8>(18, -6, 0);
    test_s::<u8, i8>(19, -6, -1);
    test_s::<u8, i8>(20, -6, 0);
    test_s::<u8, i8>(21, -6, 0);
    test_s::<u8, i8>(22, -6, 0);
    test_s::<u8, i8>(23, -6, -1);

    test_s::<u8, i8>(-1, -2, -1);
    test_s::<u8, i8>(-2, -2, 0);
    test_s::<u8, i8>(-3, -2, 1);
    test_s::<u8, i8>(-4, -2, 0);
    test_s::<u8, i8>(-5, -2, 1);
    test_s::<u8, i8>(-6, -2, 0);
    test_s::<u8, i8>(-7, -2, -1);
    test_s::<u8, i8>(-1, -4, -1);
    test_s::<u8, i8>(-2, -4, 0);
    test_s::<u8, i8>(-3, -4, -1);
    test_s::<u8, i8>(-1, -6, 1);
    test_s::<u8, i8>(-2, -6, 0);
    test_s::<u8, i8>(-3, -6, 0);
    test_s::<u8, i8>(-4, -6, 0);
    test_s::<u8, i8>(-5, -6, 1);
    test_s::<u8, i8>(-6, -6, 0);
    test_s::<u8, i8>(-7, -6, 1);
    test_s::<u8, i8>(-8, -6, 0);
    test_s::<u8, i8>(-9, -6, 0);
    test_s::<u8, i8>(-10, -6, 0);
    test_s::<u8, i8>(-11, -6, 1);
    test_s::<u8, i8>(-12, -6, 0);
    test_s::<u8, i8>(-13, -6, -1);
    test_s::<u8, i8>(-14, -6, 0);
    test_s::<u8, i8>(-15, -6, 0);
    test_s::<u8, i8>(-16, -6, 0);
    test_s::<u8, i8>(-17, -6, -1);
    test_s::<u8, i8>(-18, -6, 0);
    test_s::<u8, i8>(-19, -6, -1);
    test_s::<u8, i8>(-20, -6, 0);
    test_s::<u8, i8>(-21, -6, 0);
    test_s::<u8, i8>(-22, -6, 0);
    test_s::<u8, i8>(-23, -6, -1);

    test_s::<u16, i16>(1001, -9908, -1);
    test_s::<u16, i16>(10909, -9908, -1);
    test_s::<u16, i16>(-8907, -9908, 1);
}

fn jacobi_symbol_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_40::<T>().test_properties(|(a, n)| {
        let s = a.jacobi_symbol(n);
        assert_eq!(a.legendre_symbol(n), s);
        assert_eq!(a.kronecker_symbol(n), s);
        assert_eq!(jacobi_symbol_unsigned_simple(a, n), s);
        assert_eq!(jacobi_symbol_unsigned_fast_1(a, n), s);
        assert_eq!(jacobi_symbol_unsigned_fast_2_1(a, n), s);
        assert_eq!(jacobi_symbol_unsigned_fast_2_2(a, n), s);
        assert_eq!(jacobi_symbol_unsigned_fast_2_3(a, n), s);
        assert_eq!(jacobi_symbol_unsigned_fast_2_4(a, n), s);
        assert!(s.le_abs(&1i8));

        if let Some(b) = a.checked_add(n) {
            assert_eq!(b.jacobi_symbol(n), s);
        }
        if let Some(b) = a.checked_sub(n) {
            assert_eq!(b.jacobi_symbol(n), s);
        }
        assert_eq!(s != 0, a.coprime_with(n));
        if let Some(b) = a.checked_mul(T::TWO) {
            let n_mod_8: u8 = n.mod_power_of_2(3).wrapping_into();
            assert_eq!(
                b.jacobi_symbol(n),
                if n_mod_8 == 1 || n_mod_8 == 7 { s } else { -s }
            );
        }
    });

    unsigned_pair_gen_var_41::<T>().test_properties(|(m, n)| {
        let n_mod_4: u8 = n.mod_power_of_2(2).wrapping_into();
        let m_mod_4: u8 = m.mod_power_of_2(2).wrapping_into();
        assert_eq!(
            m.jacobi_symbol(n) * n.jacobi_symbol(m),
            if n_mod_4 == 1 || m_mod_4 == 1 { 1 } else { -1 }
        );
    });

    unsigned_triple_gen_var_22::<T>().test_properties(|(a, b, n)| {
        if let Some(c) = a.checked_mul(b) {
            assert_eq!(c.jacobi_symbol(n), a.jacobi_symbol(n) * b.jacobi_symbol(n));
        }
    });

    unsigned_triple_gen_var_23::<T>().test_properties(|(a, m, n)| {
        if let Some(o) = m.checked_mul(n) {
            assert_eq!(a.jacobi_symbol(o), a.jacobi_symbol(m) * a.jacobi_symbol(n));
        }
    });

    unsigned_gen_var_22::<T>().test_properties(|n| {
        if n != T::ONE {
            assert_eq!(T::ZERO.jacobi_symbol(n), 0);
            assert_eq!(n.jacobi_symbol(n), 0);
        }
        assert_eq!(T::ONE.jacobi_symbol(n), 1);
        assert_eq!(n.jacobi_symbol(T::ONE), 1);
        let n_mod_4: u8 = n.mod_power_of_2(2).wrapping_into();
        assert_eq!(
            (n - T::ONE).jacobi_symbol(n),
            if n_mod_4 == 1 { 1 } else { -1 }
        );
        let n_mod_8: u8 = n.mod_power_of_2(3).wrapping_into();
        assert_eq!(
            T::TWO.jacobi_symbol(n),
            if n_mod_8 == 1 || n_mod_8 == 7 { 1 } else { -1 }
        );
    });
}

fn jacobi_symbol_properties_double_helper_unsigned<
    T: PrimitiveUnsigned,
    D: HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned,
>() {
    unsigned_quadruple_gen_var_12::<T>().test_properties(|(x_1, x_0, y_1, y_0)| {
        let s = jacobi_symbol_unsigned_double_simple::<T, D>(x_1, x_0, y_1, y_0);
        assert_eq!(jacobi_symbol_unsigned_double_fast_1(x_1, x_0, y_1, y_0), s);
        assert_eq!(jacobi_symbol_unsigned_double_fast_2(x_1, x_0, y_1, y_0), s);
    });
}

fn jacobi_symbol_properties_helper_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: ModPowerOf2<Output = U> + PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>() {
    signed_pair_gen_var_8::<U, S>().test_properties(|(a, n)| {
        let s = a.jacobi_symbol(n);
        assert_eq!(a.legendre_symbol(n), s);
        assert_eq!(a.kronecker_symbol(n), s);
        assert!(s.le_abs(&1i8));

        if let Some(b) = a.checked_add(n) {
            assert_eq!(b.jacobi_symbol(n), s);
        }
        if let Some(b) = a.checked_sub(n) {
            assert_eq!(b.jacobi_symbol(n), s);
        }
        assert_eq!(s != 0, a.unsigned_abs().coprime_with(n.unsigned_abs()));
        if let Some(b) = a.checked_mul(S::TWO) {
            let n_mod_8: u8 = n.mod_power_of_2(3).wrapping_into();
            assert_eq!(
                b.jacobi_symbol(n),
                if n_mod_8 == 1 || n_mod_8 == 7 { s } else { -s }
            );
        }
        if let Some(b) = a.checked_neg() {
            let n_mod_4: u8 = n.mod_power_of_2(2).wrapping_into();
            assert_eq!(b.jacobi_symbol(n), if n_mod_4 == 1 { s } else { -s });
        }
    });

    signed_pair_gen_var_9::<U, S>().test_properties(|(m, n)| {
        let n_mod_4: u8 = n.mod_power_of_2(2).wrapping_into();
        let m_mod_4: u8 = m.mod_power_of_2(2).wrapping_into();
        assert_eq!(
            m.jacobi_symbol(n) * n.jacobi_symbol(m),
            if n_mod_4 == 1 || m_mod_4 == 1 { 1 } else { -1 }
        );
    });

    signed_triple_gen_var_6::<U, S>().test_properties(|(a, b, n)| {
        if let Some(c) = a.checked_mul(b) {
            assert_eq!(c.jacobi_symbol(n), a.jacobi_symbol(n) * b.jacobi_symbol(n));
        }
    });

    signed_triple_gen_var_7::<U, S>().test_properties(|(a, m, n)| {
        if let Some(o) = m.checked_mul(n) {
            assert_eq!(a.jacobi_symbol(o), a.jacobi_symbol(m) * a.jacobi_symbol(n));
        }
    });

    signed_gen_var_13::<U, S>().test_properties(|n| {
        if n != S::ONE {
            assert_eq!(S::ZERO.jacobi_symbol(n), 0);
            assert_eq!(n.jacobi_symbol(n), 0);
        }
        assert_eq!(S::ONE.jacobi_symbol(n), 1);
        assert_eq!(n.jacobi_symbol(S::ONE), 1);
        let n_mod_4: u8 = n.mod_power_of_2(2).wrapping_into();
        assert_eq!(
            S::NEGATIVE_ONE.jacobi_symbol(n),
            if n_mod_4 == 1 { 1 } else { -1 }
        );
        let n_mod_8: u8 = n.mod_power_of_2(3).wrapping_into();
        assert_eq!(
            S::TWO.jacobi_symbol(n),
            if n_mod_8 == 1 || n_mod_8 == 7 { 1 } else { -1 }
        );
    });
}

#[test]
fn jacobi_symbol_properties() {
    apply_fn_to_unsigneds!(jacobi_symbol_properties_helper_unsigned);
    jacobi_symbol_properties_double_helper_unsigned::<u8, u16>();
    jacobi_symbol_properties_double_helper_unsigned::<u16, u32>();
    jacobi_symbol_properties_double_helper_unsigned::<u32, u64>();
    jacobi_symbol_properties_double_helper_unsigned::<u64, u128>();
    apply_fn_to_unsigned_signed_pairs!(jacobi_symbol_properties_helper_signed);
}

fn kronecker_symbol_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_27::<T>().test_properties(|(a, n)| {
        let s = a.kronecker_symbol(n);
        assert!(s.le_abs(&1i8));

        assert_eq!(s != 0, a.coprime_with(n));
        let n_mod_4: u8 = n.mod_power_of_2(2).wrapping_into();
        if n_mod_4 == 2 {
            if let Some(four_n) = n.checked_mul(T::from(4u8)) {
                if let Some(b) = a.checked_add(four_n) {
                    assert_eq!(b.kronecker_symbol(n), s);
                }
                if let Some(b) = a.checked_sub(four_n) {
                    assert_eq!(b.kronecker_symbol(n), s);
                }
            }
        } else {
            if let Some(b) = a.checked_add(n) {
                assert_eq!(b.kronecker_symbol(n), s);
            }
            if let Some(b) = a.checked_sub(n) {
                assert_eq!(b.kronecker_symbol(n), s);
            }
        }
        let a_mod_4: u8 = a.mod_power_of_2(2).wrapping_into();
        if a != T::ZERO && a_mod_4 != 3 {
            if a_mod_4 == 2 {
                if let Some(four_a) = a.checked_mul(T::from(4u8)) {
                    if let Some(m) = n.checked_add(four_a) {
                        assert_eq!(a.kronecker_symbol(m), s);
                    }
                    if let Some(m) = n.checked_sub(four_a) {
                        assert_eq!(a.kronecker_symbol(m), s);
                    }
                }
            } else {
                if let Some(m) = n.checked_add(a) {
                    assert_eq!(a.kronecker_symbol(m), s);
                }
                if let Some(m) = n.checked_sub(a) {
                    assert_eq!(a.kronecker_symbol(m), s);
                }
            }
        }
    });

    unsigned_triple_gen_var_19::<T>().test_properties(|(x, y, z)| {
        if let Some(p) = x.checked_mul(y) {
            assert_eq!(
                p.kronecker_symbol(z),
                x.kronecker_symbol(z) * y.kronecker_symbol(z)
            );
        }
        if let Some(p) = y.checked_mul(z) {
            assert_eq!(
                x.kronecker_symbol(p),
                x.kronecker_symbol(y) * x.kronecker_symbol(z)
            );
        }
    });

    unsigned_pair_gen_var_42::<T>().test_properties(|(m, n)| {
        let n_odd = if n == T::ZERO {
            T::ONE
        } else {
            n >> n.trailing_zeros()
        };
        let m_odd = if m == T::ZERO {
            T::ONE
        } else {
            m >> m.trailing_zeros()
        };
        let n_odd_mod_4: u8 = n_odd.mod_power_of_2(2).wrapping_into();
        let m_odd_mod_4: u8 = m_odd.mod_power_of_2(2).wrapping_into();
        let p = if n_odd_mod_4 == 1 || m_odd_mod_4 == 1 {
            1
        } else {
            -1
        };
        assert_eq!(m.kronecker_symbol(n) * n.kronecker_symbol(m), p);
    });

    unsigned_gen().test_properties(|n| {
        if n != T::ONE {
            assert_eq!(T::ZERO.kronecker_symbol(n), 0);
            assert_eq!(n.kronecker_symbol(n), 0);
        }
        assert_eq!(T::ONE.kronecker_symbol(n), 1);
        assert_eq!(n.kronecker_symbol(T::ONE), 1);
    });
}

fn kronecker_symbol_properties_helper_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: ModPowerOf2<Output = U> + PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>() {
    signed_pair_gen::<S>().test_properties(|(a, n)| {
        let s = a.kronecker_symbol(n);
        assert!(s.le_abs(&1i8));

        assert_eq!(s != 0, a.unsigned_abs().coprime_with(n.unsigned_abs()));
        let n_mod_4: u8 = n.mod_power_of_2(2).wrapping_into();
        if n_mod_4 == 2 {
            if let Some(four_n) = n.checked_mul(S::from(4i8)) {
                if let Some(b) = a.checked_add(four_n) {
                    if n > S::ZERO || a.sign() == b.sign() {
                        assert_eq!(b.kronecker_symbol(n), s);
                    }
                }
                if let Some(b) = a.checked_sub(four_n) {
                    if n > S::ZERO || a.sign() == b.sign() {
                        assert_eq!(b.kronecker_symbol(n), s);
                    }
                }
            }
        } else {
            if let Some(b) = a.checked_add(n) {
                if n > S::ZERO || a.sign() == b.sign() {
                    assert_eq!(b.kronecker_symbol(n), s);
                }
            }
            if let Some(b) = a.checked_sub(n) {
                if n > S::ZERO || a.sign() == b.sign() {
                    assert_eq!(b.kronecker_symbol(n), s);
                }
            }
        }
        let a_mod_4: u8 = a.mod_power_of_2(2).wrapping_into();
        if a != S::ZERO && a_mod_4 != 3 {
            if let Some(abs_a) = a.checked_abs() {
                if a_mod_4 == 2 {
                    if let Some(four_abs_a) = abs_a.checked_mul(S::from(4i8)) {
                        if let Some(m) = n.checked_add(four_abs_a) {
                            assert_eq!(a.kronecker_symbol(m), s);
                        }
                        if let Some(m) = n.checked_sub(four_abs_a) {
                            assert_eq!(a.kronecker_symbol(m), s);
                        }
                    }
                } else {
                    if let Some(m) = n.checked_add(abs_a) {
                        assert_eq!(a.kronecker_symbol(m), s);
                    }
                    if let Some(m) = n.checked_sub(abs_a) {
                        assert_eq!(a.kronecker_symbol(m), s);
                    }
                }
            }
        }

        let m = a;
        if let Some(m_abs) = m.checked_abs() {
            let m_odd = if m == S::ZERO {
                S::ONE
            } else {
                m >> m.trailing_zeros()
            };
            let m_odd_mod_4: u8 = m_odd.mod_power_of_2(2).wrapping_into();
            // -m won't overflow since m.checked_abs() is Some
            let m_star = if m_odd_mod_4 == 1 { m } else { -m };
            assert_eq!(m_star.kronecker_symbol(n), n.kronecker_symbol(m_abs));
        }
    });

    signed_triple_gen::<S>().test_properties(|(x, y, z)| {
        if !(z == S::NEGATIVE_ONE && (x == S::ZERO && y < S::ZERO || x < S::ZERO && y == S::ZERO)) {
            if let Some(p) = x.checked_mul(y) {
                assert_eq!(
                    p.kronecker_symbol(z),
                    x.kronecker_symbol(z) * y.kronecker_symbol(z)
                );
            }
        }
        let y_odd_mod_4: u8 = if y == S::ZERO {
            0
        } else {
            (y >> y.trailing_zeros()).mod_power_of_2(2).wrapping_into()
        };
        let z_odd_mod_4: u8 = if z == S::ZERO {
            0
        } else {
            (z >> z.trailing_zeros()).mod_power_of_2(2).wrapping_into()
        };
        if !(x == S::NEGATIVE_ONE
            && (y == S::ZERO && z_odd_mod_4 == 3 || y_odd_mod_4 == 3 && z == S::ZERO))
        {
            if let Some(p) = y.checked_mul(z) {
                assert_eq!(
                    x.kronecker_symbol(p),
                    x.kronecker_symbol(y) * x.kronecker_symbol(z)
                );
            }
        }
    });

    signed_pair_gen_var_10::<U, S>().test_properties(|(m, n)| {
        let n_odd = if n == S::ZERO {
            S::ONE
        } else {
            n >> n.trailing_zeros()
        };
        let m_odd = if m == S::ZERO {
            S::ONE
        } else {
            m >> m.trailing_zeros()
        };
        let n_odd_mod_4: u8 = n_odd.mod_power_of_2(2).wrapping_into();
        let m_odd_mod_4: u8 = m_odd.mod_power_of_2(2).wrapping_into();
        let p = if n_odd_mod_4 == 1 || m_odd_mod_4 == 1 {
            1
        } else {
            -1
        };
        assert_eq!(
            m.kronecker_symbol(n) * n.kronecker_symbol(m),
            if m < S::ZERO && n < S::ZERO { -p } else { p }
        );
        if let Some(m_abs) = m.checked_abs() {
            assert_eq!(m.kronecker_symbol(n) * n.kronecker_symbol(m_abs), p);
        }
    });

    signed_gen().test_properties(|n| {
        if n != S::ONE && n != S::NEGATIVE_ONE {
            assert_eq!(S::ZERO.kronecker_symbol(n), 0);
            assert_eq!(n.kronecker_symbol(n), 0);
        }
        assert_eq!(S::ONE.kronecker_symbol(n), 1);
        assert_eq!(n.kronecker_symbol(S::ONE), 1);
        let n_odd = if n == S::ZERO {
            S::ONE
        } else {
            n >> n.trailing_zeros()
        };
        let n_odd_mod_4: u8 = n_odd.mod_power_of_2(2).wrapping_into();
        assert_eq!(
            S::NEGATIVE_ONE.kronecker_symbol(n),
            if n_odd_mod_4 == 1 { 1 } else { -1 }
        );
        assert_eq!(
            n.kronecker_symbol(S::NEGATIVE_ONE),
            if n >= S::ZERO { 1 } else { -1 }
        );
    });
}

#[test]
fn kronecker_symbol_properties() {
    apply_fn_to_unsigneds!(kronecker_symbol_properties_helper_unsigned);
    apply_fn_to_unsigned_signed_pairs!(kronecker_symbol_properties_helper_signed);
}
