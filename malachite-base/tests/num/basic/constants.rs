use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_constants() {
    assert_eq!(u8::WIDTH, 8);
    assert_eq!(u8::LOG_WIDTH, 3);
    assert_eq!(u8::WIDTH_MASK, 0x7);

    assert_eq!(u16::WIDTH, 16);
    assert_eq!(u16::LOG_WIDTH, 4);
    assert_eq!(u16::WIDTH_MASK, 0xf);

    assert_eq!(u32::WIDTH, 32);
    assert_eq!(u32::LOG_WIDTH, 5);
    assert_eq!(u32::WIDTH_MASK, 0x1f);

    assert_eq!(u64::WIDTH, 64);
    assert_eq!(u64::LOG_WIDTH, 6);
    assert_eq!(u64::WIDTH_MASK, 0x3f);

    assert_eq!(u128::WIDTH, 128);
    assert_eq!(u128::LOG_WIDTH, 7);
    assert_eq!(u128::WIDTH_MASK, 0x7f);

    assert_eq!(i8::WIDTH, 8);
    assert_eq!(i8::LOG_WIDTH, 3);
    assert_eq!(i8::WIDTH_MASK, 0x7);

    assert_eq!(i16::WIDTH, 16);
    assert_eq!(i16::LOG_WIDTH, 4);
    assert_eq!(i16::WIDTH_MASK, 0xf);

    assert_eq!(i32::WIDTH, 32);
    assert_eq!(i32::LOG_WIDTH, 5);
    assert_eq!(i32::WIDTH_MASK, 0x1f);

    assert_eq!(i64::WIDTH, 64);
    assert_eq!(i64::LOG_WIDTH, 6);
    assert_eq!(i64::WIDTH_MASK, 0x3f);

    assert_eq!(i128::WIDTH, 128);
    assert_eq!(i128::LOG_WIDTH, 7);
    assert_eq!(i128::WIDTH_MASK, 0x7f);
}
