use std::fmt::Debug;

use malachite_base::comparison::{Max, Min};
use malachite_base::num::conversion::traits::CheckedFrom;

#[test]
pub fn test_checked_from() {
    fn test_single<T: Copy + Debug + Eq>(n: T)
    where
        T: CheckedFrom<T>,
    {
        assert_eq!(T::checked_from(n), Some(n));
    };
    test_single(0u8);
    test_single(5u64);
    test_single(1_000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double<T, U: Debug + Eq>(n_in: T, n_out: Option<U>)
    where
        U: CheckedFrom<T>,
    {
        assert_eq!(U::checked_from(n_in), n_out);
    };
    test_double(0u8, Some(0u16));
    test_double(1_000u16, Some(1_000i32));
    test_double(-5i16, Some(-5i8));
    test_double(255u8, Some(255u64));

    test_double::<_, u32>(-1i8, None);
    test_double::<_, u16>(u32::MAX, None);
    test_double::<_, u32>(i32::MIN, None);
    test_double::<_, u16>(i32::MIN, None);
    test_double::<_, i16>(i32::MIN, None);
    test_double::<_, u32>(-5i32, None);
    test_double::<_, i32>(3_000_000_000u32, None);
    test_double::<_, i8>(-1000i16, None);
}
