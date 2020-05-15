use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_wrapping_div() {
    fn test<T>(x: T, y: T, out: T)
    where
        T: PrimitiveInteger,
    {
        assert_eq!(x.wrapping_div(y), out);

        let mut x = x;
        x.wrapping_div_assign(y);
        assert_eq!(x, out);
    };
    test::<u16>(0, 5, 0);
    test::<u16>(123, 456, 0);
    test::<u8>(100, 3, 33);
    test::<i8>(100, -3, -33);
    test::<i16>(-100, 3, -33);
    test::<i32>(-100, -3, 33);
    test::<i8>(-128, -1, -128);
}
