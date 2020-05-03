use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::round::RoundingMode;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_negative_signed_not_min_and_small_unsigned,
    pairs_of_positive_signed_and_small_unsigned, pairs_of_positive_unsigned_and_small_unsigned,
    pairs_of_signed_and_rounding_mode, pairs_of_signed_and_small_unsigned,
    pairs_of_signed_and_small_unsigned_var_1, pairs_of_unsigned_and_rounding_mode,
    pairs_of_unsigned_and_small_unsigned, pairs_of_unsigned_and_small_unsigned_var_1,
    triples_of_signed_small_u64_and_rounding_mode_var_2,
    triples_of_unsigned_small_u64_and_rounding_mode_var_2,
};

fn round_to_multiple_of_power_of_two_properties_unsigned_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(
        triples_of_unsigned_small_u64_and_rounding_mode_var_2::<T>,
        |&(n, pow, rm)| {
            let rounded = n.round_to_multiple_of_power_of_two(pow, rm);

            let mut mut_n = n;
            mut_n.round_to_multiple_of_power_of_two_assign(pow, rm);
            assert_eq!(mut_n, rounded);

            assert!(rounded.divisible_by_power_of_two(pow));
            match rm {
                RoundingMode::Floor | RoundingMode::Down => assert!(rounded <= n),
                RoundingMode::Ceiling | RoundingMode::Up => assert!(rounded >= n),
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
        },
    );

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<T, u64>,
        |&(n, pow)| {
            if pow < T::WIDTH {
                if let Some(shifted) = n.arithmetic_checked_shl(pow) {
                    assert_eq!(
                        shifted.round_to_multiple_of_power_of_two(pow, RoundingMode::Down),
                        shifted
                    );
                    assert_eq!(
                        shifted.round_to_multiple_of_power_of_two(pow, RoundingMode::Up),
                        shifted
                    );
                    assert_eq!(
                        shifted.round_to_multiple_of_power_of_two(pow, RoundingMode::Floor),
                        shifted
                    );
                    assert_eq!(
                        shifted.round_to_multiple_of_power_of_two(pow, RoundingMode::Ceiling),
                        shifted
                    );
                    assert_eq!(
                        shifted.round_to_multiple_of_power_of_two(pow, RoundingMode::Nearest),
                        shifted
                    );
                    assert_eq!(
                        shifted.round_to_multiple_of_power_of_two(pow, RoundingMode::Exact),
                        shifted
                    );
                }
            }
        },
    );

    test_properties(
        pairs_of_unsigned_and_small_unsigned_var_1::<T, u64>,
        |&(n, pow)| {
            let down = n.round_to_multiple_of_power_of_two(pow, RoundingMode::Down);
            if let Some(k) = T::ONE.arithmetic_checked_shl(pow) {
                if let Some(up) = down.checked_add(k) {
                    assert_eq!(
                        n.round_to_multiple_of_power_of_two(pow, RoundingMode::Up),
                        up
                    );
                    assert_eq!(
                        n.round_to_multiple_of_power_of_two(pow, RoundingMode::Floor),
                        down
                    );
                    assert_eq!(
                        n.round_to_multiple_of_power_of_two(pow, RoundingMode::Ceiling),
                        up
                    );
                    let nearest = n.round_to_multiple_of_power_of_two(pow, RoundingMode::Nearest);
                    assert!(nearest == down || nearest == up);
                }
            }
        },
    );

    test_properties(
        pairs_of_positive_unsigned_and_small_unsigned::<T, u64>,
        |&(n, pow)| {
            if let Some(shift) = pow.checked_add(T::WIDTH) {
                assert_eq!(
                    n.round_to_multiple_of_power_of_two(shift, RoundingMode::Down),
                    T::ZERO
                );
                assert_eq!(
                    n.round_to_multiple_of_power_of_two(shift, RoundingMode::Floor),
                    T::ZERO
                );
                if let Some(extra_shift) = shift.checked_add(1) {
                    assert_eq!(
                        n.round_to_multiple_of_power_of_two(extra_shift, RoundingMode::Nearest),
                        T::ZERO
                    );
                }
            }
        },
    );

    test_properties(pairs_of_unsigned_and_rounding_mode::<T>, |&(n, rm)| {
        assert_eq!(n.round_to_multiple_of_power_of_two(0, rm), n);
    });

    test_properties(pairs_of_unsigned_and_rounding_mode, |&(pow, rm)| {
        assert_eq!(T::ZERO.round_to_multiple_of_power_of_two(pow, rm), T::ZERO);
    });
}

