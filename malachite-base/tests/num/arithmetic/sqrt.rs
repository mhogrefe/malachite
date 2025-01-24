// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::sqrt::sqrt_rem_newton;
use malachite_base::num::arithmetic::sqrt::{
    ceiling_sqrt_binary, checked_sqrt_binary, floor_sqrt_binary, sqrt_rem_binary,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::{
    primitive_float_gen, signed_gen_var_2, unsigned_gen, unsigned_gen_var_17,
};
use std::panic::catch_unwind;

#[test]
fn test_sqrt_rem_newton() {
    fn test<U: PrimitiveUnsigned + WrappingFrom<S>, S: PrimitiveSigned + WrappingFrom<U>>(
        n: U,
        sqrt: U,
        rem: U,
    ) {
        let (actual_sqrt, actual_rem) = sqrt_rem_newton::<U, S>(n);
        assert_eq!(actual_sqrt, sqrt);
        assert_eq!(actual_rem, rem);
        assert_eq!(n.sqrt_rem(), (sqrt, rem));
    }
    // - no initial underestimate
    test::<u32, i32>(2000000000, 44721, 32159);
    test::<u32, i32>(u32::MAX, 65535, 131070);
    // - initial underestimate
    test::<u32, i32>(1073741824, 32768, 0);

    test::<u64, i64>(10000000000000000000, 3162277660, 1064924400);
    test::<u64, i64>(u64::MAX, 4294967295, 8589934590);
}

#[test]
fn sqrt_rem_newton_fail() {
    assert_panic!(sqrt_rem_newton::<u32, i32>(1));
    assert_panic!(sqrt_rem_newton::<u64, i64>(1));
}

#[test]
fn test_floor_sqrt() {
    fn test_u<T: PrimitiveUnsigned>(n: T, out: T) {
        assert_eq!(n.floor_sqrt(), out);
        assert_eq!(floor_sqrt_binary(n), out);

        let mut n = n;
        n.floor_sqrt_assign();
        assert_eq!(n, out);
    }
    test_u::<u8>(0, 0);
    test_u::<u8>(1, 1);
    test_u::<u8>(2, 1);
    test_u::<u8>(3, 1);
    test_u::<u8>(4, 2);
    test_u::<u8>(5, 2);
    test_u::<u8>(10, 3);
    test_u::<u8>(100, 10);
    test_u::<u32>(1000000000, 31622);
    test_u::<u64>(152415765279683, 12345677);
    test_u::<u64>(152415765279684, 12345678);
    test_u::<u64>(152415765279685, 12345678);

    fn test_i<T: PrimitiveSigned>(n: T, out: T) {
        assert_eq!(n.floor_sqrt(), out);

        let mut n = n;
        n.floor_sqrt_assign();
        assert_eq!(n, out);
    }
    test_i::<i8>(0, 0);
    test_i::<i8>(1, 1);
    test_i::<i8>(2, 1);
    test_i::<i8>(3, 1);
    test_i::<i8>(4, 2);
    test_i::<i8>(5, 2);
    test_i::<i8>(10, 3);
    test_i::<i8>(100, 10);
    test_i::<i32>(1000000000, 31622);
    test_i::<i64>(152415765279683, 12345677);
    test_i::<i64>(152415765279684, 12345678);
    test_i::<i64>(152415765279685, 12345678);
}

fn floor_sqrt_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::NEGATIVE_ONE.floor_sqrt());
}

#[test]
pub fn floor_sqrt_fail() {
    apply_fn_to_signeds!(floor_sqrt_fail_helper);
}

#[test]
fn test_ceiling_sqrt() {
    fn test_u<T: PrimitiveUnsigned>(n: T, out: T) {
        assert_eq!(n.ceiling_sqrt(), out);
        assert_eq!(ceiling_sqrt_binary(n), out);

        let mut n = n;
        n.ceiling_sqrt_assign();
        assert_eq!(n, out);
    }
    test_u::<u8>(0, 0);
    test_u::<u8>(1, 1);
    test_u::<u8>(2, 2);
    test_u::<u8>(3, 2);
    test_u::<u8>(4, 2);
    test_u::<u8>(5, 3);
    test_u::<u8>(10, 4);
    test_u::<u8>(100, 10);
    test_u::<u32>(1000000000, 31623);
    test_u::<u64>(152415765279683, 12345678);
    test_u::<u64>(152415765279684, 12345678);
    test_u::<u64>(152415765279685, 12345679);

    fn test_i<T: PrimitiveSigned>(n: T, out: T) {
        assert_eq!(n.ceiling_sqrt(), out);

        let mut n = n;
        n.ceiling_sqrt_assign();
        assert_eq!(n, out);
    }
    test_i::<i8>(0, 0);
    test_i::<i8>(1, 1);
    test_i::<i8>(2, 2);
    test_i::<i8>(3, 2);
    test_i::<i8>(4, 2);
    test_i::<i8>(5, 3);
    test_i::<i8>(10, 4);
    test_i::<i8>(100, 10);
    test_i::<i32>(1000000000, 31623);
    test_i::<i64>(152415765279683, 12345678);
    test_i::<i64>(152415765279684, 12345678);
    test_i::<i64>(152415765279685, 12345679);
}

