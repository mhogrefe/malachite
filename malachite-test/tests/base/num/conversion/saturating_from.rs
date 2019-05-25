use std::fmt::Debug;

use malachite_base::conversion::{CheckedFrom, ConvertibleFrom, SaturatingFrom, WrappingFrom};
use malachite_base::num::signeds::PrimitiveSigned;
use malachite_base::num::unsigneds::PrimitiveUnsigned;
use rand::Rand;

use common::test_properties;
use malachite_test::inputs::base::{signeds, unsigneds};

fn saturating_from_properties_helper_unsigned<T: PrimitiveUnsigned + Rand, U: Debug + Eq>()
where
    T: CheckedFrom<U>,
    U: CheckedFrom<T> + ConvertibleFrom<T> + SaturatingFrom<T>,
{
    test_properties(unsigneds, |&u| {
        let result = U::saturating_from(u);
        if let Some(u_u) = U::checked_from(u) {
            assert_eq!(result, u_u);
        }
        if let Some(result_t) = T::checked_from(result) {
            assert!(result_t.le_abs(&u));
            assert_eq!(result_t == u, U::convertible_from(u));
        }
    });
}

fn saturating_from_properties_helper_signed<T: PrimitiveSigned + Rand, U: Debug + Eq>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: CheckedFrom<U> + WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    U: CheckedFrom<T> + ConvertibleFrom<T> + SaturatingFrom<T>,
{
    test_properties(signeds, |&i| {
        let result = U::saturating_from(i);
        if let Some(i_u) = U::checked_from(i) {
            assert_eq!(result, i_u);
        }
        if let Some(result_t) = T::checked_from(result) {
            assert!(result_t.le_abs(&i));
            assert_eq!(result_t == i, U::convertible_from(i));
        }
    });
}

