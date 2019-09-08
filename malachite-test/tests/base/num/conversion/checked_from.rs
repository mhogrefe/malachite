use std::fmt::Debug;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{CheckedFrom, OverflowingFrom, WrappingFrom};
use rand::Rand;

use common::test_properties;
use malachite_test::inputs::base::{signeds, unsigneds};

fn checked_from_properties_helper_unsigned<T: PrimitiveUnsigned + Rand, U: Debug + Eq>()
where
    U: OverflowingFrom<T> + CheckedFrom<T>,
{
    test_properties(unsigneds, |&u| {
        let result = U::checked_from(u);
        assert_eq!(result.is_none(), U::overflowing_from(u).1);
    });
}

fn checked_from_properties_helper_signed<T: PrimitiveSigned + Rand, U: Debug + Eq>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    U: OverflowingFrom<T> + CheckedFrom<T>,
{
    test_properties(signeds, |&i| {
        let result = U::checked_from(i);
        assert_eq!(result.is_none(), U::overflowing_from(i).1);
    });
}

#[test]
fn checked_from_properties() {
    checked_from_properties_helper_unsigned::<u8, u8>();
    checked_from_properties_helper_unsigned::<u8, u16>();
    checked_from_properties_helper_unsigned::<u8, u32>();
    checked_from_properties_helper_unsigned::<u8, u64>();
    checked_from_properties_helper_unsigned::<u8, u128>();
    checked_from_properties_helper_unsigned::<u8, usize>();
    checked_from_properties_helper_unsigned::<u8, i8>();
    checked_from_properties_helper_unsigned::<u8, i16>();
    checked_from_properties_helper_unsigned::<u8, i32>();
    checked_from_properties_helper_unsigned::<u8, i64>();
    checked_from_properties_helper_unsigned::<u8, i128>();
    checked_from_properties_helper_unsigned::<u8, isize>();
    checked_from_properties_helper_unsigned::<u16, u8>();
    checked_from_properties_helper_unsigned::<u16, u16>();
    checked_from_properties_helper_unsigned::<u16, u32>();
    checked_from_properties_helper_unsigned::<u16, u64>();
    checked_from_properties_helper_unsigned::<u16, u128>();
    checked_from_properties_helper_unsigned::<u16, usize>();
    checked_from_properties_helper_unsigned::<u16, i8>();
    checked_from_properties_helper_unsigned::<u16, i16>();
    checked_from_properties_helper_unsigned::<u16, i32>();
    checked_from_properties_helper_unsigned::<u16, i64>();
    checked_from_properties_helper_unsigned::<u16, i128>();
    checked_from_properties_helper_unsigned::<u16, isize>();
    checked_from_properties_helper_unsigned::<u32, u8>();
    checked_from_properties_helper_unsigned::<u32, u16>();
    checked_from_properties_helper_unsigned::<u32, u32>();
    checked_from_properties_helper_unsigned::<u32, u64>();
    checked_from_properties_helper_unsigned::<u32, u128>();
    checked_from_properties_helper_unsigned::<u32, usize>();
    checked_from_properties_helper_unsigned::<u32, i8>();
    checked_from_properties_helper_unsigned::<u32, i16>();
    checked_from_properties_helper_unsigned::<u32, i32>();
    checked_from_properties_helper_unsigned::<u32, i64>();
    checked_from_properties_helper_unsigned::<u32, i128>();
    checked_from_properties_helper_unsigned::<u32, isize>();
    checked_from_properties_helper_unsigned::<u64, u8>();
    checked_from_properties_helper_unsigned::<u64, u16>();
    checked_from_properties_helper_unsigned::<u64, u32>();
    checked_from_properties_helper_unsigned::<u64, u64>();
    checked_from_properties_helper_unsigned::<u64, u128>();
    checked_from_properties_helper_unsigned::<u64, usize>();
    checked_from_properties_helper_unsigned::<u64, i8>();
    checked_from_properties_helper_unsigned::<u64, i16>();
    checked_from_properties_helper_unsigned::<u64, i32>();
    checked_from_properties_helper_unsigned::<u64, i64>();
    checked_from_properties_helper_unsigned::<u64, i128>();
    checked_from_properties_helper_unsigned::<u64, isize>();
    checked_from_properties_helper_unsigned::<usize, u8>();
    checked_from_properties_helper_unsigned::<usize, u16>();
    checked_from_properties_helper_unsigned::<usize, u32>();
    checked_from_properties_helper_unsigned::<usize, u64>();
    checked_from_properties_helper_unsigned::<usize, u128>();
    checked_from_properties_helper_unsigned::<usize, usize>();
    checked_from_properties_helper_unsigned::<usize, i8>();
    checked_from_properties_helper_unsigned::<usize, i16>();
    checked_from_properties_helper_unsigned::<usize, i32>();
    checked_from_properties_helper_unsigned::<usize, i64>();
    checked_from_properties_helper_unsigned::<usize, i128>();
    checked_from_properties_helper_unsigned::<usize, isize>();

    checked_from_properties_helper_signed::<i8, u8>();
    checked_from_properties_helper_signed::<i8, u16>();
    checked_from_properties_helper_signed::<i8, u32>();
    checked_from_properties_helper_signed::<i8, u64>();
    checked_from_properties_helper_signed::<i8, u128>();
    checked_from_properties_helper_signed::<i8, usize>();
    checked_from_properties_helper_signed::<i8, i8>();
    checked_from_properties_helper_signed::<i8, i16>();
    checked_from_properties_helper_signed::<i8, i32>();
    checked_from_properties_helper_signed::<i8, i64>();
    checked_from_properties_helper_signed::<i8, i128>();
    checked_from_properties_helper_signed::<i8, isize>();
    checked_from_properties_helper_signed::<i16, u8>();
    checked_from_properties_helper_signed::<i16, u16>();
    checked_from_properties_helper_signed::<i16, u32>();
    checked_from_properties_helper_signed::<i16, u64>();
    checked_from_properties_helper_signed::<i16, u128>();
    checked_from_properties_helper_signed::<i16, usize>();
    checked_from_properties_helper_signed::<i16, i8>();
    checked_from_properties_helper_signed::<i16, i16>();
    checked_from_properties_helper_signed::<i16, i32>();
    checked_from_properties_helper_signed::<i16, i64>();
    checked_from_properties_helper_signed::<i16, i128>();
    checked_from_properties_helper_signed::<i16, isize>();
    checked_from_properties_helper_signed::<i32, u8>();
    checked_from_properties_helper_signed::<i32, u16>();
    checked_from_properties_helper_signed::<i32, u32>();
    checked_from_properties_helper_signed::<i32, u64>();
    checked_from_properties_helper_signed::<i32, u128>();
    checked_from_properties_helper_signed::<i32, usize>();
    checked_from_properties_helper_signed::<i32, i8>();
    checked_from_properties_helper_signed::<i32, i16>();
    checked_from_properties_helper_signed::<i32, i32>();
    checked_from_properties_helper_signed::<i32, i64>();
    checked_from_properties_helper_signed::<i32, i128>();
    checked_from_properties_helper_signed::<i32, isize>();
    checked_from_properties_helper_signed::<i64, u8>();
    checked_from_properties_helper_signed::<i64, u16>();
    checked_from_properties_helper_signed::<i64, u32>();
    checked_from_properties_helper_signed::<i64, u64>();
    checked_from_properties_helper_signed::<i64, u128>();
    checked_from_properties_helper_signed::<i64, usize>();
    checked_from_properties_helper_signed::<i64, i8>();
    checked_from_properties_helper_signed::<i64, i16>();
    checked_from_properties_helper_signed::<i64, i32>();
    checked_from_properties_helper_signed::<i64, i64>();
    checked_from_properties_helper_signed::<i64, i128>();
    checked_from_properties_helper_signed::<i64, isize>();
    checked_from_properties_helper_signed::<isize, u8>();
    checked_from_properties_helper_signed::<isize, u16>();
    checked_from_properties_helper_signed::<isize, u32>();
    checked_from_properties_helper_signed::<isize, u64>();
    checked_from_properties_helper_signed::<isize, u128>();
    checked_from_properties_helper_signed::<isize, usize>();
    checked_from_properties_helper_signed::<isize, i8>();
    checked_from_properties_helper_signed::<isize, i16>();
    checked_from_properties_helper_signed::<isize, i32>();
    checked_from_properties_helper_signed::<isize, i64>();
    checked_from_properties_helper_signed::<isize, i128>();
    checked_from_properties_helper_signed::<isize, isize>();
}
