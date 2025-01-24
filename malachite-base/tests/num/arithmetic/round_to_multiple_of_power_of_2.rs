// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    signed_rounding_mode_pair_gen, signed_unsigned_pair_gen_var_1, signed_unsigned_pair_gen_var_16,
    signed_unsigned_pair_gen_var_17, signed_unsigned_pair_gen_var_8,
    signed_unsigned_rounding_mode_triple_gen_var_1, unsigned_pair_gen_var_14,
    unsigned_pair_gen_var_2, unsigned_pair_gen_var_21, unsigned_rounding_mode_pair_gen,
    unsigned_unsigned_rounding_mode_triple_gen_var_3,
};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

#[test]
fn test_round_to_multiple_of_power_of_2() {
    fn test<T: PrimitiveInt>(n: T, pow: u64, rm: RoundingMode, out: T, o: Ordering) {
        assert_eq!(n.round_to_multiple_of_power_of_2(pow, rm), (out, o));

        let mut n = n;
        assert_eq!(n.round_to_multiple_of_power_of_2_assign(pow, rm), o);
        assert_eq!(n, out);
    }
    test::<u8>(0, 10, Exact, 0, Equal);
    test::<u8>(17, 0, Exact, 17, Equal);

    test::<u8>(10, 2, Floor, 8, Less);
    test::<u16>(10, 2, Ceiling, 12, Greater);
    test::<u32>(10, 2, Down, 8, Less);
    test::<u64>(10, 2, Up, 12, Greater);
    test::<u128>(10, 2, Nearest, 8, Less);
    test::<usize>(12, 2, Exact, 12, Equal);

    test::<i8>(-10, 2, Floor, -12, Less);
    test::<i16>(-10, 2, Ceiling, -8, Greater);
    test::<i32>(-10, 2, Down, -8, Greater);
    test::<i64>(-10, 2, Up, -12, Less);
    test::<i128>(-10, 2, Nearest, -8, Greater);
    test::<isize>(-12, 2, Exact, -12, Equal);

    test::<u8>(0xff, 4, Down, 0xf0, Less);
    test::<u8>(0xff, 4, Floor, 0xf0, Less);
    test::<u8>(0xef, 4, Up, 0xf0, Greater);
    test::<u8>(0xef, 4, Ceiling, 0xf0, Greater);
    test::<u8>(0xe8, 4, Nearest, 0xe0, Less);
    test::<u8>(1, 8, Nearest, 0, Less);

    test::<i8>(0x7f, 4, Down, 0x70, Less);
    test::<i8>(0x7f, 4, Floor, 0x70, Less);
    test::<i8>(0x6f, 4, Up, 0x70, Greater);
    test::<i8>(0x6f, 4, Ceiling, 0x70, Greater);
    test::<i8>(0x68, 4, Nearest, 0x60, Less);
    test::<i8>(-0x7f, 4, Down, -0x70, Greater);
    test::<i8>(-0x7f, 4, Floor, -0x80, Less);
    test::<i8>(-0x7f, 4, Up, -0x80, Less);
    test::<i8>(-0x7f, 4, Ceiling, -0x70, Greater);
    test::<i8>(-0x78, 4, Nearest, -0x80, Less);
}

fn round_to_multiple_of_power_of_2_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::exact_from(10).round_to_multiple_of_power_of_2(4, Exact));
    assert_panic!(T::MAX.round_to_multiple_of_power_of_2(4, Up));
    assert_panic!(T::MAX.round_to_multiple_of_power_of_2(4, Ceiling));
    assert_panic!(T::MAX.round_to_multiple_of_power_of_2(4, Nearest));
    assert_panic!(T::ONE.round_to_multiple_of_power_of_2(T::WIDTH, Up));

    assert_panic!(T::exact_from(10).round_to_multiple_of_power_of_2_assign(4, Exact));
    assert_panic!({
        let mut n = T::MAX;
        n.round_to_multiple_of_power_of_2_assign(4, Up);
    });
    assert_panic!({
        let mut n = T::MAX;
        n.round_to_multiple_of_power_of_2_assign(4, Ceiling);
    });
    assert_panic!({
        let mut n = T::MAX;
        n.round_to_multiple_of_power_of_2_assign(4, Nearest);
    });
    assert_panic!({
        let mut n = T::ONE;
        n.round_to_multiple_of_power_of_2_assign(T::WIDTH, Up);
    });
}

