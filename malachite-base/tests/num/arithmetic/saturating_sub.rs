use malachite_base::num::basic::integers::PrimitiveInt;

#[test]
fn test_saturating_sub() {
    fn test<T: PrimitiveInt>(x: T, y: T, out: T) {
        assert_eq!(x.saturating_sub(y), out);

        let mut x = x;
        x.saturating_sub_assign(y);
        assert_eq!(x, out);
    };
    test::<u16>(456, 123, 333);
    test::<u8>(123, 200, 0);
    test::<i16>(123, -456, 579);
    test::<i8>(123, -45, 127);
    test::<i8>(-123, 45, -128);
}
