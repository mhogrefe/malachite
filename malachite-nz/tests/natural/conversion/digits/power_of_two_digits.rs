use malachite_base::num::arithmetic::traits::Square;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{
    PowerOfTwoDigitIterable, PowerOfTwoDigitIterator, PowerOfTwoDigits,
};
use malachite_base::strings::ToDebugString;

use malachite_nz::natural::Natural;

#[test]
pub fn test_power_of_two_digits_primitive() {
    let n = Natural::from(107u32);
    assert_eq!(
        PowerOfTwoDigits::<u8>::to_power_of_two_digits_asc(&n, 2),
        &[3, 2, 2, 1]
    );
    let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(&n, 2);
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

    let n = Natural::from(107u32);
    let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(&n, 2);
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next(), Some(3));
    assert_eq!(digits.next(), Some(2));
    assert_eq!(digits.next(), Some(2));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    let n = Natural::ZERO;
    let mut digits = PowerOfTwoDigitIterable::<u32>::power_of_two_digits(&n, 5);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    let n = Natural::from(105u32);
    assert_eq!(
        PowerOfTwoDigits::<u8>::to_power_of_two_digits_asc(&n, 1),
        &[1, 0, 0, 1, 0, 1, 1]
    );
    let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(&n, 1);
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

    let n = Natural::from(105u32);
    let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(&n, 1);
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next(), Some(1));
    assert_eq!(digits.next(), Some(0));
    assert_eq!(digits.next(), Some(0));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next_back(), Some(0));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    let n = Natural::trillion();
    assert_eq!(
        PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&n, 16),
        &[4096, 54437, 232]
    );
    let mut digits = PowerOfTwoDigitIterable::<u64>::power_of_two_digits(&n, 16);
    assert_eq!(digits.next(), Some(4096));
    assert_eq!(digits.next_back(), Some(232));
    assert_eq!(digits.next_back(), Some(54437));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 4096);
    assert_eq!(digits.get(1), 54437);
    assert_eq!(digits.get(2), 232);
    assert_eq!(digits.get(3), 0);
    assert_eq!(digits.get(4), 0);

    let n = Natural::trillion();
    assert_eq!(
        PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&n, 17),
        &[69632, 27218, 58]
    );
    let mut digits = PowerOfTwoDigitIterable::<u64>::power_of_two_digits(&n, 17);
    assert_eq!(digits.next(), Some(69632));
    assert_eq!(digits.next_back(), Some(58));
    assert_eq!(digits.next_back(), Some(27218));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 69632);
    assert_eq!(digits.get(1), 27218);
    assert_eq!(digits.get(2), 58);
    assert_eq!(digits.get(3), 0);
    assert_eq!(digits.get(4), 0);

    let n = Natural::trillion().square();
    assert_eq!(
        PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&n, 32),
        &[2701131776, 466537709, 54210]
    );
    let mut digits = PowerOfTwoDigitIterable::<u64>::power_of_two_digits(&n, 32);
    assert_eq!(digits.next(), Some(2701131776));
    assert_eq!(digits.next_back(), Some(54210));
    assert_eq!(digits.next_back(), Some(466537709));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 2701131776);
    assert_eq!(digits.get(1), 466537709);
    assert_eq!(digits.get(2), 54210);
    assert_eq!(digits.get(3), 0);
    assert_eq!(digits.get(4), 0);

    let n = Natural::trillion().square();
    assert_eq!(
        PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&n, 64),
        &[2003764205206896640, 54210]
    );
    let mut digits = PowerOfTwoDigitIterable::<u64>::power_of_two_digits(&n, 64);
    assert_eq!(digits.next(), Some(2003764205206896640));
    assert_eq!(digits.next_back(), Some(54210));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 2003764205206896640);
    assert_eq!(digits.get(1), 54210);
    assert_eq!(digits.get(2), 0);
    assert_eq!(digits.get(3), 0);

    let n = Natural::trillion().square();
    assert_eq!(
        PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&n, 37),
        &[58535706624, 129132033639, 52]
    );
    let mut digits = PowerOfTwoDigitIterable::<u64>::power_of_two_digits(&n, 37);
    assert_eq!(digits.next(), Some(58535706624));
    assert_eq!(digits.next_back(), Some(52));
    assert_eq!(digits.next_back(), Some(129132033639));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 58535706624);
    assert_eq!(digits.get(1), 129132033639);
    assert_eq!(digits.get(2), 52);
    assert_eq!(digits.get(3), 0);
    assert_eq!(digits.get(4), 0);
}

macro_rules! power_of_two_digits_primitive_fail_helper {
    ($t:ident, $fail_1:ident, $fail_2:ident) => {
        #[test]
        #[should_panic]
        fn $fail_1() {
            PowerOfTwoDigitIterable::<$t>::power_of_two_digits(&Natural::from(107u32), 0);
        }

        #[test]
        #[should_panic]
        fn $fail_2() {
            PowerOfTwoDigitIterable::<$t>::power_of_two_digits(&Natural::from(107u32), 200);
        }
    };
}

power_of_two_digits_primitive_fail_helper!(
    u8,
    natural_power_of_two_digits_u8_fail_1,
    natural_power_of_two_digits_u8_fail_2
);
power_of_two_digits_primitive_fail_helper!(
    u16,
    natural_power_of_two_digits_u16_fail_1,
    natural_power_of_two_digits_u16_fail_2
);
power_of_two_digits_primitive_fail_helper!(
    u32,
    natural_power_of_two_digits_u32_fail_1,
    natural_power_of_two_digits_u32_fail_2
);
power_of_two_digits_primitive_fail_helper!(
    u64,
    natural_power_of_two_digits_u64_fail_1,
    natural_power_of_two_digits_u64_fail_2
);
power_of_two_digits_primitive_fail_helper!(
    usize,
    natural_power_of_two_digits_usize_fail_1,
    natural_power_of_two_digits_usize_fail_2
);

