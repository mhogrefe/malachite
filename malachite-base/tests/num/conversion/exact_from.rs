use std::fmt::Debug;

use malachite_base::comparison::{Max, Min};
use malachite_base::num::conversion::traits::ExactFrom;

#[test]
pub fn test_exact_from() {
    fn test_single<T: Copy + Debug + Eq>(n: T)
    where
        T: ExactFrom<T>,
    {
        assert_eq!(T::exact_from(n), n);
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
        U: ExactFrom<T>,
    {
        assert_eq!(U::exact_from(n_in), n_out);
    };
    test_double(0u8, 0u16);
    test_double(1_000u16, 1_000i32);
    test_double(-5i16, -5i8);
    test_double(255u8, 255u64);
}

#[test]
#[should_panic]
fn exact_from_fail_1() {
    u32::exact_from(-1i8);
}

#[test]
#[should_panic]
fn exact_from_fail_2() {
    u16::exact_from(u32::MAX);
}

#[test]
#[should_panic]
fn exact_from_fail_3() {
    u32::exact_from(i32::MIN);
}

#[test]
#[should_panic]
fn exact_from_fail_4() {
    u16::exact_from(i32::MIN);
}

#[test]
#[should_panic]
fn exact_from_fail_5() {
    i16::exact_from(i32::MIN);
}

#[test]
#[should_panic]
fn exact_from_fail_6() {
    u32::exact_from(-5i32);
}

#[test]
#[should_panic]
fn exact_from_fail_7() {
    i32::exact_from(3_000_000_000u32);
}

#[test]
#[should_panic]
fn exact_from_fail_8() {
    i8::exact_from(-1000i16);
}
