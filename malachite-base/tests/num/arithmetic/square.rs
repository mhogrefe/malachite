use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_square() {
    fn test<T: PrimitiveInteger>(x: T, out: T) {
        assert_eq!(x.square(), out);

        let mut x = x;
        x.square_assign();
        assert_eq!(x, out);
    };
    test::<u8>(0, 0);
    test::<i16>(1, 1);
    test::<u32>(2, 4);
    test::<i64>(3, 9);
    test::<u128>(10, 100);
    test::<isize>(123, 15_129);
    test::<u32>(1_000, 1_000_000);

    test::<i16>(-1, 1);
    test::<i32>(-2, 4);
    test::<i64>(-3, 9);
    test::<i128>(-10, 100);
    test::<isize>(-123, 15_129);
    test::<i32>(-1_000, 1_000_000);
}
