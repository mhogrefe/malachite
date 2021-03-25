use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::PowerOfTwoDigits;
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::Natural;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_to_power_of_two_digits_asc() {
    fn test<T: PrimitiveUnsigned, F: Fn(&Natural, u64) -> Vec<T>>(
        to_power_of_two_digits_asc_naive: F,
        n: &str,
        log_base: u64,
        out: &[T],
    ) where
        Natural: PowerOfTwoDigits<T>,
    {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(
            PowerOfTwoDigits::<T>::to_power_of_two_digits_asc(&n, log_base),
            out
        );
        assert_eq!(to_power_of_two_digits_asc_naive(&n, log_base), out);
    }
    test::<u8, _>(Natural::_to_power_of_two_digits_asc_naive, "0", 1, &[]);
    test::<u16, _>(
        Natural::_to_power_of_two_digits_asc_naive,
        "123",
        10,
        &[123],
    );
    test::<u16, _>(
        Natural::_to_power_of_two_digits_asc_naive,
        "1000000000000",
        1,
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1,
            0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1,
        ],
    );
    test::<u32, _>(
        Natural::_to_power_of_two_digits_asc_naive,
        "1000000000000",
        3,
        &[0, 0, 0, 0, 1, 2, 1, 5, 4, 2, 3, 4, 6, 1],
    );
    test::<u64, _>(
        Natural::_to_power_of_two_digits_asc_naive,
        "1000000000000",
        4,
        &[0, 0, 0, 1, 5, 10, 4, 13, 8, 14],
    );
    test::<u32, _>(
        Natural::_to_power_of_two_digits_asc_naive,
        "1000000000000",
        32,
        &[3567587328, 232],
    );
    test::<u64, _>(
        Natural::_to_power_of_two_digits_asc_naive,
        "1000000000000",
        64,
        &[1000000000000],
    );
    test::<u64, _>(
        Natural::_to_power_of_two_digits_asc_naive,
        "1000000000000000000000000",
        64,
        &[2003764205206896640, 54210],
    );
}

fn to_power_of_two_digits_asc_fail_helper<T: PrimitiveUnsigned>()
where
    Natural: PowerOfTwoDigits<T>,
{
    assert_panic!(PowerOfTwoDigits::<T>::to_power_of_two_digits_asc(
        &Natural::trillion(),
        0
    ));
    assert_panic!(PowerOfTwoDigits::<T>::to_power_of_two_digits_asc(
        &Natural::trillion(),
        T::WIDTH + 1
    ));
}

#[test]
fn to_power_of_two_digits_asc_fail() {
    apply_fn_to_unsigneds!(to_power_of_two_digits_asc_fail_helper);
}

#[test]
fn test_to_power_of_two_digits_desc() {
    fn test<T: PrimitiveUnsigned>(n: &str, log_base: u64, out: &[T])
    where
        Natural: PowerOfTwoDigits<T>,
    {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(
            PowerOfTwoDigits::<T>::to_power_of_two_digits_desc(&n, log_base),
            out
        );
    }
    test::<u8>("0", 1, &[]);
    test::<u16>("123", 10, &[123]);
    test::<u16>(
        "1000000000000",
        1,
        &[
            1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    );
    test::<u32>(
        "1000000000000",
        3,
        &[1, 6, 4, 3, 2, 4, 5, 1, 2, 1, 0, 0, 0, 0],
    );
    test::<u64>("1000000000000", 4, &[14, 8, 13, 4, 10, 5, 1, 0, 0, 0]);
    test::<u32>("1000000000000", 32, &[232, 3567587328]);
    test::<u64>("1000000000000", 64, &[1000000000000]);
    test::<u64>(
        "1000000000000000000000000",
        64,
        &[54210, 2003764205206896640],
    );
}

fn to_power_of_two_digits_desc_fail_helper<T: PrimitiveUnsigned>()
where
    Natural: PowerOfTwoDigits<T>,
{
    assert_panic!(PowerOfTwoDigits::<T>::to_power_of_two_digits_desc(
        &Natural::trillion(),
        0
    ));
    assert_panic!(PowerOfTwoDigits::<T>::to_power_of_two_digits_desc(
        &Natural::trillion(),
        T::WIDTH + 1
    ));
}

#[test]
fn to_power_of_two_digits_desc_fail() {
    apply_fn_to_unsigneds!(to_power_of_two_digits_desc_fail_helper);
}

#[test]
fn test_to_power_of_two_digits_asc_natural() {
    let test = |n, log_base, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(
            PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&n, log_base).to_debug_string(),
            out
        );
        assert_eq!(
            n._to_power_of_two_digits_asc_natural_naive(log_base)
                .to_debug_string(),
            out
        );
    };
    test("0", 1, "[]");
    test("123", 10, "[123]");
    test(
        "1000000000000",
        1,
        "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, \
        0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1]",
    );
    test(
        "1000000000000",
        3,
        "[0, 0, 0, 0, 1, 2, 1, 5, 4, 2, 3, 4, 6, 1]",
    );
    test("1000000000000", 4, "[0, 0, 0, 1, 5, 10, 4, 13, 8, 14]");
    test("1000000000000", 32, "[3567587328, 232]");
    test("1000000000000", 64, "[1000000000000]");
    test(
        "1000000000000000000000000",
        64,
        "[2003764205206896640, 54210]",
    );
    test(
        "1000000000000000000000000",
        33,
        "[6996099072, 4528236150, 13552]",
    );
}

#[test]
#[should_panic]
fn to_power_of_two_digits_asc_natural_fail() {
    PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&Natural::trillion(), 0);
}

#[test]
fn test_to_power_of_two_digits_desc_natural() {
    let test = |n, log_base, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(
            PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(&n, log_base)
                .to_debug_string(),
            out
        );
    };
    test("0", 1, "[]");
    test("123", 10, "[123]");
    test(
        "1000000000000",
        1,
        "[1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, \
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]",
    );
    test(
        "1000000000000",
        3,
        "[1, 6, 4, 3, 2, 4, 5, 1, 2, 1, 0, 0, 0, 0]",
    );
    test("1000000000000", 4, "[14, 8, 13, 4, 10, 5, 1, 0, 0, 0]");
    test("1000000000000", 32, "[232, 3567587328]");
    test("1000000000000", 64, "[1000000000000]");
    test(
        "1000000000000000000000000",
        64,
        "[54210, 2003764205206896640]",
    );
    test(
        "1000000000000000000000000",
        33,
        "[13552, 4528236150, 6996099072]",
    );
}

#[test]
#[should_panic]
fn to_power_of_two_digits_desc_natural_fail() {
    PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(&Natural::trillion(), 0);
}
