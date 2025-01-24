// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;

fn checked_abs_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.checked_abs(), out);
    };
    test(T::ZERO, Some(T::ZERO));
    test(T::ONE, Some(T::ONE));
    test(T::exact_from(100), Some(T::exact_from(100)));
    test(T::NEGATIVE_ONE, Some(T::ONE));
    test(T::exact_from(-100), Some(T::exact_from(100)));
    test(T::MIN, None);
}

#[test]
fn test_checked_abs() {
    apply_fn_to_signeds!(checked_abs_helper);
}
