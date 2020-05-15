use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_overflowing_div() {
    fn test<T>(x: T, y: T, out: T, overflow: bool)
    where
        T: PrimitiveInteger,
    {
        assert_eq!(x.overflowing_div(y), (out, overflow));

        let mut x = x;
        assert_eq!(x.overflowing_div_assign(y), overflow);
        assert_eq!(x, out);
    };
    test::<u16>(0, 5, 0, false);
    test::<u16>(123, 456, 0, false);
    test::<u8>(100, 3, 33, false);
    test::<i8>(100, -3, -33, false);
    test::<i16>(-100, 3, -33, false);
    test::<i32>(-100, -3, 33, false);
    test::<i8>(-128, -1, -128, true);
}
