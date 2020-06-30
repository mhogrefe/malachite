use malachite_base::num::arithmetic::traits::WrappingDivAssign;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::One;

#[test]
fn test_wrapping_div() {
    fn test<T: PrimitiveInteger>(x: T, y: T, out: T) {
        assert_eq!(x.wrapping_div(y), out);

        let mut x = x;
        x.wrapping_div_assign(y);
        assert_eq!(x, out);
    };
    test::<u16>(0, 5, 0);
    test::<u16>(123, 456, 0);
    test::<u8>(100, 3, 33);
    test::<i8>(100, -3, -33);
    test::<i16>(-100, 3, -33);
    test::<i32>(-100, -3, 33);
    test::<i8>(-128, -1, -128);
}

macro_rules! wrapping_div_assign_fail {
    ($t:ident, $wrapping_div_assign_fail:ident) => {
        #[test]
        #[should_panic]
        fn $wrapping_div_assign_fail() {
            $t::ONE.wrapping_div_assign(0);
        }
    };
}
wrapping_div_assign_fail!(u8, wrapping_div_assign_u8_fail);
wrapping_div_assign_fail!(u16, wrapping_div_assign_u16_fail);
wrapping_div_assign_fail!(u32, wrapping_div_assign_u32_fail);
wrapping_div_assign_fail!(u64, wrapping_div_assign_u64_fail);
wrapping_div_assign_fail!(u128, wrapping_div_assign_u128_fail);
wrapping_div_assign_fail!(usize, wrapping_div_assign_usize_fail);
wrapping_div_assign_fail!(i8, wrapping_div_assign_i8_fail);
wrapping_div_assign_fail!(i16, wrapping_div_assign_i16_fail);
wrapping_div_assign_fail!(i32, wrapping_div_assign_i32_fail);
wrapping_div_assign_fail!(i64, wrapping_div_assign_i64_fail);
wrapping_div_assign_fail!(i128, wrapping_div_assign_i128_fail);
wrapping_div_assign_fail!(isize, wrapping_div_assign_isize_fail);
