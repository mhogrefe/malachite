use std::panic::catch_unwind;

use malachite_base::num::arithmetic::xx_div_mod_y_is_qr::_explicit_xx_div_mod_y_is_qr;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

#[test]
fn test_xx_div_mod_y_is_qr() {
    fn test<T: PrimitiveUnsigned>(x_1: T, x_0: T, y: T, q: T, r: T) {
        assert_eq!(T::xx_div_mod_y_is_qr(x_1, x_0, y), (q, r));
        assert_eq!(_explicit_xx_div_mod_y_is_qr(x_1, x_0, y), (q, r));
    }
    test::<u8>(0, 0, 1, 0, 0);
    test::<u32>(0, 1, 1, 1, 0);
    test::<u16>(1, 0, 2, 0x8000, 0);
    test::<u16>(1, 7, 2, 0x8003, 1);
    test::<u8>(0x78, 0x9a, 0xbc, 0xa4, 0x2a);
    test::<u64>(0x12, 0x34, 0x33, 0x5a5a5a5a5a5a5a5b, 0x13);
}

fn xx_div_mod_y_is_qr_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::xx_div_mod_y_is_qr(
        T::exact_from(3),
        T::exact_from(5),
        T::ZERO
    ));
    assert_panic!(T::xx_div_mod_y_is_qr(
        T::exact_from(3),
        T::exact_from(5),
        T::TWO
    ));
}

#[test]
fn xx_div_mod_y_is_qr_fail() {
    apply_fn_to_unsigneds!(xx_div_mod_y_is_qr_fail_helper);
}
