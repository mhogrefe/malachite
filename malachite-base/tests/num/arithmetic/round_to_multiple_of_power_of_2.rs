use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::{
    signed_rounding_mode_pair_gen, signed_unsigned_pair_gen_var_1, signed_unsigned_pair_gen_var_16,
    signed_unsigned_pair_gen_var_17, signed_unsigned_pair_gen_var_8,
    signed_unsigned_rounding_mode_triple_gen_var_1, unsigned_pair_gen_var_14,
    unsigned_pair_gen_var_2, unsigned_pair_gen_var_21, unsigned_rounding_mode_pair_gen,
    unsigned_unsigned_rounding_mode_triple_gen_var_3,
};
use std::cmp::Ordering;
use std::panic::catch_unwind;

#[test]
fn test_round_to_multiple_of_power_of_2() {
    fn test<T: PrimitiveInt>(n: T, pow: u64, rm: RoundingMode, out: T, o: Ordering) {
        assert_eq!(n.round_to_multiple_of_power_of_2(pow, rm), (out, o));

        let mut n = n;
        assert_eq!(n.round_to_multiple_of_power_of_2_assign(pow, rm), o);
        assert_eq!(n, out);
    }
    test::<u8>(0, 10, RoundingMode::Exact, 0, Ordering::Equal);
    test::<u8>(17, 0, RoundingMode::Exact, 17, Ordering::Equal);

    test::<u8>(10, 2, RoundingMode::Floor, 8, Ordering::Less);
    test::<u16>(10, 2, RoundingMode::Ceiling, 12, Ordering::Greater);
    test::<u32>(10, 2, RoundingMode::Down, 8, Ordering::Less);
    test::<u64>(10, 2, RoundingMode::Up, 12, Ordering::Greater);
    test::<u128>(10, 2, RoundingMode::Nearest, 8, Ordering::Less);
    test::<usize>(12, 2, RoundingMode::Exact, 12, Ordering::Equal);

    test::<i8>(-10, 2, RoundingMode::Floor, -12, Ordering::Less);
    test::<i16>(-10, 2, RoundingMode::Ceiling, -8, Ordering::Greater);
    test::<i32>(-10, 2, RoundingMode::Down, -8, Ordering::Greater);
    test::<i64>(-10, 2, RoundingMode::Up, -12, Ordering::Less);
    test::<i128>(-10, 2, RoundingMode::Nearest, -8, Ordering::Greater);
    test::<isize>(-12, 2, RoundingMode::Exact, -12, Ordering::Equal);

    test::<u8>(0xff, 4, RoundingMode::Down, 0xf0, Ordering::Less);
    test::<u8>(0xff, 4, RoundingMode::Floor, 0xf0, Ordering::Less);
    test::<u8>(0xef, 4, RoundingMode::Up, 0xf0, Ordering::Greater);
    test::<u8>(0xef, 4, RoundingMode::Ceiling, 0xf0, Ordering::Greater);
    test::<u8>(0xe8, 4, RoundingMode::Nearest, 0xe0, Ordering::Less);
    test::<u8>(1, 8, RoundingMode::Nearest, 0, Ordering::Less);

    test::<i8>(0x7f, 4, RoundingMode::Down, 0x70, Ordering::Less);
    test::<i8>(0x7f, 4, RoundingMode::Floor, 0x70, Ordering::Less);
    test::<i8>(0x6f, 4, RoundingMode::Up, 0x70, Ordering::Greater);
    test::<i8>(0x6f, 4, RoundingMode::Ceiling, 0x70, Ordering::Greater);
    test::<i8>(0x68, 4, RoundingMode::Nearest, 0x60, Ordering::Less);
    test::<i8>(-0x7f, 4, RoundingMode::Down, -0x70, Ordering::Greater);
    test::<i8>(-0x7f, 4, RoundingMode::Floor, -0x80, Ordering::Less);
    test::<i8>(-0x7f, 4, RoundingMode::Up, -0x80, Ordering::Less);
    test::<i8>(-0x7f, 4, RoundingMode::Ceiling, -0x70, Ordering::Greater);
    test::<i8>(-0x78, 4, RoundingMode::Nearest, -0x80, Ordering::Less);
}

