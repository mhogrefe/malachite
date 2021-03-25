use malachite_base::num::basic::integers::PrimitiveInt;
use std::panic::catch_unwind;

#[test]
fn test_wrapping_div() {
    fn test<T: PrimitiveInt>(x: T, y: T, out: T) {
        assert_eq!(x.wrapping_div(y), out);

        let mut x = x;
        x.wrapping_div_assign(y);
        assert_eq!(x, out);
    }
    test::<u16>(0, 5, 0);
    test::<u16>(123, 456, 0);
    test::<u8>(100, 3, 33);
    test::<i8>(100, -3, -33);
    test::<i16>(-100, 3, -33);
    test::<i32>(-100, -3, 33);
    test::<i8>(-128, -1, -128);
}

fn wrapping_div_assign_fail_helper<T: PrimitiveInt>() {
    assert_panic!({
        let mut n = T::ONE;
        n.wrapping_div_assign(T::ZERO);
    });
}

#[test]
fn wrapping_div_assign_fail() {
    apply_fn_to_primitive_ints!(wrapping_div_assign_fail_helper);
}
