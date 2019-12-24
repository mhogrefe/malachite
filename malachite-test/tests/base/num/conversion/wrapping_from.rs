use std::fmt::Debug;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{OverflowingFrom, WrappingFrom};
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{signeds, unsigneds};

fn wrapping_from_properties_helper_unsigned<T: PrimitiveUnsigned + Rand, U: Debug + Eq>()
where
    U: OverflowingFrom<T> + WrappingFrom<T>,
{
    test_properties(unsigneds, |&u| {
        let result = U::wrapping_from(u);
        assert_eq!(result, U::overflowing_from(u).0)
    });
}

fn wrapping_from_properties_helper_signed<T: PrimitiveSigned + Rand, U: Debug + Eq>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    U: OverflowingFrom<T> + WrappingFrom<T>,
{
    test_properties(signeds, |&i| {
        let result = U::wrapping_from(i);
        assert_eq!(result, U::overflowing_from(i).0)
    });
}

#[test]
fn wrapping_from_properties() {
    wrapping_from_properties_helper_unsigned::<u8, u8>();
    wrapping_from_properties_helper_unsigned::<u8, u16>();
    wrapping_from_properties_helper_unsigned::<u8, u32>();
    wrapping_from_properties_helper_unsigned::<u8, u64>();
    wrapping_from_properties_helper_unsigned::<u8, u128>();
    wrapping_from_properties_helper_unsigned::<u8, usize>();
    wrapping_from_properties_helper_unsigned::<u8, i8>();
    wrapping_from_properties_helper_unsigned::<u8, i16>();
    wrapping_from_properties_helper_unsigned::<u8, i32>();
    wrapping_from_properties_helper_unsigned::<u8, i64>();
    wrapping_from_properties_helper_unsigned::<u8, i128>();
    wrapping_from_properties_helper_unsigned::<u8, isize>();
    wrapping_from_properties_helper_unsigned::<u16, u8>();
    wrapping_from_properties_helper_unsigned::<u16, u16>();
    wrapping_from_properties_helper_unsigned::<u16, u32>();
    wrapping_from_properties_helper_unsigned::<u16, u64>();
    wrapping_from_properties_helper_unsigned::<u16, u128>();
    wrapping_from_properties_helper_unsigned::<u16, usize>();
    wrapping_from_properties_helper_unsigned::<u16, i8>();
    wrapping_from_properties_helper_unsigned::<u16, i16>();
    wrapping_from_properties_helper_unsigned::<u16, i32>();
    wrapping_from_properties_helper_unsigned::<u16, i64>();
    wrapping_from_properties_helper_unsigned::<u16, i128>();
    wrapping_from_properties_helper_unsigned::<u16, isize>();
    wrapping_from_properties_helper_unsigned::<u32, u8>();
    wrapping_from_properties_helper_unsigned::<u32, u16>();
    wrapping_from_properties_helper_unsigned::<u32, u32>();
    wrapping_from_properties_helper_unsigned::<u32, u64>();
    wrapping_from_properties_helper_unsigned::<u32, u128>();
    wrapping_from_properties_helper_unsigned::<u32, usize>();
    wrapping_from_properties_helper_unsigned::<u32, i8>();
    wrapping_from_properties_helper_unsigned::<u32, i16>();
    wrapping_from_properties_helper_unsigned::<u32, i32>();
    wrapping_from_properties_helper_unsigned::<u32, i64>();
    wrapping_from_properties_helper_unsigned::<u32, i128>();
    wrapping_from_properties_helper_unsigned::<u32, isize>();
    wrapping_from_properties_helper_unsigned::<u64, u8>();
    wrapping_from_properties_helper_unsigned::<u64, u16>();
    wrapping_from_properties_helper_unsigned::<u64, u32>();
    wrapping_from_properties_helper_unsigned::<u64, u64>();
    wrapping_from_properties_helper_unsigned::<u64, u128>();
    wrapping_from_properties_helper_unsigned::<u64, usize>();
    wrapping_from_properties_helper_unsigned::<u64, i8>();
    wrapping_from_properties_helper_unsigned::<u64, i16>();
    wrapping_from_properties_helper_unsigned::<u64, i32>();
    wrapping_from_properties_helper_unsigned::<u64, i64>();
    wrapping_from_properties_helper_unsigned::<u64, i128>();
    wrapping_from_properties_helper_unsigned::<u64, isize>();
    wrapping_from_properties_helper_unsigned::<usize, u8>();
    wrapping_from_properties_helper_unsigned::<usize, u16>();
    wrapping_from_properties_helper_unsigned::<usize, u32>();
    wrapping_from_properties_helper_unsigned::<usize, u64>();
    wrapping_from_properties_helper_unsigned::<usize, u128>();
    wrapping_from_properties_helper_unsigned::<usize, usize>();
    wrapping_from_properties_helper_unsigned::<usize, i8>();
    wrapping_from_properties_helper_unsigned::<usize, i16>();
    wrapping_from_properties_helper_unsigned::<usize, i32>();
    wrapping_from_properties_helper_unsigned::<usize, i64>();
    wrapping_from_properties_helper_unsigned::<usize, i128>();
    wrapping_from_properties_helper_unsigned::<usize, isize>();

    wrapping_from_properties_helper_signed::<i8, u8>();
    wrapping_from_properties_helper_signed::<i8, u16>();
    wrapping_from_properties_helper_signed::<i8, u32>();
    wrapping_from_properties_helper_signed::<i8, u64>();
    wrapping_from_properties_helper_signed::<i8, u128>();
    wrapping_from_properties_helper_signed::<i8, usize>();
    wrapping_from_properties_helper_signed::<i8, i8>();
    wrapping_from_properties_helper_signed::<i8, i16>();
    wrapping_from_properties_helper_signed::<i8, i32>();
    wrapping_from_properties_helper_signed::<i8, i64>();
    wrapping_from_properties_helper_signed::<i8, i128>();
    wrapping_from_properties_helper_signed::<i8, isize>();
    wrapping_from_properties_helper_signed::<i16, u8>();
    wrapping_from_properties_helper_signed::<i16, u16>();
    wrapping_from_properties_helper_signed::<i16, u32>();
    wrapping_from_properties_helper_signed::<i16, u64>();
    wrapping_from_properties_helper_signed::<i16, u128>();
    wrapping_from_properties_helper_signed::<i16, usize>();
    wrapping_from_properties_helper_signed::<i16, i8>();
    wrapping_from_properties_helper_signed::<i16, i16>();
    wrapping_from_properties_helper_signed::<i16, i32>();
    wrapping_from_properties_helper_signed::<i16, i64>();
    wrapping_from_properties_helper_signed::<i16, i128>();
    wrapping_from_properties_helper_signed::<i16, isize>();
    wrapping_from_properties_helper_signed::<i32, u8>();
    wrapping_from_properties_helper_signed::<i32, u16>();
    wrapping_from_properties_helper_signed::<i32, u32>();
    wrapping_from_properties_helper_signed::<i32, u64>();
    wrapping_from_properties_helper_signed::<i32, u128>();
    wrapping_from_properties_helper_signed::<i32, usize>();
    wrapping_from_properties_helper_signed::<i32, i8>();
    wrapping_from_properties_helper_signed::<i32, i16>();
    wrapping_from_properties_helper_signed::<i32, i32>();
    wrapping_from_properties_helper_signed::<i32, i64>();
    wrapping_from_properties_helper_signed::<i32, i128>();
    wrapping_from_properties_helper_signed::<i32, isize>();
    wrapping_from_properties_helper_signed::<i64, u8>();
    wrapping_from_properties_helper_signed::<i64, u16>();
    wrapping_from_properties_helper_signed::<i64, u32>();
    wrapping_from_properties_helper_signed::<i64, u64>();
    wrapping_from_properties_helper_signed::<i64, u128>();
    wrapping_from_properties_helper_signed::<i64, usize>();
    wrapping_from_properties_helper_signed::<i64, i8>();
    wrapping_from_properties_helper_signed::<i64, i16>();
    wrapping_from_properties_helper_signed::<i64, i32>();
    wrapping_from_properties_helper_signed::<i64, i64>();
    wrapping_from_properties_helper_signed::<i64, i128>();
    wrapping_from_properties_helper_signed::<i64, isize>();
    wrapping_from_properties_helper_signed::<isize, u8>();
    wrapping_from_properties_helper_signed::<isize, u16>();
    wrapping_from_properties_helper_signed::<isize, u32>();
    wrapping_from_properties_helper_signed::<isize, u64>();
    wrapping_from_properties_helper_signed::<isize, u128>();
    wrapping_from_properties_helper_signed::<isize, usize>();
    wrapping_from_properties_helper_signed::<isize, i8>();
    wrapping_from_properties_helper_signed::<isize, i16>();
    wrapping_from_properties_helper_signed::<isize, i32>();
    wrapping_from_properties_helper_signed::<isize, i64>();
    wrapping_from_properties_helper_signed::<isize, i128>();
    wrapping_from_properties_helper_signed::<isize, isize>();
}
