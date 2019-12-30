use std::fmt::Debug;
use std::{u16, u8};

use malachite_base::comparison::{Max, Min};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, FromOtherTypeSlice, JoinHalves, OverflowingFrom,
    SaturatingFrom, SplitInHalf, VecFromOtherTypeSlice, WrappingFrom,
};

#[test]
pub fn test_checked_from() {
    fn test_single<T: Copy + Debug + Eq>(n: T)
    where
        T: CheckedFrom<T>,
    {
        assert_eq!(T::checked_from(n), Some(n));
    };
    test_single(0u8);
    test_single(5u64);
    test_single(1_000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double<T, U: Debug + Eq>(n_in: T, n_out: Option<U>)
    where
        U: CheckedFrom<T>,
    {
        assert_eq!(U::checked_from(n_in), n_out);
    };
    test_double(0u8, Some(0u16));
    test_double(1_000u16, Some(1_000i32));
    test_double(-5i16, Some(-5i8));
    test_double(255u8, Some(255u64));

    test_double::<_, u32>(-1i8, None);
    test_double::<_, u16>(u32::MAX, None);
    test_double::<_, u32>(i32::MIN, None);
    test_double::<_, u16>(i32::MIN, None);
    test_double::<_, i16>(i32::MIN, None);
    test_double::<_, u32>(-5i32, None);
    test_double::<_, i32>(3_000_000_000u32, None);
    test_double::<_, i8>(-1000i16, None);
}

#[test]
pub fn test_exact_from() {
    fn test_single<T: Copy + Debug + Eq>(n: T)
    where
        T: ExactFrom<T>,
    {
        assert_eq!(T::exact_from(n), n);
    };
    test_single(0u8);
    test_single(5u64);
    test_single(1_000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double<T, U: Debug + Eq>(n_in: T, n_out: U)
    where
        U: ExactFrom<T>,
    {
        assert_eq!(U::exact_from(n_in), n_out);
    };
    test_double(0u8, 0u16);
    test_double(1_000u16, 1_000i32);
    test_double(-5i16, -5i8);
    test_double(255u8, 255u64);
}

#[test]
#[should_panic]
fn exact_from_fail_1() {
    u32::exact_from(-1i8);
}

#[test]
#[should_panic]
fn exact_from_fail_2() {
    u16::exact_from(u32::MAX);
}

#[test]
#[should_panic]
fn exact_from_fail_3() {
    u32::exact_from(i32::MIN);
}

#[test]
#[should_panic]
fn exact_from_fail_4() {
    u16::exact_from(i32::MIN);
}

#[test]
#[should_panic]
fn exact_from_fail_5() {
    i16::exact_from(i32::MIN);
}

#[test]
#[should_panic]
fn exact_from_fail_6() {
    u32::exact_from(-5i32);
}

#[test]
#[should_panic]
fn exact_from_fail_7() {
    i32::exact_from(3_000_000_000u32);
}

#[test]
#[should_panic]
fn exact_from_fail_8() {
    i8::exact_from(-1000i16);
}

#[test]
pub fn test_convertible_from() {
    fn test_single<T: Copy + Debug>(n: T)
    where
        T: ConvertibleFrom<T>,
    {
        assert!(T::convertible_from(n));
    };
    test_single(0u8);
    test_single(5u64);
    test_single(1_000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double<T, U>(n_in: T, convertible: bool)
    where
        U: ConvertibleFrom<T>,
    {
        assert_eq!(U::convertible_from(n_in), convertible);
    };
    test_double::<_, u16>(0u8, true);
    test_double::<_, i32>(1_000u16, true);
    test_double::<_, i8>(-5i16, true);
    test_double::<_, u64>(255u8, true);

    test_double::<_, u32>(-1i8, false);
    test_double::<_, u16>(u32::MAX, false);
    test_double::<_, u32>(i32::MIN, false);
    test_double::<_, u16>(i32::MIN, false);
    test_double::<_, i16>(i32::MIN, false);
    test_double::<_, u32>(-5i32, false);
    test_double::<_, i32>(3_000_000_000u32, false);
    test_double::<_, i8>(-1000i16, false);
}

#[test]
pub fn test_wrapping_from() {
    fn test_single<T: Copy + Debug + Eq>(n: T)
    where
        T: WrappingFrom<T>,
    {
        assert_eq!(T::wrapping_from(n), n);
    };
    test_single(0u8);
    test_single(5u64);
    test_single(1_000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double<T, U: Debug + Eq>(n_in: T, n_out: U)
    where
        U: WrappingFrom<T>,
    {
        assert_eq!(U::wrapping_from(n_in), n_out);
    };
    test_double(0u8, 0u16);
    test_double(1_000u16, 1_000i32);
    test_double(-5i16, -5i8);
    test_double(255u8, 255u64);

    test_double(-1i8, u32::MAX);
    test_double(u32::MAX, u16::MAX);
    test_double(i32::MIN, 2_147_483_648u32);
    test_double(i32::MIN, 0u16);
    test_double(i32::MIN, 0i16);
    test_double(-5i32, 4_294_967_291u32);
    test_double(3_000_000_000u32, -1_294_967_296i32);
    test_double(-1000i16, 24i8);
}

#[test]
pub fn test_overflowing_from() {
    fn test_single<T: Copy + Debug + Eq>(n: T)
    where
        T: OverflowingFrom<T>,
    {
        assert_eq!(T::overflowing_from(n), (n, false));
    };
    test_single(0u8);
    test_single(5u64);
    test_single(1_000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double<T, U: Debug + Eq>(n_in: T, n_out: (U, bool))
    where
        U: OverflowingFrom<T>,
    {
        assert_eq!(U::overflowing_from(n_in), n_out);
    };
    test_double(0u8, (0u16, false));
    test_double(1_000u16, (1_000i32, false));
    test_double(-5i16, (-5i8, false));
    test_double(255u8, (255u64, false));

    test_double(-1i8, (u32::MAX, true));
    test_double(u32::MAX, (u16::MAX, true));
    test_double(i32::MIN, (2_147_483_648u32, true));
    test_double(i32::MIN, (0u16, true));
    test_double(i32::MIN, (0i16, true));
    test_double(-5i32, (4_294_967_291u32, true));
    test_double(3_000_000_000u32, (-1_294_967_296i32, true));
    test_double(-1000i16, (24i8, true));
}

#[test]
pub fn test_saturating_from() {
    fn test_single<T: Copy + Debug + Eq>(n: T)
    where
        T: SaturatingFrom<T>,
    {
        assert_eq!(T::saturating_from(n), n);
    };
    test_single(0u8);
    test_single(5u64);
    test_single(1_000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double<T, U: Debug + Eq>(n_in: T, n_out: U)
    where
        U: SaturatingFrom<T>,
    {
        assert_eq!(U::saturating_from(n_in), n_out);
    };
    test_double(0u8, 0u16);
    test_double(1_000u16, 1_000i32);
    test_double(-5i16, -5i8);
    test_double(255u8, 255u64);

    test_double(-1i8, 0u32);
    test_double(u32::MAX, u16::MAX);
    test_double(i32::MIN, 0u32);
    test_double(i32::MIN, 0u16);
    test_double(i32::MIN, i16::MIN);
    test_double(-5i32, 0u32);
    test_double(3_000_000_000u32, i32::MAX);
    test_double(-1000i16, i8::MIN);
}

fn split_in_half_helper<T: PrimitiveUnsigned + SplitInHalf>(n: T, out: (T::Half, T::Half))
where
    T::Half: PrimitiveUnsigned,
{
    assert_eq!(n.split_in_half(), out);
}

#[test]
pub fn test_split_in_half() {
    split_in_half_helper(0u64, (0u32, 0u32));
    split_in_half_helper(1u64, (0u32, 1u32));
    split_in_half_helper(u16::from(u8::MAX), (0, u8::MAX));
    split_in_half_helper(u16::from(u8::MAX) + 1, (1, 0));
    split_in_half_helper(u16::MAX, (u8::MAX, u8::MAX));
    split_in_half_helper(258u16, (1u8, 2u8));
    split_in_half_helper(0xabcd_1234u32, (0xabcd, 0x1234));
}

fn lower_half_helper<T: PrimitiveUnsigned + SplitInHalf>(n: T, out: T::Half)
where
    T::Half: PrimitiveUnsigned,
{
    assert_eq!(n.lower_half(), out);
}

#[test]
pub fn test_lower_half() {
    lower_half_helper(0u64, 0u32);
    lower_half_helper(1u64, 1u32);
    lower_half_helper(u16::from(u8::MAX), u8::MAX);
    lower_half_helper(u16::from(u8::MAX) + 1, 0);
    lower_half_helper(u16::MAX, u8::MAX);
    lower_half_helper(258u16, 2u8);
    lower_half_helper(0xabcd_1234u32, 0x1234);
}

fn upper_half_helper<T: PrimitiveUnsigned + SplitInHalf>(n: T, out: T::Half)
where
    T::Half: PrimitiveUnsigned,
{
    assert_eq!(n.upper_half(), out);
}

#[test]
pub fn test_upper_half() {
    upper_half_helper(0u64, 0u32);
    upper_half_helper(1u64, 0u32);
    upper_half_helper(u16::from(u8::MAX), 0);
    upper_half_helper(u16::from(u8::MAX) + 1, 1);
    upper_half_helper(u16::MAX, u8::MAX);
    upper_half_helper(258u16, 1u8);
    upper_half_helper(0xabcd_1234u32, 0xabcd);
}

fn join_halves_helper<T: JoinHalves + PrimitiveUnsigned>(upper: T::Half, lower: T::Half, out: T) {
    assert_eq!(T::join_halves(upper, lower), out);
}

#[test]
pub fn test_join_halves() {
    join_halves_helper(0u32, 0u32, 0u64);
    join_halves_helper(0u32, 1u32, 1u64);
    join_halves_helper(0, u8::MAX, u16::from(u8::MAX));
    join_halves_helper(1, 0, u16::from(u8::MAX) + 1);
    join_halves_helper(u8::MAX, u8::MAX, u16::MAX);
    join_halves_helper(1, 2, 258u16);
    join_halves_helper(0xabcd, 0x1234, 0xabcd_1234u32);
}

#[test]
pub fn test_from_other_type_slice() {
    fn test<T: Debug + Eq, U: Debug + Eq>(slice: &[T], n: U)
    where
        U: FromOtherTypeSlice<T>,
    {
        assert_eq!(U::from_other_type_slice(slice), n);
    };
    test::<u32, u32>(&[], 0);
    test::<u32, u32>(&[123], 123);
    test::<u32, u32>(&[123, 456], 123);

    test::<u8, u16>(&[0xab], 0xab);
    test::<u8, u16>(&[0xab, 0xcd], 0xcdab);
    test::<u8, u16>(&[0xab, 0xcd, 0xef], 0xcdab);
    test::<u8, u64>(
        &[0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67],
        0x67452301efcdab,
    );
    test::<u8, u64>(
        &[0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xff],
        0x8967452301efcdab,
    );

    test::<u64, u32>(&[], 0);
    test::<u16, u8>(&[0xabcd, 0xef01], 0xcd);
    test::<u128, u8>(&[0x1234567890abcdef01234567890abcde], 0xde);
}

#[test]
pub fn test_vec_from_other_type_slice() {
    fn test<T: Debug + Eq, U: Debug + Eq>(slice: &[T], vec: Vec<U>)
    where
        U: VecFromOtherTypeSlice<T>,
    {
        assert_eq!(U::vec_from_other_type_slice(slice), vec);
    };
    test::<u32, u32>(&[123, 456], vec![123, 456]);
    test::<u8, u16>(
        &[0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xff],
        vec![0xcdab, 0x01ef, 0x4523, 0x8967, 0xff],
    );
    test::<u8, u16>(&[0xab], vec![0xab]);
    test::<u16, u8>(
        &[0xcdab, 0x01ef, 0x4523, 0x8967],
        vec![0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89],
    );
}
