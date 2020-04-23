use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_overflowing_sub_mul() {
    fn test<T>(x: T, y: T, z: T, out: T, overflow: bool)
    where
        T: PrimitiveInteger,
    {
        assert_eq!(x.overflowing_sub_mul(y, z), (out, overflow));

        let mut x = x;
        assert_eq!(x.overflowing_sub_mul_assign(y, z), overflow);
        assert_eq!(x, out);
    };
    test::<u8>(100, 3, 7, 79, false);
    test::<u32>(60, 5, 10, 10, false);
    test::<u64>(1_000_000, 456, 789, 640_216, false);
    test::<i32>(123, -456, 789, 359_907, false);
    test::<i128>(-123, 456, 789, -359_907, false);
    test::<i8>(127, 2, 100, -73, false);
    test::<i8>(-127, -2, 100, 73, false);
    test::<i8>(-128, 1, 0, -128, false);

    test::<u8>(2, 10, 5, 208, true);
    test::<i8>(-127, 2, 100, -71, true);
    test::<i8>(-127, 1, 100, 29, true);
    test::<i8>(127, -1, 100, -29, true);
    test::<i8>(127, -10, 100, 103, true);
}
