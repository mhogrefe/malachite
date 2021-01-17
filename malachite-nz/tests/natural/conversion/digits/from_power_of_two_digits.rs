use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::PowerOfTwoDigits;
use malachite_base::vecs::vec_from_str;
use malachite_nz::natural::Natural;
use std::panic::catch_unwind;

#[test]
fn test_from_power_of_two_digits_asc() {
    fn test<T: PrimitiveUnsigned>(log_base: u64, digits: &[T], out: &str)
    where
        Natural: From<T> + PowerOfTwoDigits<T>,
    {
        assert_eq!(
            Natural::from_power_of_two_digits_asc(log_base, digits.iter().cloned()).to_string(),
            out
        );
        assert_eq!(
            Natural::_from_power_of_two_digits_asc_naive(log_base, digits.iter().cloned())
                .to_string(),
            out
        );
    };
    test::<u8>(1, &[], "0");
    test::<u8>(1, &[0, 0, 0], "0");
    test::<u16>(10, &[123], "123");
    test::<u16>(
        1,
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1,
            0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1,
        ],
        "1000000000000",
    );
    test::<u32>(
        3,
        &[0, 0, 0, 0, 1, 2, 1, 5, 4, 2, 3, 4, 6, 1, 0],
        "1000000000000",
    );
    test::<u64>(4, &[0, 0, 0, 1, 5, 10, 4, 13, 8, 14], "1000000000000");
    test::<u32>(32, &[3567587328, 232], "1000000000000");
    test::<u64>(64, &[1000000000000], "1000000000000");
    test::<u64>(
        64,
        &[2003764205206896640, 54210],
        "1000000000000000000000000",
    );
}

fn from_power_of_two_digits_asc_fail_helper<T: PrimitiveUnsigned>()
where
    Natural: PowerOfTwoDigits<T>,
{
    assert_panic!(Natural::from_power_of_two_digits_asc(
        0,
        [T::ZERO].iter().cloned()
    ));
    assert_panic!(Natural::from_power_of_two_digits_asc(
        T::WIDTH + 1,
        [T::TWO].iter().cloned()
    ));
    assert_panic!(Natural::from_power_of_two_digits_asc(
        1,
        [T::TWO].iter().cloned()
    ));
}

#[test]
fn from_power_of_two_digits_asc_fail() {
    apply_fn_to_unsigneds!(from_power_of_two_digits_asc_fail_helper);
}

#[test]
fn test_from_power_of_two_digits_desc() {
    fn test<T: PrimitiveUnsigned>(log_base: u64, digits: &[T], out: &str)
    where
        Natural: PowerOfTwoDigits<T>,
    {
        assert_eq!(
            Natural::from_power_of_two_digits_desc(log_base, digits.iter().cloned()).to_string(),
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

fn from_power_of_two_digits_desc_fail_helper<T: PrimitiveUnsigned>()
where
    Natural: PowerOfTwoDigits<T>,
{
    assert_panic!(Natural::from_power_of_two_digits_desc(
        0,
        [T::ZERO].iter().cloned()
    ));
    assert_panic!(Natural::from_power_of_two_digits_desc(
        T::WIDTH + 1,
        [T::TWO].iter().cloned()
    ));
    assert_panic!(Natural::from_power_of_two_digits_desc(
        1,
        [T::TWO].iter().cloned()
    ));
}

#[test]
fn from_power_of_two_digits_desc_fail() {
    apply_fn_to_unsigneds!(from_power_of_two_digits_desc_fail_helper);
}

#[test]
fn test_from_power_of_two_digits_asc_natural() {
    let test = |log_base, digits, out| {
        let digits = vec_from_str(digits).unwrap();
        assert_eq!(
            Natural::from_power_of_two_digits_asc(log_base, digits.iter().cloned()).to_string(),
            out
        );
        assert_eq!(
            Natural::_from_power_of_two_digits_asc_natural_naive(log_base, digits.iter().cloned())
                .to_string(),
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
    let digits: Vec<Natural> = vec_from_str("[0, 0, 0]").unwrap();
    Natural::from_power_of_two_digits_asc(0, digits.iter().cloned());
}

#[test]
#[should_panic]
fn from_power_of_two_digits_asc_natural_fail_2() {
    let digits: Vec<Natural> = vec_from_str("[2]").unwrap();
    Natural::from_power_of_two_digits_asc(1, digits.iter().cloned());
}

#[test]
fn test_from_power_of_two_digits_desc_natural() {
    let test = |log_base, digits, out| {
        let digits: Vec<Natural> = vec_from_str(digits).unwrap();
        assert_eq!(
            Natural::from_power_of_two_digits_desc(log_base, digits.iter().cloned()).to_string(),
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
    let digits: Vec<Natural> = vec_from_str("[0, 0, 0]").unwrap();
    Natural::from_power_of_two_digits_desc(0, digits.iter().cloned());
}

#[test]
#[should_panic]
fn from_power_of_two_digits_desc_natural_fail_2() {
    let digits: Vec<Natural> = vec_from_str("[2]").unwrap();
    Natural::from_power_of_two_digits_desc(1, digits.iter().cloned());
}
