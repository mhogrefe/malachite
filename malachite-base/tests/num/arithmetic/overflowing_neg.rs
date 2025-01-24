// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};

fn unsigned_overflowing_neg_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, out, overflow| {
        assert_eq!(n.overflowing_neg(), (out, overflow));

        let mut n = n;
        assert_eq!(n.overflowing_neg_assign(), overflow);
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO, false);
    test(T::ONE, T::MAX, true);
    test(
        T::exact_from(100),
        T::MAX - T::exact_from(100) + T::ONE,
        true,
    );
    test(T::MAX, T::ONE, true);
}

fn signed_overflowing_neg_helper<T: PrimitiveSigned>() {
    let test = |n: T, out, overflow| {
        assert_eq!(n.overflowing_neg(), (out, overflow));

        let mut n = n;
        assert_eq!(n.overflowing_neg_assign(), overflow);
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO, false);
    test(T::ONE, T::NEGATIVE_ONE, false);
    test(T::exact_from(100), T::exact_from(-100), false);
    test(T::MAX, T::MIN + T::ONE, false);
    test(T::NEGATIVE_ONE, T::ONE, false);
    test(T::exact_from(-100), T::exact_from(100), false);
    test(T::MIN, T::MIN, true);
}

#[test]
fn test_overflowing_neg() {
    apply_fn_to_unsigneds!(unsigned_overflowing_neg_helper);
    apply_fn_to_signeds!(signed_overflowing_neg_helper);
}

fn overflowing_neg_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|n| {
        let mut neg = n;
        let overflow = neg.overflowing_neg_assign();
        assert_eq!((neg, overflow), n.overflowing_neg());
        assert_eq!(neg, n.wrapping_neg());
        assert_eq!(neg == n, n == T::ZERO || n == T::power_of_2(T::WIDTH - 1));
        assert_eq!(n != T::ZERO, overflow);
    });
}

fn overflowing_neg_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|n| {
        let mut neg = n;
        let overflow = neg.overflowing_neg_assign();
        assert_eq!((neg, overflow), n.overflowing_neg());
        assert_eq!(neg, n.wrapping_neg());
        assert_eq!(neg == n, n == T::ZERO || n == T::MIN);
        assert_eq!(n == T::MIN, overflow);
    });
}

#[test]
fn overflowing_neg_properties() {
    apply_fn_to_unsigneds!(overflowing_neg_properties_helper_unsigned);
    apply_fn_to_signeds!(overflowing_neg_properties_helper_signed);
}
