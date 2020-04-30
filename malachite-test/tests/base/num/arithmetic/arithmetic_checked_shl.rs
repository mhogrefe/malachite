use std::ops::Shr;

use malachite_base::num::arithmetic::traits::{ArithmeticCheckedShl, ArithmeticCheckedShr};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::LeadingZeros;
use rand::Rand;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_signed, pairs_of_signed_and_small_unsigned,
    pairs_of_unsigned_and_small_signed, pairs_of_unsigned_and_small_unsigned, signeds,
    small_signeds, small_unsigneds, unsigneds,
};

fn arithmetic_checked_shl_unsigned_unsigned_helper<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>()
where
    T: Shr<U, Output = T> + ArithmeticCheckedShl<U, Output = T>,
    u64: ExactFrom<U>,
{
    test_properties(pairs_of_unsigned_and_small_unsigned::<T, U>, |&(n, u)| {
        if let Some(shifted) = n.arithmetic_checked_shl(u) {
            assert!(shifted >= n);
            if n != T::ZERO {
                assert_eq!(shifted >> u, n)
            }
        } else {
            assert_ne!(n, T::ZERO);
            assert!(LeadingZeros::leading_zeros(n) < u64::exact_from(u));
        }
    });

    test_properties(unsigneds::<T>, |&n| {
        assert_eq!(n.arithmetic_checked_shl(U::ZERO), Some(n));
    });

    test_properties_no_special(small_unsigneds::<U>, |&u| {
        assert_eq!(T::ZERO.arithmetic_checked_shl(u), Some(T::ZERO));
    });
}

fn arithmetic_checked_shl_unsigned_signed_helper<T: PrimitiveUnsigned + Rand, U: PrimitiveSigned>()
where
    T: ArithmeticCheckedShl<U, Output = T> + ArithmeticCheckedShr<U, Output = T>,
{
    test_properties(pairs_of_unsigned_and_small_signed::<T, U>, |&(n, i)| {
        let shifted = n.arithmetic_checked_shl(i);
        if shifted.is_none() {
            assert_ne!(n, T::ZERO);
        }
        if i != U::MIN {
            assert_eq!(n.arithmetic_checked_shr(-i), shifted);
        }
    });

    test_properties(unsigneds::<T>, |&n| {
        assert_eq!(n.arithmetic_checked_shl(U::ZERO), Some(n));
    });

    test_properties_no_special(small_signeds::<U>, |&i| {
        assert_eq!(T::ZERO.arithmetic_checked_shl(i), Some(T::ZERO));
    });
}

fn arithmetic_checked_shl_signed_unsigned_helper<T: PrimitiveSigned + Rand, U: PrimitiveUnsigned>()
where
    T: Shr<U, Output = T>
        + ArithmeticCheckedShl<U, Output = T>
        + WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    u64: ExactFrom<U>,
    T::UnsignedOfEqualWidth: Rand,
{
    test_properties(pairs_of_signed_and_small_unsigned::<T, U>, |&(n, u)| {
        if let Some(shifted) = n.arithmetic_checked_shl(u) {
            assert!(shifted.ge_abs(&n));
            if n != T::ZERO {
                assert_eq!(shifted >> u, n)
            }
        } else {
            assert_ne!(n, T::ZERO);
        }
    });

    test_properties(signeds::<T>, |&n| {
        assert_eq!(n.arithmetic_checked_shl(U::ZERO), Some(n));
    });

    test_properties_no_special(small_unsigneds::<U>, |&u| {
        assert_eq!(T::ZERO.arithmetic_checked_shl(u), Some(T::ZERO));
    });
}

fn arithmetic_checked_shl_signed_signed_helper<T: PrimitiveSigned + Rand, U: PrimitiveSigned>()
where
    T: ArithmeticCheckedShl<U, Output = T>
        + ArithmeticCheckedShr<U, Output = T>
        + WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: Rand,
{
    test_properties(pairs_of_signed_and_small_signed::<T, U>, |&(n, i)| {
        let shifted = n.arithmetic_checked_shl(i);
        if shifted.is_none() {
            assert_ne!(n, T::ZERO);
        }
        if i != U::MIN {
            assert_eq!(n.arithmetic_checked_shr(-i), shifted);
        }
    });

    test_properties(signeds::<T>, |&n| {
        assert_eq!(n.arithmetic_checked_shl(U::ZERO), Some(n));
    });

    test_properties_no_special(small_signeds::<U>, |&i| {
        assert_eq!(T::ZERO.arithmetic_checked_shl(i), Some(T::ZERO));
    });
}

