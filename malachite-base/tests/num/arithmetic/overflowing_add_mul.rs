use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_overflowing_add_mul() {
    fn test<T: PrimitiveInteger>(x: T, y: T, z: T, out: T, overflow: bool) {
        assert_eq!(x.overflowing_add_mul(y, z), (out, overflow));

        let mut x = x;
        assert_eq!(x.overflowing_add_mul_assign(y, z), overflow);
        assert_eq!(x, out);
    };
    test::<u8>(2, 3, 7, 23, false);
    test::<u32>(7, 5, 10, 57, false);
    test::<u64>(123, 456, 789, 359_907, false);
    test::<i32>(123, -456, 789, -359_661, false);
    test::<i128>(-123, 456, 789, 359_661, false);
    test::<i8>(127, -2, 100, -73, false);
    test::<i8>(-127, 2, 100, 73, false);
    test::<i8>(-128, 1, 0, -128, false);

    test::<u8>(2, 20, 20, 146, true);
    test::<i8>(-127, -2, 100, -71, true);
    test::<i8>(127, 1, 100, -29, true);
    test::<i8>(-127, -1, 100, 29, true);
    test::<i8>(-127, -10, 100, -103, true);
}
