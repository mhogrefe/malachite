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
fn test_div_euclidean_unsigned() {
    fn test<T: PrimitiveUnsigned>(n: T, d: T, q: T, r: T) {
        assert_eq!(n.div_euclidean(d), (q, r));

        let mut mut_n = n;
        assert_eq!(mut_n.div_assign_euclidean(d), r);
        assert_eq!(mut_n, q);

        // For unsigned integers, Euclidean division coincides with `div_mod`.
        assert_eq!(n.div_mod(d), (q, r));
    }
    test::<u8>(0, 1, 0, 0);
    test::<u16>(0, 123, 0, 0);
    test::<u32>(1, 1, 1, 0);
    test::<u64>(123, 1, 123, 0);
    test::<usize>(123, 123, 1, 0);
    test::<u128>(123, 456, 0, 123);
    test::<u16>(456, 123, 3, 87);
}

#[test]
fn test_div_euclidean_signed() {
    fn test<T: PrimitiveSigned>(n: T, d: T, q: T, r: T) {
        assert_eq!(n.div_euclidean(d), (q, r));

        let mut mut_n = n;
        assert_eq!(mut_n.div_assign_euclidean(d), r);
        assert_eq!(mut_n, q);
    }
    test::<i8>(0, 1, 0, 0);
    test::<i16>(0, 123, 0, 0);
    test::<i32>(1, 1, 1, 0);
    test::<i64>(123, 1, 123, 0);
    test::<i128>(123, 123, 1, 0);
    test::<isize>(123, 456, 0, 123);
    // The remainder is always nonnegative, regardless of the signs of the operands.
    test::<i16>(23, 10, 2, 3);
    test::<i16>(23, -10, -2, 3);
    test::<i16>(-23, 10, -3, 7);
    test::<i16>(-23, -10, 3, 7);
    test::<i32>(-50, -23, 3, 19);
    test::<i64>(50, -23, -2, 4);
    // Division by -1 negates without producing a remainder.
    test::<i32>(123, -1, -123, 0);
}

fn div_euclidean_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::ONE.div_euclidean(T::ZERO));
    assert_panic!({
        let mut x = T::ONE;
        x.div_assign_euclidean(T::ZERO)
    });
}

fn div_euclidean_signed_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::MIN.div_euclidean(T::NEGATIVE_ONE));
    assert_panic!({
        let mut x = T::MIN;
        x.div_assign_euclidean(T::NEGATIVE_ONE)
    });
}

#[test]
pub fn div_euclidean_fail() {
    apply_fn_to_primitive_ints!(div_euclidean_fail_helper);
    apply_fn_to_signeds!(div_euclidean_signed_fail_helper);
}

fn div_euclidean_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_12::<T, T>().test_properties(|(x, y)| {
        let mut mut_x = x;
        let r = mut_x.div_assign_euclidean(y);
        let q = mut_x;
        assert_eq!(x.div_euclidean(y), (q, r));

        // For unsigned integers, Euclidean division coincides with `div_mod`.
        assert_eq!(x.div_mod(y), (q, r));
        assert!(r < y);
        assert_eq!(q * y + r, x);
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.div_euclidean(T::ONE), (x, T::ZERO));
        assert_panic!(x.div_euclidean(T::ZERO));
        assert_panic!({
            let mut y = x;
            y.div_assign_euclidean(T::ZERO)
        });
    });

    unsigned_gen_var_1::<T>().test_properties(|x| {
        assert_eq!(x.div_euclidean(x), (T::ONE, T::ZERO));
        assert_eq!(T::ZERO.div_euclidean(x), (T::ZERO, T::ZERO));
        if x > T::ONE {
            assert_eq!(T::ONE.div_euclidean(x), (T::ZERO, T::ONE));
        }
    });
}

fn div_euclidean_properties_helper_signed<T: PrimitiveSigned>() {
    signed_pair_gen_var_4::<T>().test_properties(|(x, y)| {
        let mut mut_x = x;
        let r = mut_x.div_assign_euclidean(y);
        let q = mut_x;
        assert_eq!(x.div_euclidean(y), (q, r));

        // The remainder is nonnegative and smaller in magnitude than the divisor.
        assert!(r >= T::ZERO);
        assert!(r.lt_abs(&y));
        // The defining relation x = q * y + r holds (modulo overflow of the q * y product).
        if let Some(product) = q.checked_mul(y) {
            assert_eq!(product + r, x);
        } else if q > T::ZERO {
            assert_eq!((q - T::ONE) * y + r + y, x);
        } else {
            assert_eq!((q + T::ONE) * y + r - y, x);
        }
        // For a positive divisor, Euclidean division coincides with `div_mod`.
        if y > T::ZERO {
            assert_eq!(x.div_mod(y), (q, r));
        }
    });

    signed_gen::<T>().test_properties(|x| {
        assert_eq!(x.div_euclidean(T::ONE), (x, T::ZERO));
        if x != T::MIN {
            assert_eq!(x.div_euclidean(T::NEGATIVE_ONE), (-x, T::ZERO));
        }
        assert_panic!(x.div_euclidean(T::ZERO));
        assert_panic!({
            let mut y = x;
            y.div_assign_euclidean(T::ZERO)
        });
    });

    signed_gen_var_6::<T>().test_properties(|x| {
        assert_eq!(x.div_euclidean(T::ONE), (x, T::ZERO));
        assert_eq!(x.div_euclidean(x), (T::ONE, T::ZERO));
        assert_eq!(T::ZERO.div_euclidean(x), (T::ZERO, T::ZERO));
        if x > T::ONE {
            assert_eq!(T::ONE.div_euclidean(x), (T::ZERO, T::ONE));
        }
    });
}

#[test]
fn div_euclidean_properties() {
    apply_fn_to_unsigneds!(div_euclidean_properties_helper_unsigned);
    apply_fn_to_signeds!(div_euclidean_properties_helper_signed);
}
