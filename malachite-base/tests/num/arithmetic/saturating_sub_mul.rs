use malachite_base::num::basic::integers::PrimitiveInt;

#[test]
fn test_saturating_sub_mul() {
    fn test<T: PrimitiveInt>(x: T, y: T, z: T, out: T) {
        assert_eq!(x.saturating_sub_mul(y, z), out);

        let mut x = x;
        x.saturating_sub_mul_assign(y, z);
        assert_eq!(x, out);
    }
    test::<u8>(100, 3, 7, 79);
    test::<u32>(60, 5, 10, 10);
    test::<u64>(1000000, 456, 789, 640216);
    test::<i32>(123, -456, 789, 359907);
    test::<i128>(-123, 456, 789, -359907);
    test::<i8>(127, 2, 100, -73);
    test::<i8>(-127, -2, 100, 73);
    test::<i8>(-128, 1, 0, -128);

    test::<u8>(2, 10, 5, 0);
    test::<i8>(-127, 2, 100, -128);
    test::<i8>(-127, 1, 100, -128);
    test::<i8>(127, -1, 100, 127);
    test::<i8>(127, -10, 100, 127);
}