fn ceiling_sqrt_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::NEGATIVE_ONE.ceiling_sqrt());
}

#[test]
pub fn ceiling_sqrt_fail() {
    apply_fn_to_signeds!(ceiling_sqrt_fail_helper);
}

#[test]
fn test_checked_sqrt() {
    fn test_u<T: PrimitiveUnsigned>(n: T, out: Option<T>) {
        assert_eq!(n.checked_sqrt(), out);
        assert_eq!(checked_sqrt_binary(n), out);
    }
    test_u::<u8>(0, Some(0));
    test_u::<u8>(1, Some(1));
    test_u::<u8>(2, None);
    test_u::<u8>(3, None);
    test_u::<u8>(4, Some(2));
    test_u::<u8>(5, None);
    test_u::<u8>(10, None);
    test_u::<u8>(100, Some(10));
    test_u::<u32>(1000000000, None);
    test_u::<u64>(152415765279683, None);
    test_u::<u64>(152415765279684, Some(12345678));
    test_u::<u64>(152415765279685, None);

    fn test_i<T: PrimitiveSigned>(n: T, out: Option<T>) {
        assert_eq!(n.checked_sqrt(), out);
    }
    test_i::<i8>(0, Some(0));
    test_i::<i8>(1, Some(1));
    test_i::<i8>(2, None);
    test_i::<i8>(3, None);
    test_i::<i8>(4, Some(2));
    test_i::<i8>(5, None);
    test_i::<i8>(10, None);
    test_i::<i8>(100, Some(10));
    test_i::<i32>(1000000000, None);
    test_i::<i64>(152415765279683, None);
    test_i::<i64>(152415765279684, Some(12345678));
    test_i::<i64>(152415765279685, None);
}

fn checked_sqrt_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::NEGATIVE_ONE.checked_sqrt());
}

#[test]
pub fn checked_sqrt_fail() {
    apply_fn_to_signeds!(checked_sqrt_fail_helper);
}

#[test]
fn test_sqrt_rem() {
    fn test<T: PrimitiveUnsigned>(n: T, sqrt: T, rem: T) {
        let (actual_sqrt, actual_rem) = n.sqrt_rem();
        assert_eq!(actual_sqrt, sqrt);
        assert_eq!(actual_rem, rem);
        assert_eq!(sqrt_rem_binary(n), (sqrt, rem));

        let mut n = n;
        assert_eq!(n.sqrt_assign_rem(), rem);
        assert_eq!(n, sqrt);
    }
    test::<u8>(0, 0, 0);
    test::<u8>(1, 1, 0);
    test::<u8>(2, 1, 1);
    test::<u8>(3, 1, 2);
    test::<u8>(4, 2, 0);
    test::<u8>(5, 2, 1);
    test::<u8>(10, 3, 1);
    test::<u8>(100, 10, 0);
    test::<u32>(1000000000, 31622, 49116);
    test::<u64>(152415765279683, 12345677, 24691354);
    test::<u64>(152415765279684, 12345678, 0);
    test::<u64>(152415765279685, 12345678, 1);
}

#[test]
fn test_sqrt() {
    fn test<T: PrimitiveFloat>(n: T, out: T) {
        assert_eq!(NiceFloat(n.sqrt()), NiceFloat(out));

        let mut n = n;
        n.sqrt_assign();
        assert_eq!(NiceFloat(n), NiceFloat(out));
    }
    test::<f32>(0.0, 0.0);
    test::<f32>(-0.0, -0.0);
    test::<f32>(1.0, 1.0);
    test::<f32>(f32::INFINITY, f32::INFINITY);
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(2.0, std::f32::consts::SQRT_2);
    test::<f32>(-1.0, f32::NAN);
}

fn floor_sqrt_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|n| {
        let sqrt = n.floor_sqrt();
        let mut n_alt = n;
        n_alt.floor_sqrt_assign();
        assert_eq!(n_alt, sqrt);
        assert_eq!(floor_sqrt_binary(n), sqrt);
        assert_eq!(n.floor_root(2), sqrt);
        let square = sqrt.square();
        let ceiling_sqrt = n.ceiling_sqrt();
        if square == n {
            assert_eq!(ceiling_sqrt, sqrt);
        } else {
            assert_eq!(ceiling_sqrt, sqrt + T::ONE);
        }
        assert!(square <= n);
        if let Some(next_square) = (sqrt + T::ONE).checked_square() {
            assert!(next_square > n);
        }
    });
}

