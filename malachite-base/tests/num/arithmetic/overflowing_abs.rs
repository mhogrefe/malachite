// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::test_util::generators::signed_gen;

fn overflowing_abs_helper<T: PrimitiveSigned>() {
    let test = |n: T, out, overflow| {
        assert_eq!(n.overflowing_abs(), (out, overflow));

        let mut n = n;
        assert_eq!(n.overflowing_abs_assign(), overflow);
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO, false);
    test(T::ONE, T::ONE, false);
    test(T::exact_from(100), T::exact_from(100), false);
    test(T::MAX, T::MAX, false);
    test(T::NEGATIVE_ONE, T::ONE, false);
    test(T::exact_from(-100), T::exact_from(100), false);
    test(T::MIN, T::MIN, true);
}

#[test]
fn test_overflowing_abs() {
    apply_fn_to_signeds!(overflowing_abs_helper);
}

fn overflowing_abs_properties_helper<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|n| {
        let mut abs = n;
        let overflow = abs.overflowing_abs_assign();
        assert_eq!((abs, overflow), n.overflowing_abs());
        assert_eq!(abs, n.wrapping_abs());
        if n != T::MIN {
            assert_eq!(n.abs(), abs);
        }
        assert_eq!(abs == n, n >= T::ZERO || n == T::MIN);
        assert_eq!(n == T::MIN, overflow);
    });
}

#[test]
fn overflowing_abs_properties() {
    apply_fn_to_signeds!(overflowing_abs_properties_helper);
}
