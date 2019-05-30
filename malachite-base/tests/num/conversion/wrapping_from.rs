use std::fmt::Debug;

use malachite_base::comparison::{Max, Min};
use malachite_base::num::conversion::traits::WrappingFrom;

#[test]
pub fn test_wrapping_from() {
    fn test_single<T: Copy + Debug + Eq>(n: T)
    where
        T: WrappingFrom<T>,
    {
        assert_eq!(T::wrapping_from(n), n);
    };
    test_single(0u8);
    test_single(5u64);
    test_single(1_000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double<T, U: Debug + Eq>(n_in: T, n_out: U)
    where
        U: WrappingFrom<T>,
    {
        assert_eq!(U::wrapping_from(n_in), n_out);
    };
    test_double(0u8, 0u16);
    test_double(1_000u16, 1_000i32);
    test_double(-5i16, -5i8);
    test_double(255u8, 255u64);

    test_double(-1i8, u32::MAX);
    test_double(u32::MAX, u16::MAX);
    test_double(i32::MIN, 2_147_483_648u32);
    test_double(i32::MIN, 0u16);
    test_double(i32::MIN, 0i16);
    test_double(-5i32, 4_294_967_291u32);
    test_double(3_000_000_000u32, -1_294_967_296i32);
    test_double(-1000i16, 24i8);
}
