use std::fmt::Debug;

use malachite_base::comparison::{Max, Min};
use malachite_base::num::conversion::traits::ConvertibleFrom;

#[test]
pub fn test_wrapping_from() {
    fn test_single<T: Copy + Debug>(n: T)
    where
        T: ConvertibleFrom<T>,
    {
        assert!(T::convertible_from(n));
    };
    test_single(0u8);
    test_single(5u64);
    test_single(1_000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double<T, U>(n_in: T, convertible: bool)
    where
        U: ConvertibleFrom<T>,
    {
        assert_eq!(U::convertible_from(n_in), convertible);
    };
    test_double::<_, u16>(0u8, true);
    test_double::<_, i32>(1_000u16, true);
    test_double::<_, i8>(-5i16, true);
    test_double::<_, u64>(255u8, true);

    test_double::<_, u32>(-1i8, false);
    test_double::<_, u16>(u32::MAX, false);
    test_double::<_, u32>(i32::MIN, false);
    test_double::<_, u16>(i32::MIN, false);
    test_double::<_, i16>(i32::MIN, false);
    test_double::<_, u32>(-5i32, false);
    test_double::<_, i32>(3_000_000_000u32, false);
    test_double::<_, i8>(-1000i16, false);
}
