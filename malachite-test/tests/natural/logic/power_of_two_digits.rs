use malachite_base::num::arithmetic::traits::DivRound;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{
    PowerOfTwoDigitIterable, PowerOfTwoDigitIterator, PowerOfTwoDigits, SignificantBits,
};
use malachite_base::rounding_mode::RoundingMode;
use malachite_nz::natural::Natural;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_small_unsigneds_single_var_1, pairs_of_u64_and_small_unsigned_var_1,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_small_u64_var_3, pairs_of_natural_and_small_unsigned_var_3,
    triples_of_natural_small_u64_and_small_u64_var_2,
    triples_of_natural_small_u64_and_small_u64_var_3,
    triples_of_natural_small_u64_and_vec_of_bool_var_1,
    triples_of_natural_small_u64_and_vec_of_bool_var_2,
};

macro_rules! properties {
    ($t:ident) => {
        test_properties(
            pairs_of_natural_and_small_u64_var_3::<$t>,
            |&(ref n, log_base)| {
                let significant_digits = usize::exact_from(
                    n.significant_bits()
                        .div_round(log_base, RoundingMode::Ceiling),
                );
                assert_eq!(
                    PowerOfTwoDigitIterable::<$t>::power_of_two_digits(n, log_base).size_hint(),
                    (significant_digits, Some(significant_digits))
                );
            },
        );

        test_properties(
            triples_of_natural_small_u64_and_vec_of_bool_var_1::<$t>,
            |&(ref n, log_base, ref bs)| {
                let mut digits = PowerOfTwoDigitIterable::<$t>::power_of_two_digits(n, log_base);
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
                    PowerOfTwoDigits::<$t>::to_power_of_two_digits_asc(n, log_base),
                    digit_vec
                );
            },
        );

        test_properties(
            triples_of_natural_small_u64_and_small_u64_var_2::<$t>,
            |&(ref n, log_base, i)| {
                let digits = PowerOfTwoDigitIterable::<$t>::power_of_two_digits(n, log_base);
                if i < n
                    .significant_bits()
                    .div_round(log_base, RoundingMode::Ceiling)
                {
                    assert_eq!(
                        digits.get(i),
                        PowerOfTwoDigits::<$t>::to_power_of_two_digits_asc(n, log_base)
                            [usize::exact_from(i)],
                    );
                } else {
                    assert_eq!(digits.get(i), 0);
                }
            },
        );

        test_properties_no_special(
            pairs_of_u64_and_small_unsigned_var_1::<$t, u64>,
            |&(log_base, i)| {
                let n = Natural::ZERO;
                let digits = PowerOfTwoDigitIterable::<$t>::power_of_two_digits(&n, log_base);
                assert_eq!(digits.get(i), 0);
            },
        );
    };
}

#[test]
fn power_of_two_digits_primitive_properties() {
    apply_to_unsigneds!(properties);
}

#[test]
fn power_of_two_digits_properties() {
    test_properties(
        pairs_of_natural_and_small_unsigned_var_3,
        |&(ref n, log_base)| {
            let significant_digits = usize::exact_from(
                n.significant_bits()
                    .div_round(log_base, RoundingMode::Ceiling),
            );
            assert_eq!(
                PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(n, log_base).size_hint(),
                (significant_digits, Some(significant_digits))
            );
        },
    );

    test_properties(
        triples_of_natural_small_u64_and_vec_of_bool_var_2,
        |&(ref n, log_base, ref bs)| {
            let mut digits = PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(n, log_base);
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
                PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(n, log_base),
                digit_vec
            );
        },
    );

    test_properties(
        triples_of_natural_small_u64_and_small_u64_var_3,
        |&(ref n, log_base, i)| {
            let digits = PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(n, log_base);
            if i < n
                .significant_bits()
                .div_round(log_base, RoundingMode::Ceiling)
            {
                assert_eq!(
                    digits.get(i),
                    PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(n, log_base)
                        [usize::exact_from(i)],
                );
            } else {
                assert_eq!(digits.get(i), 0);
            }
        },
    );

    test_properties_no_special(pairs_of_small_unsigneds_single_var_1, |&(log_base, i)| {
        let n = Natural::ZERO;
        let digits = PowerOfTwoDigitIterable::<Natural>::power_of_two_digits(&n, log_base);
        assert_eq!(digits.get(i), 0);
    });
}
