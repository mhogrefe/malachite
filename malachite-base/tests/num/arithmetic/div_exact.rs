use std::panic::catch_unwind;

use malachite_base::num::basic::integers::PrimitiveInt;

#[test]
fn test_div_exact() {
    fn test<T: PrimitiveInt>(x: T, y: T, out: T) {
        assert_eq!(x.div_exact(y), out);

        let mut x = x;
        x.div_exact_assign(y);
        assert_eq!(x, out);
    };
    test::<u8>(0, 123, 0);
    test::<u16>(123, 1, 123);
    test::<u32>(123, 123, 1);
    test::<usize>(56088, 123, 456);
    test::<u64>(0, 1000000000000, 0);
    test::<u128>(1000000000000, 1, 1000000000000);
    test::<usize>(1000000000000, 1000000000000, 1);
    test::<usize>(123000000000000, 1000000000000, 123);
    test::<usize>(123000000000000, 123, 1000000000000);
    test::<u128>(121932631112635269000000, 123456789000, 987654321000);
    test::<u64>(0x1fffffffe, 0xffffffff, 2);
    test::<u64>(18446744065119617025, 0xffffffff, 0xffffffff);

    test::<i8>(0, -123, 0);
    test::<i16>(123, -1, -123);
    test::<i32>(123, -123, -1);
    test::<isize>(56088, -123, -456);
    test::<i64>(0, -1000000000000, 0);
    test::<i128>(1000000000000, -1, -1000000000000);
    test::<isize>(1000000000000, -1000000000000, -1);
    test::<isize>(123000000000000, -1000000000000, -123);
    test::<isize>(123000000000000, -123, -1000000000000);
    test::<i128>(121932631112635269000000, -123456789000, -987654321000);
    test::<i64>(0x1fffffffe, -0xffffffff, -2);
    test::<i128>(18446744065119617025, -0xffffffff, -0xffffffff);

    test::<i16>(-123, 1, -123);
    test::<i32>(-123, 123, -1);
    test::<isize>(-56088, 123, -456);
    test::<i128>(-1000000000000, 1, -1000000000000);
    test::<isize>(-1000000000000, 1000000000000, -1);
    test::<isize>(-123000000000000, 1000000000000, -123);
    test::<isize>(-123000000000000, 123, -1000000000000);
    test::<i128>(-121932631112635269000000, 123456789000, -987654321000);
    test::<i64>(-0x1fffffffe, 0xffffffff, -2);
    test::<i128>(-18446744065119617025, 0xffffffff, -0xffffffff);

    test::<i16>(-123, -1, 123);
    test::<i32>(-123, -123, 1);
    test::<isize>(-56088, -123, 456);
    test::<i128>(-1000000000000, -1, 1000000000000);
    test::<isize>(-1000000000000, -1000000000000, 1);
    test::<isize>(-123000000000000, -1000000000000, 123);
    test::<isize>(-123000000000000, -123, 1000000000000);
    test::<i128>(-121932631112635269000000, -123456789000, 987654321000);
    test::<i64>(-0x1fffffffe, -0xffffffff, 2);
    test::<i128>(-18446744065119617025, -0xffffffff, 0xffffffff);
}

fn div_exact_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::ONE.div_exact(T::ZERO));
    assert_panic!({
        let mut n = T::ONE;
        n.div_exact_assign(T::ZERO);
    });
}

#[test]
pub fn div_exact_fail() {
    apply_fn_to_primitive_ints!(div_exact_fail_helper);
}
