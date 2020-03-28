use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_saturating_add() {
    fn test<T>(x: T, y: T, out: T)
    where
        T: PrimitiveInteger,
    {
        assert_eq!(x.saturating_add(y), out);

        let mut x = x;
        x.saturating_add_assign(y);
        assert_eq!(x, out);
    };
    test::<u16>(123, 456, 579);
    test::<u8>(123, 200, 255);
    test::<i16>(123, -456, -333);
    test::<i8>(123, 45, 127);
    test::<i8>(-123, -45, -128);
}
