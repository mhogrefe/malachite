// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};

fn not_assign_helper<T: PrimitiveInt>() {
    let test = |n: T| {
        let mut x = n;
        x.not_assign();
        assert_eq!(x, !n);
    };
    test(T::ZERO);
    test(T::ONE);
    test(T::exact_from(2));
    test(T::exact_from(3));
    test(T::exact_from(4));
    test(T::exact_from(5));
    test(T::exact_from(100));
    test(T::exact_from(63));
    test(T::exact_from(64));
    test(T::MIN);
    test(T::MAX);
}

#[test]
fn test_not_assign() {
    apply_fn_to_primitive_ints!(not_assign_helper);
}

fn not_assign_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|u| {
        let mut x = u;
        x.not_assign();
        assert_eq!(x, !u);
        x.not_assign();
        assert_eq!(x, u);
    });
}

fn not_assign_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|i| {
        let mut x = i;
        x.not_assign();
        assert_eq!(x, !i);
        x.not_assign();
        assert_eq!(x, i);
    });
}

#[test]
fn not_assign_properties() {
    apply_fn_to_unsigneds!(not_assign_properties_helper_unsigned);
    apply_fn_to_signeds!(not_assign_properties_helper_signed);
}
