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
fn test_overflowing_mul() {
    fn test<T: PrimitiveInt>(x: T, y: T, out: T, overflow: bool) {
        assert_eq!(x.overflowing_mul(y), (out, overflow));

        let mut x = x;
        assert_eq!(x.overflowing_mul_assign(y), overflow);
        assert_eq!(x, out);
    }
    test::<u16>(123, 456, 56088, false);
    test::<u8>(123, 200, 24, true);
    test::<i16>(123, -45, -5535, false);
    test::<i8>(123, 45, -97, true);
    test::<i8>(-123, 45, 97, true);
}

fn overflowing_mul_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        let mut product = x;
        let overflow = product.overflowing_mul_assign(y);
        assert_eq!((product, overflow), x.overflowing_mul(y));
        assert_eq!(x.wrapping_mul(y), product);
        if !overflow {
            assert_eq!(product, x * y);
        }
    });
}

fn overflowing_mul_properties_helper_signed<T: PrimitiveSigned>() {
    signed_pair_gen::<T>().test_properties(|(x, y)| {
        let mut product = x;
        let overflow = product.overflowing_mul_assign(y);
        assert_eq!((product, overflow), x.overflowing_mul(y));
        assert_eq!(x.wrapping_mul(y), product);
        if !overflow {
            assert_eq!(product, x * y);
        }
    });
}

#[test]
fn overflowing_mul_properties() {
    apply_fn_to_unsigneds!(overflowing_mul_properties_helper_unsigned);
    apply_fn_to_signeds!(overflowing_mul_properties_helper_signed);
}
