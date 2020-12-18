use std::panic::catch_unwind;

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    PowerOfTwoDigitIterable, PowerOfTwoDigitIterator, PowerOfTwoDigits,
};

#[test]
pub fn test_power_of_two_digits() {
    assert_eq!(
        PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&107u32, 2),
        &[3, 2, 2, 1]
    );
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

    assert_eq!(
        PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&105u32, 1),
        &[1, 0, 0, 1, 0, 1, 1]
    );
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

fn power_of_two_digits_fail_helper<
    T: PowerOfTwoDigitIterable<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    assert_panic!(PowerOfTwoDigitIterable::<U>::power_of_two_digits(
        T::exact_from(107),
        0
    ));
    assert_panic!(PowerOfTwoDigitIterable::<U>::power_of_two_digits(
        T::exact_from(107),
        200
    ));
}

#[test]
fn power_of_two_digits_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(power_of_two_digits_fail_helper);
}
