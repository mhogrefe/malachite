// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::{
    MulShrRound, MulShrRoundAssign, ShrRound, UnsignedAbs,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::SaturatingFrom;
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    signed_signed_unsigned_rounding_mode_quadruple_gen_var_1,
    unsigned_unsigned_unsigned_rounding_mode_quadruple_gen_var_1,
};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

#[test]
fn test_mul_shr_round() {
    fn test<T: MulShrRound<T, u64, Output = T> + MulShrRoundAssign<T, u64> + PrimitiveInt>(
        x: T,
        y: T,
        bits: u64,
        rm: RoundingMode,
        out: T,
        o: Ordering,
    ) {
        assert_eq!(x.mul_shr_round(y, bits, rm), (out, o));
        let mut mut_x = x;
        assert_eq!(mut_x.mul_shr_round_assign(y, bits, rm), o);
        assert_eq!(mut_x, out);
    }
    // - zero operands
    test(0u8, 0u8, 0, Down, 0, Equal);
    test(0u8, 200u8, 100, Exact, 0, Equal);
    // - bits < WIDTH, fitting
    test(5u8, 102u8, 2, Down, 127, Less);
    // - a tie, broken upward because the floor is odd
    test(5u8, 102u8, 2, Nearest, 128, Greater);
    // - bits == 0, fitting: a plain product
    test(3u8, 4u8, 0, Exact, 12, Equal);
    // - bits == WIDTH, all modes
    test(100u8, 200u8, 8, Down, 78, Less);
    test(100u8, 200u8, 8, Floor, 78, Less);
    test(100u8, 200u8, 8, Up, 79, Greater);
    test(100u8, 200u8, 8, Ceiling, 79, Greater);
    test(100u8, 200u8, 8, Nearest, 78, Less);
    test(96u8, 8u8, 8, Exact, 3, Equal);
    // - bits > 2 * WIDTH: everything is shifted out
    test(100u8, 200u8, 100, Down, 0, Less);
    test(100u8, 200u8, 100, Up, 1, Greater);
    test(100u8, 200u8, 100, Nearest, 0, Less);
    // - the flagship fixed-point case: the high word of a full-width product
    test(u64::MAX, u64::MAX, 64, Down, u64::MAX - 1, Less);
    test(u64::MAX, u64::MAX, 64, Up, u64::MAX, Greater);
    test(u64::MAX, u64::MAX, 64, Nearest, u64::MAX - 1, Less);
    // - u128: the cut in the low word, at the word boundary, and in the high word
    test(
        10u128.pow(30),
        10u128.pow(30),
        127,
        Down,
        5877471754111437539843,
        Less,
    );
    test(
        10u128.pow(30),
        10u128.pow(30),
        127,
        Nearest,
        5877471754111437539844,
        Greater,
    );
    test(
        10u128.pow(30),
        10u128.pow(30),
        128,
        Down,
        2938735877055718769921,
        Less,
    );
    test(
        10u128.pow(30),
        10u128.pow(30),
        128,
        Nearest,
        2938735877055718769922,
        Greater,
    );
    test(
        10u128.pow(30),
        10u128.pow(30),
        129,
        Down,
        1469367938527859384960,
        Less,
    );
    test(
        10u128.pow(30),
        10u128.pow(30),
        129,
        Up,
        1469367938527859384961,
        Greater,
    );
    // - a Nearest that rounds a sub-half value up to 1: 10^60 is between 2^199 and 2^200
    test(10u128.pow(30), 10u128.pow(30), 200, Nearest, 1, Greater);
    test(10u128.pow(30), 10u128.pow(30), 255, Nearest, 0, Less);
    // - usize, width-independently
    test(10usize, 20usize, 4, Down, 12, Less);
    // - signed: Floor and Down split on a negative product
    test(-100i16, 200i16, 8, Floor, -79, Less);
    test(-100i16, 200i16, 8, Down, -78, Greater);
    test(-100i16, 200i16, 8, Up, -79, Less);
    test(-100i16, 200i16, 8, Ceiling, -78, Greater);
    test(-100i16, 200i16, 8, Nearest, -78, Greater);
    test(100i16, -200i16, 8, Floor, -79, Less);
    test(-96i16, 8i16, 8, Exact, -3, Equal);
    // - a negative tie rounds its magnitude up because the floor of the magnitude is odd
    test(-5i16, 102i16, 2, Nearest, -128, Less);
    // - the result lands exactly on MIN
    test(-128i8, 64i8, 6, Down, i8::MIN, Equal);
    test(-128i8, 64i8, 7, Down, -64, Equal);
    test(-128i8, -128i8, 8, Down, 64, Equal);
}

