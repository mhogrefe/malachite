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
use malachite_base::test_util::generators::{signed_pair_gen_var_6, unsigned_pair_gen_var_12};
use std::panic::catch_unwind;

#[test]
fn test_wrapping_div() {
    fn test<T: PrimitiveInt>(x: T, y: T, out: T) {
        assert_eq!(x.wrapping_div(y), out);

        let mut x = x;
        x.wrapping_div_assign(y);
        assert_eq!(x, out);
    }
    test::<u16>(0, 5, 0);
    test::<u16>(123, 456, 0);
    test::<u8>(100, 3, 33);
    test::<i8>(100, -3, -33);
    test::<i16>(-100, 3, -33);
    test::<i32>(-100, -3, 33);
    test::<i8>(-128, -1, -128);
}

fn wrapping_div_assign_fail_helper<T: PrimitiveInt>() {
    assert_panic!({
        let mut n = T::ONE;
        n.wrapping_div_assign(T::ZERO);
    });
}

#[test]
fn wrapping_div_assign_fail() {
    apply_fn_to_primitive_ints!(wrapping_div_assign_fail_helper);
}

fn wrapping_div_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_12::<T, T>().test_properties(|(x, y)| {
        let mut quotient = x;
        quotient.wrapping_div_assign(y);
        assert_eq!(quotient, x.wrapping_div(y));
        assert_eq!(x / y, quotient);
    });
}

fn wrapping_div_properties_helper_signed<T: PrimitiveSigned>() {
    signed_pair_gen_var_6::<T>().test_properties(|(x, y)| {
        let mut quotient = x;
        quotient.wrapping_div_assign(y);
        assert_eq!(quotient, x.wrapping_div(y));
        if x != T::MIN || y != T::NEGATIVE_ONE {
            assert_eq!(x / y, quotient);
        }
    });
}

#[test]
fn wrapping_div_properties() {
    apply_fn_to_unsigneds!(wrapping_div_properties_helper_unsigned);
    apply_fn_to_signeds!(wrapping_div_properties_helper_signed);
}
