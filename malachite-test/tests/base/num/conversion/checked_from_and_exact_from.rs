use std::fmt::Debug;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ExactFrom, OverflowingFrom, WrappingFrom,
};
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{signeds, unsigneds};

fn properties_helper_unsigned<T: PrimitiveUnsigned + Rand, U: Debug + Eq>()
where
    T: ExactFrom<U>,
    U: OverflowingFrom<T> + CheckedFrom<T> + ExactFrom<T>,
{
    test_properties(unsigneds, |&u| {
        let result = U::checked_from(u);
        assert_eq!(result.is_none(), U::overflowing_from(u).1);
        if let Some(x) = result {
            assert_eq!(x, U::exact_from(u));
            assert_eq!(u, T::exact_from(x));
        }
    });
}

fn properties_helper_signed<T: PrimitiveSigned + Rand, U: Debug + Eq>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth> + ExactFrom<U>,
    U: OverflowingFrom<T> + CheckedFrom<T> + ExactFrom<T>,
{
    test_properties(signeds, |&i| {
        let result = U::checked_from(i);
        assert_eq!(result.is_none(), U::overflowing_from(i).1);
        if let Some(x) = result {
            assert_eq!(x, U::exact_from(i));
            assert_eq!(i, T::exact_from(x));
        }
    });
}

#[test]
fn checked_from_properties() {
    properties_helper_unsigned::<u8, u8>();
    properties_helper_unsigned::<u8, u16>();
    properties_helper_unsigned::<u8, u32>();
    properties_helper_unsigned::<u8, u64>();
    properties_helper_unsigned::<u8, u128>();
    properties_helper_unsigned::<u8, usize>();
    properties_helper_unsigned::<u8, i8>();
    properties_helper_unsigned::<u8, i16>();
    properties_helper_unsigned::<u8, i32>();
    properties_helper_unsigned::<u8, i64>();
    properties_helper_unsigned::<u8, i128>();
    properties_helper_unsigned::<u8, isize>();
    properties_helper_unsigned::<u16, u8>();
    properties_helper_unsigned::<u16, u16>();
    properties_helper_unsigned::<u16, u32>();
    properties_helper_unsigned::<u16, u64>();
    properties_helper_unsigned::<u16, u128>();
    properties_helper_unsigned::<u16, usize>();
    properties_helper_unsigned::<u16, i8>();
    properties_helper_unsigned::<u16, i16>();
    properties_helper_unsigned::<u16, i32>();
    properties_helper_unsigned::<u16, i64>();
    properties_helper_unsigned::<u16, i128>();
    properties_helper_unsigned::<u16, isize>();
    properties_helper_unsigned::<u32, u8>();
    properties_helper_unsigned::<u32, u16>();
    properties_helper_unsigned::<u32, u32>();
    properties_helper_unsigned::<u32, u64>();
    properties_helper_unsigned::<u32, u128>();
    properties_helper_unsigned::<u32, usize>();
    properties_helper_unsigned::<u32, i8>();
    properties_helper_unsigned::<u32, i16>();
    properties_helper_unsigned::<u32, i32>();
    properties_helper_unsigned::<u32, i64>();
    properties_helper_unsigned::<u32, i128>();
    properties_helper_unsigned::<u32, isize>();
    properties_helper_unsigned::<u64, u8>();
    properties_helper_unsigned::<u64, u16>();
    properties_helper_unsigned::<u64, u32>();
    properties_helper_unsigned::<u64, u64>();
    properties_helper_unsigned::<u64, u128>();
    properties_helper_unsigned::<u64, usize>();
    properties_helper_unsigned::<u64, i8>();
    properties_helper_unsigned::<u64, i16>();
    properties_helper_unsigned::<u64, i32>();
    properties_helper_unsigned::<u64, i64>();
    properties_helper_unsigned::<u64, i128>();
    properties_helper_unsigned::<u64, isize>();
    properties_helper_unsigned::<usize, u8>();
    properties_helper_unsigned::<usize, u16>();
    properties_helper_unsigned::<usize, u32>();
    properties_helper_unsigned::<usize, u64>();
    properties_helper_unsigned::<usize, u128>();
    properties_helper_unsigned::<usize, usize>();
    properties_helper_unsigned::<usize, i8>();
    properties_helper_unsigned::<usize, i16>();
    properties_helper_unsigned::<usize, i32>();
    properties_helper_unsigned::<usize, i64>();
    properties_helper_unsigned::<usize, i128>();
    properties_helper_unsigned::<usize, isize>();

    properties_helper_signed::<i8, u8>();
    properties_helper_signed::<i8, u16>();
    properties_helper_signed::<i8, u32>();
    properties_helper_signed::<i8, u64>();
    properties_helper_signed::<i8, u128>();
    properties_helper_signed::<i8, usize>();
    properties_helper_signed::<i8, i8>();
    properties_helper_signed::<i8, i16>();
    properties_helper_signed::<i8, i32>();
    properties_helper_signed::<i8, i64>();
    properties_helper_signed::<i8, i128>();
    properties_helper_signed::<i8, isize>();
    properties_helper_signed::<i16, u8>();
    properties_helper_signed::<i16, u16>();
    properties_helper_signed::<i16, u32>();
    properties_helper_signed::<i16, u64>();
    properties_helper_signed::<i16, u128>();
    properties_helper_signed::<i16, usize>();
    properties_helper_signed::<i16, i8>();
    properties_helper_signed::<i16, i16>();
    properties_helper_signed::<i16, i32>();
    properties_helper_signed::<i16, i64>();
    properties_helper_signed::<i16, i128>();
    properties_helper_signed::<i16, isize>();
    properties_helper_signed::<i32, u8>();
    properties_helper_signed::<i32, u16>();
    properties_helper_signed::<i32, u32>();
    properties_helper_signed::<i32, u64>();
    properties_helper_signed::<i32, u128>();
    properties_helper_signed::<i32, usize>();
    properties_helper_signed::<i32, i8>();
    properties_helper_signed::<i32, i16>();
    properties_helper_signed::<i32, i32>();
    properties_helper_signed::<i32, i64>();
    properties_helper_signed::<i32, i128>();
    properties_helper_signed::<i32, isize>();
    properties_helper_signed::<i64, u8>();
    properties_helper_signed::<i64, u16>();
    properties_helper_signed::<i64, u32>();
    properties_helper_signed::<i64, u64>();
    properties_helper_signed::<i64, u128>();
    properties_helper_signed::<i64, usize>();
    properties_helper_signed::<i64, i8>();
    properties_helper_signed::<i64, i16>();
    properties_helper_signed::<i64, i32>();
    properties_helper_signed::<i64, i64>();
    properties_helper_signed::<i64, i128>();
    properties_helper_signed::<i64, isize>();
    properties_helper_signed::<isize, u8>();
    properties_helper_signed::<isize, u16>();
    properties_helper_signed::<isize, u32>();
    properties_helper_signed::<isize, u64>();
    properties_helper_signed::<isize, u128>();
    properties_helper_signed::<isize, usize>();
    properties_helper_signed::<isize, i8>();
    properties_helper_signed::<isize, i16>();
    properties_helper_signed::<isize, i32>();
    properties_helper_signed::<isize, i64>();
    properties_helper_signed::<isize, i128>();
    properties_helper_signed::<isize, isize>();
}
