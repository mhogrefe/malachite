use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;

fn divisible_by_power_of_two_primitive_helper<T: PrimitiveInteger>() {
    let test = |n: T, pow, out| {
        assert_eq!(n.divisible_by_power_of_two(pow), out);
    };
    test(T::ZERO, 0, true);
    test(T::ZERO, 10, true);
    test(T::ZERO, 100, true);
    test(T::exact_from(123), 0, true);
    test(T::exact_from(123), 1, false);
    if T::WIDTH >= u64::WIDTH {
        test(T::exact_from(1_000_000_000_000u64), 0, true);
        test(T::exact_from(1_000_000_000_000u64), 12, true);
        test(T::exact_from(1_000_000_000_000u64), 13, false);
    }
    test(T::MAX, 0, true);
    test(T::MAX, 1, false);
    test(T::power_of_two(T::WIDTH >> 1), 0, true);
    test(T::power_of_two(T::WIDTH >> 1), T::WIDTH >> 1, true);
    test(T::power_of_two(T::WIDTH >> 1), (T::WIDTH >> 1) + 1, false);
}

fn divisible_by_power_of_two_signed_helper<T: PrimitiveSigned>() {
    let test = |n: T, pow, out| {
        assert_eq!(n.divisible_by_power_of_two(pow), out);
    };
    test(T::exact_from(-123), 0, true);
    test(T::exact_from(-123), 1, false);
    if T::WIDTH >= u64::WIDTH {
        test(T::exact_from(-1_000_000_000_000i64), 0, true);
        test(T::exact_from(-1_000_000_000_000i64), 12, true);
        test(T::exact_from(-1_000_000_000_000i64), 13, false);
    }
    test(T::MIN + T::ONE, 0, true);
    test(T::MIN + T::ONE, 1, false);
    test(T::MIN, 0, true);
    test(T::MIN, T::WIDTH - 1, true);
    test(T::MIN, T::WIDTH, false);
}

#[test]
fn test_divisible_by_power_of_two() {
    divisible_by_power_of_two_primitive_helper::<u8>();
    divisible_by_power_of_two_primitive_helper::<u16>();
    divisible_by_power_of_two_primitive_helper::<u32>();
    divisible_by_power_of_two_primitive_helper::<u64>();
    divisible_by_power_of_two_primitive_helper::<u128>();
    divisible_by_power_of_two_primitive_helper::<usize>();
    divisible_by_power_of_two_primitive_helper::<i8>();
    divisible_by_power_of_two_primitive_helper::<i16>();
    divisible_by_power_of_two_primitive_helper::<i32>();
    divisible_by_power_of_two_primitive_helper::<i64>();
    divisible_by_power_of_two_primitive_helper::<i128>();
    divisible_by_power_of_two_primitive_helper::<isize>();

    divisible_by_power_of_two_signed_helper::<i8>();
    divisible_by_power_of_two_signed_helper::<i16>();
    divisible_by_power_of_two_signed_helper::<i32>();
    divisible_by_power_of_two_signed_helper::<i64>();
    divisible_by_power_of_two_signed_helper::<i128>();
    divisible_by_power_of_two_signed_helper::<isize>();
}
