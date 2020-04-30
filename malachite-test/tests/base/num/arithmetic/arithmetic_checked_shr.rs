use malachite_base::num::arithmetic::traits::{ArithmeticCheckedShl, ArithmeticCheckedShr};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_signed, pairs_of_unsigned_and_small_signed, signeds, small_signeds,
    unsigneds,
};

fn arithmetic_checked_shr_unsigned_signed_helper<T: PrimitiveUnsigned + Rand, U: PrimitiveSigned>()
where
    T: ArithmeticCheckedShl<U, Output = T> + ArithmeticCheckedShr<U, Output = T>,
{
    test_properties(pairs_of_unsigned_and_small_signed::<T, U>, |&(n, i)| {
        let shifted = n.arithmetic_checked_shr(i);
        if shifted.is_none() {
            assert_ne!(n, T::ZERO);
        }
        if i != U::MIN {
            assert_eq!(n.arithmetic_checked_shl(-i), shifted);
        }
    });

    test_properties(unsigneds::<T>, |&n| {
        assert_eq!(n.arithmetic_checked_shr(U::ZERO), Some(n));
    });

    test_properties_no_special(small_signeds::<U>, |&i| {
        assert_eq!(T::ZERO.arithmetic_checked_shr(i), Some(T::ZERO));
    });
}

fn arithmetic_checked_shr_signed_signed_helper<T: PrimitiveSigned + Rand, U: PrimitiveSigned>()
where
    T: ArithmeticCheckedShl<U, Output = T>
        + ArithmeticCheckedShr<U, Output = T>
        + WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: Rand,
{
    test_properties(pairs_of_signed_and_small_signed::<T, U>, |&(n, i)| {
        let shifted = n.arithmetic_checked_shr(i);
        if shifted.is_none() {
            assert_ne!(n, T::ZERO);
        }
        if i != U::MIN {
            assert_eq!(n.arithmetic_checked_shl(-i), shifted);
        }
    });

    test_properties(signeds::<T>, |&n| {
        assert_eq!(n.arithmetic_checked_shr(U::ZERO), Some(n));
    });

    test_properties_no_special(small_signeds::<U>, |&i| {
        assert_eq!(T::ZERO.arithmetic_checked_shr(i), Some(T::ZERO));
    });
}

#[test]
fn arithmetic_checked_shr_properties() {
    arithmetic_checked_shr_unsigned_signed_helper::<u8, i8>();
    arithmetic_checked_shr_unsigned_signed_helper::<u8, i16>();
    arithmetic_checked_shr_unsigned_signed_helper::<u8, i32>();
    arithmetic_checked_shr_unsigned_signed_helper::<u8, i64>();
    arithmetic_checked_shr_unsigned_signed_helper::<u8, isize>();
    arithmetic_checked_shr_unsigned_signed_helper::<u16, i8>();
    arithmetic_checked_shr_unsigned_signed_helper::<u16, i16>();
    arithmetic_checked_shr_unsigned_signed_helper::<u16, i32>();
    arithmetic_checked_shr_unsigned_signed_helper::<u16, i64>();
    arithmetic_checked_shr_unsigned_signed_helper::<u16, isize>();
    arithmetic_checked_shr_unsigned_signed_helper::<u32, i8>();
    arithmetic_checked_shr_unsigned_signed_helper::<u32, i16>();
    arithmetic_checked_shr_unsigned_signed_helper::<u32, i32>();
    arithmetic_checked_shr_unsigned_signed_helper::<u32, i64>();
    arithmetic_checked_shr_unsigned_signed_helper::<u32, isize>();
    arithmetic_checked_shr_unsigned_signed_helper::<u64, i8>();
    arithmetic_checked_shr_unsigned_signed_helper::<u64, i16>();
    arithmetic_checked_shr_unsigned_signed_helper::<u64, i32>();
    arithmetic_checked_shr_unsigned_signed_helper::<u64, i64>();
    arithmetic_checked_shr_unsigned_signed_helper::<u64, isize>();
    arithmetic_checked_shr_unsigned_signed_helper::<usize, i8>();
    arithmetic_checked_shr_unsigned_signed_helper::<usize, i16>();
    arithmetic_checked_shr_unsigned_signed_helper::<usize, i32>();
    arithmetic_checked_shr_unsigned_signed_helper::<usize, i64>();
    arithmetic_checked_shr_unsigned_signed_helper::<usize, isize>();

    arithmetic_checked_shr_signed_signed_helper::<i8, i8>();
    arithmetic_checked_shr_signed_signed_helper::<i8, i16>();
    arithmetic_checked_shr_signed_signed_helper::<i8, i32>();
    arithmetic_checked_shr_signed_signed_helper::<i8, i64>();
    arithmetic_checked_shr_signed_signed_helper::<i8, isize>();
    arithmetic_checked_shr_signed_signed_helper::<i16, i8>();
    arithmetic_checked_shr_signed_signed_helper::<i16, i16>();
    arithmetic_checked_shr_signed_signed_helper::<i16, i32>();
    arithmetic_checked_shr_signed_signed_helper::<i16, i64>();
    arithmetic_checked_shr_signed_signed_helper::<i16, isize>();
    arithmetic_checked_shr_signed_signed_helper::<i32, i8>();
    arithmetic_checked_shr_signed_signed_helper::<i32, i16>();
    arithmetic_checked_shr_signed_signed_helper::<i32, i32>();
    arithmetic_checked_shr_signed_signed_helper::<i32, i64>();
    arithmetic_checked_shr_signed_signed_helper::<i32, isize>();
    arithmetic_checked_shr_signed_signed_helper::<i64, i8>();
    arithmetic_checked_shr_signed_signed_helper::<i64, i16>();
    arithmetic_checked_shr_signed_signed_helper::<i64, i32>();
    arithmetic_checked_shr_signed_signed_helper::<i64, i64>();
    arithmetic_checked_shr_signed_signed_helper::<i64, isize>();
    arithmetic_checked_shr_signed_signed_helper::<isize, i8>();
    arithmetic_checked_shr_signed_signed_helper::<isize, i16>();
    arithmetic_checked_shr_signed_signed_helper::<isize, i32>();
    arithmetic_checked_shr_signed_signed_helper::<isize, i64>();
    arithmetic_checked_shr_signed_signed_helper::<isize, isize>();
}
