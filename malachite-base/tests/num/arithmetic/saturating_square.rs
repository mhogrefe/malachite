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
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};

#[test]
fn test_saturating_square() {
    fn test<T: PrimitiveInt>(x: T, out: T) {
        assert_eq!(x.saturating_square(), out);

        let mut x = x;
        x.saturating_square_assign();
        assert_eq!(x, out);
    }
    test::<u8>(0, 0);
    test::<i16>(1, 1);
    test::<u32>(2, 4);
    test::<i64>(3, 9);
    test::<u128>(10, 100);
    test::<isize>(123, 15129);
    test::<u32>(1000, 1000000);

    test::<i16>(-1, 1);
    test::<i32>(-2, 4);
    test::<i64>(-3, 9);
    test::<i128>(-10, 100);
    test::<isize>(-123, 15129);
    test::<i32>(-1000, 1000000);

    test::<u16>(1000, u16::MAX);
    test::<i16>(-1000, i16::MAX);
}

fn saturating_square_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|x| {
        let mut square = x;
        square.saturating_square_assign();
        assert_eq!(square, x.saturating_square());
        assert_eq!(square, x.saturating_pow(2));
        assert!(square >= x);
        if square < T::MAX {
            assert_eq!(square, x.square());
        }
    });
}

fn saturating_square_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|x| {
        let mut square = x;
        square.saturating_square_assign();
        assert_eq!(square, x.saturating_square());
        assert_eq!(square, x.saturating_pow(2));
        if square > T::MIN && square < T::MAX {
            assert_eq!(square, x.square());
        }
    });
}

#[test]
fn saturating_square_properties() {
    apply_fn_to_unsigneds!(saturating_square_properties_helper_unsigned);
    apply_fn_to_signeds!(saturating_square_properties_helper_signed);
}
