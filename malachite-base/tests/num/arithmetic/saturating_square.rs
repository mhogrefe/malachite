use malachite_base::num::basic::integers::PrimitiveInt;

#[test]
fn test_saturating_square() {
    fn test<T: PrimitiveInt>(x: T, out: T) {
        assert_eq!(x.saturating_square(), out);

        let mut x = x;
        x.saturating_square_assign();
        assert_eq!(x, out);
    }
    test::<u8>(0, 0);
    test::<i16>(1, 1);
    test::<u32>(2, 4);
    test::<i64>(3, 9);
    test::<u128>(10, 100);
    test::<isize>(123, 15129);
    test::<u32>(1000, 1000000);

    test::<i16>(-1, 1);
    test::<i32>(-2, 4);
    test::<i64>(-3, 9);
    test::<i128>(-10, 100);
    test::<isize>(-123, 15129);
    test::<i32>(-1000, 1000000);

    test::<u16>(1000, u16::MAX);
    test::<i16>(-1000, i16::MAX);
}
