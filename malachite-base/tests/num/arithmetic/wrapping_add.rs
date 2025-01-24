// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{signed_pair_gen, unsigned_pair_gen_var_27};

#[test]
fn test_wrapping_add() {
    fn test<T: PrimitiveInt>(x: T, y: T, out: T) {
        assert_eq!(x.wrapping_add(y), out);

        let mut x = x;
        x.wrapping_add_assign(y);
        assert_eq!(x, out);
    }
    test::<u16>(123, 456, 579);
    test::<u8>(123, 200, 67);
    test::<i16>(123, -456, -333);
    test::<i8>(123, 45, -88);
    test::<i8>(-123, -45, 88);
}

fn wrapping_add_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        let mut sum = x;
        sum.wrapping_add_assign(y);
        assert_eq!(sum, x.wrapping_add(y));
        assert_eq!(sum, x.wrapping_sub(y.wrapping_neg()));
        assert_eq!(sum.wrapping_sub(x), y);
        assert_eq!(sum.wrapping_sub(y), x);
        assert_eq!(y.wrapping_add(x), sum);
    });
}

fn wrapping_add_properties_helper_signed<T: PrimitiveSigned>() {
    signed_pair_gen::<T>().test_properties(|(x, y)| {
        let mut sum = x;
        sum.wrapping_add_assign(y);
        assert_eq!(sum, x.wrapping_add(y));
        assert_eq!(sum, x.wrapping_sub(y.wrapping_neg()));
        assert_eq!(sum.wrapping_sub(x), y);
        assert_eq!(sum.wrapping_sub(y), x);
        assert_eq!(y.wrapping_add(x), sum);
    });
}

#[test]
fn wrapping_add_properties() {
    apply_fn_to_unsigneds!(wrapping_add_properties_helper_unsigned);
    apply_fn_to_signeds!(wrapping_add_properties_helper_signed);
}
