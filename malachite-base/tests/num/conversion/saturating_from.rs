use std::fmt::Debug;

use malachite_base::comparison::{Max, Min};
use malachite_base::num::conversion::traits::SaturatingFrom;

#[test]
pub fn test_saturating_from() {
    fn test_single<T: Copy + Debug + Eq>(n: T)
    where
        T: SaturatingFrom<T>,
    {
        assert_eq!(T::saturating_from(n), n);
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
        U: SaturatingFrom<T>,
    {
        assert_eq!(U::saturating_from(n_in), n_out);
    };
    test_double(0u8, 0u16);
    test_double(1_000u16, 1_000i32);
    test_double(-5i16, -5i8);
    test_double(255u8, 255u64);

    test_double(-1i8, 0u32);
    test_double(u32::MAX, u16::MAX);
    test_double(i32::MIN, 0u32);
    test_double(i32::MIN, 0u16);
    test_double(i32::MIN, i16::MIN);
    test_double(-5i32, 0u32);
    test_double(3_000_000_000u32, i32::MAX);
    test_double(-1000i16, i8::MIN);
}
