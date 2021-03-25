use malachite_base::num::basic::integers::PrimitiveInt;

#[test]
fn test_saturating_add_mul() {
    fn test<T: PrimitiveInt>(x: T, y: T, z: T, out: T) {
        assert_eq!(x.saturating_add_mul(y, z), out);

        let mut x = x;
        x.saturating_add_mul_assign(y, z);
        assert_eq!(x, out);
    }
    test::<u8>(2, 3, 7, 23);
    test::<u32>(7, 5, 10, 57);
    test::<u64>(123, 456, 789, 359907);
    test::<i32>(123, -456, 789, -359661);
    test::<i128>(-123, 456, 789, 359661);
    test::<i8>(127, -2, 100, -73);
    test::<i8>(-127, 2, 100, 73);
    test::<i8>(-128, 1, 0, -128);

    test::<u8>(2, 20, 20, 255);
    test::<i8>(-127, -2, 100, -128);
    test::<i8>(127, 1, 100, 127);
    test::<i8>(-127, -1, 100, -128);
    test::<i8>(-127, -10, 100, -128);
}