fn round_to_multiple_of_power_of_2_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::exact_from(10).round_to_multiple_of_power_of_2(4, RoundingMode::Exact));
    assert_panic!(T::MAX.round_to_multiple_of_power_of_2(4, RoundingMode::Up));
    assert_panic!(T::MAX.round_to_multiple_of_power_of_2(4, RoundingMode::Ceiling));
    assert_panic!(T::MAX.round_to_multiple_of_power_of_2(4, RoundingMode::Nearest));
    assert_panic!(T::ONE.round_to_multiple_of_power_of_2(T::WIDTH, RoundingMode::Up));

    assert_panic!(T::exact_from(10).round_to_multiple_of_power_of_2_assign(4, RoundingMode::Exact));
    assert_panic!({
        let mut n = T::MAX;
        n.round_to_multiple_of_power_of_2_assign(4, RoundingMode::Up);
    });
    assert_panic!({
        let mut n = T::MAX;
        n.round_to_multiple_of_power_of_2_assign(4, RoundingMode::Ceiling);
    });
    assert_panic!({
        let mut n = T::MAX;
        n.round_to_multiple_of_power_of_2_assign(4, RoundingMode::Nearest);
    });
    assert_panic!({
        let mut n = T::ONE;
        n.round_to_multiple_of_power_of_2_assign(T::WIDTH, RoundingMode::Up);
    });
}

fn round_to_multiple_of_power_of_2_signed_fail_helper<T: PrimitiveSigned>() {
    assert_panic!((-T::MAX).round_to_multiple_of_power_of_2(T::WIDTH, RoundingMode::Up));
    assert_panic!((-T::MAX).round_to_multiple_of_power_of_2(T::WIDTH, RoundingMode::Floor));

    assert_panic!((-T::MAX).round_to_multiple_of_power_of_2_assign(T::WIDTH, RoundingMode::Up));
    assert_panic!({
        (-T::MAX).round_to_multiple_of_power_of_2_assign(T::WIDTH, RoundingMode::Floor);
    });
}

#[test]
fn round_to_multiple_of_power_of_2_fail() {
    apply_fn_to_primitive_ints!(round_to_multiple_of_power_of_2_fail_helper);
    apply_fn_to_signeds!(round_to_multiple_of_power_of_2_signed_fail_helper);
}

