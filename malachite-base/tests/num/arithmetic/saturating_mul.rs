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
fn test_saturating_mul() {
    fn test<T: PrimitiveInt>(x: T, y: T, out: T) {
        assert_eq!(x.saturating_mul(y), out);

        let mut x = x;
        x.saturating_mul_assign(y);
        assert_eq!(x, out);
    }
    test::<u16>(123, 456, 56088);
    test::<u8>(123, 200, 255);
    test::<i16>(123, -45, -5535);
    test::<i8>(123, 45, 127);
    test::<i8>(-123, 45, -128);
}

fn saturating_mul_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        let mut product = x;
        product.saturating_mul_assign(y);
        assert_eq!(product, x.saturating_mul(y));
        assert_eq!(y.saturating_mul(x), product);
        assert!(product == T::ZERO || product >= x);
        assert!(product == T::ZERO || product >= y);
        if product < T::MAX {
            assert_eq!(product, x * y);
        }
    });
}

fn saturating_mul_properties_helper_signed<T: PrimitiveSigned>() {
    signed_pair_gen::<T>().test_properties(|(x, y)| {
        let mut product = x;
        product.saturating_mul_assign(y);
        assert_eq!(product, x.saturating_mul(y));
        assert_eq!(y.saturating_mul(x), product);
        if product > T::MIN && product < T::MAX {
            assert_eq!(product, x * y);
        }
    });
}

#[test]
fn saturating_mul_properties() {
    apply_fn_to_unsigneds!(saturating_mul_properties_helper_unsigned);
    apply_fn_to_signeds!(saturating_mul_properties_helper_signed);
}
