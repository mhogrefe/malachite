// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::test_util::generators::signed_gen;

fn saturating_neg_assign_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.saturating_neg(), out);

        let mut n = n;
        n.saturating_neg_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO);
    test(T::ONE, T::NEGATIVE_ONE);
    test(T::exact_from(100), T::exact_from(-100));
    test(T::MAX, T::MIN + T::ONE);
    test(T::NEGATIVE_ONE, T::ONE);
    test(T::exact_from(-100), T::exact_from(100));
    test(T::MIN, T::MAX);
}

#[test]
fn test_saturating_neg_assign() {
    apply_fn_to_signeds!(saturating_neg_assign_helper);
}

fn saturating_neg_properties_helper<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|n| {
        let mut neg = n;
        neg.saturating_neg_assign();
        assert_eq!(neg, n.saturating_neg());
        if n != T::MIN {
            assert_eq!(neg.saturating_neg(), n);
        }
        assert_eq!(neg == n, n == T::ZERO);
    });
}

#[test]
fn saturating_neg_properties() {
    apply_fn_to_signeds!(saturating_neg_properties_helper);
}
