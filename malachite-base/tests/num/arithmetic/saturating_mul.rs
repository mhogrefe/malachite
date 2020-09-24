use malachite_base::num::basic::integers::PrimitiveInt;

#[test]
fn test_saturating_mul() {
    fn test<T: PrimitiveInt>(x: T, y: T, out: T) {
        assert_eq!(x.saturating_mul(y), out);

        let mut x = x;
        x.saturating_mul_assign(y);
        assert_eq!(x, out);
    };
    test::<u16>(123, 456, 56088);
    test::<u8>(123, 200, 255);
    test::<i16>(123, -45, -5535);
    test::<i8>(123, 45, 127);
    test::<i8>(-123, 45, -128);
}
