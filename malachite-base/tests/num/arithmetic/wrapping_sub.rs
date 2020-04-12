use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_wrapping_sub() {
    fn test<T>(x: T, y: T, out: T)
    where
        T: PrimitiveInteger,
    {
        assert_eq!(x.wrapping_sub(y), out);

        let mut x = x;
        x.wrapping_sub_assign(y);
        assert_eq!(x, out);
    };
    test::<u16>(456, 123, 333);
    test::<u8>(123, 200, 179);
    test::<i16>(123, -456, 579);
    test::<i8>(123, -45, -88);
    test::<i8>(-123, 45, 88);
}