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

#[test]
fn test_wrapping_square() {
    fn test<T: PrimitiveInt>(x: T, out: T) {
        assert_eq!(x.wrapping_square(), out);

        let mut x = x;
        x.wrapping_square_assign();
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

    test::<u16>(1000, 16960);
    test::<i16>(-1000, 16960);
}

fn wrapping_square_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|x| {
        let mut square = x;
        square.wrapping_square_assign();
        assert_eq!(square, x.wrapping_square());
        assert_eq!(square, x.wrapping_pow(2));
    });
}

fn wrapping_square_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|x| {
        let mut square = x;
        square.wrapping_square_assign();
        assert_eq!(square, x.wrapping_square());
        assert_eq!(square, x.wrapping_pow(2));
        if x != T::MIN {
            assert_eq!((-x).wrapping_square(), square);
        }
    });
}

#[test]
fn saturating_square_properties() {
    apply_fn_to_unsigneds!(wrapping_square_properties_helper_unsigned);
    apply_fn_to_signeds!(wrapping_square_properties_helper_signed);
}
