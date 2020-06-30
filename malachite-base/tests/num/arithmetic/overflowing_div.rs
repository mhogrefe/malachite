use malachite_base::num::arithmetic::traits::OverflowingDivAssign;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::One;

#[test]
fn test_overflowing_div() {
    fn test<T: PrimitiveInteger>(x: T, y: T, out: T, overflow: bool) {
        assert_eq!(x.overflowing_div(y), (out, overflow));

        let mut x = x;
        assert_eq!(x.overflowing_div_assign(y), overflow);
        assert_eq!(x, out);
    };
    test::<u16>(0, 5, 0, false);
    test::<u16>(123, 456, 0, false);
    test::<u8>(100, 3, 33, false);
    test::<i8>(100, -3, -33, false);
    test::<i16>(-100, 3, -33, false);
    test::<i32>(-100, -3, 33, false);
    test::<i8>(-128, -1, -128, true);
}

macro_rules! overflowing_div_assign_fail {
    ($t:ident, $overflowing_div_assign_fail:ident) => {
        #[test]
        #[should_panic]
        fn $overflowing_div_assign_fail() {
            $t::ONE.overflowing_div_assign(0);
        }
    };
}
overflowing_div_assign_fail!(u8, overflowing_div_assign_u8_fail);
overflowing_div_assign_fail!(u16, overflowing_div_assign_u16_fail);
overflowing_div_assign_fail!(u32, overflowing_div_assign_u32_fail);
overflowing_div_assign_fail!(u64, overflowing_div_assign_u64_fail);
overflowing_div_assign_fail!(u128, overflowing_div_assign_u128_fail);
overflowing_div_assign_fail!(usize, overflowing_div_assign_usize_fail);
overflowing_div_assign_fail!(i8, overflowing_div_assign_i8_fail);
overflowing_div_assign_fail!(i16, overflowing_div_assign_i16_fail);
overflowing_div_assign_fail!(i32, overflowing_div_assign_i32_fail);
overflowing_div_assign_fail!(i64, overflowing_div_assign_i64_fail);
overflowing_div_assign_fail!(i128, overflowing_div_assign_i128_fail);
overflowing_div_assign_fail!(isize, overflowing_div_assign_isize_fail);
