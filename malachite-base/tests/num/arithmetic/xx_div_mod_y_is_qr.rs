use malachite_base::num::arithmetic::traits::XXDivModYIsQR;
use malachite_base::num::arithmetic::unsigneds::_explicit_xx_div_mod_y_is_qr;
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

macro_rules! xx_div_mod_y_is_qr_fail {
    ($t:ident, $xx_div_mod_y_is_qr_fail_1:ident, $xx_div_mod_y_is_qr_fail_2:ident) => {
        #[test]
        #[should_panic]
        fn $xx_div_mod_y_is_qr_fail_1() {
            $t::xx_div_mod_y_is_qr(3, 5, 0);
        }

        #[test]
        #[should_panic]
        fn $xx_div_mod_y_is_qr_fail_2() {
            $t::xx_div_mod_y_is_qr(3, 5, 2);
        }
    };
}

xx_div_mod_y_is_qr_fail!(
    u8,
    xx_div_mod_y_is_qr_u8_fail_1,
    xx_div_mod_y_is_qr_u8_fail_2
);
xx_div_mod_y_is_qr_fail!(
    u16,
    xx_div_mod_y_is_qr_u16_fail_1,
    xx_div_mod_y_is_qr_u16_fail_2
);
xx_div_mod_y_is_qr_fail!(
    u32,
    xx_div_mod_y_is_qr_u32_fail_1,
    xx_div_mod_y_is_qr_u32_fail_2
);
xx_div_mod_y_is_qr_fail!(
    u64,
    xx_div_mod_y_is_qr_u64_fail_1,
    xx_div_mod_y_is_qr_u64_fail_2
);
xx_div_mod_y_is_qr_fail!(
    u128,
    xx_div_mod_y_is_qr_u128_fail_1,
    xx_div_mod_y_is_qr_u128_fail_2
);
xx_div_mod_y_is_qr_fail!(
    usize,
    xx_div_mod_y_is_qr_usize_fail_1,
    xx_div_mod_y_is_qr_usize_fail_2
);