#[test]
pub fn test_power_of_two_digits() {
    let n = Natural::from(107u32);
    assert_eq!(
        PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&n, 2).to_debug_string(),
        "[3, 2, 2, 1]"
    );
    let mut digits = PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&n, 2);
    assert_eq!(digits.next().unwrap(), 3);
    assert_eq!(digits.next_back().unwrap(), 1);
    assert_eq!(digits.next_back().unwrap(), 2);
    assert_eq!(digits.next().unwrap(), 2);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 3);
    assert_eq!(digits.get(1), 2);
    assert_eq!(digits.get(2), 2);
    assert_eq!(digits.get(3), 1);
    assert_eq!(digits.get(4), 0);
    assert_eq!(digits.get(5), 0);

    let n = Natural::from(107u32);
    let mut digits = PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&n, 2);
    assert_eq!(digits.next_back().unwrap(), 1);
    assert_eq!(digits.next().unwrap(), 3);
    assert_eq!(digits.next().unwrap(), 2);
    assert_eq!(digits.next().unwrap(), 2);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    let n = Natural::ZERO;
    let mut digits = PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&n, 5);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    let n = Natural::from(105u32);
    assert_eq!(
        PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&n, 1).to_debug_string(),
        "[1, 0, 0, 1, 0, 1, 1]"
    );
    let mut digits = PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&n, 1);
    assert_eq!(digits.next().unwrap(), 1);
    assert_eq!(digits.next_back().unwrap(), 1);
    assert_eq!(digits.next_back().unwrap(), 1);
    assert_eq!(digits.next_back().unwrap(), 0);
    assert_eq!(digits.next().unwrap(), 0);
    assert_eq!(digits.next().unwrap(), 0);
    assert_eq!(digits.next().unwrap(), 1);
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

    let n = Natural::from(105u32);
    let mut digits = PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&n, 1);
    assert_eq!(digits.next_back().unwrap(), 1);
    assert_eq!(digits.next().unwrap(), 1);
    assert_eq!(digits.next().unwrap(), 0);
    assert_eq!(digits.next().unwrap(), 0);
    assert_eq!(digits.next_back().unwrap(), 1);
    assert_eq!(digits.next_back().unwrap(), 0);
    assert_eq!(digits.next_back().unwrap(), 1);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    let n = Natural::trillion();
    assert_eq!(
        PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&n, 16).to_debug_string(),
        "[4096, 54437, 232]"
    );
    let mut digits = PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&n, 16);
    assert_eq!(digits.next().unwrap(), 4096);
    assert_eq!(digits.next_back().unwrap(), 232);
    assert_eq!(digits.next_back().unwrap(), 54437);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 4096);
    assert_eq!(digits.get(1), 54437);
    assert_eq!(digits.get(2), 232);
    assert_eq!(digits.get(3), 0);
    assert_eq!(digits.get(4), 0);

    let n = Natural::trillion();
    assert_eq!(
        PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&n, 17).to_debug_string(),
        "[69632, 27218, 58]"
    );
    let mut digits = PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&n, 17);
    assert_eq!(digits.next().unwrap(), 69632);
    assert_eq!(digits.next_back().unwrap(), 58);
    assert_eq!(digits.next_back().unwrap(), 27218);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 69632);
    assert_eq!(digits.get(1), 27218);
    assert_eq!(digits.get(2), 58);
    assert_eq!(digits.get(3), 0);
    assert_eq!(digits.get(4), 0);

    let n = Natural::trillion().square();
    assert_eq!(
        PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&n, 32).to_debug_string(),
        "[2701131776, 466537709, 54210]"
    );
    let mut digits = PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&n, 32);
    assert_eq!(digits.next().unwrap(), 2701131776u32);
    assert_eq!(digits.next_back().unwrap(), 54210u32);
    assert_eq!(digits.next_back().unwrap(), 466537709u32);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 2701131776u32);
    assert_eq!(digits.get(1), 466537709u32);
    assert_eq!(digits.get(2), 54210u32);
    assert_eq!(digits.get(3), 0u32);
    assert_eq!(digits.get(4), 0u32);

    let n = Natural::trillion().square();
    assert_eq!(
        PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&n, 64).to_debug_string(),
        "[2003764205206896640, 54210]"
    );
    let mut digits = PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&n, 64);
    assert_eq!(digits.next().unwrap(), 2003764205206896640u64);
    assert_eq!(digits.next_back().unwrap(), 54210u64);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 2003764205206896640u64);
    assert_eq!(digits.get(1), 54210u64);
    assert_eq!(digits.get(2), 0u64);
    assert_eq!(digits.get(3), 0u64);

    let n = Natural::trillion().square();
    assert_eq!(
        PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&n, 37).to_debug_string(),
        "[58535706624, 129132033639, 52]"
    );
    let mut digits = PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&n, 37);
    assert_eq!(digits.next().unwrap(), 58535706624u64);
    assert_eq!(digits.next_back().unwrap(), 52u64);
    assert_eq!(digits.next_back().unwrap(), 129132033639u64);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 58535706624u64);
    assert_eq!(digits.get(1), 129132033639u64);
    assert_eq!(digits.get(2), 52u64);
    assert_eq!(digits.get(3), 0u64);
    assert_eq!(digits.get(4), 0u64);
}

#[test]
#[should_panic]
fn natural_power_of_two_digits_natural_fail() {
    PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&Natural::from(107u32), 0);
}
