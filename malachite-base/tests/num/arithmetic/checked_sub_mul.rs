use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_checked_sub_mul() {
    fn test<T>(x: T, y: T, z: T, out: Option<T>)
    where
        T: PrimitiveInteger,
    {
        assert_eq!(x.checked_sub_mul(y, z), out);
    };
    test::<u8>(100, 3, 7, Some(79));
    test::<u32>(60, 5, 10, Some(10));
    test::<u64>(1_000_000, 456, 789, Some(640_216));
    test::<i32>(123, -456, 789, Some(359_907));
    test::<i128>(-123, 456, 789, Some(-359_907));
    test::<i8>(127, 2, 100, Some(-73));
    test::<i8>(-127, -2, 100, Some(73));
    test::<i8>(-128, 1, 0, Some(-128));

    test::<u8>(2, 10, 5, None);
    test::<i8>(-127, 2, 100, None);
    test::<i8>(-127, 1, 100, None);
    test::<i8>(127, -1, 100, None);
    test::<i8>(127, -10, 100, None);
}
