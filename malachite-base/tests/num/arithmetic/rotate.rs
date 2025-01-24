// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::NegMod;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    signed_gen, signed_unsigned_pair_gen, unsigned_gen, unsigned_pair_gen,
};

#[test]
fn test_rotate_left() {
    fn test<T: PrimitiveInt>(x: T, n: u64, out: T) {
        assert_eq!(x.rotate_left(n), out);

        let mut x = x;
        x.rotate_left_assign(n);
        assert_eq!(x, out);
    }
    test::<u8>(0, 0, 0);
    test::<u8>(100, 0, 100);
    test::<u16>(0xabcd, 0, 0xabcd);
    test::<u16>(0xabcd, 4, 0xbcda);
    test::<u16>(0xabcd, 8, 0xcdab);
    test::<u16>(0xabcd, 12, 0xdabc);
    test::<u16>(0xabcd, 16, 0xabcd);
    test::<u16>(0xabcd, 160, 0xabcd);
    test::<i8>(10, 0, 10);
    test::<i8>(10, 1, 20);
    test::<i8>(10, 2, 40);
    test::<i8>(10, 3, 80);
    test::<i8>(10, 4, -96);
    test::<i8>(10, 5, 65);
    test::<i8>(10, 6, -126);
    test::<i8>(10, 7, 5);
    test::<i8>(10, 8, 10);
}

#[test]
fn test_rotate_right() {
    fn test<T: PrimitiveInt>(x: T, n: u64, out: T) {
        assert_eq!(x.rotate_right(n), out);

        let mut x = x;
        x.rotate_right_assign(n);
        assert_eq!(x, out);
    }
    test::<u8>(0, 0, 0);
    test::<u8>(100, 0, 100);
    test::<u16>(0xabcd, 0, 0xabcd);
    test::<u16>(0xabcd, 4, 0xdabc);
    test::<u16>(0xabcd, 8, 0xcdab);
    test::<u16>(0xabcd, 12, 0xbcda);
    test::<u16>(0xabcd, 16, 0xabcd);
    test::<u16>(0xabcd, 160, 0xabcd);
    test::<i8>(10, 0, 10);
    test::<i8>(10, 1, 5);
    test::<i8>(10, 2, -126);
    test::<i8>(10, 3, 65);
    test::<i8>(10, 4, -96);
    test::<i8>(10, 5, 80);
    test::<i8>(10, 6, 40);
    test::<i8>(10, 7, 20);
    test::<i8>(10, 8, 10);
}

fn rotate_left_properties_unsigned_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen::<T, u64>().test_properties(|(x, n)| {
        let rotated = x.rotate_left(n);
        let mut mut_x = x;
        mut_x.rotate_left_assign(n);
        assert_eq!(mut_x, rotated);
        if let Some(m) = n.checked_add(T::WIDTH) {
            assert_eq!(x.rotate_left(m), rotated);
        }
        assert_eq!(x.rotate_left(n & T::WIDTH_MASK), rotated);
        assert_eq!(x.rotate_right(n.neg_mod(T::WIDTH)), rotated);
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.rotate_left(0), x);
        assert_eq!(x.rotate_left(T::WIDTH), x);
    });

    unsigned_gen::<u64>().test_properties(|n| {
        assert_eq!(T::ZERO.rotate_left(n), T::ZERO);
        assert_eq!(T::MAX.rotate_left(n), T::MAX);
    });
}

fn rotate_left_properties_signed_helper<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen::<T, u64>().test_properties(|(x, n)| {
        let rotated = x.rotate_left(n);
        let mut mut_x = x;
        mut_x.rotate_left_assign(n);
        assert_eq!(mut_x, rotated);
        if let Some(m) = n.checked_add(T::WIDTH) {
            assert_eq!(x.rotate_left(m), rotated);
        }
        assert_eq!(x.rotate_left(n & T::WIDTH_MASK), rotated);
        assert_eq!(x.rotate_right(n.neg_mod(T::WIDTH)), rotated);
    });

    signed_gen::<T>().test_properties(|x| {
        assert_eq!(x.rotate_left(0), x);
        assert_eq!(x.rotate_left(T::WIDTH), x);
    });

    unsigned_gen::<u64>().test_properties(|n| {
        assert_eq!(T::ZERO.rotate_left(n), T::ZERO);
        assert_eq!(T::NEGATIVE_ONE.rotate_left(n), T::NEGATIVE_ONE);
    });
}

#[test]
fn rotate_left_properties() {
    apply_fn_to_unsigneds!(rotate_left_properties_unsigned_helper);
    apply_fn_to_signeds!(rotate_left_properties_signed_helper);
}

fn rotate_right_properties_unsigned_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen::<T, u64>().test_properties(|(x, n)| {
        let rotated = x.rotate_right(n);
        let mut mut_x = x;
        mut_x.rotate_right_assign(n);
        assert_eq!(mut_x, rotated);
        if let Some(m) = n.checked_add(T::WIDTH) {
            assert_eq!(x.rotate_right(m), rotated);
        }
        assert_eq!(x.rotate_right(n & T::WIDTH_MASK), rotated);
        assert_eq!(x.rotate_left(n.neg_mod(T::WIDTH)), rotated);
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.rotate_right(0), x);
        assert_eq!(x.rotate_right(T::WIDTH), x);
    });

    unsigned_gen::<u64>().test_properties(|n| {
        assert_eq!(T::ZERO.rotate_right(n), T::ZERO);
        assert_eq!(T::MAX.rotate_right(n), T::MAX);
    });
}

fn rotate_right_properties_signed_helper<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen::<T, u64>().test_properties(|(x, n)| {
        let rotated = x.rotate_right(n);
        let mut mut_x = x;
        mut_x.rotate_right_assign(n);
        assert_eq!(mut_x, rotated);
        if let Some(m) = n.checked_add(T::WIDTH) {
            assert_eq!(x.rotate_right(m), rotated);
        }
        assert_eq!(x.rotate_right(n & T::WIDTH_MASK), rotated);
        assert_eq!(x.rotate_left(n.neg_mod(T::WIDTH)), rotated);
    });

    signed_gen::<T>().test_properties(|x| {
        assert_eq!(x.rotate_right(0), x);
        assert_eq!(x.rotate_right(T::WIDTH), x);
    });

    unsigned_gen::<u64>().test_properties(|n| {
        assert_eq!(T::ZERO.rotate_right(n), T::ZERO);
        assert_eq!(T::NEGATIVE_ONE.rotate_right(n), T::NEGATIVE_ONE);
    });
}

#[test]
fn rotate_right_properties() {
    apply_fn_to_unsigneds!(rotate_right_properties_unsigned_helper);
    apply_fn_to_signeds!(rotate_right_properties_signed_helper);
}