fn round_to_multiple_of_power_of_2_signed_fail_helper<T: PrimitiveSigned>() {
    assert_panic!((-T::MAX).round_to_multiple_of_power_of_2(T::WIDTH, Up));
    assert_panic!((-T::MAX).round_to_multiple_of_power_of_2(T::WIDTH, Floor));

    assert_panic!((-T::MAX).round_to_multiple_of_power_of_2_assign(T::WIDTH, Up));
    assert_panic!({
        (-T::MAX).round_to_multiple_of_power_of_2_assign(T::WIDTH, Floor);
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
            Floor | Down => assert_ne!(o, Greater),
            Ceiling | Up => assert_ne!(o, Less),
            Exact => assert_eq!(o, Equal),
            _ => {}
        }
        match rm {
            Floor | Down => {
                assert!(rounded <= n);
            }
            Ceiling | Up => {
                assert!(rounded >= n);
            }
            Exact => assert_eq!(rounded, n),
            Nearest => {
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

        if o == Equal {
            for rm in exhaustive_rounding_modes() {
                assert_eq!(n.round_to_multiple_of_power_of_2(pow, rm), (rounded, Equal));
            }
        } else {
            assert_panic!(n.round_to_multiple_of_power_of_2(pow, Exact));
        }
    });

    unsigned_pair_gen_var_2::<T, u64>().test_properties(|(n, pow)| {
        if pow < T::WIDTH {
            if let Some(shifted) = n.arithmetic_checked_shl(pow) {
                let so = (shifted, Equal);
                assert_eq!(shifted.round_to_multiple_of_power_of_2(pow, Down), so);
                assert_eq!(shifted.round_to_multiple_of_power_of_2(pow, Up), so);
                assert_eq!(shifted.round_to_multiple_of_power_of_2(pow, Floor), so);
                assert_eq!(shifted.round_to_multiple_of_power_of_2(pow, Ceiling), so);
                assert_eq!(shifted.round_to_multiple_of_power_of_2(pow, Nearest), so);
                assert_eq!(shifted.round_to_multiple_of_power_of_2(pow, Exact), so);
            }
        }
    });

    unsigned_pair_gen_var_14::<T, u64>().test_properties(|(n, pow)| {
        let down = n.round_to_multiple_of_power_of_2(pow, Down);
        assert_eq!(down.1, Less);
        if let Some(k) = T::ONE.arithmetic_checked_shl(pow) {
            if let Some(up) = down.0.checked_add(k) {
                let up = (up, Greater);
                assert_eq!(n.round_to_multiple_of_power_of_2(pow, Up), up);
                assert_eq!(n.round_to_multiple_of_power_of_2(pow, Floor), down);
                assert_eq!(n.round_to_multiple_of_power_of_2(pow, Ceiling), up);
                let nearest = n.round_to_multiple_of_power_of_2(pow, Nearest);
                assert!(nearest == down || nearest == up);
            }
        }
    });

    unsigned_pair_gen_var_21::<T, u64>().test_properties(|(n, pow)| {
        if let Some(shift) = pow.checked_add(T::WIDTH) {
            assert_eq!(
                n.round_to_multiple_of_power_of_2(shift, Down),
                (T::ZERO, if n == T::ZERO { Equal } else { Less })
            );
            assert_eq!(
                n.round_to_multiple_of_power_of_2(shift, Floor),
                (T::ZERO, if n == T::ZERO { Equal } else { Less })
            );
            if let Some(extra_shift) = shift.checked_add(1) {
                assert_eq!(
                    n.round_to_multiple_of_power_of_2(extra_shift, Nearest),
                    (T::ZERO, if n == T::ZERO { Equal } else { Less })
                );
            }
        }
    });

    unsigned_rounding_mode_pair_gen::<T>().test_properties(|(n, rm)| {
        assert_eq!(n.round_to_multiple_of_power_of_2(0, rm), (n, Equal));
    });

    unsigned_rounding_mode_pair_gen().test_properties(|(pow, rm)| {
        assert_eq!(
            T::ZERO.round_to_multiple_of_power_of_2(pow, rm),
            (T::ZERO, Equal)
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
            (_, Floor) | (true, Down) | (false, Up) => {
                assert_ne!(o, Greater);
            }
            (_, Ceiling) | (true, Up) | (false, Down) => {
                assert_ne!(o, Less);
            }
            (_, Exact) => assert_eq!(o, Equal),
            _ => {}
        }
        match rm {
            Floor => assert!(rounded <= n),
            Ceiling => assert!(rounded >= n),
            Down => assert!(rounded.le_abs(&n)),
            Up => assert!(rounded.ge_abs(&n)),
            Exact => assert_eq!(rounded, n),
            Nearest => {
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

        if o == Equal {
            for rm in exhaustive_rounding_modes() {
                assert_eq!(n.round_to_multiple_of_power_of_2(pow, rm), (rounded, Equal));
            }
        } else {
            assert_panic!(n.round_to_multiple_of_power_of_2(pow, Exact));
        }
    });

    signed_unsigned_pair_gen_var_1::<S, u64>().test_properties(|(n, pow)| {
        if pow < S::WIDTH {
            if let Some(shifted) = n.arithmetic_checked_shl(pow) {
                let so = (shifted, Equal);
                assert_eq!(shifted.round_to_multiple_of_power_of_2(pow, Down), so);
                assert_eq!(shifted.round_to_multiple_of_power_of_2(pow, Up), so);
                assert_eq!(shifted.round_to_multiple_of_power_of_2(pow, Floor), so);
                assert_eq!(shifted.round_to_multiple_of_power_of_2(pow, Ceiling), so);
                assert_eq!(shifted.round_to_multiple_of_power_of_2(pow, Nearest), so);
                assert_eq!(shifted.round_to_multiple_of_power_of_2(pow, Exact), so);
            }
        }
    });

    signed_unsigned_pair_gen_var_8::<S, u64>().test_properties(|(n, pow)| {
        let down = n.round_to_multiple_of_power_of_2(pow, Down);
        assert_eq!(down.1, if n >= S::ZERO { Less } else { Greater });
        if let Some(k) = S::ONE.arithmetic_checked_shl(pow) {
            if let Some(up) = if n >= S::ZERO {
                down.0.checked_add(k)
            } else {
                down.0.checked_sub(k)
            } {
                let up = (up, if n >= S::ZERO { Greater } else { Less });
                assert_eq!(n.round_to_multiple_of_power_of_2(pow, Up), up);
                if n >= S::ZERO {
                    assert_eq!(n.round_to_multiple_of_power_of_2(pow, Floor), down);
                    assert_eq!(n.round_to_multiple_of_power_of_2(pow, Ceiling), up);
                } else {
                    assert_eq!(n.round_to_multiple_of_power_of_2(pow, Floor), up);
                    assert_eq!(n.round_to_multiple_of_power_of_2(pow, Ceiling), down);
                }
                let nearest = n.round_to_multiple_of_power_of_2(pow, Nearest);
                assert!(nearest == down || nearest == up);
            }
        }
    });

    signed_unsigned_pair_gen_var_16::<S, u64>().test_properties(|(i, pow)| {
        if let Some(shift) = pow.checked_add(S::WIDTH - 1) {
            assert_eq!(
                i.round_to_multiple_of_power_of_2(shift, Down),
                (S::ZERO, if i == S::ZERO { Equal } else { Less })
            );
            assert_eq!(
                i.round_to_multiple_of_power_of_2(shift, Floor),
                (S::ZERO, if i == S::ZERO { Equal } else { Less })
            );
            if let Some(extra_shift) = shift.checked_add(1) {
                assert_eq!(
                    i.round_to_multiple_of_power_of_2(extra_shift, Nearest),
                    (S::ZERO, if i == S::ZERO { Equal } else { Less })
                );
            }
        }
    });

    signed_unsigned_pair_gen_var_17::<U, S, u64>().test_properties(|(i, pow)| {
        if let Some(shift) = pow.checked_add(S::WIDTH - 1) {
            assert_eq!(
                i.round_to_multiple_of_power_of_2(shift, Down),
                (S::ZERO, if i == S::ZERO { Equal } else { Greater })
            );
            assert_eq!(
                i.round_to_multiple_of_power_of_2(shift, Ceiling),
                (S::ZERO, if i == S::ZERO { Equal } else { Greater })
            );
            if let Some(extra_shift) = shift.checked_add(1) {
                assert_eq!(
                    i.round_to_multiple_of_power_of_2(extra_shift, Nearest),
                    (S::ZERO, if i == S::ZERO { Equal } else { Greater })
                );
            }
        }
    });

    signed_rounding_mode_pair_gen::<S>().test_properties(|(n, rm)| {
        assert_eq!(n.round_to_multiple_of_power_of_2(0, rm), (n, Equal));
    });

    unsigned_rounding_mode_pair_gen().test_properties(|(pow, rm)| {
        assert_eq!(
            S::ZERO.round_to_multiple_of_power_of_2(pow, rm),
            (S::ZERO, Equal)
        );
    });
}

#[test]
fn round_to_multiple_of_power_of_2_properties() {
    apply_fn_to_unsigneds!(round_to_multiple_of_power_of_2_properties_helper_unsigned);
    apply_fn_to_unsigned_signed_pairs!(round_to_multiple_of_power_of_2_properties_helper_signed);
}
