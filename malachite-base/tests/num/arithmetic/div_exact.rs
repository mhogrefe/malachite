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
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::generators::{
    signed_gen, signed_gen_var_6, signed_pair_gen_var_3, unsigned_gen, unsigned_gen_var_1,
    unsigned_pair_gen_var_11,
};
use std::panic::catch_unwind;

#[test]
fn test_div_exact() {
    fn test<T: PrimitiveInt>(x: T, y: T, out: T) {
        assert_eq!(x.div_exact(y), out);

        let mut x = x;
        x.div_exact_assign(y);
        assert_eq!(x, out);
    }
    test::<u8>(0, 123, 0);
    test::<u16>(123, 1, 123);
    test::<u32>(123, 123, 1);
    test::<usize>(56088, 123, 456);
    test::<u64>(0, 1000000000000, 0);
    test::<u128>(1000000000000, 1, 1000000000000);
    test::<usize>(1000000000000, 1000000000000, 1);
    test::<usize>(123000000000000, 1000000000000, 123);
    test::<usize>(123000000000000, 123, 1000000000000);
    test::<u128>(121932631112635269000000, 123456789000, 987654321000);
    test::<u64>(0x1fffffffe, 0xffffffff, 2);
    test::<u64>(18446744065119617025, 0xffffffff, 0xffffffff);

    test::<i8>(0, -123, 0);
    test::<i16>(123, -1, -123);
    test::<i32>(123, -123, -1);
    test::<isize>(56088, -123, -456);
    test::<i64>(0, -1000000000000, 0);
    test::<i128>(1000000000000, -1, -1000000000000);
    test::<isize>(1000000000000, -1000000000000, -1);
    test::<isize>(123000000000000, -1000000000000, -123);
    test::<isize>(123000000000000, -123, -1000000000000);
    test::<i128>(121932631112635269000000, -123456789000, -987654321000);
    test::<i64>(0x1fffffffe, -0xffffffff, -2);
    test::<i128>(18446744065119617025, -0xffffffff, -0xffffffff);

    test::<i16>(-123, 1, -123);
    test::<i32>(-123, 123, -1);
    test::<isize>(-56088, 123, -456);
    test::<i128>(-1000000000000, 1, -1000000000000);
    test::<isize>(-1000000000000, 1000000000000, -1);
    test::<isize>(-123000000000000, 1000000000000, -123);
    test::<isize>(-123000000000000, 123, -1000000000000);
    test::<i128>(-121932631112635269000000, 123456789000, -987654321000);
    test::<i64>(-0x1fffffffe, 0xffffffff, -2);
    test::<i128>(-18446744065119617025, 0xffffffff, -0xffffffff);

    test::<i16>(-123, -1, 123);
    test::<i32>(-123, -123, 1);
    test::<isize>(-56088, -123, 456);
    test::<i128>(-1000000000000, -1, 1000000000000);
    test::<isize>(-1000000000000, -1000000000000, 1);
    test::<isize>(-123000000000000, -1000000000000, 123);
    test::<isize>(-123000000000000, -123, 1000000000000);
    test::<i128>(-121932631112635269000000, -123456789000, 987654321000);
    test::<i64>(-0x1fffffffe, -0xffffffff, 2);
    test::<i128>(-18446744065119617025, -0xffffffff, 0xffffffff);
}

fn div_exact_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::ONE.div_exact(T::ZERO));
    assert_panic!({
        let mut n = T::ONE;
        n.div_exact_assign(T::ZERO);
    });
}

#[test]
pub fn div_exact_fail() {
    apply_fn_to_primitive_ints!(div_exact_fail_helper);
}

fn div_exact_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_11::<T>().test_properties(|(x, y)| {
        let mut mut_x = x;
        mut_x.div_exact_assign(y);
        let q = mut_x;

        assert_eq!(x.div_exact(y), q);
        assert_eq!(x.div_round(y, Exact).0, q);
        assert_eq!(q * y, x);
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.div_exact(T::ONE), x);
    });

    unsigned_gen_var_1::<T>().test_properties(|x| {
        assert_eq!(T::ZERO.div_exact(x), T::ZERO);
        assert_eq!(x.div_exact(x), T::ONE);
    });
}

fn div_exact_properties_helper_signed<T: PrimitiveSigned>() {
    signed_pair_gen_var_3::<T>().test_properties(|(x, y)| {
        let mut mut_x = x;
        mut_x.div_exact_assign(y);
        let q = mut_x;

        assert_eq!(x.div_exact(y), q);
        assert_eq!(x.div_round(y, Exact).0, q);
        assert_eq!(q * y, x);

        if x != T::MIN {
            assert_eq!((-x).div_exact(y), -q);
        }
        if y != T::MIN && q != T::MIN {
            assert_eq!(x.div_exact(-y), -q);
        }
    });

    signed_gen::<T>().test_properties(|x| {
        assert_eq!(x.div_exact(T::ONE), x);
    });

    signed_gen_var_6::<T>().test_properties(|x| {
        assert_eq!(T::ZERO.div_exact(x), T::ZERO);
        assert_eq!(x.div_exact(x), T::ONE);
    });
}

#[test]
fn div_exact_properties() {
    apply_fn_to_unsigneds!(div_exact_properties_helper_unsigned);
    apply_fn_to_signeds!(div_exact_properties_helper_signed);
}
