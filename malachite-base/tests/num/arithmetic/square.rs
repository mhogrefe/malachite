use malachite_base::num::basic::integers::PrimitiveInt;

#[test]
fn test_square() {
    fn test<T: PrimitiveInt>(x: T, out: T) {
        assert_eq!(x.square(), out);

        let mut x = x;
        x.square_assign();
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
}
