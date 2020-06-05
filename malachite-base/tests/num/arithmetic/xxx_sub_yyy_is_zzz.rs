use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

#[test]
#[allow(clippy::too_many_arguments)]
fn test_xxx_sub_yyy_is_zzz() {
    fn test<T: PrimitiveUnsigned>(
        x_2: T,
        x_1: T,
        x_0: T,
        y_2: T,
        y_1: T,
        y_0: T,
        z_2: T,
        z_1: T,
        z_0: T,
    ) {
        assert_eq!(
            T::xxx_sub_yyy_is_zzz(x_2, x_1, x_0, y_2, y_1, y_0),
            (z_2, z_1, z_0)
        );
    }
    test::<u32>(0, 0, 0, 0, 0, 0, 0, 0, 0);
    test::<u64>(0x67, 0x89, 0xab, 0x33, 0x33, 0x33, 0x34, 0x56, 0x78);
    test::<u8>(0x78, 0x9a, 0xbc, 0xde, 0xfe, 0xdc, 0x99, 0x9b, 0xe0);
    test::<u8>(0, 0, 0, 0, 0, 1, u8::MAX, u8::MAX, u8::MAX);
    test(
        u16::MAX,
        u16::MAX,
        u16::MAX,
        u16::MAX,
        u16::MAX,
        u16::MAX,
        0,
        0,
        0,
    );
}