#[test]
fn test_mul_shr_round_signed_bits() {
    fn test<T: MulShrRound<T, i64, Output = T> + MulShrRoundAssign<T, i64> + PrimitiveInt>(
        x: T,
        y: T,
        bits: i64,
        rm: RoundingMode,
        out: T,
        o: Ordering,
    ) {
        assert_eq!(x.mul_shr_round(y, bits, rm), (out, o));
        let mut mut_x = x;
        assert_eq!(mut_x.mul_shr_round_assign(y, bits, rm), o);
        assert_eq!(mut_x, out);
    }
    // - nonnegative bits delegate to the unsigned-bits implementation
    test(100u8, 200u8, 8, Down, 78, Less);
    test(-100i16, 200i16, 8, Floor, -79, Less);
    // - negative bits shift left, exactly
    test(3u8, 5u8, -2, Floor, 60, Equal);
    test(-3i8, 5i8, -3, Down, -120, Equal);
    // - a left shift landing exactly on MIN
    test(-64i8, 1i8, -1, Down, i8::MIN, Equal);
}

#[test]
fn mul_shr_round_fail() {
    // - bits < WIDTH and the floor does not fit
    assert_panic!(255u8.mul_shr_round(255u8, 2u64, Down));
    // - bits == 0 and the product does not fit
    assert_panic!(255u8.mul_shr_round(255u8, 0u64, Down));
    // - the floor fits but rounding up overflows: 7 * 73 = 511, and 511 / 2 = 255.5
    assert_panic!(7u8.mul_shr_round(73u8, 1u64, Up));
    assert_panic!(7u8.mul_shr_round(73u8, 1u64, Nearest));
    // - Exact but inexact
    assert_panic!(100u8.mul_shr_round(200u8, 8u64, Exact));
    // - signed results out of range on both sides
    assert_panic!((-128i8).mul_shr_round(127i8, 6u64, Floor));
    assert_panic!((-128i8).mul_shr_round(-128i8, 7u64, Down));
    // - left shifts that do not fit
    assert_panic!(16u8.mul_shr_round(16u8, -1i64, Down));
    assert_panic!(64u8.mul_shr_round(2u8, -1i64, Down));
}

fn mul_shr_round_properties_helper_unsigned_unsigned<
    T: MulShrRound<T, U, Output = T>
        + MulShrRoundAssign<T, U>
        + PrimitiveUnsigned
        + ShrRound<U, Output = T>,
    U: PrimitiveUnsigned,
>()
where
    u64: SaturatingFrom<U>,
{
    unsigned_unsigned_unsigned_rounding_mode_quadruple_gen_var_1::<T, U>().test_properties(
        |(x, y, bits, rm)| {
            let (r, o) = x.mul_shr_round(y, bits, rm);
            let mut mut_x = x;
            assert_eq!(mut_x.mul_shr_round_assign(y, bits, rm), o);
            assert_eq!(mut_x, r);

            // multiplication commutes
            assert_eq!(y.mul_shr_round(x, bits, rm), (r, o));

            // exactness is a trailing-zeros question, decidable independently of the product
            if x == T::ZERO || y == T::ZERO {
                assert_eq!((r, o), (T::ZERO, Equal));
            } else {
                let exact = TrailingZeros::trailing_zeros(x) + TrailingZeros::trailing_zeros(y)
                    >= u64::saturating_from(bits);
                assert_eq!(o == Equal, exact);
            }

            // the rounding direction respects the mode
            match rm {
                Down | Floor => assert_ne!(o, Greater),
                Up | Ceiling => assert_ne!(o, Less),
                Exact => assert_eq!(o, Equal),
                Nearest => {}
            }

            // all modes relate to the floor as they should
            let (down, o_down) = x.mul_shr_round(y, bits, Down);
            assert_eq!(x.mul_shr_round(y, bits, Floor), (down, o_down));
            if o_down == Equal {
                assert_eq!((r, o), (down, Equal));
            } else if down != T::MAX {
                assert_eq!(x.mul_shr_round(y, bits, Up), (down + T::ONE, Greater));
                assert_eq!(x.mul_shr_round(y, bits, Ceiling), (down + T::ONE, Greater));
                let (nearest, o_nearest) = x.mul_shr_round(y, bits, Nearest);
                assert!(nearest == down || nearest == down + T::ONE);
                assert_eq!(o_nearest, if nearest == down { Less } else { Greater });
            }

            // A multiplier of 1 turns the operation into shr_round. `Exact` must be skipped: the
            // generator validated exactness for x * y, not for x alone.
            if rm != Exact {
                assert_eq!(x.mul_shr_round(T::ONE, bits, rm), x.shr_round(bits, rm));
            }
        },
    );
}

fn mul_shr_round_properties_helper_signed_unsigned<
    T: MulShrRound<T, B, Output = T>
        + MulShrRoundAssign<T, B>
        + PrimitiveSigned
        + UnsignedAbs<Output = U>,
    U: PrimitiveUnsigned,
    B: PrimitiveUnsigned,
