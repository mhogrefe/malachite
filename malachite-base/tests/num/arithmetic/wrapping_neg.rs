// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};

fn unsigned_wrapping_neg_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, out| {
        assert_eq!(n.wrapping_neg(), out);

        let mut n = n;
        n.wrapping_neg_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO);
    test(T::ONE, T::MAX);
    test(T::exact_from(100), T::MAX - T::exact_from(100) + T::ONE);
    test(T::MAX, T::ONE);
}

fn signed_wrapping_neg_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.wrapping_neg(), out);

        let mut n = n;
        n.wrapping_neg_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO);
    test(T::ONE, T::NEGATIVE_ONE);
    test(T::exact_from(100), T::exact_from(-100));
    test(T::MAX, T::MIN + T::ONE);
    test(T::NEGATIVE_ONE, T::ONE);
    test(T::exact_from(-100), T::exact_from(100));
    test(T::MIN, T::MIN);
}

#[test]
fn test_wrapping_neg() {
    apply_fn_to_unsigneds!(unsigned_wrapping_neg_helper);
    apply_fn_to_signeds!(signed_wrapping_neg_helper);
}

fn wrapping_neg_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|n| {
        let mut neg = n;
        neg.wrapping_neg_assign();
        assert_eq!(neg, n.wrapping_neg());
        assert_eq!(neg.wrapping_neg(), n);
        assert_eq!(neg == n, n == T::ZERO || n == T::power_of_2(T::WIDTH - 1));
        assert_eq!(n.wrapping_add(neg), T::ZERO);
    });
}

fn wrapping_neg_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|n| {
        let mut neg = n;
        neg.wrapping_neg_assign();
        assert_eq!(neg, n.wrapping_neg());
        assert_eq!(neg.wrapping_neg(), n);
        assert_eq!(neg == n, n == T::ZERO || n == T::MIN);
        assert_eq!(n.wrapping_add(neg), T::ZERO);
    });
}

#[test]
fn wrapping_neg_properties() {
    apply_fn_to_unsigneds!(wrapping_neg_properties_helper_unsigned);
    apply_fn_to_signeds!(wrapping_neg_properties_helper_signed);
}
