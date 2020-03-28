use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_overflowing_add() {
    fn test<T>(x: T, y: T, out: T, overflow: bool)
    where
        T: PrimitiveInteger,
    {
        assert_eq!(x.overflowing_add(y), (out, overflow));

        let mut x = x;
        assert_eq!(x.overflowing_add_assign(y), overflow);
        assert_eq!(x, out);
    };
    test::<u16>(123, 456, 579, false);
    test::<u8>(123, 200, 67, true);
    test::<i16>(123, -456, -333, false);
    test::<i8>(123, 45, -88, true);
    test::<i8>(-123, -45, 88, true);
}
