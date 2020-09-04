use malachite_base::num::basic::integers::PrimitiveInt;

#[test]
fn test_pow() {
    fn test<T: PrimitiveInt>(x: T, y: u64, out: T) {
        assert_eq!(x.pow(y), out);

        let mut x = x;
        x.pow_assign(y);
        assert_eq!(x, out);
    };
    test::<u8>(0, 0, 1);
    test::<u64>(123, 0, 1);
    test::<u64>(123, 1, 123);
    test::<u16>(0, 123, 0);
    test::<u16>(1, 123, 1);
    test::<i16>(-1, 123, -1);
    test::<i16>(-1, 124, 1);
    test::<u8>(3, 3, 27);
    test::<i32>(-10, 9, -1_000_000_000);
    test::<i32>(-10, 8, 100_000_000);
}
