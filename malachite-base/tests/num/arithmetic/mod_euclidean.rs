// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    signed_gen, signed_gen_var_6, signed_pair_gen_var_4, unsigned_gen, unsigned_gen_var_1,
    unsigned_pair_gen_var_12,
};
use std::panic::catch_unwind;

#[test]
fn test_mod_euclidean_unsigned() {
    fn test<T: PrimitiveUnsigned>(n: T, d: T, r: T) {
        assert_eq!(n.mod_euclidean(d), r);

        let mut mut_n = n;
        mut_n.mod_euclidean_assign(d);
        assert_eq!(mut_n, r);

        // For unsigned integers, the Euclidean remainder coincides with `mod_op`.
        assert_eq!(n.mod_op(d), r);
    }
    test::<u8>(0, 1, 0);
    test::<u16>(0, 123, 0);
    test::<u32>(1, 1, 0);
    test::<u64>(123, 1, 0);
    test::<usize>(123, 123, 0);
    test::<u128>(123, 456, 123);
    test::<u16>(456, 123, 87);
}

#[test]
fn test_mod_euclidean_signed() {
    fn test<T: PrimitiveSigned>(n: T, d: T, r: T) {
        assert_eq!(n.mod_euclidean(d), r);

        let mut mut_n = n;
        mut_n.mod_euclidean_assign(d);
        assert_eq!(mut_n, r);
    }
    test::<i8>(0, 1, 0);
    test::<i16>(0, 123, 0);
    test::<i32>(1, 1, 0);
    test::<i64>(123, 1, 0);
    test::<i128>(123, 123, 0);
    test::<isize>(123, 456, 123);
    // The remainder is always nonnegative, regardless of the signs of the operands.
    test::<i16>(23, 10, 3);
    test::<i16>(23, -10, 3);
    test::<i16>(-23, 10, 7);
    test::<i16>(-23, -10, 7);
    test::<i32>(-50, -23, 19);
    test::<i64>(50, -23, 4);
    // Division by -1 leaves no remainder.
    test::<i32>(123, -1, 0);
}

fn mod_euclidean_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::ONE.mod_euclidean(T::ZERO));
    assert_panic!({
        let mut x = T::ONE;
        x.mod_euclidean_assign(T::ZERO)
    });
}

fn mod_euclidean_signed_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::MIN.mod_euclidean(T::NEGATIVE_ONE));
    assert_panic!({
        let mut x = T::MIN;
        x.mod_euclidean_assign(T::NEGATIVE_ONE)
    });
}

#[test]
pub fn mod_euclidean_fail() {
    apply_fn_to_primitive_ints!(mod_euclidean_fail_helper);
    apply_fn_to_signeds!(mod_euclidean_signed_fail_helper);
}

fn mod_euclidean_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_12::<T, T>().test_properties(|(x, y)| {
        let r = x.mod_euclidean(y);

        let mut mut_x = x;
        mut_x.mod_euclidean_assign(y);
        assert_eq!(mut_x, r);

        // The remainder is the one Euclidean division produces.
        assert_eq!(x.div_mod_euclidean(y).1, r);
        // For unsigned integers, the Euclidean remainder coincides with `mod_op`.
        assert_eq!(x.mod_op(y), r);
        assert!(r < y);
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.mod_euclidean(T::ONE), T::ZERO);
        assert_panic!(x.mod_euclidean(T::ZERO));
        assert_panic!({
            let mut y = x;
            y.mod_euclidean_assign(T::ZERO)
        });
    });

    unsigned_gen_var_1::<T>().test_properties(|x| {
        assert_eq!(x.mod_euclidean(x), T::ZERO);
        assert_eq!(T::ZERO.mod_euclidean(x), T::ZERO);
        if x > T::ONE {
            assert_eq!(T::ONE.mod_euclidean(x), T::ONE);
        }
    });
}

fn mod_euclidean_properties_helper_signed<T: PrimitiveSigned>() {
    signed_pair_gen_var_4::<T>().test_properties(|(x, y)| {
        let r = x.mod_euclidean(y);

        let mut mut_x = x;
        mut_x.mod_euclidean_assign(y);
        assert_eq!(mut_x, r);

        // The remainder is the one Euclidean division produces.
        assert_eq!(x.div_mod_euclidean(y).1, r);
        // The remainder is nonnegative and smaller in magnitude than the divisor.
        assert!(r >= T::ZERO);
        assert!(r.lt_abs(&y));
        // For a positive divisor, the Euclidean remainder coincides with `mod_op`.
        if y > T::ZERO {
            assert_eq!(x.mod_op(y), r);
        }
    });

    signed_gen::<T>().test_properties(|x| {
        assert_eq!(x.mod_euclidean(T::ONE), T::ZERO);
        if x != T::MIN {
            assert_eq!(x.mod_euclidean(T::NEGATIVE_ONE), T::ZERO);
        }
        assert_panic!(x.mod_euclidean(T::ZERO));
        assert_panic!({
            let mut y = x;
            y.mod_euclidean_assign(T::ZERO)
        });
    });

    signed_gen_var_6::<T>().test_properties(|x| {
        assert_eq!(x.mod_euclidean(x), T::ZERO);
        assert_eq!(T::ZERO.mod_euclidean(x), T::ZERO);
        if x > T::ONE {
            assert_eq!(T::ONE.mod_euclidean(x), T::ONE);
        }
    });
}

#[test]
fn mod_euclidean_properties() {
    apply_fn_to_unsigneds!(mod_euclidean_properties_helper_unsigned);
    apply_fn_to_signeds!(mod_euclidean_properties_helper_signed);
}
