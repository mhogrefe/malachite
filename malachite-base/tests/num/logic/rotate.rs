use malachite_base::num::basic::integers::PrimitiveInt;

#[test]
pub fn test_rotate_left() {
    fn test<T: PrimitiveInt>(n: T, bits: u64, out: T) {
        assert_eq!(n.rotate_left(bits), out);
    };

    test(123u8, 0, 123);
    test(123u8, 5, 111);
    test(123u8, 6, 222);
    test(123u8, 1005, 111);

    test(123u32, 0, 123);
    test(123u32, 5, 3936);
    test(123u32, 6, 7872);
    test(123u32, 1005, 1007616);

    test(123i8, 0, 123);
    test(123i8, 5, 111);
    test(123i8, 6, -34);
    test(123i8, 1005, 111);

    test(123i32, 0, 123);
    test(123i32, 5, 3936);
    test(123i32, 6, 7872);
    test(123i32, 1005, 1007616);
}

#[test]
pub fn test_rotate_right() {
    fn test<T: PrimitiveInt>(n: T, bits: u64, out: T) {
        assert_eq!(n.rotate_right(bits), out);
    };

    test(123u8, 0, 123);
    test(123u8, 3, 111);
    test(123u8, 2, 222);
    test(123u8, 1003, 111);

    test(123u32, 0, 123);
    test(123u32, 27, 3936);
    test(123u32, 26, 7872);
    test(123u32, 1005, 64487424);

    test(123i8, 0, 123);
    test(123i8, 3, 111);
    test(123i8, 2, -34);
    test(123i8, 1003, 111);

    test(123i32, 0, 123);
    test(123i32, 27, 3936);
    test(123i32, 26, 7872);
    test(123i32, 1005, 64487424);
}
