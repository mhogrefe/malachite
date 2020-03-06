use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{PowerOfTwoDigitIterable, PowerOfTwoDigitIterator};

#[test]
pub fn test_power_of_two_digits() {
    let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(107u32, 2);
    assert_eq!(digits.next(), Some(3));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next_back(), Some(2));
    assert_eq!(digits.next(), Some(2));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 3);
    assert_eq!(digits.get(1), 2);
    assert_eq!(digits.get(2), 2);
    assert_eq!(digits.get(3), 1);
    assert_eq!(digits.get(4), 0);
    assert_eq!(digits.get(5), 0);

    let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(107u32, 2);
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next(), Some(3));
    assert_eq!(digits.next(), Some(2));
    assert_eq!(digits.next(), Some(2));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    let mut digits = PowerOfTwoDigitIterable::<u32>::power_of_two_digits(0u8, 5);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(105u32, 1);
    assert_eq!(digits.next(), Some(1));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next_back(), Some(0));
    assert_eq!(digits.next(), Some(0));
    assert_eq!(digits.next(), Some(0));
    assert_eq!(digits.next(), Some(1));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 1);
    assert_eq!(digits.get(1), 0);
    assert_eq!(digits.get(2), 0);
    assert_eq!(digits.get(3), 1);
    assert_eq!(digits.get(4), 0);
    assert_eq!(digits.get(5), 1);
    assert_eq!(digits.get(6), 1);
    assert_eq!(digits.get(7), 0);
    assert_eq!(digits.get(8), 0);

    let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(105u32, 1);
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next(), Some(1));
    assert_eq!(digits.next(), Some(0));
    assert_eq!(digits.next(), Some(0));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next_back(), Some(0));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);
}

macro_rules! power_of_two_digits_fail_helper {
    ($t:ident, $u:ident, $fail_1:ident, $fail_2:ident) => {
        #[test]
        #[should_panic]
        fn $fail_1() {
            PowerOfTwoDigitIterable::<$u>::power_of_two_digits($t::exact_from(107), 0);
        }

        #[test]
        #[should_panic]
        fn $fail_2() {
            PowerOfTwoDigitIterable::<$u>::power_of_two_digits($t::exact_from(107), 200);
        }
    };
}

