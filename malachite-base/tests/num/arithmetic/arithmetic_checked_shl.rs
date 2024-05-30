// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ArithmeticCheckedShl, ArithmeticCheckedShr};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::LeadingZeros;
use malachite_base::test_util::generators::{
    signed_gen, signed_gen_var_5, signed_pair_gen_var_2, signed_unsigned_pair_gen_var_1,
    unsigned_gen, unsigned_gen_var_5, unsigned_pair_gen_var_2, unsigned_signed_pair_gen_var_1,
};
use std::ops::Shr;

#[test]
fn test_arithmetic_checked_shl() {
    fn test<T: ArithmeticCheckedShl<U, Output = T> + PrimitiveInt, U: PrimitiveInt>(
        t: T,
        u: U,
        out: Option<T>,
    ) {
        assert_eq!(t.arithmetic_checked_shl(u), out);
    }
    test::<u16, u8>(0, 0, Some(0));
    test::<u8, u16>(3, 6, Some(192));
    test::<u8, u32>(3, 7, None);
    test::<u64, u64>(3, 100, None);
    test::<u64, u128>(0, 100, Some(0));

    test::<u32, i8>(100, -3, Some(12));
    test::<u32, i16>(100, -100, Some(0));

    test::<i8, u8>(3, 5, Some(96));
    test::<i8, u16>(3, 6, None);
    test::<i8, u32>(-3, 5, Some(-96));
    test::<i8, u64>(-3, 6, None);
    test::<i16, u128>(3, 100, None);
    test::<i16, usize>(-3, 100, None);
    test::<i64, u8>(0, 100, Some(0));

    test::<i8, i8>(3, 5, Some(96));
    test::<i8, i16>(3, 6, None);
    test::<i8, i32>(-3, 5, Some(-96));
    test::<i8, i64>(-3, 6, None);
    test::<i16, i128>(3, 100, None);
    test::<i16, isize>(-3, 100, None);
    test::<i32, i8>(0, 100, Some(0));
    test::<i32, i16>(100, -3, Some(12));
    test::<i32, i32>(-100, -3, Some(-13));
    test::<i64, i64>(100, -100, Some(0));
    test::<i64, i128>(-100, -100, Some(-1));
}

fn arithmetic_checked_shl_properties_helper_unsigned_unsigned<
    T: ArithmeticCheckedShl<U, Output = T> + PrimitiveUnsigned + Shr<U, Output = T>,
    U: PrimitiveUnsigned,
>()
where
    u64: ExactFrom<U>,
{
    unsigned_pair_gen_var_2::<T, U>().test_properties(|(n, u)| {
        if let Some(shifted) = n.arithmetic_checked_shl(u) {
            assert!(shifted >= n);
            if n != T::ZERO {
                assert_eq!(shifted >> u, n);
            }
        } else {
            assert_ne!(n, T::ZERO);
            assert!(LeadingZeros::leading_zeros(n) < u64::exact_from(u));
        }
    });

    unsigned_gen::<T>().test_properties(|n| {
        assert_eq!(n.arithmetic_checked_shl(U::ZERO), Some(n));
    });

    unsigned_gen_var_5::<U>().test_properties(|u| {
        assert_eq!(T::ZERO.arithmetic_checked_shl(u), Some(T::ZERO));
    });
}

// Type repetition to avoid long line
#[allow(
    clippy::type_repetition_in_bounds,
    clippy::trait_duplication_in_bounds,
    clippy::multiple_bound_locations
)]
fn arithmetic_checked_shl_properties_helper_unsigned_signed<
    T: ArithmeticCheckedShl<U, Output = T> + ArithmeticCheckedShr<U, Output = T>,
    U: PrimitiveSigned,
>()
where
    u64: ExactFrom<U>,
    T: PrimitiveUnsigned,
{
    unsigned_signed_pair_gen_var_1::<T, U>().test_properties(|(n, i)| {
        let shifted = n.arithmetic_checked_shl(i);
        if shifted.is_none() {
            assert_ne!(n, T::ZERO);
        }
        if i != U::MIN {
            assert_eq!(n.arithmetic_checked_shr(-i), shifted);
        }
    });

    unsigned_gen::<T>().test_properties(|n| {
        assert_eq!(n.arithmetic_checked_shl(U::ZERO), Some(n));
    });

    signed_gen_var_5::<U>().test_properties(|i| {
        assert_eq!(T::ZERO.arithmetic_checked_shl(i), Some(T::ZERO));
    });
}

fn arithmetic_checked_shl_properties_helper_signed_unsigned<
    T: ArithmeticCheckedShl<U, Output = T> + PrimitiveSigned + Shr<U, Output = T>,
    U: PrimitiveUnsigned,
>()
where
    u64: ExactFrom<U>,
{
    signed_unsigned_pair_gen_var_1::<T, U>().test_properties(|(n, u)| {
        if let Some(shifted) = n.arithmetic_checked_shl(u) {
            assert!(shifted.ge_abs(&n));
            if n != T::ZERO {
                assert_eq!(shifted >> u, n);
            }
        } else {
            assert_ne!(n, T::ZERO);
        }
    });

    signed_gen::<T>().test_properties(|n| {
        assert_eq!(n.arithmetic_checked_shl(U::ZERO), Some(n));
    });

    unsigned_gen_var_5::<U>().test_properties(|u| {
        assert_eq!(T::ZERO.arithmetic_checked_shl(u), Some(T::ZERO));
    });
}

fn arithmetic_checked_shl_properties_helper_signed_signed<
    T: ArithmeticCheckedShl<U, Output = T> + ArithmeticCheckedShr<U, Output = T> + PrimitiveSigned,
    U: PrimitiveSigned,
>()
where
    u64: ExactFrom<U>,
{
    signed_pair_gen_var_2::<T, U>().test_properties(|(n, i)| {
        let shifted = n.arithmetic_checked_shl(i);
        if shifted.is_none() {
            assert_ne!(n, T::ZERO);
        }
        if i != U::MIN {
            assert_eq!(n.arithmetic_checked_shr(-i), shifted);
        }
    });

    signed_gen::<T>().test_properties(|n| {
        assert_eq!(n.arithmetic_checked_shl(U::ZERO), Some(n));
    });

    signed_gen_var_5::<U>().test_properties(|i| {
        assert_eq!(T::ZERO.arithmetic_checked_shl(i), Some(T::ZERO));
    });
}

#[test]
fn arithmetic_checked_shl_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(
        arithmetic_checked_shl_properties_helper_unsigned_unsigned
    );
    apply_fn_to_unsigneds_and_signeds!(arithmetic_checked_shl_properties_helper_unsigned_signed);
    apply_fn_to_signeds_and_unsigneds!(arithmetic_checked_shl_properties_helper_signed_unsigned);
    apply_fn_to_signeds_and_signeds!(arithmetic_checked_shl_properties_helper_signed_signed);
}
