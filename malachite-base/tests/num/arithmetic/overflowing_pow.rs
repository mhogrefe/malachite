use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_overflowing_pow() {
    fn test<T>(x: T, y: u64, out: T, overflow: bool)
    where
        T: PrimitiveInteger,
    {
        assert_eq!(x.overflowing_pow(y), (out, overflow));

        let mut x = x;
        assert_eq!(x.overflowing_pow_assign(y), overflow);
        assert_eq!(x, out);
    };
    test::<u8>(0, 0, 1, false);
    test::<u64>(123, 0, 1, false);
    test::<u64>(123, 1, 123, false);
    test::<u16>(0, 123, 0, false);
    test::<u16>(1, 123, 1, false);
    test::<i16>(-1, 123, -1, false);
    test::<i16>(-1, 124, 1, false);
    test::<u8>(3, 3, 27, false);
    test::<i32>(-10, 9, -1_000_000_000, false);
    test::<i32>(-10, 10, 1_410_065_408, true);
    test::<i16>(-10, 9, 13_824, true);
    test::<i16>(10, 9, -13_824, true);
    test::<i64>(123, 456, 2_409_344_748_064_316_129, true);
}
