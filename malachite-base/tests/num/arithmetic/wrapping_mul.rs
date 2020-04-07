use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_wrapping_mul() {
    fn test<T>(x: T, y: T, out: T)
    where
        T: PrimitiveInteger,
    {
        assert_eq!(x.wrapping_mul(y), out);

        let mut x = x;
        x.wrapping_mul_assign(y);
        assert_eq!(x, out);
    };
    test::<u16>(123, 456, 56_088);
    test::<u8>(123, 200, 24);
    test::<i16>(123, -45, -5_535);
    test::<i8>(123, 45, -97);
    test::<i8>(-123, 45, 97);
}
