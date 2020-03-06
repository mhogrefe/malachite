use malachite_base::num::arithmetic::traits::DivRound;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{
    PowerOfTwoDigitIterable, PowerOfTwoDigitIterator, PowerOfTwoDigits,
};
use malachite_base::round::RoundingMode;
use rand::Rand;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_u64_and_small_unsigned_var_1, pairs_of_unsigned_and_small_u64_var_1,
    triples_of_unsigned_small_u64_and_small_u64_var_1,
    triples_of_unsigned_small_u64_and_vec_of_bool_var_1,
};

fn power_of_two_digit_properties_helper<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned>()
where
    T: PowerOfTwoDigitIterable<U> + PowerOfTwoDigits<U>,
{
    test_properties(
        pairs_of_unsigned_and_small_u64_var_1::<T, U>,
        |&(u, log_base)| {
            let significant_digits = usize::exact_from(
                u.significant_bits()
                    .div_round(log_base, RoundingMode::Ceiling),
            );
            assert_eq!(
                PowerOfTwoDigitIterable::<U>::power_of_two_digits(u, log_base).size_hint(),
                (significant_digits, Some(significant_digits))
            );
        },
    );

    test_properties(
        triples_of_unsigned_small_u64_and_vec_of_bool_var_1::<T, U>,
        |&(u, log_base, ref bs)| {
            let mut digits = PowerOfTwoDigitIterable::<U>::power_of_two_digits(u, log_base);
            let mut digit_vec = Vec::new();
            let mut i = 0;
            for &b in bs {
                if b {
                    digit_vec.insert(i, digits.next().unwrap());
                    i += 1;
                } else {
                    digit_vec.insert(i, digits.next_back().unwrap())
                }
            }
            assert!(digits.next().is_none());
            assert!(digits.next_back().is_none());
            assert_eq!(
                PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(&u, log_base),
                digit_vec
            );
        },
    );

    test_properties(
        triples_of_unsigned_small_u64_and_small_u64_var_1::<T, U>,
        |&(u, log_base, i)| {
            let digits = PowerOfTwoDigitIterable::<U>::power_of_two_digits(u, log_base);
            if i < u
                .significant_bits()
                .div_round(log_base, RoundingMode::Ceiling)
            {
                assert_eq!(
                    digits.get(i),
                    PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(&u, log_base)
                        [usize::exact_from(i)]
                );
            } else {
                assert_eq!(digits.get(i), U::ZERO);
            }
        },
    );

    test_properties_no_special(
        pairs_of_u64_and_small_unsigned_var_1::<U, u64>,
        |&(log_base, i)| {
            let digits = PowerOfTwoDigitIterable::<U>::power_of_two_digits(T::ZERO, log_base);
            assert_eq!(digits.get(i), U::ZERO);
        },
    );
}

#[test]
fn power_of_two_digit_properties() {
    power_of_two_digit_properties_helper::<u8, u8>();
    power_of_two_digit_properties_helper::<u8, u16>();
    power_of_two_digit_properties_helper::<u8, u32>();
    power_of_two_digit_properties_helper::<u8, u64>();
    power_of_two_digit_properties_helper::<u8, u128>();
    power_of_two_digit_properties_helper::<u8, usize>();
    power_of_two_digit_properties_helper::<u16, u8>();
    power_of_two_digit_properties_helper::<u16, u16>();
    power_of_two_digit_properties_helper::<u16, u32>();
    power_of_two_digit_properties_helper::<u16, u64>();
    power_of_two_digit_properties_helper::<u16, u128>();
    power_of_two_digit_properties_helper::<u16, usize>();
    power_of_two_digit_properties_helper::<u32, u8>();
    power_of_two_digit_properties_helper::<u32, u16>();
    power_of_two_digit_properties_helper::<u32, u32>();
    power_of_two_digit_properties_helper::<u32, u64>();
    power_of_two_digit_properties_helper::<u32, u128>();
    power_of_two_digit_properties_helper::<u32, usize>();
    power_of_two_digit_properties_helper::<u64, u8>();
    power_of_two_digit_properties_helper::<u64, u16>();
    power_of_two_digit_properties_helper::<u64, u32>();
    power_of_two_digit_properties_helper::<u64, u64>();
    power_of_two_digit_properties_helper::<u64, u128>();
    power_of_two_digit_properties_helper::<u64, usize>();
    power_of_two_digit_properties_helper::<usize, u8>();
    power_of_two_digit_properties_helper::<usize, u16>();
    power_of_two_digit_properties_helper::<usize, u32>();
    power_of_two_digit_properties_helper::<usize, u64>();
    power_of_two_digit_properties_helper::<usize, u128>();
    power_of_two_digit_properties_helper::<usize, usize>();
}