#[test]
fn arithmetic_checked_shl_properties() {
    arithmetic_checked_shl_unsigned_unsigned_helper::<u8, u8>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u8, u16>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u8, u32>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u8, u64>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u8, usize>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u16, u8>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u16, u16>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u16, u32>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u16, u64>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u16, usize>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u32, u8>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u32, u16>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u32, u32>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u32, u64>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u32, usize>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u64, u8>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u64, u16>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u64, u32>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u64, u64>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<u64, usize>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<usize, u8>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<usize, u16>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<usize, u32>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<usize, u64>();
    arithmetic_checked_shl_unsigned_unsigned_helper::<usize, usize>();

    arithmetic_checked_shl_unsigned_signed_helper::<u8, i8>();
    arithmetic_checked_shl_unsigned_signed_helper::<u8, i16>();
    arithmetic_checked_shl_unsigned_signed_helper::<u8, i32>();
    arithmetic_checked_shl_unsigned_signed_helper::<u8, i64>();
    arithmetic_checked_shl_unsigned_signed_helper::<u8, isize>();
    arithmetic_checked_shl_unsigned_signed_helper::<u16, i8>();
    arithmetic_checked_shl_unsigned_signed_helper::<u16, i16>();
    arithmetic_checked_shl_unsigned_signed_helper::<u16, i32>();
    arithmetic_checked_shl_unsigned_signed_helper::<u16, i64>();
    arithmetic_checked_shl_unsigned_signed_helper::<u16, isize>();
    arithmetic_checked_shl_unsigned_signed_helper::<u32, i8>();
    arithmetic_checked_shl_unsigned_signed_helper::<u32, i16>();
    arithmetic_checked_shl_unsigned_signed_helper::<u32, i32>();
    arithmetic_checked_shl_unsigned_signed_helper::<u32, i64>();
    arithmetic_checked_shl_unsigned_signed_helper::<u32, isize>();
    arithmetic_checked_shl_unsigned_signed_helper::<u64, i8>();
    arithmetic_checked_shl_unsigned_signed_helper::<u64, i16>();
    arithmetic_checked_shl_unsigned_signed_helper::<u64, i32>();
    arithmetic_checked_shl_unsigned_signed_helper::<u64, i64>();
    arithmetic_checked_shl_unsigned_signed_helper::<u64, isize>();
    arithmetic_checked_shl_unsigned_signed_helper::<usize, i8>();
    arithmetic_checked_shl_unsigned_signed_helper::<usize, i16>();
    arithmetic_checked_shl_unsigned_signed_helper::<usize, i32>();
    arithmetic_checked_shl_unsigned_signed_helper::<usize, i64>();
    arithmetic_checked_shl_unsigned_signed_helper::<usize, isize>();

    arithmetic_checked_shl_signed_unsigned_helper::<i8, u8>();
    arithmetic_checked_shl_signed_unsigned_helper::<i8, u16>();
    arithmetic_checked_shl_signed_unsigned_helper::<i8, u32>();
    arithmetic_checked_shl_signed_unsigned_helper::<i8, u64>();
    arithmetic_checked_shl_signed_unsigned_helper::<i8, usize>();
    arithmetic_checked_shl_signed_unsigned_helper::<i16, u8>();
    arithmetic_checked_shl_signed_unsigned_helper::<i16, u16>();
    arithmetic_checked_shl_signed_unsigned_helper::<i16, u32>();
    arithmetic_checked_shl_signed_unsigned_helper::<i16, u64>();
    arithmetic_checked_shl_signed_unsigned_helper::<i16, usize>();
    arithmetic_checked_shl_signed_unsigned_helper::<i32, u8>();
    arithmetic_checked_shl_signed_unsigned_helper::<i32, u16>();
    arithmetic_checked_shl_signed_unsigned_helper::<i32, u32>();
    arithmetic_checked_shl_signed_unsigned_helper::<i32, u64>();
    arithmetic_checked_shl_signed_unsigned_helper::<i32, usize>();
    arithmetic_checked_shl_signed_unsigned_helper::<i64, u8>();
    arithmetic_checked_shl_signed_unsigned_helper::<i64, u16>();
    arithmetic_checked_shl_signed_unsigned_helper::<i64, u32>();
    arithmetic_checked_shl_signed_unsigned_helper::<i64, u64>();
    arithmetic_checked_shl_signed_unsigned_helper::<i64, usize>();
    arithmetic_checked_shl_signed_unsigned_helper::<isize, u8>();
    arithmetic_checked_shl_signed_unsigned_helper::<isize, u16>();
    arithmetic_checked_shl_signed_unsigned_helper::<isize, u32>();
    arithmetic_checked_shl_signed_unsigned_helper::<isize, u64>();
    arithmetic_checked_shl_signed_unsigned_helper::<isize, usize>();

    arithmetic_checked_shl_signed_signed_helper::<i8, i8>();
    arithmetic_checked_shl_signed_signed_helper::<i8, i16>();
    arithmetic_checked_shl_signed_signed_helper::<i8, i32>();
    arithmetic_checked_shl_signed_signed_helper::<i8, i64>();
    arithmetic_checked_shl_signed_signed_helper::<i8, isize>();
    arithmetic_checked_shl_signed_signed_helper::<i16, i8>();
    arithmetic_checked_shl_signed_signed_helper::<i16, i16>();
    arithmetic_checked_shl_signed_signed_helper::<i16, i32>();
    arithmetic_checked_shl_signed_signed_helper::<i16, i64>();
    arithmetic_checked_shl_signed_signed_helper::<i16, isize>();
    arithmetic_checked_shl_signed_signed_helper::<i32, i8>();
    arithmetic_checked_shl_signed_signed_helper::<i32, i16>();
    arithmetic_checked_shl_signed_signed_helper::<i32, i32>();
    arithmetic_checked_shl_signed_signed_helper::<i32, i64>();
    arithmetic_checked_shl_signed_signed_helper::<i32, isize>();
    arithmetic_checked_shl_signed_signed_helper::<i64, i8>();
    arithmetic_checked_shl_signed_signed_helper::<i64, i16>();
    arithmetic_checked_shl_signed_signed_helper::<i64, i32>();
    arithmetic_checked_shl_signed_signed_helper::<i64, i64>();
    arithmetic_checked_shl_signed_signed_helper::<i64, isize>();
    arithmetic_checked_shl_signed_signed_helper::<isize, i8>();
    arithmetic_checked_shl_signed_signed_helper::<isize, i16>();
    arithmetic_checked_shl_signed_signed_helper::<isize, i32>();
    arithmetic_checked_shl_signed_signed_helper::<isize, i64>();
    arithmetic_checked_shl_signed_signed_helper::<isize, isize>();
}
