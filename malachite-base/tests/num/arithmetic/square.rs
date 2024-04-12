// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::{
    primitive_float_gen, signed_gen_var_10, unsigned_gen_var_21,
};

#[test]
fn test_square() {
    fn test<T: PrimitiveInt>(x: T, out: T) {
        assert_eq!(x.square(), out);

        let mut x = x;
        x.square_assign();
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
}

fn square_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen_var_21::<T>().test_properties(|x| {
        let mut square = x;
        square.square_assign();
        assert_eq!(square, x.square());
        assert_eq!(square, x.pow(2));
        assert_eq!(square.checked_sqrt(), Some(x));
        if x > T::ONE {
            assert_eq!(square.checked_log_base(x), Some(2));
        }
    });
}

fn square_properties_helper_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>() {
    signed_gen_var_10::<U, S>().test_properties(|x| {
        let mut square = x;
        square.square_assign();
        assert_eq!(square, x.square());
        assert_eq!(square, x.pow(2));
        if x != S::MIN {
            assert_eq!((-x).square(), square);
        }
        assert_eq!(
            U::wrapping_from(square).checked_sqrt().unwrap(),
            x.unsigned_abs()
        );
    });
}

fn square_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|x| {
        let mut square = x;
        square.square_assign();
        assert_eq!(NiceFloat(square), NiceFloat(x.square()));
        assert_eq!(NiceFloat(square), NiceFloat(x.pow(2)));
        assert_eq!(NiceFloat((-x).square()), NiceFloat(square));
    });
}

#[test]
fn square_properties() {
    apply_fn_to_unsigneds!(square_properties_helper_unsigned);
    apply_fn_to_unsigned_signed_pairs!(square_properties_helper_signed);
    apply_fn_to_primitive_floats!(square_properties_helper_primitive_float);
}
