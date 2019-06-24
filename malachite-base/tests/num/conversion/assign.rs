use std::fmt::Debug;

use malachite_base::comparison::{Max, Min};
use malachite_base::num::conversion::traits::Assign;

#[test]
pub fn test_assign() {
    fn test<T: Copy + Eq + Debug, U: Copy>(old: T, n: U)
    where
        T: Assign<U>,
        T: From<U>,
    {
        let mut old = old;
        old.assign(n);
        assert_eq!(old, T::from(n));
    };
    test(2u8, 0u8);
    test(100u64, 5u64);
    test(0u32, 1_000u32);
    test(95u8, 123u8);
    test(123i16, -123i16);
    test(-100i64, i64::MIN);
    test(23usize, usize::MAX);

    test(3u16, 0u8);
    test(-5i32, 1_000u16);
    test(-5i32, 10i16);
}