#[test]
fn saturating_from_properties() {
    saturating_from_properties_helper_unsigned::<u8, u8>();
    saturating_from_properties_helper_unsigned::<u8, u16>();
    saturating_from_properties_helper_unsigned::<u8, u32>();
    saturating_from_properties_helper_unsigned::<u8, u64>();
    saturating_from_properties_helper_unsigned::<u8, u128>();
    saturating_from_properties_helper_unsigned::<u8, usize>();
    saturating_from_properties_helper_unsigned::<u8, i8>();
    saturating_from_properties_helper_unsigned::<u8, i16>();
    saturating_from_properties_helper_unsigned::<u8, i32>();
    saturating_from_properties_helper_unsigned::<u8, i64>();
    saturating_from_properties_helper_unsigned::<u8, i128>();
    saturating_from_properties_helper_unsigned::<u8, isize>();
    saturating_from_properties_helper_unsigned::<u16, u8>();
    saturating_from_properties_helper_unsigned::<u16, u16>();
    saturating_from_properties_helper_unsigned::<u16, u32>();
    saturating_from_properties_helper_unsigned::<u16, u64>();
    saturating_from_properties_helper_unsigned::<u16, u128>();
    saturating_from_properties_helper_unsigned::<u16, usize>();
    saturating_from_properties_helper_unsigned::<u16, i8>();
    saturating_from_properties_helper_unsigned::<u16, i16>();
    saturating_from_properties_helper_unsigned::<u16, i32>();
    saturating_from_properties_helper_unsigned::<u16, i64>();
    saturating_from_properties_helper_unsigned::<u16, i128>();
    saturating_from_properties_helper_unsigned::<u16, isize>();
    saturating_from_properties_helper_unsigned::<u32, u8>();
    saturating_from_properties_helper_unsigned::<u32, u16>();
    saturating_from_properties_helper_unsigned::<u32, u32>();
    saturating_from_properties_helper_unsigned::<u32, u64>();
    saturating_from_properties_helper_unsigned::<u32, u128>();
    saturating_from_properties_helper_unsigned::<u32, usize>();
    saturating_from_properties_helper_unsigned::<u32, i8>();
    saturating_from_properties_helper_unsigned::<u32, i16>();
    saturating_from_properties_helper_unsigned::<u32, i32>();
    saturating_from_properties_helper_unsigned::<u32, i64>();
    saturating_from_properties_helper_unsigned::<u32, i128>();
    saturating_from_properties_helper_unsigned::<u32, isize>();
    saturating_from_properties_helper_unsigned::<u64, u8>();
    saturating_from_properties_helper_unsigned::<u64, u16>();
    saturating_from_properties_helper_unsigned::<u64, u32>();
    saturating_from_properties_helper_unsigned::<u64, u64>();
    saturating_from_properties_helper_unsigned::<u64, u128>();
    saturating_from_properties_helper_unsigned::<u64, usize>();
    saturating_from_properties_helper_unsigned::<u64, i8>();
    saturating_from_properties_helper_unsigned::<u64, i16>();
    saturating_from_properties_helper_unsigned::<u64, i32>();
    saturating_from_properties_helper_unsigned::<u64, i64>();
    saturating_from_properties_helper_unsigned::<u64, i128>();
    saturating_from_properties_helper_unsigned::<u64, isize>();
    saturating_from_properties_helper_unsigned::<usize, u8>();
    saturating_from_properties_helper_unsigned::<usize, u16>();
    saturating_from_properties_helper_unsigned::<usize, u32>();
    saturating_from_properties_helper_unsigned::<usize, u64>();
    saturating_from_properties_helper_unsigned::<usize, u128>();
    saturating_from_properties_helper_unsigned::<usize, usize>();
    saturating_from_properties_helper_unsigned::<usize, i8>();
    saturating_from_properties_helper_unsigned::<usize, i16>();
    saturating_from_properties_helper_unsigned::<usize, i32>();
    saturating_from_properties_helper_unsigned::<usize, i64>();
    saturating_from_properties_helper_unsigned::<usize, i128>();
    saturating_from_properties_helper_unsigned::<usize, isize>();

    saturating_from_properties_helper_signed::<i8, u8>();
    saturating_from_properties_helper_signed::<i8, u16>();
    saturating_from_properties_helper_signed::<i8, u32>();
    saturating_from_properties_helper_signed::<i8, u64>();
    saturating_from_properties_helper_signed::<i8, u128>();
    saturating_from_properties_helper_signed::<i8, usize>();
    saturating_from_properties_helper_signed::<i8, i8>();
    saturating_from_properties_helper_signed::<i8, i16>();
    saturating_from_properties_helper_signed::<i8, i32>();
    saturating_from_properties_helper_signed::<i8, i64>();
    saturating_from_properties_helper_signed::<i8, i128>();
    saturating_from_properties_helper_signed::<i8, isize>();
    saturating_from_properties_helper_signed::<i16, u8>();
    saturating_from_properties_helper_signed::<i16, u16>();
    saturating_from_properties_helper_signed::<i16, u32>();
    saturating_from_properties_helper_signed::<i16, u64>();
    saturating_from_properties_helper_signed::<i16, u128>();
    saturating_from_properties_helper_signed::<i16, usize>();
    saturating_from_properties_helper_signed::<i16, i8>();
    saturating_from_properties_helper_signed::<i16, i16>();
    saturating_from_properties_helper_signed::<i16, i32>();
    saturating_from_properties_helper_signed::<i16, i64>();
    saturating_from_properties_helper_signed::<i16, i128>();
    saturating_from_properties_helper_signed::<i16, isize>();
    saturating_from_properties_helper_signed::<i32, u8>();
    saturating_from_properties_helper_signed::<i32, u16>();
    saturating_from_properties_helper_signed::<i32, u32>();
    saturating_from_properties_helper_signed::<i32, u64>();
    saturating_from_properties_helper_signed::<i32, u128>();
    saturating_from_properties_helper_signed::<i32, usize>();
    saturating_from_properties_helper_signed::<i32, i8>();
    saturating_from_properties_helper_signed::<i32, i16>();
    saturating_from_properties_helper_signed::<i32, i32>();
    saturating_from_properties_helper_signed::<i32, i64>();
    saturating_from_properties_helper_signed::<i32, i128>();
    saturating_from_properties_helper_signed::<i32, isize>();
    saturating_from_properties_helper_signed::<i64, u8>();
    saturating_from_properties_helper_signed::<i64, u16>();
    saturating_from_properties_helper_signed::<i64, u32>();
    saturating_from_properties_helper_signed::<i64, u64>();
    saturating_from_properties_helper_signed::<i64, u128>();
    saturating_from_properties_helper_signed::<i64, usize>();
    saturating_from_properties_helper_signed::<i64, i8>();
    saturating_from_properties_helper_signed::<i64, i16>();
    saturating_from_properties_helper_signed::<i64, i32>();
    saturating_from_properties_helper_signed::<i64, i64>();
    saturating_from_properties_helper_signed::<i64, i128>();
    saturating_from_properties_helper_signed::<i64, isize>();
    saturating_from_properties_helper_signed::<isize, u8>();
    saturating_from_properties_helper_signed::<isize, u16>();
    saturating_from_properties_helper_signed::<isize, u32>();
    saturating_from_properties_helper_signed::<isize, u64>();
    saturating_from_properties_helper_signed::<isize, u128>();
    saturating_from_properties_helper_signed::<isize, usize>();
    saturating_from_properties_helper_signed::<isize, i8>();
    saturating_from_properties_helper_signed::<isize, i16>();
    saturating_from_properties_helper_signed::<isize, i32>();
    saturating_from_properties_helper_signed::<isize, i64>();
    saturating_from_properties_helper_signed::<isize, i128>();
    saturating_from_properties_helper_signed::<isize, isize>();
}
