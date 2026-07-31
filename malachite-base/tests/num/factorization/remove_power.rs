// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Parity, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::factorization::traits::RemovePower;
use malachite_base::test_util::generators::{signed_pair_gen_var_52, unsigned_pair_gen_var_52};
use std::panic::catch_unwind;

#[test]
fn test_remove_power_unsigned() {
    fn test<T: PrimitiveUnsigned>(x: T, y: T, q: T, k: u64) {
        assert_eq!(x.remove_power(y), (q, k));
        let mut mut_x = x;
        assert_eq!(mut_x.remove_power_assign(y), k);
        assert_eq!(mut_x, q);
    }
    // - x == T::ZERO: zero is left alone
    test::<u8>(0, 3, 0, 0);
    // - r != T::ZERO on the first division
    test::<u8>(7, 3, 7, 0);
    test::<u16>(1, 3, 1, 0);
    // - y == T::TWO: counted with trailing_zeros rather than by dividing
    test::<u32>(12, 2, 3, 2);
    // - the factor divides repeatedly
    test::<u32>(1215, 3, 5, 5);
    // - the factor need not be prime
    test::<u64>(1000, 10, 1, 3);
    test::<u64>(96, 6, 16, 1);
    // - the factor equals the value
    test::<usize>(3, 3, 1, 1);
    // - the widest values
    test::<u128>(u128::MAX, 3, 113427455640312821154458202477256070485, 1);
    // - a factor of 2 that divides every bit but the last
    test::<u8>(128, 2, 1, 7);
    // - a factor of 2 that does not divide at all
    test::<u8>(129, 2, 129, 0);
}

#[test]
fn test_remove_power_signed() {
    fn test<T: PrimitiveSigned>(x: T, y: T, q: T, k: u64) {
        assert_eq!(x.remove_power(y), (q, k));
        let mut mut_x = x;
        assert_eq!(mut_x.remove_power_assign(y), k);
        assert_eq!(mut_x, q);
    }
    test::<i8>(0, 3, 0, 0);
    test::<i8>(7, 3, 7, 0);
    // - x < T::ZERO with a positive factor
    test::<i32>(-12, 2, -3, 2);
    // - y < T::ZERO with an even power, which leaves the sign alone
    test::<i32>(12, -2, 3, 2);
    test::<i32>(-12, -2, -3, 2);
    // - y < T::ZERO with an odd power, which flips the sign
    test::<i64>(-8, 2, -1, 3);
    test::<i64>(-8, -2, 1, 3);
    test::<i64>(8, -2, -1, 3);
    // - the most negative value, whose magnitude is not representable as a positive
    test::<i8>(i8::MIN, 2, -1, 7);
    test::<i8>(i8::MIN, -2, 1, 7);
    // - the most negative value with a factor that does not divide it, so k is 0
    test::<i8>(i8::MIN, 3, i8::MIN, 0);
}

fn remove_power_fail_helper_unsigned<T: PrimitiveUnsigned>() {
    assert_panic!(T::exact_from(12).remove_power(T::ZERO));
    assert_panic!(T::exact_from(12).remove_power(T::ONE));
    assert_panic!(T::exact_from(12).remove_power_assign(T::ONE));
}

fn remove_power_fail_helper_signed<T: PrimitiveSigned>() {
    assert_panic!(T::exact_from(12).remove_power(T::ZERO));
    assert_panic!(T::exact_from(12).remove_power(T::ONE));
    assert_panic!(T::exact_from(12).remove_power(T::NEGATIVE_ONE));
}

#[test]
fn remove_power_fail() {
    apply_fn_to_unsigneds!(remove_power_fail_helper_unsigned);
    apply_fn_to_signeds!(remove_power_fail_helper_signed);
}

fn remove_power_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_52::<T>().test_properties(|(x, y)| {
        let (q, k) = x.remove_power(y);
        let mut mut_x = x;
        assert_eq!(mut_x.remove_power_assign(y), k);
        assert_eq!(mut_x, q);

        // the defining identity, and nothing is left to remove
        assert_eq!(q * y.pow(k), x);
        if x != T::ZERO {
            assert!(!q.divisible_by(y));
        } else {
            assert_eq!(k, 0);
        }
        assert!(q <= x);
    });
}

fn remove_power_properties_helper_signed<T: PrimitiveSigned>()
where
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    signed_pair_gen_var_52::<T>().test_properties(|(x, y)| {
        let (q, k) = x.remove_power(y);
        let mut mut_x = x;
        assert_eq!(mut_x.remove_power_assign(y), k);
        assert_eq!(mut_x, q);

        // the exponent depends only on the magnitudes
        assert_eq!(x.unsigned_abs().remove_power(y.unsigned_abs()).1, k);
        if k != 0 {
            // the quotient's sign follows the signed power
            assert_eq!(q < T::ZERO, (x < T::ZERO) != (y < T::ZERO && k.odd()));
        }
    });
}

#[test]
fn remove_power_properties() {
    apply_fn_to_unsigneds!(remove_power_properties_helper_unsigned);
    apply_fn_to_signeds!(remove_power_properties_helper_signed);
}
