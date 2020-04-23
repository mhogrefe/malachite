use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_wrapping_add_mul() {
    fn test<T>(x: T, y: T, z: T, out: T)
    where
        T: PrimitiveInteger,
    {
        assert_eq!(x.wrapping_add_mul(y, z), out);

        let mut x = x;
        x.wrapping_add_mul_assign(y, z);
        assert_eq!(x, out);
    };
    test::<u8>(2, 3, 7, 23);
    test::<u32>(7, 5, 10, 57);
    test::<u64>(123, 456, 789, 359_907);
    test::<i32>(123, -456, 789, -359_661);
    test::<i128>(-123, 456, 789, 359_661);
    test::<i8>(127, -2, 100, -73);
    test::<i8>(-127, 2, 100, 73);
    test::<i8>(-128, 1, 0, -128);

    test::<u8>(2, 20, 20, 146);
    test::<i8>(-127, -2, 100, -71);
    test::<i8>(127, 1, 100, -29);
    test::<i8>(-127, -1, 100, 29);
    test::<i8>(-127, -10, 100, -103);
}
