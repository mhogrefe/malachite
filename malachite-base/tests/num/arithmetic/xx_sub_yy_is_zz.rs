use malachite_base::num::arithmetic::xx_sub_yy_is_zz::_explicit_xx_sub_yy_is_zz;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

#[test]
fn test_xx_sub_yy_is_zz() {
    fn test<T: PrimitiveUnsigned>(x_1: T, x_0: T, y_1: T, y_0: T, z_1: T, z_0: T) {
        assert_eq!(T::xx_sub_yy_is_zz(x_1, x_0, y_1, y_0), (z_1, z_0));
        assert_eq!(_explicit_xx_sub_yy_is_zz(x_1, x_0, y_1, y_0), (z_1, z_0));
    }
    test::<u32>(0, 0, 0, 0, 0, 0);
    test::<u64>(0x67, 0x89, 0x33, 0x33, 0x34, 0x56);
    test::<u8>(0x78, 0x9a, 0xbc, 0xde, 0xbb, 0xbc);
    test::<u8>(0, 0, 0, 1, u8::MAX, u8::MAX);
    test(u16::MAX, u16::MAX, u16::MAX, u16::MAX, 0, 0);
}
