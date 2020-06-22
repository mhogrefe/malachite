use std::ops::{Shl, Shr};

use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, ShlRound, ShrRound, ShrRoundAssign,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::rounding_modes::RoundingMode;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_negative_signed_not_min_and_small_unsigned,
    pairs_of_positive_signed_and_small_unsigned, pairs_of_positive_unsigned_and_small_unsigned,
    pairs_of_signed_and_rounding_mode, pairs_of_signed_and_small_unsigned,
    pairs_of_signed_and_small_unsigned_var_1, pairs_of_unsigned_and_rounding_mode,
    pairs_of_unsigned_and_small_unsigned, pairs_of_unsigned_and_small_unsigned_var_1,
    triples_of_signed_small_signed_and_rounding_mode_var_1,
    triples_of_signed_small_unsigned_and_rounding_mode_var_1,
    triples_of_unsigned_small_signed_and_rounding_mode_var_1,
    triples_of_unsigned_small_unsigned_and_rounding_mode_var_1,
};

fn shr_round_unsigned_unsigned_helper<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned + Rand>()
where
    T: Shl<U, Output = T>
        + Shr<U, Output = T>
        + ShrRound<U, Output = T>
        + ShrRoundAssign<U>
        + ArithmeticCheckedShl<U, Output = T>,
{
    test_properties(
        triples_of_unsigned_small_unsigned_and_rounding_mode_var_1::<T, U>,
        |&(n, u, rm)| {
            let mut mut_n = n;
            mut_n.shr_round_assign(u, rm);
            let shifted = mut_n;

            assert_eq!(n.shr_round(u, rm), shifted);
            assert!(shifted <= n);
        },
    );

    test_properties(pairs_of_unsigned_and_small_unsigned::<T, U>, |&(n, u)| {
        if u < U::exact_from(T::WIDTH) {
            if let Some(shifted) = n.arithmetic_checked_shl(u) {
                assert_eq!(shifted.shr_round(u, RoundingMode::Down), n);
                assert_eq!(shifted.shr_round(u, RoundingMode::Up), n);
                assert_eq!(shifted.shr_round(u, RoundingMode::Floor), n);
                assert_eq!(shifted.shr_round(u, RoundingMode::Ceiling), n);
                assert_eq!(shifted.shr_round(u, RoundingMode::Nearest), n);
                assert_eq!(shifted.shr_round(u, RoundingMode::Exact), n);
            }
        }
    });

    // TODO test using Rationals
    test_properties(
        pairs_of_unsigned_and_small_unsigned_var_1::<T, U>,
        |&(n, u)| {
            let down = n.shr_round(u, RoundingMode::Down);
            if let Some(up) = down.checked_add(T::ONE) {
                assert_eq!(n.shr_round(u, RoundingMode::Up), up);
                assert_eq!(n.shr_round(u, RoundingMode::Floor), down);
                assert_eq!(n.shr_round(u, RoundingMode::Ceiling), up);
                let nearest = n.shr_round(u, RoundingMode::Nearest);
                assert!(nearest == down || nearest == up);
            }
        },
    );

    test_properties(
        pairs_of_positive_unsigned_and_small_unsigned::<T, U>,
        |&(t, u)| {
            if let Some(shift) = u.checked_add(U::exact_from(T::WIDTH)) {
                assert_eq!(t.shr_round(shift, RoundingMode::Down), T::ZERO);
                assert_eq!(t.shr_round(shift, RoundingMode::Floor), T::ZERO);
                assert_eq!(t.shr_round(shift, RoundingMode::Up), T::ONE);
                assert_eq!(t.shr_round(shift, RoundingMode::Ceiling), T::ONE);
                if let Some(extra_shift) = shift.checked_add(U::ONE) {
                    assert_eq!(t.shr_round(extra_shift, RoundingMode::Nearest), T::ZERO);
                }
            }
        },
    );

    test_properties(pairs_of_unsigned_and_rounding_mode::<T>, |&(n, rm)| {
        assert_eq!(n.shr_round(U::ZERO, rm), n);
    });

    test_properties(pairs_of_unsigned_and_rounding_mode::<U>, |&(u, rm)| {
        assert_eq!(T::ZERO.shr_round(u, rm), T::ZERO);
    });
}

