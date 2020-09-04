use malachite_base::num::basic::integers::PrimitiveInt;

#[test]
fn test_sub_mul() {
    fn test<T: PrimitiveInt>(x: T, y: T, z: T, out: T) {
        assert_eq!(x.sub_mul(y, z), out);

        let mut x = x;
        x.sub_mul_assign(y, z);
        assert_eq!(x, out);
    };
    test::<u8>(100, 3, 7, 79);
    test::<u32>(60, 5, 10, 10);
    test::<u64>(1_000_000, 456, 789, 640_216);
    test::<i32>(123, -456, 789, 359_907);
    test::<i128>(-123, 456, 789, -359_907);
    test::<i8>(127, 2, 100, -73);
    test::<i8>(-127, -2, 100, 73);
    test::<i8>(-128, 1, 0, -128);
}
