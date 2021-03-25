use malachite_base::num::basic::integers::PrimitiveInt;

#[test]
fn test_overflowing_mul() {
    fn test<T: PrimitiveInt>(x: T, y: T, out: T, overflow: bool) {
        assert_eq!(x.overflowing_mul(y), (out, overflow));

        let mut x = x;
        assert_eq!(x.overflowing_mul_assign(y), overflow);
        assert_eq!(x, out);
    }
    test::<u16>(123, 456, 56088, false);
    test::<u8>(123, 200, 24, true);
    test::<i16>(123, -45, -5535, false);
    test::<i8>(123, 45, -97, true);
    test::<i8>(-123, 45, 97, true);
}
