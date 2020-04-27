use malachite_base::num::arithmetic::xx_add_yy_is_zz::_explicit_xx_add_yy_is_zz;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

#[test]
fn test_xx_add_yy_is_zz() {
    fn test<T: PrimitiveUnsigned>(x_1: T, x_0: T, y_1: T, y_0: T, z_1: T, z_0: T) {
        assert_eq!(T::xx_add_yy_is_zz(x_1, x_0, y_1, y_0), (z_1, z_0));
        assert_eq!(_explicit_xx_add_yy_is_zz(x_1, x_0, y_1, y_0), (z_1, z_0));
    }
    test::<u32>(0, 0, 0, 0, 0, 0);
    test::<u64>(0x12, 0x34, 0x33, 0x33, 0x45, 0x67);
    test::<u8>(0x78, 0x9a, 0xbc, 0xde, 0x35, 0x78);
    test::<u8>(u8::MAX, u8::MAX, 0, 1, 0, 0);
    test(
        u16::MAX,
        u16::MAX,
        u16::MAX,
        u16::MAX,
        u16::MAX,
        u16::MAX - 1,
    );
}
