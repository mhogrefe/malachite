use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::One;

#[test]
fn test_div_exact() {
    fn test<T>(x: T, y: T, out: T)
    where
        T: PrimitiveInteger,
    {
        assert_eq!(x.div_exact(y), out);

        let mut x = x;
        x.div_exact_assign(y);
        assert_eq!(x, out);
    };
    test::<u8>(0, 123, 0);
    test::<u16>(123, 1, 123);
    test::<u32>(123, 123, 1);
    test::<usize>(56_088, 123, 456);
    test::<u64>(0, 1_000_000_000_000, 0);
    test::<u128>(1_000_000_000_000, 1, 1_000_000_000_000);
    test::<usize>(1_000_000_000_000, 1_000_000_000_000, 1);
    test::<usize>(123_000_000_000_000, 1_000_000_000_000, 123);
    test::<usize>(123_000_000_000_000, 123, 1_000_000_000_000);
    test::<u128>(
        121_932_631_112_635_269_000_000,
        123_456_789_000,
        987_654_321_000,
    );
    test::<u64>(0x1_ffff_fffe, 0xffff_ffff, 2);
    test::<u64>(18_446_744_065_119_617_025, 0xffff_ffff, 0xffff_ffff);

    test::<i8>(0, -123, 0);
    test::<i16>(123, -1, -123);
    test::<i32>(123, -123, -1);
    test::<isize>(56_088, -123, -456);
    test::<i64>(0, -1_000_000_000_000, 0);
    test::<i128>(1_000_000_000_000, -1, -1_000_000_000_000);
    test::<isize>(1_000_000_000_000, -1_000_000_000_000, -1);
    test::<isize>(123_000_000_000_000, -1_000_000_000_000, -123);
    test::<isize>(123_000_000_000_000, -123, -1_000_000_000_000);
    test::<i128>(
        121_932_631_112_635_269_000_000,
        -123_456_789_000,
        -987_654_321_000,
    );
    test::<i64>(0x1_ffff_fffe, -0xffff_ffff, -2);
    test::<i128>(18_446_744_065_119_617_025, -0xffff_ffff, -0xffff_ffff);

    test::<i16>(-123, 1, -123);
    test::<i32>(-123, 123, -1);
    test::<isize>(-56_088, 123, -456);
    test::<i128>(-1_000_000_000_000, 1, -1_000_000_000_000);
    test::<isize>(-1_000_000_000_000, 1_000_000_000_000, -1);
    test::<isize>(-123_000_000_000_000, 1_000_000_000_000, -123);
    test::<isize>(-123_000_000_000_000, 123, -1_000_000_000_000);
    test::<i128>(
        -121_932_631_112_635_269_000_000,
        123_456_789_000,
        -987_654_321_000,
    );
    test::<i64>(-0x1_ffff_fffe, 0xffff_ffff, -2);
    test::<i128>(-18_446_744_065_119_617_025, 0xffff_ffff, -0xffff_ffff);

    test::<i16>(-123, -1, 123);
    test::<i32>(-123, -123, 1);
    test::<isize>(-56_088, -123, 456);
    test::<i128>(-1_000_000_000_000, -1, 1_000_000_000_000);
    test::<isize>(-1_000_000_000_000, -1_000_000_000_000, 1);
    test::<isize>(-123_000_000_000_000, -1_000_000_000_000, 123);
    test::<isize>(-123_000_000_000_000, -123, 1_000_000_000_000);
    test::<i128>(
        -121_932_631_112_635_269_000_000,
        -123_456_789_000,
        987_654_321_000,
    );
    test::<i64>(-0x1_ffff_fffe, -0xffff_ffff, 2);
    test::<i128>(-18_446_744_065_119_617_025, -0xffff_ffff, 0xffff_ffff);
}

macro_rules! div_exact_fail {
    ($t:ident, $div_exact_fail:ident) => {
        #[test]
        #[should_panic]
        fn $div_exact_fail() {
            $t::ONE.div_exact(0);
        }
    };
}
div_exact_fail!(u8, div_exact_u8_fail);
div_exact_fail!(u16, div_exact_u16_fail);
div_exact_fail!(u32, div_exact_u32_fail);
div_exact_fail!(u64, div_exact_u64_fail);
div_exact_fail!(u128, div_exact_u128_fail);
div_exact_fail!(usize, div_exact_usize_fail);
div_exact_fail!(i8, div_exact_i8_fail);
div_exact_fail!(i16, div_exact_i16_fail);
div_exact_fail!(i32, div_exact_i32_fail);
div_exact_fail!(i64, div_exact_i64_fail);
div_exact_fail!(i128, div_exact_i128_fail);
div_exact_fail!(isize, div_exact_isize_fail);

macro_rules! div_exact_assign_fail {
    ($t:ident, $div_exact_assign_fail:ident) => {
        #[test]
        #[should_panic]
        fn $div_exact_assign_fail() {
            $t::ONE.div_exact_assign(0);
        }
    };
}
div_exact_assign_fail!(u8, div_exact_assign_u8_fail);
div_exact_assign_fail!(u16, div_exact_assign_u16_fail);
div_exact_assign_fail!(u32, div_exact_assign_u32_fail);
div_exact_assign_fail!(u64, div_exact_assign_u64_fail);
div_exact_assign_fail!(u128, div_exact_assign_u128_fail);
div_exact_assign_fail!(usize, div_exact_assign_usize_fail);
div_exact_assign_fail!(i8, div_exact_assign_i8_fail);
div_exact_assign_fail!(i16, div_exact_assign_i16_fail);
div_exact_assign_fail!(i32, div_exact_assign_i32_fail);
div_exact_assign_fail!(i64, div_exact_assign_i64_fail);
div_exact_assign_fail!(i128, div_exact_assign_i128_fail);
div_exact_assign_fail!(isize, div_exact_assign_isize_fail);
