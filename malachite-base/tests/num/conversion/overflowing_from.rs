use std::fmt::Debug;

use malachite_base::comparison::{Max, Min};
use malachite_base::conversion::OverflowingFrom;

#[test]
pub fn test_overflowing_from() {
    fn test_single<T: Copy + Debug + Eq>(n: T)
    where
        T: OverflowingFrom<T>,
    {
        assert_eq!(T::overflowing_from(n), (n, false));
    };
    test_single(0u8);
    test_single(5u64);
    test_single(1_000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double<T, U: Debug + Eq>(n_in: T, n_out: (U, bool))
    where
        U: OverflowingFrom<T>,
    {
        assert_eq!(U::overflowing_from(n_in), n_out);
    };
    test_double(0u8, (0u16, false));
    test_double(1_000u16, (1_000i32, false));
    test_double(-5i16, (-5i8, false));
    test_double(255u8, (255u64, false));

    test_double(-1i8, (u32::MAX, true));
    test_double(u32::MAX, (u16::MAX, true));
    test_double(i32::MIN, (2_147_483_648u32, true));
    test_double(i32::MIN, (0u16, true));
    test_double(i32::MIN, (0i16, true));
    test_double(-5i32, (4_294_967_291u32, true));
    test_double(3_000_000_000u32, (-1_294_967_296i32, true));
    test_double(-1000i16, (24i8, true));
}
