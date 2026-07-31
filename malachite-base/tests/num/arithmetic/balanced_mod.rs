// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    BalancedMod, BalancedModAssign, Parity, UnsignedAbs,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{signed_pair_gen_var_52, unsigned_pair_gen_var_52};
use std::panic::catch_unwind;

#[test]
fn test_balanced_mod_unsigned() {
    assert_eq!(0u32.balanced_mod(10), 0);
    assert_eq!(3u32.balanced_mod(10), 3);
    // - exactly half the modulus is the top of the range, so it stays positive
    assert_eq!(5u32.balanced_mod(10), 5);
    assert_eq!(6u32.balanced_mod(10), -4);
    assert_eq!(23u32.balanced_mod(10), 3);
    assert_eq!(27u32.balanced_mod(10), -3);
    // - an odd modulus has no tie
    assert_eq!(4u32.balanced_mod(9), 4);
    assert_eq!(5u32.balanced_mod(9), -4);
    assert_eq!(7u8.balanced_mod(1), 0);
    // - the widest modulus: the result still fits in the signed type of the same width
    assert_eq!(u8::MAX.balanced_mod(u8::MAX), 0);
    assert_eq!((u8::MAX - 1).balanced_mod(u8::MAX), -1);
    assert_eq!(127u8.balanced_mod(u8::MAX), 127);
    assert_eq!(128u8.balanced_mod(u8::MAX), -127);
    assert_eq!(u64::MAX.balanced_mod(u64::MAX), 0);
    assert_eq!((u64::MAX >> 1).balanced_mod(u64::MAX), i64::MAX);
}

#[test]
fn test_balanced_mod_signed() {
    assert_eq!(23i32.balanced_mod(10), 3);
    assert_eq!(27i32.balanced_mod(10), -3);
    assert_eq!(25i32.balanced_mod(10), 5);
    // - a negative value is reduced into the same range
    assert_eq!((-23i32).balanced_mod(10), -3);
    assert_eq!((-27i32).balanced_mod(10), 3);
    assert_eq!((-25i32).balanced_mod(10), 5);
    // - only the magnitude of the modulus matters
    assert_eq!(23i32.balanced_mod(-10), 3);
    assert_eq!((-27i32).balanced_mod(-10), 3);
    assert_eq!(0i32.balanced_mod(10), 0);
    // - the most negative modulus, whose magnitude is not representable
    assert_eq!(0i8.balanced_mod(i8::MIN), 0);
    assert_eq!(64i8.balanced_mod(i8::MIN), 64);
    assert_eq!(65i8.balanced_mod(i8::MIN), -63);
    assert_eq!((-64i8).balanced_mod(i8::MIN), 64);
    assert_eq!(i8::MIN.balanced_mod(i8::MIN), 0);
    assert_eq!(i8::MAX.balanced_mod(i8::MIN), -1);
    // - the in-place form
    let mut x = 27i32;
    x.balanced_mod_assign(10);
    assert_eq!(x, -3);
}

fn balanced_mod_fail_helper_unsigned<T: PrimitiveUnsigned>() {
    assert_panic!(T::exact_from(10).balanced_mod(T::ZERO));
}

fn balanced_mod_fail_helper_signed<T: PrimitiveSigned>() {
    assert_panic!(T::exact_from(10).balanced_mod(T::ZERO));
    assert_panic!(T::exact_from(10).balanced_mod_assign(T::ZERO));
}

#[test]
fn balanced_mod_fail() {
    apply_fn_to_unsigneds!(balanced_mod_fail_helper_unsigned);
    apply_fn_to_signeds!(balanced_mod_fail_helper_signed);
}

// The congruence and the range determine the balanced remainder uniquely, so asserting both is a
// complete specification; no external oracle is needed.
fn balanced_mod_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_52::<T>().test_properties(|(x, y)| {
        let r = x.balanced_mod(y);
        type S<T> = <T as BalancedMod<T>>::Output;
        // Congruence, stated against the ordinary remainder rather than by subtracting: wrapping
        // the difference into the word size would destroy divisibility by `y`.
        let m = x % y;
        let abs_r = r.unsigned_abs();
        if r >= S::<T>::ZERO {
            assert_eq!(abs_r, m);
        } else {
            assert_eq!(y - abs_r, m);
        }
        let half = y >> 1;
        assert!(abs_r <= half);
        // the endpoint at exactly half the modulus belongs to the positive side
        if abs_r == half && y.even() && r != S::<T>::ZERO {
            assert!(r > S::<T>::ZERO);
        }
    });
}

fn balanced_mod_properties_helper_signed<T: PrimitiveSigned>()
where
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    signed_pair_gen_var_52::<T>().test_properties(|(x, y)| {
        let r = x.balanced_mod(y);
        let mut mut_x = x;
        mut_x.balanced_mod_assign(y);
        assert_eq!(mut_x, r);

        // Congruence, stated against the Euclidean remainder, which is an independent code path;
        // the comparison happens in the unsigned domain, where the most negative divisor's
        // magnitude is representable.
        let abs_y = y.unsigned_abs();
        let m = x.mod_euclidean(y).unsigned_abs();
        let abs_r = r.unsigned_abs();
        if r >= T::ZERO {
            assert_eq!(abs_r, m);
        } else {
            assert_eq!(abs_y - abs_r, m);
        }
        let half = abs_y >> 1;
        assert!(abs_r <= half);
        if abs_r == half && abs_y.even() && r != T::ZERO {
            assert!(r > T::ZERO);
        }
        // only the magnitude of the modulus matters
        if y != T::MIN {
            assert_eq!(x.balanced_mod(-y), r);
        }
    });
}

#[test]
fn balanced_mod_properties() {
    apply_fn_to_unsigneds!(balanced_mod_properties_helper_unsigned);
    apply_fn_to_signeds!(balanced_mod_properties_helper_signed);
}
