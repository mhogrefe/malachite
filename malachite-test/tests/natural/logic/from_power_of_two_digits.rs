use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::PowerOfTwoDigits;
use malachite_base::slices::{slice_leading_zeros, slice_trailing_zeros};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use rand::distributions::range::SampleRange;
use rand::Rand;
use rust_wheels::io::readers::parse_vec;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_small_u64_and_small_usize_var_2, pairs_of_u64_and_small_usize_var_1,
    pairs_of_u64_and_unsigned_vec_var_1, pairs_of_u64_and_unsigned_vec_var_2,
    pairs_of_u64_and_unsigned_vec_var_3, vecs_of_unsigned,
};
use malachite_test::inputs::natural::pairs_of_u64_and_natural_vec_var_1;

#[test]
fn test_from_power_of_two_digits_asc() {
    fn test<T: PrimitiveUnsigned, F: Fn(u64, &[T]) -> Natural>(
        from_power_of_two_digits_asc_naive: F,
        log_base: u64,
        digits: &[T],
        out: &str,
    ) where
        Natural: PowerOfTwoDigits<T>,
    {
        assert_eq!(
            Natural::from_power_of_two_digits_asc(log_base, digits).to_string(),
            out
        );
        assert_eq!(
            from_power_of_two_digits_asc_naive(log_base, digits).to_string(),
            out
        );
    };
    test::<u8, _>(Natural::_from_power_of_two_digits_asc_u8_naive, 1, &[], "0");
    test::<u8, _>(
        Natural::_from_power_of_two_digits_asc_u8_naive,
        1,
        &[0, 0, 0],
        "0",
    );
    test::<u16, _>(
        Natural::_from_power_of_two_digits_asc_u16_naive,
        10,
        &[123],
        "123",
    );
    test::<u16, _>(
        Natural::_from_power_of_two_digits_asc_u16_naive,
        1,
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1,
            0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1,
        ],
        "1000000000000",
    );
    test::<u32, _>(
        Natural::_from_power_of_two_digits_asc_u32_naive,
        3,
        &[0, 0, 0, 0, 1, 2, 1, 5, 4, 2, 3, 4, 6, 1, 0],
        "1000000000000",
    );
    test::<u64, _>(
        Natural::_from_power_of_two_digits_asc_u64_naive,
        4,
        &[0, 0, 0, 1, 5, 10, 4, 13, 8, 14],
        "1000000000000",
    );
    test::<u32, _>(
        Natural::_from_power_of_two_digits_asc_u32_naive,
        32,
        &[3567587328, 232],
        "1000000000000",
    );
    test::<u64, _>(
        Natural::_from_power_of_two_digits_asc_u64_naive,
        64,
        &[1000000000000],
        "1000000000000",
    );
    test::<u64, _>(
        Natural::_from_power_of_two_digits_asc_u64_naive,
        64,
        &[2003764205206896640, 54210],
        "1000000000000000000000000",
    );
}

macro_rules! from_power_of_two_digits_asc_fail_helper {
    ($t:ident, $fail_1:ident, $fail_2:ident, $fail_3:ident) => {
        #[test]
        #[should_panic]
        fn $fail_1() {
            Natural::from_power_of_two_digits_asc(0, &[0u32]);
        }

        #[test]
        #[should_panic]
        fn $fail_2() {
            Natural::from_power_of_two_digits_asc(33, &[2u32]);
        }

        #[test]
        #[should_panic]
        fn $fail_3() {
            Natural::from_power_of_two_digits_asc(1, &[2u32]);
        }
    };
}

