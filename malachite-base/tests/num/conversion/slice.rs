use std::fmt::Debug;

use malachite_base::num::conversion::traits::{
    FromOtherTypeSlice, VecFromOtherType, VecFromOtherTypeSlice,
};

#[test]
pub fn test_from_other_type_slice() {
    fn test<T: Debug + Eq, U: Copy + Debug + Eq>(slice: &[T], n: U)
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
        0x67_4523_01ef_cdab,
    );
    test::<u8, u64>(
        &[0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xff],
        0x8967_4523_01ef_cdab,
    );

    test::<u64, u32>(&[], 0);
    test::<u16, u8>(&[0xabcd, 0xef01], 0xcd);
    test::<u128, u8>(&[0x1234_5678_90ab_cdef_0123_4567_890a_bcde], 0xde);
}

#[test]
pub fn test_vec_from_other_type_slice() {
    fn test<T: Debug + Eq, U: Debug + Eq>(slice: &[T], vec: &[U])
    where
        U: VecFromOtherTypeSlice<T>,
    {
        assert_eq!(U::vec_from_other_type_slice(slice), vec);
    };
    test::<u32, u32>(&[123, 456], &[123, 456]);
    test::<u8, u16>(
        &[0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xff],
        &[0xcdab, 0x01ef, 0x4523, 0x8967, 0xff],
    );
    test::<u8, u16>(&[0xab], &[0xab]);
    test::<u16, u8>(
        &[0xcdab, 0x01ef, 0x4523, 0x8967],
        &[0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89],
    );
}

#[test]
pub fn test_vec_from_other_type() {
    fn test<T: Debug + Eq, U: Debug + Eq>(value: T, vec: &[U])
    where
        U: VecFromOtherType<T>,
    {
        assert_eq!(U::vec_from_other_type(value), vec);
    };
    test::<u32, u32>(123, &[123]);
    test::<u8, u16>(0xab, &[0xab]);
    test::<u16, u8>(0xcdab, &[0xab, 0xcd]);
}
