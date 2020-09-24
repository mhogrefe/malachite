use malachite_base::num::basic::integers::PrimitiveInt;

#[test]
fn test_checked_add_mul() {
    fn test<T: PrimitiveInt>(x: T, y: T, z: T, out: Option<T>) {
        assert_eq!(x.checked_add_mul(y, z), out);
    };
    test::<u8>(2, 3, 7, Some(23));
    test::<u32>(7, 5, 10, Some(57));
    test::<u64>(123, 456, 789, Some(359907));
    test::<i32>(123, -456, 789, Some(-359661));
    test::<i128>(-123, 456, 789, Some(359661));
    test::<i8>(127, -2, 100, Some(-73));
    test::<i8>(-127, 2, 100, Some(73));
    test::<i8>(-128, 1, 0, Some(-128));

    test::<u8>(2, 20, 20, None);
    test::<i8>(-127, -2, 100, None);
    test::<i8>(127, 1, 100, None);
    test::<i8>(-127, -1, 100, None);
    test::<i8>(-127, -10, 100, None);
}