fn shr_round_unsigned_signed_helper<T: PrimitiveUnsigned + Rand, U: PrimitiveSigned + Rand>()
where
    T: Shl<U, Output = T>
        + Shr<U, Output = T>
        + ShlRound<U, Output = T>
        + ShrRound<U, Output = T>
        + ShrRoundAssign<U>,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(
        triples_of_unsigned_small_signed_and_rounding_mode_var_1::<T, U>,
        |&(n, i, rm)| {
            let mut mut_n = n;
            mut_n.shr_round_assign(i, rm);
            let shifted = mut_n;

            assert_eq!(n.shr_round(i, rm), shifted);
            if i >= U::ZERO {
                assert!(shifted <= n);
            }
            if i != U::MIN {
                assert_eq!(n.shl_round(-i, rm), shifted);
            }
        },
    );

    test_properties(pairs_of_unsigned_and_rounding_mode::<T>, |&(n, rm)| {
        assert_eq!(n.shr_round(U::ZERO, rm), n);
    });

    test_properties(pairs_of_signed_and_rounding_mode::<U>, |&(u, rm)| {
        assert_eq!(T::ZERO.shr_round(u, rm), T::ZERO);
    });
}

fn shr_round_signed_unsigned_helper<T: PrimitiveSigned + Rand, U: PrimitiveUnsigned + Rand>()
where
    T: Shl<U, Output = T>
        + ArithmeticCheckedShl<U, Output = T>
        + Shr<U, Output = T>
        + ShrRound<U, Output = T>
        + ShrRoundAssign<U>
        + WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: Rand,
{
    test_properties(
        triples_of_signed_small_unsigned_and_rounding_mode_var_1::<T, U>,
        |&(n, u, rm)| {
            let mut mut_n = n;
            mut_n.shr_round_assign(u, rm);
            let shifted = mut_n;
            assert_eq!(n.shr_round(u, rm), shifted);

            assert!(n.shr_round(u, rm).le_abs(&n));
            if n != T::MIN {
                let x = (-n).shr_round(u, -rm);
                if x != T::MIN {
                    assert_eq!(-x, shifted);
                }
            }
        },
    );

    test_properties(pairs_of_signed_and_small_unsigned::<T, U>, |&(n, u)| {
        if u < U::exact_from(T::WIDTH) {
            if let Some(shifted) = n.arithmetic_checked_shl(u) {
                assert_eq!(shifted.shr_round(u, RoundingMode::Down), n);
                assert_eq!(shifted.shr_round(u, RoundingMode::Up), n);
                assert_eq!(shifted.shr_round(u, RoundingMode::Floor), n);
                assert_eq!(shifted.shr_round(u, RoundingMode::Ceiling), n);
                assert_eq!(shifted.shr_round(u, RoundingMode::Nearest), n);
                assert_eq!(shifted.shr_round(u, RoundingMode::Exact), n);
            }
        }
    });

    // TODO test using Rationals
    test_properties(
        pairs_of_signed_and_small_unsigned_var_1::<T, U>,
        |&(n, u)| {
            let floor = n.shr_round(u, RoundingMode::Floor);
            if let Some(ceiling) = floor.checked_add(T::ONE) {
                assert_eq!(n.shr_round(u, RoundingMode::Ceiling), ceiling);
                if n >= T::ZERO {
                    assert_eq!(n.shr_round(u, RoundingMode::Up), ceiling);
                    assert_eq!(n.shr_round(u, RoundingMode::Down), floor);
                } else {
                    assert_eq!(n.shr_round(u, RoundingMode::Up), floor);
                    assert_eq!(n.shr_round(u, RoundingMode::Down), ceiling);
                }
                let nearest = n.shr_round(u, RoundingMode::Nearest);
                assert!(nearest == floor || nearest == ceiling);
            }
        },
    );

    test_properties(
        pairs_of_positive_signed_and_small_unsigned::<T, U>,
        |&(i, u)| {
            if let Some(shift) = u.checked_add(U::exact_from(T::WIDTH - 1)) {
                assert_eq!(i.shr_round(shift, RoundingMode::Down), T::ZERO);
                assert_eq!(i.shr_round(shift, RoundingMode::Floor), T::ZERO);
                assert_eq!(i.shr_round(shift, RoundingMode::Up), T::ONE);
                assert_eq!(i.shr_round(shift, RoundingMode::Ceiling), T::ONE);
                if let Some(extra_shift) = shift.checked_add(U::ONE) {
                    assert_eq!(i.shr_round(extra_shift, RoundingMode::Nearest), T::ZERO);
                }
            }
        },
    );

    test_properties(
        pairs_of_negative_signed_not_min_and_small_unsigned::<T, U>,
        |&(i, u)| {
            if let Some(shift) = u.checked_add(U::exact_from(T::WIDTH - 1)) {
                assert_eq!(i.shr_round(shift, RoundingMode::Down), T::ZERO);
                assert_eq!(i.shr_round(shift, RoundingMode::Floor), T::NEGATIVE_ONE);
                assert_eq!(i.shr_round(shift, RoundingMode::Up), T::NEGATIVE_ONE);
                assert_eq!(i.shr_round(shift, RoundingMode::Ceiling), T::ZERO);
                if let Some(extra_shift) = shift.checked_add(U::ONE) {
                    assert_eq!(i.shr_round(extra_shift, RoundingMode::Nearest), T::ZERO);
                }
            }
        },
    );

    test_properties(pairs_of_signed_and_rounding_mode::<T>, |&(n, rm)| {
        assert_eq!(n.shr_round(U::ZERO, rm), n);
    });

    test_properties(pairs_of_unsigned_and_rounding_mode::<U>, |&(u, rm)| {
        assert_eq!(T::ZERO.shr_round(u, rm), T::ZERO);
    });
}