fn round_to_multiple_of_power_of_2_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_unsigned_rounding_mode_triple_gen_var_3::<T>().test_properties(|(n, pow, rm)| {
        let (rounded, o) = n.round_to_multiple_of_power_of_2(pow, rm);

        let mut mut_n = n;
        assert_eq!(mut_n.round_to_multiple_of_power_of_2_assign(pow, rm), o);
        assert_eq!(mut_n, rounded);

        assert!(rounded.divisible_by_power_of_2(pow));
        assert_eq!(rounded.cmp(&n), o);
        match rm {
            RoundingMode::Floor | RoundingMode::Down => assert_ne!(o, Ordering::Greater),
            RoundingMode::Ceiling | RoundingMode::Up => assert_ne!(o, Ordering::Less),
            RoundingMode::Exact => assert_eq!(o, Ordering::Equal),
            _ => {}
        }
        match rm {
            RoundingMode::Floor | RoundingMode::Down => {
                assert!(rounded <= n)
            }
            RoundingMode::Ceiling | RoundingMode::Up => {
                assert!(rounded >= n)
            }
            RoundingMode::Exact => assert_eq!(rounded, n),
            RoundingMode::Nearest => {
                if let Some(k) = T::ONE.arithmetic_checked_shl(pow) {
                    let mut closest = None;
                    let mut second_closest = None;
                    if rounded <= n {
                        if let Some(above) = rounded.checked_add(k) {
                            closest = Some(n - rounded);
                            second_closest = Some(above - n);
                        }
                    } else if let Some(below) = rounded.checked_sub(k) {
                        closest = Some(rounded - n);
                        second_closest = Some(n - below);
                    }
                    if let (Some(closest), Some(second_closest)) = (closest, second_closest) {
                        assert!(closest <= second_closest);
                        if closest == second_closest {
                            assert!(!rounded.get_bit(pow));
                        }
                    }
                }
            }
        }
    });

    unsigned_pair_gen_var_2::<T, u64>().test_properties(|(n, pow)| {
        if pow < T::WIDTH {
            if let Some(shifted) = n.arithmetic_checked_shl(pow) {
                let so = (shifted, Ordering::Equal);
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_2(pow, RoundingMode::Down),
                    so
                );
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_2(pow, RoundingMode::Up),
                    so
                );
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_2(pow, RoundingMode::Floor),
                    so
                );
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_2(pow, RoundingMode::Ceiling),
                    so
                );
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest),
                    so
                );
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_2(pow, RoundingMode::Exact),
                    so
                );
            }
        }
    });

    unsigned_pair_gen_var_14::<T, u64>().test_properties(|(n, pow)| {
        let down = n.round_to_multiple_of_power_of_2(pow, RoundingMode::Down);
        assert_eq!(down.1, Ordering::Less);
        if let Some(k) = T::ONE.arithmetic_checked_shl(pow) {
            if let Some(up) = down.0.checked_add(k) {
                let up = (up, Ordering::Greater);
                assert_eq!(n.round_to_multiple_of_power_of_2(pow, RoundingMode::Up), up);
                assert_eq!(
                    n.round_to_multiple_of_power_of_2(pow, RoundingMode::Floor),
                    down
                );
                assert_eq!(
                    n.round_to_multiple_of_power_of_2(pow, RoundingMode::Ceiling),
                    up
                );
                let nearest = n.round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest);
                assert!(nearest == down || nearest == up);
            }
        }
    });

    unsigned_pair_gen_var_21::<T, u64>().test_properties(|(n, pow)| {
        if let Some(shift) = pow.checked_add(T::WIDTH) {
            assert_eq!(
                n.round_to_multiple_of_power_of_2(shift, RoundingMode::Down),
                (
                    T::ZERO,
                    if n == T::ZERO {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                )
            );
            assert_eq!(
                n.round_to_multiple_of_power_of_2(shift, RoundingMode::Floor),
                (
                    T::ZERO,
                    if n == T::ZERO {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                )
            );
            if let Some(extra_shift) = shift.checked_add(1) {
                assert_eq!(
                    n.round_to_multiple_of_power_of_2(extra_shift, RoundingMode::Nearest),
                    (
                        T::ZERO,
                        if n == T::ZERO {
                            Ordering::Equal
                        } else {
                            Ordering::Less
                        }
                    )
                );
            }
        }
    });

    unsigned_rounding_mode_pair_gen::<T>().test_properties(|(n, rm)| {
        assert_eq!(
            n.round_to_multiple_of_power_of_2(0, rm),
            (n, Ordering::Equal)
        );
    });

    unsigned_rounding_mode_pair_gen().test_properties(|(pow, rm)| {
        assert_eq!(
            T::ZERO.round_to_multiple_of_power_of_2(pow, rm),
            (T::ZERO, Ordering::Equal)
        );
    });
}

fn round_to_multiple_of_power_of_2_properties_helper_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() {
    signed_unsigned_rounding_mode_triple_gen_var_1::<S>().test_properties(|(n, pow, rm)| {
        let (rounded, o) = n.round_to_multiple_of_power_of_2(pow, rm);

        let mut mut_n = n;
        assert_eq!(mut_n.round_to_multiple_of_power_of_2_assign(pow, rm), o);
        assert_eq!(mut_n, rounded);

        assert!(rounded.divisible_by_power_of_2(pow));
        assert_eq!(rounded.cmp(&n), o);
        match (n >= S::ZERO, rm) {
            (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
                assert_ne!(o, Ordering::Greater)
            }
            (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
                assert_ne!(o, Ordering::Less)
            }
            (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
            _ => {}
        }
        match rm {
            RoundingMode::Floor => assert!(rounded <= n),
            RoundingMode::Ceiling => assert!(rounded >= n),
            RoundingMode::Down => assert!(rounded.le_abs(&n)),
            RoundingMode::Up => assert!(rounded.ge_abs(&n)),
            RoundingMode::Exact => assert_eq!(rounded, n),
            RoundingMode::Nearest => {
                if let Some(k) = S::ONE.arithmetic_checked_shl(pow) {
                    let mut closest = None;
                    let mut second_closest = None;
                    if rounded <= n {
                        if let Some(above) = rounded.checked_add(k) {
                            closest = Some(n - rounded);
                            second_closest = Some(above - n);
                        }
                    } else if let Some(below) = rounded.checked_sub(k) {
                        closest = Some(rounded - n);
                        second_closest = Some(n - below);
                    }
                    if let (Some(closest), Some(second_closest)) = (closest, second_closest) {
                        assert!(closest <= second_closest);
                        if closest == second_closest {
                            assert!(!rounded.get_bit(pow));
                        }
                    }
                }
            }
        }
    });

    signed_unsigned_pair_gen_var_1::<S, u64>().test_properties(|(n, pow)| {
        if pow < S::WIDTH {
            if let Some(shifted) = n.arithmetic_checked_shl(pow) {
                let so = (shifted, Ordering::Equal);
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_2(pow, RoundingMode::Down),
                    so
                );
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_2(pow, RoundingMode::Up),
                    so
                );
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_2(pow, RoundingMode::Floor),
                    so
                );
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_2(pow, RoundingMode::Ceiling),
                    so
                );
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest),
                    so
                );
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_2(pow, RoundingMode::Exact),
                    so
                );
            }
        }
    });

    signed_unsigned_pair_gen_var_8::<S, u64>().test_properties(|(n, pow)| {
        let down = n.round_to_multiple_of_power_of_2(pow, RoundingMode::Down);
        assert_eq!(
            down.1,
            if n >= S::ZERO {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        );
        if let Some(k) = S::ONE.arithmetic_checked_shl(pow) {
            if let Some(up) = if n >= S::ZERO {
                down.0.checked_add(k)
            } else {
                down.0.checked_sub(k)
            } {
                let up = (
                    up,
                    if n >= S::ZERO {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    },
                );
                assert_eq!(n.round_to_multiple_of_power_of_2(pow, RoundingMode::Up), up);
                if n >= S::ZERO {
                    assert_eq!(
                        n.round_to_multiple_of_power_of_2(pow, RoundingMode::Floor),
                        down
                    );
                    assert_eq!(
                        n.round_to_multiple_of_power_of_2(pow, RoundingMode::Ceiling),
                        up
                    );
                } else {
                    assert_eq!(
                        n.round_to_multiple_of_power_of_2(pow, RoundingMode::Floor),
                        up
                    );
                    assert_eq!(
                        n.round_to_multiple_of_power_of_2(pow, RoundingMode::Ceiling),
                        down
                    );
                }
                let nearest = n.round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest);
                assert!(nearest == down || nearest == up);
            }
        }
    });

    signed_unsigned_pair_gen_var_16::<S, u64>().test_properties(|(i, pow)| {
        if let Some(shift) = pow.checked_add(S::WIDTH - 1) {
            assert_eq!(
                i.round_to_multiple_of_power_of_2(shift, RoundingMode::Down),
                (
                    S::ZERO,
                    if i == S::ZERO {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                )
            );
            assert_eq!(
                i.round_to_multiple_of_power_of_2(shift, RoundingMode::Floor),
                (
                    S::ZERO,
                    if i == S::ZERO {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                )
            );
            if let Some(extra_shift) = shift.checked_add(1) {
                assert_eq!(
                    i.round_to_multiple_of_power_of_2(extra_shift, RoundingMode::Nearest),
                    (
                        S::ZERO,
                        if i == S::ZERO {
                            Ordering::Equal
                        } else {
                            Ordering::Less
                        }
                    )
                );
            }
        }
    });

    signed_unsigned_pair_gen_var_17::<U, S, u64>().test_properties(|(i, pow)| {
        if let Some(shift) = pow.checked_add(S::WIDTH - 1) {
            assert_eq!(
                i.round_to_multiple_of_power_of_2(shift, RoundingMode::Down),
                (
                    S::ZERO,
                    if i == S::ZERO {
                        Ordering::Equal
                    } else {
                        Ordering::Greater
                    }
                )
            );
            assert_eq!(
                i.round_to_multiple_of_power_of_2(shift, RoundingMode::Ceiling),
                (
                    S::ZERO,
                    if i == S::ZERO {
                        Ordering::Equal
                    } else {
                        Ordering::Greater
                    }
                )
            );
            if let Some(extra_shift) = shift.checked_add(1) {
                assert_eq!(
                    i.round_to_multiple_of_power_of_2(extra_shift, RoundingMode::Nearest),
                    (
                        S::ZERO,
                        if i == S::ZERO {
                            Ordering::Equal
                        } else {
                            Ordering::Greater
                        }
                    )
                );
            }
        }
    });

    signed_rounding_mode_pair_gen::<S>().test_properties(|(n, rm)| {
        assert_eq!(
            n.round_to_multiple_of_power_of_2(0, rm),
            (n, Ordering::Equal)
        );
    });

    unsigned_rounding_mode_pair_gen().test_properties(|(pow, rm)| {
        assert_eq!(
            S::ZERO.round_to_multiple_of_power_of_2(pow, rm),
            (S::ZERO, Ordering::Equal)
        );
    });
}

#[test]
fn round_to_multiple_of_power_of_2_properties() {
    apply_fn_to_unsigneds!(round_to_multiple_of_power_of_2_properties_helper_unsigned);
    apply_fn_to_unsigned_signed_pairs!(round_to_multiple_of_power_of_2_properties_helper_signed);
}
