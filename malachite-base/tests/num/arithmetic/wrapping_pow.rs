use malachite_base::num::basic::integers::PrimitiveInt;

#[test]
fn test_wrapping_pow() {
    fn test<T: PrimitiveInt>(x: T, y: u64, out: T) {
        assert_eq!(x.wrapping_pow(y), out);

        let mut x = x;
        x.wrapping_pow_assign(y);
        assert_eq!(x, out);
    }
    test::<u8>(0, 0, 1);
    test::<u64>(123, 0, 1);
    test::<u64>(123, 1, 123);
    test::<u16>(0, 123, 0);
    test::<u16>(1, 123, 1);
    test::<i16>(-1, 123, -1);
    test::<i16>(-1, 124, 1);
    test::<u8>(3, 3, 27);
    test::<i32>(-10, 9, -1000000000);
    test::<i32>(-10, 10, 1410065408);
    test::<i16>(-10, 9, 13824);
    test::<i16>(10, 9, -13824);
    test::<i64>(123, 456, 2409344748064316129);
}