from_power_of_two_digits_asc_fail_helper!(
    u8,
    from_power_of_two_digits_asc_u8_fail_1,
    from_power_of_two_digits_asc_u8_fail_2,
    from_power_of_two_digits_asc_u8_fail_3
);
from_power_of_two_digits_asc_fail_helper!(
    u16,
    from_power_of_two_digits_asc_u16_fail_1,
    from_power_of_two_digits_asc_u16_fail_2,
    from_power_of_two_digits_asc_u16_fail_3
);
from_power_of_two_digits_asc_fail_helper!(
    u32,
    from_power_of_two_digits_asc_u32_fail_1,
    from_power_of_two_digits_asc_u32_fail_2,
    from_power_of_two_digits_asc_u32_fail_3
);
from_power_of_two_digits_asc_fail_helper!(
    u64,
    from_power_of_two_digits_asc_u64_fail_1,
    from_power_of_two_digits_asc_u64_fail_2,
    from_power_of_two_digits_asc_u64_fail_3
);
from_power_of_two_digits_asc_fail_helper!(
    u128,
    from_power_of_two_digits_asc_u128_fail_1,
    from_power_of_two_digits_asc_u128_fail_2,
    from_power_of_two_digits_asc_u128_fail_3
);
from_power_of_two_digits_asc_fail_helper!(
    usize,
    from_power_of_two_digits_asc_usize_fail_1,
    from_power_of_two_digits_asc_usize_fail_2,
    from_power_of_two_digits_asc_usize_fail_3
);