>()
where
    u64: SaturatingFrom<B>,
{
    signed_signed_unsigned_rounding_mode_quadruple_gen_var_1::<T, U, B>().test_properties(
        |(x, y, bits, rm)| {
            let (r, o) = x.mul_shr_round(y, bits, rm);
            let mut mut_x = x;
            assert_eq!(mut_x.mul_shr_round_assign(y, bits, rm), o);
            assert_eq!(mut_x, r);

            // multiplication commutes
            assert_eq!(y.mul_shr_round(x, bits, rm), (r, o));

            // exactness is a trailing-zeros question, decidable independently of the product
            if x == T::ZERO || y == T::ZERO {
                assert_eq!((r, o), (T::ZERO, Equal));
            } else {
                let exact = TrailingZeros::trailing_zeros(x.unsigned_abs())
                    + TrailingZeros::trailing_zeros(y.unsigned_abs())
                    >= u64::saturating_from(bits);
                assert_eq!(o == Equal, exact);
            }

            // the rounding direction respects the mode; Down and Up are relative to zero, so their
            // direction depends on the product's sign
            let negative = (x < T::ZERO) != (y < T::ZERO) && x != T::ZERO && y != T::ZERO;
            match rm {
                Floor => assert_ne!(o, Greater),
                Ceiling => assert_ne!(o, Less),
                Down => assert_ne!(o, if negative { Less } else { Greater }),
                Up => assert_ne!(o, if negative { Greater } else { Less }),
                Exact => assert_eq!(o, Equal),
                Nearest => {}
            }

            // negating one factor negates the result, with the mode and Ordering mirrored
            if y != T::MIN && r != T::MIN {
                assert_eq!(x.mul_shr_round(-y, bits, -rm), (-r, o.reverse()));
            }
        },
    );
}

macro_rules! widening_oracle_unsigned {
    ($t:ident, $w:ident, $f:ident) => {
        fn $f() {
            unsigned_unsigned_unsigned_rounding_mode_quadruple_gen_var_1::<$t, u64>()
                .test_properties(|(x, y, bits, rm)| {
                    let wide = $w::from(x) * $w::from(y);
                    let (wr, wo) = wide.shr_round(bits, rm);
                    let (r, o) = x.mul_shr_round(y, bits, rm);
                    assert_eq!(($w::from(r), o), (wr, wo));
                });
        }
    };
}
widening_oracle_unsigned!(u8, u16, widening_oracle_u8);
widening_oracle_unsigned!(u16, u32, widening_oracle_u16);
widening_oracle_unsigned!(u32, u64, widening_oracle_u32);
widening_oracle_unsigned!(u64, u128, widening_oracle_u64);

macro_rules! widening_oracle_signed {
    ($t:ident, $u:ident, $w:ident, $f:ident) => {
        fn $f() {
            signed_signed_unsigned_rounding_mode_quadruple_gen_var_1::<$t, $u, u64>()
                .test_properties(|(x, y, bits, rm)| {
                    let wide = $w::from(x) * $w::from(y);
                    let (wr, wo) = wide.shr_round(bits, rm);
                    let (r, o) = x.mul_shr_round(y, bits, rm);
                    assert_eq!(($w::from(r), o), (wr, wo));
                });
        }
    };
}
widening_oracle_signed!(i8, u8, i16, widening_oracle_i8);
widening_oracle_signed!(i16, u16, i32, widening_oracle_i16);
widening_oracle_signed!(i32, u32, i64, widening_oracle_i32);
widening_oracle_signed!(i64, u64, i128, widening_oracle_i64);

macro_rules! apply_to_signed_mag_and_bits {
    ($f:ident) => {
        $f::<i8, u8, u8>();
        $f::<i8, u8, u16>();
        $f::<i8, u8, u32>();
        $f::<i8, u8, u64>();
        $f::<i8, u8, u128>();
        $f::<i8, u8, usize>();
        $f::<i16, u16, u8>();
        $f::<i16, u16, u16>();
        $f::<i16, u16, u32>();
        $f::<i16, u16, u64>();
        $f::<i16, u16, u128>();
        $f::<i16, u16, usize>();
        $f::<i32, u32, u8>();
        $f::<i32, u32, u16>();
        $f::<i32, u32, u32>();
        $f::<i32, u32, u64>();
        $f::<i32, u32, u128>();
        $f::<i32, u32, usize>();
        $f::<i64, u64, u8>();
        $f::<i64, u64, u16>();
        $f::<i64, u64, u32>();
        $f::<i64, u64, u64>();
        $f::<i64, u64, u128>();
        $f::<i64, u64, usize>();
        $f::<i128, u128, u8>();
        $f::<i128, u128, u16>();
        $f::<i128, u128, u32>();
        $f::<i128, u128, u64>();
        $f::<i128, u128, u128>();
        $f::<i128, u128, usize>();
        $f::<isize, usize, u8>();
        $f::<isize, usize, u16>();
        $f::<isize, usize, u32>();
        $f::<isize, usize, u64>();
        $f::<isize, usize, u128>();
        $f::<isize, usize, usize>();
    };
}

#[test]
fn mul_shr_round_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(mul_shr_round_properties_helper_unsigned_unsigned);
    apply_to_signed_mag_and_bits!(mul_shr_round_properties_helper_signed_unsigned);
    widening_oracle_u8();
    widening_oracle_u16();
    widening_oracle_u32();
    widening_oracle_u64();
    widening_oracle_i8();
    widening_oracle_i16();
    widening_oracle_i32();
    widening_oracle_i64();
}
