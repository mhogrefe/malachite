use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_overflowing_square() {
    fn test<T>(x: T, out: T, overflow: bool)
    where
        T: PrimitiveInteger,
    {
        assert_eq!(x.overflowing_square(), (out, overflow));

        let mut x = x;
        assert_eq!(x.overflowing_square_assign(), overflow);
        assert_eq!(x, out);
    };
    test::<u8>(0, 0, false);
    test::<i16>(1, 1, false);
    test::<u32>(2, 4, false);
    test::<i64>(3, 9, false);
    test::<u128>(10, 100, false);
    test::<isize>(123, 15_129, false);
    test::<u32>(1_000, 1_000_000, false);

    test::<i16>(-1, 1, false);
    test::<i32>(-2, 4, false);
    test::<i64>(-3, 9, false);
    test::<i128>(-10, 100, false);
    test::<isize>(-123, 15_129, false);
    test::<i32>(-1_000, 1_000_000, false);

    test::<u16>(1_000, 16_960, true);
    test::<i16>(-1_000, 16_960, true);
}