fn floor_sqrt_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen_var_2::<T>().test_properties(|n| {
        let sqrt = n.floor_sqrt();
        let mut n_alt = n;
        n_alt.floor_sqrt_assign();
        assert_eq!(n_alt, sqrt);
        assert_eq!(n.floor_root(2), sqrt);
        let square = sqrt.square();
        let ceiling_sqrt = n.ceiling_sqrt();
        if square == n {
            assert_eq!(ceiling_sqrt, sqrt);
        } else {
            assert_eq!(ceiling_sqrt, sqrt + T::ONE);
        }
        assert!(square <= n);
        if let Some(next_square) = (sqrt + T::ONE).checked_square() {
            assert!(next_square > n);
        }
    });
}

#[test]
fn floor_sqrt_properties() {
    apply_fn_to_unsigneds!(floor_sqrt_properties_helper_unsigned);
    apply_fn_to_signeds!(floor_sqrt_properties_helper_signed);
}

fn ceiling_sqrt_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|n| {
        let sqrt = n.ceiling_sqrt();
        let mut n_alt = n;
        n_alt.ceiling_sqrt_assign();
        assert_eq!(n_alt, sqrt);
        assert_eq!(ceiling_sqrt_binary(n), sqrt);
        assert_eq!(n.ceiling_root(2), sqrt);
        if let Some(square) = sqrt.checked_square() {
            let floor_sqrt = n.floor_sqrt();
            if square == n {
                assert_eq!(floor_sqrt, sqrt);
            } else {
                assert_eq!(floor_sqrt, sqrt - T::ONE);
            }
            assert!(square >= n);
        }
        if n != T::ZERO {
            assert!((sqrt - T::ONE).square() < n);
        }
    });
}

fn ceiling_sqrt_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen_var_2::<T>().test_properties(|n| {
        let sqrt = n.ceiling_sqrt();
        let mut n_alt = n;
        n_alt.ceiling_sqrt_assign();
        assert_eq!(n_alt, sqrt);
        assert_eq!(n.ceiling_root(2), sqrt);
        if let Some(square) = sqrt.checked_square() {
            let floor_sqrt = n.floor_sqrt();
            if square == n {
                assert_eq!(floor_sqrt, sqrt);
            } else {
                assert_eq!(floor_sqrt, sqrt - T::ONE);
            }
            assert!(square >= n);
        }
        if n != T::ZERO {
            assert!((sqrt - T::ONE).square() < n);
        }
    });
}

#[test]
fn ceiling_sqrt_properties() {
    apply_fn_to_unsigneds!(ceiling_sqrt_properties_helper_unsigned);
    apply_fn_to_signeds!(ceiling_sqrt_properties_helper_signed);
}

fn checked_sqrt_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|n| {
        let sqrt = n.checked_sqrt();
        assert_eq!(checked_sqrt_binary(n), sqrt);
        assert_eq!(n.checked_root(2), sqrt);
        if let Some(sqrt) = sqrt {
            assert_eq!(sqrt.square(), n);
            assert_eq!(n.floor_sqrt(), sqrt);
            assert_eq!(n.ceiling_sqrt(), sqrt);
        }
    });
}

fn checked_sqrt_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen_var_2::<T>().test_properties(|n| {
        let sqrt = n.checked_sqrt();
        assert_eq!(n.checked_root(2), sqrt);
        if let Some(sqrt) = sqrt {
            assert_eq!(sqrt.square(), n);
            assert_eq!(n.floor_sqrt(), sqrt);
            assert_eq!(n.ceiling_sqrt(), sqrt);
        }
    });
}

#[test]
fn checked_sqrt_properties() {
    apply_fn_to_unsigneds!(checked_sqrt_properties_helper_unsigned);
    apply_fn_to_signeds!(checked_sqrt_properties_helper_signed);
}

fn sqrt_rem_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|n| {
        let (sqrt, rem) = n.sqrt_rem();
        let mut n_alt = n;
        assert_eq!(n_alt.sqrt_assign_rem(), rem);
        assert_eq!(n_alt, sqrt);
        assert_eq!(sqrt_rem_binary(n), (sqrt, rem));
        assert_eq!(n.root_rem(2), (sqrt, rem));
        assert_eq!(n.floor_sqrt(), sqrt);
        assert!(rem <= sqrt << 1);
        assert_eq!(sqrt.square() + rem, n);
    });
}

fn sqrt_rem_properties_helper_extra<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() {
    unsigned_gen_var_17::<U>().test_properties(|n| {
        assert_eq!(sqrt_rem_newton::<U, S>(n), n.sqrt_rem());
    });
}

#[test]
fn sqrt_rem_properties() {
    apply_fn_to_unsigneds!(sqrt_rem_properties_helper);
    sqrt_rem_properties_helper_extra::<u32, i32>();
    sqrt_rem_properties_helper_extra::<u64, i64>();
}

fn sqrt_assign_properties_helper<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|f| {
        let mut sqrt = f;
        sqrt.sqrt_assign();
        assert_eq!(NiceFloat(sqrt), NiceFloat(f.sqrt()));
        assert!(sqrt.is_nan() || sqrt >= T::ZERO);
    });
}

#[test]
fn sqrt_assign_properties() {
    apply_fn_to_primitive_floats!(sqrt_assign_properties_helper);
}
