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
fn test_checked_square() {
    fn test<T: PrimitiveInt>(x: T, out: Option<T>) {
        assert_eq!(x.checked_square(), out);
    }
    test::<u8>(0, Some(0));
    test::<i16>(1, Some(1));
    test::<u32>(2, Some(4));
    test::<i64>(3, Some(9));
    test::<u128>(10, Some(100));
    test::<isize>(123, Some(15129));
    test::<u32>(1000, Some(1000000));

    test::<i16>(-1, Some(1));
    test::<i32>(-2, Some(4));
    test::<i64>(-3, Some(9));
    test::<i128>(-10, Some(100));
    test::<isize>(-123, Some(15129));
    test::<i32>(-1000, Some(1000000));

    test::<u16>(1000, None);
    test::<i16>(-1000, None);
}

fn unsigned_checked_square_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|x| {
        let square = x.checked_square();
        assert_eq!(square, x.checked_pow(2));
        if let Some(square) = square {
            assert_eq!(x.square(), square);
        }
    });
}

fn signed_checked_square_properties_helper<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|x| {
        let square = x.checked_square();
        assert_eq!(square, x.checked_pow(2));
        if let Some(square) = square {
            assert_eq!(x.square(), square);
        }
    });
}

#[test]
fn checked_square_properties() {
    apply_fn_to_unsigneds!(unsigned_checked_square_properties_helper);
    apply_fn_to_signeds!(signed_checked_square_properties_helper);
}
