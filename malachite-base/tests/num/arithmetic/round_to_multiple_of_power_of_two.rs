use std::panic::catch_unwind;

use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::rounding_modes::RoundingMode;

#[test]
fn test_round_to_multiple_of_power_of_two() {
    fn test<T: PrimitiveInteger>(n: T, pow: u64, rm: RoundingMode, out: T) {
        assert_eq!(n.round_to_multiple_of_power_of_two(pow, rm), out);

        let mut n = n;
        n.round_to_multiple_of_power_of_two_assign(pow, rm);
        assert_eq!(n, out);
    };

    test::<u8>(0, 10, RoundingMode::Exact, 0);
    test::<u8>(17, 0, RoundingMode::Exact, 17);

    test::<u8>(10, 2, RoundingMode::Floor, 8);
    test::<u16>(10, 2, RoundingMode::Ceiling, 12);
    test::<u32>(10, 2, RoundingMode::Down, 8);
    test::<u64>(10, 2, RoundingMode::Up, 12);
    test::<u128>(10, 2, RoundingMode::Nearest, 8);
    test::<usize>(12, 2, RoundingMode::Exact, 12);

    test::<i8>(-10, 2, RoundingMode::Floor, -12);
    test::<i16>(-10, 2, RoundingMode::Ceiling, -8);
    test::<i32>(-10, 2, RoundingMode::Down, -8);
    test::<i64>(-10, 2, RoundingMode::Up, -12);
    test::<i128>(-10, 2, RoundingMode::Nearest, -8);
    test::<isize>(-12, 2, RoundingMode::Exact, -12);

    test::<u8>(0xff, 4, RoundingMode::Down, 0xf0);
    test::<u8>(0xff, 4, RoundingMode::Floor, 0xf0);
    test::<u8>(0xef, 4, RoundingMode::Up, 0xf0);
    test::<u8>(0xef, 4, RoundingMode::Ceiling, 0xf0);
    test::<u8>(0xe8, 4, RoundingMode::Nearest, 0xe0);
    test::<u8>(1, 8, RoundingMode::Nearest, 0);

    test::<i8>(0x7f, 4, RoundingMode::Down, 0x70);
    test::<i8>(0x7f, 4, RoundingMode::Floor, 0x70);
    test::<i8>(0x6f, 4, RoundingMode::Up, 0x70);
    test::<i8>(0x6f, 4, RoundingMode::Ceiling, 0x70);
    test::<i8>(0x68, 4, RoundingMode::Nearest, 0x60);
    test::<i8>(-0x7f, 4, RoundingMode::Down, -0x70);
    test::<i8>(-0x7f, 4, RoundingMode::Floor, -0x80);
    test::<i8>(-0x7f, 4, RoundingMode::Up, -0x80);
    test::<i8>(-0x7f, 4, RoundingMode::Ceiling, -0x70);
    test::<i8>(-0x78, 4, RoundingMode::Nearest, -0x80);
}

fn round_to_multiple_of_power_of_two_fail_helper<T: PrimitiveInteger>() {
    assert_panic!(T::exact_from(10).round_to_multiple_of_power_of_two(4, RoundingMode::Exact));
    assert_panic!(T::MAX.round_to_multiple_of_power_of_two(4, RoundingMode::Up));
    assert_panic!(T::MAX.round_to_multiple_of_power_of_two(4, RoundingMode::Ceiling));
    assert_panic!(T::MAX.round_to_multiple_of_power_of_two(4, RoundingMode::Nearest));
    assert_panic!(T::ONE.round_to_multiple_of_power_of_two(T::WIDTH, RoundingMode::Up));

    assert_panic!(
        T::exact_from(10).round_to_multiple_of_power_of_two_assign(4, RoundingMode::Exact)
    );
    assert_panic!(T::MAX.round_to_multiple_of_power_of_two_assign(4, RoundingMode::Up));
    assert_panic!(T::MAX.round_to_multiple_of_power_of_two_assign(4, RoundingMode::Ceiling));
    assert_panic!(T::MAX.round_to_multiple_of_power_of_two_assign(4, RoundingMode::Nearest));
    assert_panic!(T::ONE.round_to_multiple_of_power_of_two_assign(T::WIDTH, RoundingMode::Up));
}

fn round_to_multiple_of_power_of_two_signed_fail_helper<T: PrimitiveSigned>() {
    assert_panic!((-T::MAX).round_to_multiple_of_power_of_two(T::WIDTH, RoundingMode::Up));
    assert_panic!((-T::MAX).round_to_multiple_of_power_of_two(T::WIDTH, RoundingMode::Floor));

    assert_panic!((-T::MAX).round_to_multiple_of_power_of_two_assign(T::WIDTH, RoundingMode::Up));
    assert_panic!({
        (-T::MAX).round_to_multiple_of_power_of_two_assign(T::WIDTH, RoundingMode::Floor);
    });
}

#[test]
fn round_to_multiple_of_power_of_two_fail() {
    apply_fn_to_primitive_ints!(round_to_multiple_of_power_of_two_fail_helper);
    apply_fn_to_signeds!(round_to_multiple_of_power_of_two_signed_fail_helper);
}
