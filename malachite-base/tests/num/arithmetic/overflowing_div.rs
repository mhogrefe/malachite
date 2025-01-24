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
use malachite_base::test_util::generators::{signed_pair_gen_var_6, unsigned_pair_gen_var_12};
use std::panic::catch_unwind;

#[test]
fn test_overflowing_div() {
    fn test<T: PrimitiveInt>(x: T, y: T, out: T, overflow: bool) {
        assert_eq!(x.overflowing_div(y), (out, overflow));

        let mut x = x;
        assert_eq!(x.overflowing_div_assign(y), overflow);
        assert_eq!(x, out);
    }
    test::<u16>(0, 5, 0, false);
    test::<u16>(123, 456, 0, false);
    test::<u8>(100, 3, 33, false);
    test::<i8>(100, -3, -33, false);
    test::<i16>(-100, 3, -33, false);
    test::<i32>(-100, -3, 33, false);
    test::<i8>(-128, -1, -128, true);
}

fn overflowing_div_assign_fail_helper<T: PrimitiveInt>() {
    assert_panic!({
        let mut n = T::ONE;
        n.overflowing_div_assign(T::ZERO);
    });
}

#[test]
fn overflowing_div_assign_fail() {
    apply_fn_to_primitive_ints!(overflowing_div_assign_fail_helper);
}

fn overflowing_div_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_12::<T, T>().test_properties(|(x, y)| {
        let mut quotient = x;
        let overflow = quotient.overflowing_div_assign(y);
        assert_eq!((quotient, overflow), x.overflowing_div(y));
        assert_eq!(x.wrapping_div(y), quotient);
        assert!(!overflow);
        assert_eq!(quotient, x / y);
    });
}

fn overflowing_div_properties_helper_signed<T: PrimitiveSigned>() {
    signed_pair_gen_var_6::<T>().test_properties(|(x, y)| {
        let mut quotient = x;
        let overflow = quotient.overflowing_div_assign(y);
        assert_eq!((quotient, overflow), x.overflowing_div(y));
        assert_eq!(x.wrapping_div(y), quotient);
        if !overflow {
            assert_eq!(quotient, x / y);
        }
    });
}

#[test]
fn overflowing_div_properties() {
    apply_fn_to_unsigneds!(overflowing_div_properties_helper_unsigned);
    apply_fn_to_signeds!(overflowing_div_properties_helper_signed);
}