power_of_two_digits_fail_helper!(
    u8,
    u8,
    power_of_two_digits_u8_u8_fail_1,
    power_of_two_digits_u8_u8_fail_2
);
power_of_two_digits_fail_helper!(
    u8,
    u16,
    power_of_two_digits_u8_u16_fail_1,
    power_of_two_digits_u8_u16_fail_2
);
power_of_two_digits_fail_helper!(
    u8,
    u32,
    power_of_two_digits_u8_u32_fail_1,
    power_of_two_digits_u8_u32_fail_2
);
power_of_two_digits_fail_helper!(
    u8,
    u64,
    power_of_two_digits_u8_u64_fail_1,
    power_of_two_digits_u8_u64_fail_2
);
power_of_two_digits_fail_helper!(
    u8,
    u128,
    power_of_two_digits_u8_u128_fail_1,
    power_of_two_digits_u8_u128_fail_2
);
power_of_two_digits_fail_helper!(
    u8,
    usize,
    power_of_two_digits_u8_usize_fail_1,
    power_of_two_digits_u8_usize_fail_2
);
power_of_two_digits_fail_helper!(
    u16,
    u8,
    power_of_two_digits_u16_u8_fail_1,
    power_of_two_digits_u16_u8_fail_2
);
power_of_two_digits_fail_helper!(
    u16,
    u16,
    power_of_two_digits_u16_u16_fail_1,
    power_of_two_digits_u16_u16_fail_2
);
power_of_two_digits_fail_helper!(
    u16,
    u32,
    power_of_two_digits_u16_u32_fail_1,
    power_of_two_digits_u16_u32_fail_2
);
power_of_two_digits_fail_helper!(
    u16,
    u64,
    power_of_two_digits_u16_u64_fail_1,
    power_of_two_digits_u16_u64_fail_2
);
power_of_two_digits_fail_helper!(
    u16,
    u128,
    power_of_two_digits_u16_u128_fail_1,
    power_of_two_digits_u16_u128_fail_2
);
power_of_two_digits_fail_helper!(
    u16,
    usize,
    power_of_two_digits_u16_usize_fail_1,
    power_of_two_digits_u16_usize_fail_2
);
power_of_two_digits_fail_helper!(
    u32,
    u8,
    power_of_two_digits_u32_u8_fail_1,
    power_of_two_digits_u32_u8_fail_2
);
power_of_two_digits_fail_helper!(
    u32,
    u16,
    power_of_two_digits_u32_u16_fail_1,
    power_of_two_digits_u32_u16_fail_2
);
power_of_two_digits_fail_helper!(
    u32,
    u32,
    power_of_two_digits_u32_u32_fail_1,
    power_of_two_digits_u32_u32_fail_2
);
power_of_two_digits_fail_helper!(
    u32,
    u64,
    power_of_two_digits_u32_u64_fail_1,
    power_of_two_digits_u32_u64_fail_2
);
power_of_two_digits_fail_helper!(
    u32,
    u128,
    power_of_two_digits_u32_u128_fail_1,
    power_of_two_digits_u32_u128_fail_2
);
power_of_two_digits_fail_helper!(
    u32,
    usize,
    power_of_two_digits_u32_usize_fail_1,
    power_of_two_digits_u32_usize_fail_2
);
power_of_two_digits_fail_helper!(
    u64,
    u8,
    power_of_two_digits_u64_u8_fail_1,
    power_of_two_digits_u64_u8_fail_2
);
power_of_two_digits_fail_helper!(
    u64,
    u16,
    power_of_two_digits_u64_u16_fail_1,
    power_of_two_digits_u64_u16_fail_2
);
power_of_two_digits_fail_helper!(
    u64,
    u32,
    power_of_two_digits_u64_u32_fail_1,
    power_of_two_digits_u64_u32_fail_2
);
power_of_two_digits_fail_helper!(
    u64,
    u64,
    power_of_two_digits_u64_u64_fail_1,
    power_of_two_digits_u64_u64_fail_2
);
power_of_two_digits_fail_helper!(
    u64,
    u128,
    power_of_two_digits_u64_u128_fail_1,
    power_of_two_digits_u64_u128_fail_2
);
power_of_two_digits_fail_helper!(
    u64,
    usize,
    power_of_two_digits_u64_usize_fail_1,
    power_of_two_digits_u64_usize_fail_2
);
power_of_two_digits_fail_helper!(
    u128,
    u8,
    power_of_two_digits_u128_u8_fail_1,
    power_of_two_digits_u128_u8_fail_2
);
power_of_two_digits_fail_helper!(
    u128,
    u16,
    power_of_two_digits_u128_u16_fail_1,
    power_of_two_digits_u128_u16_fail_2
);
power_of_two_digits_fail_helper!(
    u128,
    u32,
    power_of_two_digits_u128_u32_fail_1,
    power_of_two_digits_u128_u32_fail_2
);
power_of_two_digits_fail_helper!(
    u128,
    u64,
    power_of_two_digits_u128_u64_fail_1,
    power_of_two_digits_u128_u64_fail_2
);
power_of_two_digits_fail_helper!(
    u128,
    u128,
    power_of_two_digits_u128_u128_fail_1,
    power_of_two_digits_u128_u128_fail_2
);
power_of_two_digits_fail_helper!(
    u128,
    usize,
    power_of_two_digits_u128_usize_fail_1,
    power_of_two_digits_u128_usize_fail_2
);
power_of_two_digits_fail_helper!(
    usize,
    u8,
    power_of_two_digits_usize_u8_fail_1,
    power_of_two_digits_usize_u8_fail_2
);
power_of_two_digits_fail_helper!(
    usize,
    u16,
    power_of_two_digits_usize_u16_fail_1,
    power_of_two_digits_usize_u16_fail_2
);
power_of_two_digits_fail_helper!(
    usize,
    u32,
    power_of_two_digits_usize_u32_fail_1,
    power_of_two_digits_usize_u32_fail_2
);
power_of_two_digits_fail_helper!(
    usize,
    u64,
    power_of_two_digits_usize_u64_fail_1,
    power_of_two_digits_usize_u64_fail_2
);
power_of_two_digits_fail_helper!(
    usize,
    u128,
    power_of_two_digits_usize_u128_fail_1,
    power_of_two_digits_usize_u128_fail_2
);
power_of_two_digits_fail_helper!(
    usize,
    usize,
    power_of_two_digits_usize_usize_fail_1,
    power_of_two_digits_usize_usize_fail_2
);
