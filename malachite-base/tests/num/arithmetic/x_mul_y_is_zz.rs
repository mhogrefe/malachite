use malachite_base::comparison::Max;
use malachite_base::num::arithmetic::unsigneds::_explicit_x_mul_y_is_zz;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

#[test]
fn test_x_mul_y_is_zz() {
    fn test<T: PrimitiveUnsigned>(x: T, y: T, z_1: T, z_0: T) {
        assert_eq!(T::x_mul_y_is_zz(x, y), (z_1, z_0));
        assert_eq!(_explicit_x_mul_y_is_zz(x, y), (z_1, z_0));
    }
    test::<u32>(0, 0, 0, 0);
    test::<u64>(15, 3, 0, 45);
    test::<u8>(0x78, 0x9a, 0x48, 0x30);
    test::<u8>(u8::MAX, 0, 0, 0);
    test::<u8>(u8::MAX, 1, 0, u8::MAX);
    test(u16::MAX, u16::MAX, u16::MAX - 1, 1);
}