#[test]
fn test_from_power_of_two_digits_desc() {
    fn test<T: PrimitiveUnsigned>(log_base: u64, digits: &[T], out: &str)
    where
        Natural: PowerOfTwoDigits<T>,
    {
        assert_eq!(
            Natural::from_power_of_two_digits_desc(log_base, digits).to_string(),
            out
        );
    };
    test::<u8>(1, &[], "0");
    test::<u8>(1, &[0, 0, 0], "0");
    test::<u16>(10, &[123], "123");
    test::<u16>(
        1,
        &[
            1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        "1000000000000",
    );
    test::<u32>(
        3,
        &[0, 1, 6, 4, 3, 2, 4, 5, 1, 2, 1, 0, 0, 0, 0],
        "1000000000000",
    );
    test::<u64>(4, &[14, 8, 13, 4, 10, 5, 1, 0, 0, 0], "1000000000000");
    test::<u32>(32, &[232, 3567587328], "1000000000000");
    test::<u64>(64, &[1000000000000], "1000000000000");
    test::<u64>(
        64,
        &[54210, 2003764205206896640],
        "1000000000000000000000000",
    );
}

macro_rules! from_power_of_two_digits_desc_fail_helper {
    ($t:ident, $fail_1:ident, $fail_2:ident, $fail_3:ident) => {
        #[test]
        #[should_panic]
        fn $fail_1() {
            Natural::from_power_of_two_digits_desc(0, &[0u32]);
        }

        #[test]
        #[should_panic]
        fn $fail_2() {
            Natural::from_power_of_two_digits_desc(33, &[2u32]);
        }

        #[test]
        #[should_panic]
        fn $fail_3() {
            Natural::from_power_of_two_digits_desc(1, &[2u32]);
        }
    };
}

from_power_of_two_digits_desc_fail_helper!(
    u8,
    from_power_of_two_digits_desc_u8_fail_1,
    from_power_of_two_digits_desc_u8_fail_2,
    from_power_of_two_digits_desc_u8_fail_3
);
from_power_of_two_digits_desc_fail_helper!(
    u16,
    from_power_of_two_digits_desc_u16_fail_1,
    from_power_of_two_digits_desc_u16_fail_2,
    from_power_of_two_digits_desc_u16_fail_3
);
from_power_of_two_digits_desc_fail_helper!(
    u32,
    from_power_of_two_digits_desc_u32_fail_1,
    from_power_of_two_digits_desc_u32_fail_2,
    from_power_of_two_digits_desc_u32_fail_3
);
from_power_of_two_digits_desc_fail_helper!(
    u64,
    from_power_of_two_digits_desc_u64_fail_1,
    from_power_of_two_digits_desc_u64_fail_2,
    from_power_of_two_digits_desc_u64_fail_3
);
from_power_of_two_digits_desc_fail_helper!(
    u128,
    from_power_of_two_digits_desc_u128_fail_1,
    from_power_of_two_digits_desc_u128_fail_2,
    from_power_of_two_digits_desc_u128_fail_3
);
from_power_of_two_digits_desc_fail_helper!(
    usize,
    from_power_of_two_digits_desc_usize_fail_1,
    from_power_of_two_digits_desc_usize_fail_2,
    from_power_of_two_digits_desc_usize_fail_3
);

#[test]
fn test_from_power_of_two_digits_asc_natural() {
    let test = |log_base, digits, out| {
        let digits = parse_vec(digits).unwrap();
        assert_eq!(
            Natural::from_power_of_two_digits_asc(log_base, &digits).to_string(),
            out
        );
        assert_eq!(
            Natural::_from_power_of_two_digits_asc_natural_naive(log_base, &digits).to_string(),
            out
        );
    };
    test(1, "[]", "0");
    test(1, "[0, 0, 0]", "0");
    test(10, "[123]", "123");
    test(
        1,
        "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, \
        0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1]",
        "1000000000000",
    );
    test(
        3,
        "[0, 0, 0, 0, 1, 2, 1, 5, 4, 2, 3, 4, 6, 1]",
        "1000000000000",
    );
    test(4, "[0, 0, 0, 1, 5, 10, 4, 13, 8, 14, 0]", "1000000000000");
    test(32, "[3567587328, 232]", "1000000000000");
    test(64, "[1000000000000]", "1000000000000");
    test(
        64,
        "[2003764205206896640, 54210]",
        "1000000000000000000000000",
    );
    test(
        33,
        "[6996099072, 4528236150, 13552]",
        "1000000000000000000000000",
    );
}

#[test]
#[should_panic]
fn from_power_of_two_digits_asc_natural_fail_1() {
    let digits: Vec<Natural> = parse_vec("[0, 0, 0]").unwrap();
    Natural::from_power_of_two_digits_asc(0, &digits);
}

#[test]
#[should_panic]
fn from_power_of_two_digits_asc_natural_fail_2() {
    let digits: Vec<Natural> = parse_vec("[2]").unwrap();
    Natural::from_power_of_two_digits_asc(1, &digits);
}

#[test]
fn test_from_power_of_two_digits_desc_natural() {
    let test = |log_base, digits, out| {
        let digits: Vec<Natural> = parse_vec(digits).unwrap();
        assert_eq!(
            Natural::from_power_of_two_digits_desc(log_base, &digits).to_string(),
            out
        );
    };
    test(1, "[]", "0");
    test(1, "[0, 0, 0]", "0");
    test(10, "[123]", "123");
    test(
        1,
        "[1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, \
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]",
        "1000000000000",
    );
    test(
        3,
        "[1, 6, 4, 3, 2, 4, 5, 1, 2, 1, 0, 0, 0, 0]",
        "1000000000000",
    );
    test(4, "[0, 14, 8, 13, 4, 10, 5, 1, 0, 0, 0]", "1000000000000");
    test(32, "[232, 3567587328]", "1000000000000");
    test(64, "[1000000000000]", "1000000000000");
    test(
        64,
        "[54210, 2003764205206896640]",
        "1000000000000000000000000",
    );
    test(
        33,
        "[13552, 4528236150, 6996099072]",
        "1000000000000000000000000",
    );
}

#[test]
#[should_panic]
fn from_power_of_two_digits_desc_natural_fail_1() {
    let digits: Vec<Natural> = parse_vec("[0, 0, 0]").unwrap();
    Natural::from_power_of_two_digits_desc(0, &digits);
}

#[test]
#[should_panic]
fn from_power_of_two_digits_desc_natural_fail_2() {
    let digits: Vec<Natural> = parse_vec("[2]").unwrap();
    Natural::from_power_of_two_digits_desc(1, &digits);
}

fn from_power_of_two_digits_asc_properties_helper<
    T: PrimitiveUnsigned + Rand + SampleRange,
    F: Fn(u64, &[T]) -> Natural,
>(
    from_power_of_two_digits_asc_naive: F,
) where
    Natural: PowerOfTwoDigits<T>,
    Limb: PowerOfTwoDigits<T>,
{
    test_properties_no_special(
        pairs_of_u64_and_unsigned_vec_var_3::<T>,
        |&(log_base, ref digits)| {
            let n = Natural::from_power_of_two_digits_asc(log_base, &digits);
            assert_eq!(from_power_of_two_digits_asc_naive(log_base, &digits), n);
            let digits_rev: Vec<T> = digits.iter().rev().cloned().collect();
            assert_eq!(
                Natural::from_power_of_two_digits_desc(log_base, &digits_rev),
                n
            );
            let trailing_zeros = slice_trailing_zeros(&digits);
            let trimmed_digits = digits[..digits.len() - trailing_zeros].to_vec();
            assert_eq!(
                PowerOfTwoDigits::<T>::to_power_of_two_digits_asc(&n, log_base),
                trimmed_digits
            );
        },
    );

    test_properties_no_special(pairs_of_u64_and_small_usize_var_1::<T>, |&(log_base, u)| {
        assert_eq!(
            Natural::from_power_of_two_digits_asc(log_base, &vec![T::ZERO; u]),
            0
        );
    });

    test_properties_no_special(
        pairs_of_u64_and_unsigned_vec_var_1::<Limb, T>,
        |&(log_base, ref digits)| {
            let n = Limb::from_power_of_two_digits_asc(log_base, &digits);
            assert_eq!(
                Natural::from_power_of_two_digits_asc(log_base, &digits),
                Natural::from(n)
            );
        },
    );
}

#[test]
fn from_power_of_two_digits_asc_properties() {
    from_power_of_two_digits_asc_properties_helper::<u8, _>(
        Natural::_from_power_of_two_digits_asc_u8_naive,
    );
    from_power_of_two_digits_asc_properties_helper::<u16, _>(
        Natural::_from_power_of_two_digits_asc_u16_naive,
    );
    from_power_of_two_digits_asc_properties_helper::<u32, _>(
        Natural::_from_power_of_two_digits_asc_u32_naive,
    );
    from_power_of_two_digits_asc_properties_helper::<u64, _>(
        Natural::_from_power_of_two_digits_asc_u64_naive,
    );
    from_power_of_two_digits_asc_properties_helper::<usize, _>(
        Natural::_from_power_of_two_digits_asc_usize_naive,
    );

    test_properties(vecs_of_unsigned, |limbs| {
        assert_eq!(
            Natural::from_power_of_two_digits_asc(Limb::WIDTH, &limbs),
            Natural::from_limbs_asc(&limbs)
        );
    });
}

fn from_power_of_two_digits_desc_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>()
where
    Natural: PowerOfTwoDigits<T>,
    Limb: PowerOfTwoDigits<T>,
{
    test_properties_no_special(
        pairs_of_u64_and_unsigned_vec_var_3::<T>,
        |&(log_base, ref digits)| {
            let n = Natural::from_power_of_two_digits_desc(log_base, &digits);
            let digits_rev: Vec<T> = digits.iter().rev().cloned().collect();
            assert_eq!(
                Natural::from_power_of_two_digits_asc(log_base, &digits_rev),
                n
            );
            let leading_zeros = slice_leading_zeros(&digits);
            let trimmed_digits = digits[leading_zeros..].to_vec();
            assert_eq!(
                PowerOfTwoDigits::<T>::to_power_of_two_digits_desc(&n, log_base),
                trimmed_digits
            );
        },
    );

    test_properties_no_special(pairs_of_u64_and_small_usize_var_1::<T>, |&(log_base, u)| {
        assert_eq!(
            Natural::from_power_of_two_digits_desc(log_base, &vec![T::ZERO; u]),
            0
        );
    });

    test_properties_no_special(
        pairs_of_u64_and_unsigned_vec_var_2::<Limb, T>,
        |&(log_base, ref digits)| {
            let n = Limb::from_power_of_two_digits_desc(log_base, &digits);
            assert_eq!(
                Natural::from_power_of_two_digits_desc(log_base, &digits),
                Natural::from(n)
            );
        },
    );
}

#[test]
fn from_power_of_two_digits_desc_properties() {
    from_power_of_two_digits_desc_properties_helper::<u8>();
    from_power_of_two_digits_desc_properties_helper::<u16>();
    from_power_of_two_digits_desc_properties_helper::<u32>();
    from_power_of_two_digits_desc_properties_helper::<u64>();
    from_power_of_two_digits_desc_properties_helper::<usize>();

    test_properties(vecs_of_unsigned, |limbs| {
        assert_eq!(
            Natural::from_power_of_two_digits_desc(Limb::WIDTH, &limbs),
            Natural::from_limbs_desc(&limbs)
        );
    });
}

#[test]
fn from_power_of_two_digits_asc_natural_properties() {
    test_properties(
        pairs_of_u64_and_natural_vec_var_1,
        |&(log_base, ref digits)| {
            let n = Natural::from_power_of_two_digits_asc(log_base, &digits);
            assert_eq!(
                Natural::_from_power_of_two_digits_asc_natural_naive(log_base, &digits),
                n
            );
            let digits_rev: Vec<Natural> = digits.iter().rev().cloned().collect();
            assert_eq!(
                Natural::from_power_of_two_digits_desc(log_base, &digits_rev),
                n
            );
            let trailing_zeros = slice_trailing_zeros(&digits);
            let trimmed_digits = digits[..digits.len() - trailing_zeros].to_vec();
            assert_eq!(
                PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&n, log_base),
                trimmed_digits
            );
        },
    );

    test_properties_no_special(
        pairs_of_small_u64_and_small_usize_var_2,
        |&(log_base, u)| {
            assert_eq!(
                Natural::from_power_of_two_digits_asc(log_base, &vec![Natural::ZERO; u]),
                0
            );
        },
    );

    test_properties_no_special(
        pairs_of_u64_and_unsigned_vec_var_3::<Limb>,
        |&(log_base, ref digits)| {
            let n = Natural::from_power_of_two_digits_asc(log_base, &digits);
            let digits: Vec<Natural> = digits.iter().cloned().map(Natural::from).collect();
            assert_eq!(Natural::from_power_of_two_digits_asc(log_base, &digits), n);
        },
    );
}