fn round_to_multiple_of_power_of_two_properties_signed_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(
        triples_of_signed_small_u64_and_rounding_mode_var_2::<T>,
        |&(n, pow, rm)| {
            let rounded = n.round_to_multiple_of_power_of_two(pow, rm);

            let mut mut_n = n;
            mut_n.round_to_multiple_of_power_of_two_assign(pow, rm);
            assert_eq!(mut_n, rounded);

            assert!(rounded.divisible_by_power_of_two(pow));
            match rm {
                RoundingMode::Floor => assert!(rounded <= n),
                RoundingMode::Ceiling => assert!(rounded >= n),
                RoundingMode::Down => assert!(rounded.le_abs(&n)),
                RoundingMode::Up => assert!(rounded.ge_abs(&n)),
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
        },
    );

    test_properties(pairs_of_signed_and_small_unsigned::<T, u64>, |&(n, pow)| {
        if pow < T::WIDTH {
            if let Some(shifted) = n.arithmetic_checked_shl(pow) {
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_two(pow, RoundingMode::Down),
                    shifted
                );
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_two(pow, RoundingMode::Up),
                    shifted
                );
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_two(pow, RoundingMode::Floor),
                    shifted
                );
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_two(pow, RoundingMode::Ceiling),
                    shifted
                );
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_two(pow, RoundingMode::Nearest),
                    shifted
                );
                assert_eq!(
                    shifted.round_to_multiple_of_power_of_two(pow, RoundingMode::Exact),
                    shifted
                );
            }
        }
    });

    test_properties(
        pairs_of_signed_and_small_unsigned_var_1::<T, u64>,
        |&(n, pow)| {
            let down = n.round_to_multiple_of_power_of_two(pow, RoundingMode::Down);
            if let Some(k) = T::ONE.arithmetic_checked_shl(pow) {
                if let Some(up) = if n >= T::ZERO {
                    down.checked_add(k)
                } else {
                    down.checked_sub(k)
                } {
                    assert_eq!(
                        n.round_to_multiple_of_power_of_two(pow, RoundingMode::Up),
                        up
                    );
                    if n >= T::ZERO {
                        assert_eq!(
                            n.round_to_multiple_of_power_of_two(pow, RoundingMode::Floor),
                            down
                        );
                        assert_eq!(
                            n.round_to_multiple_of_power_of_two(pow, RoundingMode::Ceiling),
                            up
                        );
                    } else {
                        assert_eq!(
                            n.round_to_multiple_of_power_of_two(pow, RoundingMode::Floor),
                            up
                        );
                        assert_eq!(
                            n.round_to_multiple_of_power_of_two(pow, RoundingMode::Ceiling),
                            down
                        );
                    }
                    let nearest = n.round_to_multiple_of_power_of_two(pow, RoundingMode::Nearest);
                    assert!(nearest == down || nearest == up);
                }
            }
        },
    );

    test_properties(
        pairs_of_positive_signed_and_small_unsigned::<T, u64>,
        |&(i, pow)| {
            if let Some(shift) = pow.checked_add(T::WIDTH - 1) {
                assert_eq!(
                    i.round_to_multiple_of_power_of_two(shift, RoundingMode::Down),
                    T::ZERO
                );
                assert_eq!(
                    i.round_to_multiple_of_power_of_two(shift, RoundingMode::Floor),
                    T::ZERO
                );
                if let Some(extra_shift) = shift.checked_add(1) {
                    assert_eq!(
                        i.round_to_multiple_of_power_of_two(extra_shift, RoundingMode::Nearest),
                        T::ZERO
                    );
                }
            }
        },
    );

    test_properties(
        pairs_of_negative_signed_not_min_and_small_unsigned::<T, u64>,
        |&(i, pow)| {
            if let Some(shift) = pow.checked_add(T::WIDTH - 1) {
                assert_eq!(
                    i.round_to_multiple_of_power_of_two(shift, RoundingMode::Down),
                    T::ZERO
                );
                assert_eq!(
                    i.round_to_multiple_of_power_of_two(shift, RoundingMode::Ceiling),
                    T::ZERO
                );
                if let Some(extra_shift) = shift.checked_add(1) {
                    assert_eq!(
                        i.round_to_multiple_of_power_of_two(extra_shift, RoundingMode::Nearest),
                        T::ZERO
                    );
                }
            }
        },
    );

    test_properties(pairs_of_signed_and_rounding_mode::<T>, |&(n, rm)| {
        assert_eq!(n.round_to_multiple_of_power_of_two(0, rm), n);
    });

    test_properties(pairs_of_unsigned_and_rounding_mode, |&(pow, rm)| {
        assert_eq!(T::ZERO.round_to_multiple_of_power_of_two(pow, rm), T::ZERO);
    });
}

#[test]
fn round_to_multiple_of_power_of_two_properties() {
    round_to_multiple_of_power_of_two_properties_unsigned_helper::<u8>();
    round_to_multiple_of_power_of_two_properties_unsigned_helper::<u16>();
    round_to_multiple_of_power_of_two_properties_unsigned_helper::<u32>();
    round_to_multiple_of_power_of_two_properties_unsigned_helper::<u64>();
    round_to_multiple_of_power_of_two_properties_unsigned_helper::<usize>();
    round_to_multiple_of_power_of_two_properties_signed_helper::<i8>();
    round_to_multiple_of_power_of_two_properties_signed_helper::<i16>();
    round_to_multiple_of_power_of_two_properties_signed_helper::<i32>();
    round_to_multiple_of_power_of_two_properties_signed_helper::<i64>();
    round_to_multiple_of_power_of_two_properties_signed_helper::<isize>();
}
