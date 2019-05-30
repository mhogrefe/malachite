use std::fmt::Debug;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{CheckedFrom, ConvertibleFrom, WrappingFrom};
use rand::Rand;

use common::test_properties;
use malachite_test::inputs::base::{signeds, unsigneds};

fn convertible_from_properties_helper_unsigned<T: PrimitiveUnsigned + Rand, U: Debug + Eq>()
where
    U: CheckedFrom<T> + ConvertibleFrom<T>,
{
    test_properties(unsigneds, |&u| {
        let convertible = U::convertible_from(u);
        assert_eq!(convertible, U::checked_from(u).is_some())
    });
}

fn convertible_from_properties_helper_signed<T: PrimitiveSigned + Rand, U: Debug + Eq>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    U: CheckedFrom<T> + ConvertibleFrom<T>,
{
    test_properties(signeds, |&i| {
        let convertible = U::convertible_from(i);
        assert_eq!(convertible, U::checked_from(i).is_some())
    });
}

#[test]
fn convertible_from_properties() {
    convertible_from_properties_helper_unsigned::<u8, u8>();
    convertible_from_properties_helper_unsigned::<u8, u16>();
    convertible_from_properties_helper_unsigned::<u8, u32>();
    convertible_from_properties_helper_unsigned::<u8, u64>();
    convertible_from_properties_helper_unsigned::<u8, u128>();
    convertible_from_properties_helper_unsigned::<u8, usize>();
    convertible_from_properties_helper_unsigned::<u8, i8>();
    convertible_from_properties_helper_unsigned::<u8, i16>();
    convertible_from_properties_helper_unsigned::<u8, i32>();
    convertible_from_properties_helper_unsigned::<u8, i64>();
    convertible_from_properties_helper_unsigned::<u8, i128>();
    convertible_from_properties_helper_unsigned::<u8, isize>();
    convertible_from_properties_helper_unsigned::<u16, u8>();
    convertible_from_properties_helper_unsigned::<u16, u16>();
    convertible_from_properties_helper_unsigned::<u16, u32>();
    convertible_from_properties_helper_unsigned::<u16, u64>();
    convertible_from_properties_helper_unsigned::<u16, u128>();
    convertible_from_properties_helper_unsigned::<u16, usize>();
    convertible_from_properties_helper_unsigned::<u16, i8>();
    convertible_from_properties_helper_unsigned::<u16, i16>();
    convertible_from_properties_helper_unsigned::<u16, i32>();
    convertible_from_properties_helper_unsigned::<u16, i64>();
    convertible_from_properties_helper_unsigned::<u16, i128>();
    convertible_from_properties_helper_unsigned::<u16, isize>();
    convertible_from_properties_helper_unsigned::<u32, u8>();
    convertible_from_properties_helper_unsigned::<u32, u16>();
    convertible_from_properties_helper_unsigned::<u32, u32>();
    convertible_from_properties_helper_unsigned::<u32, u64>();
    convertible_from_properties_helper_unsigned::<u32, u128>();
    convertible_from_properties_helper_unsigned::<u32, usize>();
    convertible_from_properties_helper_unsigned::<u32, i8>();
    convertible_from_properties_helper_unsigned::<u32, i16>();
    convertible_from_properties_helper_unsigned::<u32, i32>();
    convertible_from_properties_helper_unsigned::<u32, i64>();
    convertible_from_properties_helper_unsigned::<u32, i128>();
    convertible_from_properties_helper_unsigned::<u32, isize>();
    convertible_from_properties_helper_unsigned::<u64, u8>();
    convertible_from_properties_helper_unsigned::<u64, u16>();
    convertible_from_properties_helper_unsigned::<u64, u32>();
    convertible_from_properties_helper_unsigned::<u64, u64>();
    convertible_from_properties_helper_unsigned::<u64, u128>();
    convertible_from_properties_helper_unsigned::<u64, usize>();
    convertible_from_properties_helper_unsigned::<u64, i8>();
    convertible_from_properties_helper_unsigned::<u64, i16>();
    convertible_from_properties_helper_unsigned::<u64, i32>();
    convertible_from_properties_helper_unsigned::<u64, i64>();
    convertible_from_properties_helper_unsigned::<u64, i128>();
    convertible_from_properties_helper_unsigned::<u64, isize>();
    convertible_from_properties_helper_unsigned::<usize, u8>();
    convertible_from_properties_helper_unsigned::<usize, u16>();
    convertible_from_properties_helper_unsigned::<usize, u32>();
    convertible_from_properties_helper_unsigned::<usize, u64>();
    convertible_from_properties_helper_unsigned::<usize, u128>();
    convertible_from_properties_helper_unsigned::<usize, usize>();
    convertible_from_properties_helper_unsigned::<usize, i8>();
    convertible_from_properties_helper_unsigned::<usize, i16>();
    convertible_from_properties_helper_unsigned::<usize, i32>();
    convertible_from_properties_helper_unsigned::<usize, i64>();
    convertible_from_properties_helper_unsigned::<usize, i128>();
    convertible_from_properties_helper_unsigned::<usize, isize>();

    convertible_from_properties_helper_signed::<i8, u8>();
    convertible_from_properties_helper_signed::<i8, u16>();
    convertible_from_properties_helper_signed::<i8, u32>();
    convertible_from_properties_helper_signed::<i8, u64>();
    convertible_from_properties_helper_signed::<i8, u128>();
    convertible_from_properties_helper_signed::<i8, usize>();
    convertible_from_properties_helper_signed::<i8, i8>();
    convertible_from_properties_helper_signed::<i8, i16>();
    convertible_from_properties_helper_signed::<i8, i32>();
    convertible_from_properties_helper_signed::<i8, i64>();
    convertible_from_properties_helper_signed::<i8, i128>();
    convertible_from_properties_helper_signed::<i8, isize>();
    convertible_from_properties_helper_signed::<i16, u8>();
    convertible_from_properties_helper_signed::<i16, u16>();
    convertible_from_properties_helper_signed::<i16, u32>();
    convertible_from_properties_helper_signed::<i16, u64>();
    convertible_from_properties_helper_signed::<i16, u128>();
    convertible_from_properties_helper_signed::<i16, usize>();
    convertible_from_properties_helper_signed::<i16, i8>();
    convertible_from_properties_helper_signed::<i16, i16>();
    convertible_from_properties_helper_signed::<i16, i32>();
    convertible_from_properties_helper_signed::<i16, i64>();
    convertible_from_properties_helper_signed::<i16, i128>();
    convertible_from_properties_helper_signed::<i16, isize>();
    convertible_from_properties_helper_signed::<i32, u8>();
    convertible_from_properties_helper_signed::<i32, u16>();
    convertible_from_properties_helper_signed::<i32, u32>();
    convertible_from_properties_helper_signed::<i32, u64>();
    convertible_from_properties_helper_signed::<i32, u128>();
    convertible_from_properties_helper_signed::<i32, usize>();
    convertible_from_properties_helper_signed::<i32, i8>();
    convertible_from_properties_helper_signed::<i32, i16>();
    convertible_from_properties_helper_signed::<i32, i32>();
    convertible_from_properties_helper_signed::<i32, i64>();
    convertible_from_properties_helper_signed::<i32, i128>();
    convertible_from_properties_helper_signed::<i32, isize>();
    convertible_from_properties_helper_signed::<i64, u8>();
    convertible_from_properties_helper_signed::<i64, u16>();
    convertible_from_properties_helper_signed::<i64, u32>();
    convertible_from_properties_helper_signed::<i64, u64>();
    convertible_from_properties_helper_signed::<i64, u128>();
    convertible_from_properties_helper_signed::<i64, usize>();
    convertible_from_properties_helper_signed::<i64, i8>();
    convertible_from_properties_helper_signed::<i64, i16>();
    convertible_from_properties_helper_signed::<i64, i32>();
    convertible_from_properties_helper_signed::<i64, i64>();
    convertible_from_properties_helper_signed::<i64, i128>();
    convertible_from_properties_helper_signed::<i64, isize>();
    convertible_from_properties_helper_signed::<isize, u8>();
    convertible_from_properties_helper_signed::<isize, u16>();
    convertible_from_properties_helper_signed::<isize, u32>();
    convertible_from_properties_helper_signed::<isize, u64>();
    convertible_from_properties_helper_signed::<isize, u128>();
    convertible_from_properties_helper_signed::<isize, usize>();
    convertible_from_properties_helper_signed::<isize, i8>();
    convertible_from_properties_helper_signed::<isize, i16>();
    convertible_from_properties_helper_signed::<isize, i32>();
    convertible_from_properties_helper_signed::<isize, i64>();
    convertible_from_properties_helper_signed::<isize, i128>();
    convertible_from_properties_helper_signed::<isize, isize>();
}