#[test]
fn from_power_of_two_digits_desc_natural_properties() {
    test_properties(
        pairs_of_u64_and_natural_vec_var_1,
        |&(log_base, ref digits)| {
            let n = Natural::from_power_of_two_digits_desc(log_base, &digits);
            let digits_rev: Vec<Natural> = digits.iter().rev().cloned().collect();
            assert_eq!(
                Natural::from_power_of_two_digits_asc(log_base, &digits_rev),
                n
            );
            let leading_zeros = slice_leading_zeros(&digits);
            let trimmed_digits = digits[leading_zeros..].to_vec();
            assert_eq!(
                PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(&n, log_base),
                trimmed_digits
            );
        },
    );

    test_properties_no_special(
        pairs_of_small_u64_and_small_usize_var_2,
        |&(log_base, u)| {
            assert_eq!(
                Natural::from_power_of_two_digits_desc(log_base, &vec![Natural::ZERO; u]),
                0
            );
        },
    );

    test_properties_no_special(
        pairs_of_u64_and_unsigned_vec_var_3::<Limb>,
        |&(log_base, ref digits)| {
            let n = Natural::from_power_of_two_digits_desc(log_base, &digits);
            let digits: Vec<Natural> = digits.iter().cloned().map(Natural::from).collect();
            assert_eq!(Natural::from_power_of_two_digits_desc(log_base, &digits), n);
        },
    );
}
