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
use malachite_base::test_util::generators::{signed_pair_gen, unsigned_pair_gen_var_27};

#[test]
fn test_overflowing_sub() {
    fn test<T: PrimitiveInt>(x: T, y: T, out: T, overflow: bool) {
        assert_eq!(x.overflowing_sub(y), (out, overflow));

        let mut x = x;
        assert_eq!(x.overflowing_sub_assign(y), overflow);
        assert_eq!(x, out);
    }
    test::<u16>(456, 123, 333, false);
    test::<u8>(123, 200, 179, true);
    test::<i16>(123, -456, 579, false);
    test::<i8>(123, -45, -88, true);
    test::<i8>(-123, 45, 88, true);
}

fn overflowing_sub_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        let mut diff = x;
        let overflow = diff.overflowing_sub_assign(y);
        assert_eq!((diff, overflow), x.overflowing_sub(y));
        assert_eq!(x.wrapping_sub(y), diff);
        if !overflow {
            assert_eq!(diff, x - y);
        }
    });
}

fn overflowing_sub_properties_helper_signed<T: PrimitiveSigned>() {
    signed_pair_gen::<T>().test_properties(|(x, y)| {
        let mut diff = x;
        let overflow = diff.overflowing_sub_assign(y);
        assert_eq!((diff, overflow), x.overflowing_sub(y));
        assert_eq!(x.wrapping_sub(y), diff);
        if !overflow {
            assert_eq!(diff, x - y);
        }
    });
}

#[test]
fn overflowing_sub_properties() {
    apply_fn_to_unsigneds!(overflowing_sub_properties_helper_unsigned);
    apply_fn_to_signeds!(overflowing_sub_properties_helper_signed);
}