fn shr_round_signed_signed_helper<T: PrimitiveSigned + Rand, U: PrimitiveSigned + Rand>()
where
    T: Shl<U, Output = T>
        + Shr<U, Output = T>
        + ShlRound<U, Output = T>
        + ShrRound<U, Output = T>
        + ShrRoundAssign<U>
        + ArithmeticCheckedShl<U, Output = T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(
        triples_of_signed_small_signed_and_rounding_mode_var_1::<T, U>,
        |&(n, i, rm)| {
            let mut mut_n = n;
            mut_n.shr_round_assign(i, rm);
            let shifted = mut_n;

            assert_eq!(n.shr_round(i, rm), shifted);
            if i >= U::ZERO {
                assert!(shifted.le_abs(&n));
            }
            if i != U::MIN {
                assert_eq!(n.shl_round(-i, rm), shifted);
            }
        },
    );

    test_properties(pairs_of_signed_and_rounding_mode::<T>, |&(n, rm)| {
        assert_eq!(n.shr_round(U::ZERO, rm), n);
    });

    test_properties(pairs_of_signed_and_rounding_mode::<U>, |&(u, rm)| {
        assert_eq!(T::ZERO.shr_round(u, rm), T::ZERO);
    });
}

#[test]
fn shr_round_properties() {
    shr_round_unsigned_unsigned_helper::<u8, u8>();
    shr_round_unsigned_unsigned_helper::<u8, u16>();
    shr_round_unsigned_unsigned_helper::<u8, u32>();
    shr_round_unsigned_unsigned_helper::<u8, u64>();
    shr_round_unsigned_unsigned_helper::<u8, usize>();
    shr_round_unsigned_unsigned_helper::<u16, u8>();
    shr_round_unsigned_unsigned_helper::<u16, u16>();
    shr_round_unsigned_unsigned_helper::<u16, u32>();
    shr_round_unsigned_unsigned_helper::<u16, u64>();
    shr_round_unsigned_unsigned_helper::<u16, usize>();
    shr_round_unsigned_unsigned_helper::<u32, u8>();
    shr_round_unsigned_unsigned_helper::<u32, u16>();
    shr_round_unsigned_unsigned_helper::<u32, u32>();
    shr_round_unsigned_unsigned_helper::<u32, u64>();
    shr_round_unsigned_unsigned_helper::<u32, usize>();
    shr_round_unsigned_unsigned_helper::<u64, u8>();
    shr_round_unsigned_unsigned_helper::<u64, u16>();
    shr_round_unsigned_unsigned_helper::<u64, u32>();
    shr_round_unsigned_unsigned_helper::<u64, u64>();
    shr_round_unsigned_unsigned_helper::<u64, usize>();
    shr_round_unsigned_unsigned_helper::<usize, u8>();
    shr_round_unsigned_unsigned_helper::<usize, u16>();
    shr_round_unsigned_unsigned_helper::<usize, u32>();
    shr_round_unsigned_unsigned_helper::<usize, u64>();
    shr_round_unsigned_unsigned_helper::<usize, usize>();

    shr_round_unsigned_signed_helper::<u8, i8>();
    shr_round_unsigned_signed_helper::<u8, i16>();
    shr_round_unsigned_signed_helper::<u8, i32>();
    shr_round_unsigned_signed_helper::<u8, i64>();
    shr_round_unsigned_signed_helper::<u8, isize>();
    shr_round_unsigned_signed_helper::<u16, i8>();
    shr_round_unsigned_signed_helper::<u16, i16>();
    shr_round_unsigned_signed_helper::<u16, i32>();
    shr_round_unsigned_signed_helper::<u16, i64>();
    shr_round_unsigned_signed_helper::<u16, isize>();
    shr_round_unsigned_signed_helper::<u32, i8>();
    shr_round_unsigned_signed_helper::<u32, i16>();
    shr_round_unsigned_signed_helper::<u32, i32>();
    shr_round_unsigned_signed_helper::<u32, i64>();
    shr_round_unsigned_signed_helper::<u32, isize>();
    shr_round_unsigned_signed_helper::<u64, i8>();
    shr_round_unsigned_signed_helper::<u64, i16>();
    shr_round_unsigned_signed_helper::<u64, i32>();
    shr_round_unsigned_signed_helper::<u64, i64>();
    shr_round_unsigned_signed_helper::<u64, isize>();
    shr_round_unsigned_signed_helper::<usize, i8>();
    shr_round_unsigned_signed_helper::<usize, i16>();
    shr_round_unsigned_signed_helper::<usize, i32>();
    shr_round_unsigned_signed_helper::<usize, i64>();
    shr_round_unsigned_signed_helper::<usize, isize>();

    shr_round_signed_unsigned_helper::<i8, u8>();
    shr_round_signed_unsigned_helper::<i8, u16>();
    shr_round_signed_unsigned_helper::<i8, u32>();
    shr_round_signed_unsigned_helper::<i8, u64>();
    shr_round_signed_unsigned_helper::<i8, usize>();
    shr_round_signed_unsigned_helper::<i16, u8>();
    shr_round_signed_unsigned_helper::<i16, u16>();
    shr_round_signed_unsigned_helper::<i16, u32>();
    shr_round_signed_unsigned_helper::<i16, u64>();
    shr_round_signed_unsigned_helper::<i16, usize>();
    shr_round_signed_unsigned_helper::<i32, u8>();
    shr_round_signed_unsigned_helper::<i32, u16>();
    shr_round_signed_unsigned_helper::<i32, u32>();
    shr_round_signed_unsigned_helper::<i32, u64>();
    shr_round_signed_unsigned_helper::<i32, usize>();
    shr_round_signed_unsigned_helper::<i64, u8>();
    shr_round_signed_unsigned_helper::<i64, u16>();
    shr_round_signed_unsigned_helper::<i64, u32>();
    shr_round_signed_unsigned_helper::<i64, u64>();
    shr_round_signed_unsigned_helper::<i64, usize>();
    shr_round_signed_unsigned_helper::<isize, u8>();
    shr_round_signed_unsigned_helper::<isize, u16>();
    shr_round_signed_unsigned_helper::<isize, u32>();
    shr_round_signed_unsigned_helper::<isize, u64>();
    shr_round_signed_unsigned_helper::<isize, usize>();

    shr_round_signed_signed_helper::<i8, i8>();
    shr_round_signed_signed_helper::<i8, i16>();
    shr_round_signed_signed_helper::<i8, i32>();
    shr_round_signed_signed_helper::<i8, i64>();
    shr_round_signed_signed_helper::<i8, isize>();
    shr_round_signed_signed_helper::<i16, i8>();
    shr_round_signed_signed_helper::<i16, i16>();
    shr_round_signed_signed_helper::<i16, i32>();
    shr_round_signed_signed_helper::<i16, i64>();
    shr_round_signed_signed_helper::<i16, isize>();
    shr_round_signed_signed_helper::<i32, i8>();
    shr_round_signed_signed_helper::<i32, i16>();
    shr_round_signed_signed_helper::<i32, i32>();
    shr_round_signed_signed_helper::<i32, i64>();
    shr_round_signed_signed_helper::<i32, isize>();
    shr_round_signed_signed_helper::<i64, i8>();
    shr_round_signed_signed_helper::<i64, i16>();
    shr_round_signed_signed_helper::<i64, i32>();
    shr_round_signed_signed_helper::<i64, i64>();
    shr_round_signed_signed_helper::<i64, isize>();
    shr_round_signed_signed_helper::<isize, i8>();
    shr_round_signed_signed_helper::<isize, i16>();
    shr_round_signed_signed_helper::<isize, i32>();
    shr_round_signed_signed_helper::<isize, i64>();
    shr_round_signed_signed_helper::<isize, isize>();
}
