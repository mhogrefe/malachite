// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Average, DivRound};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    primitive_float_pair_gen, signed_pair_gen, signed_signed_rounding_mode_triple_gen_var_5,
    unsigned_pair_gen_var_27, unsigned_unsigned_rounding_mode_triple_gen_var_9,
};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

#[test]
fn test_average() {
    fn test_u<T: PrimitiveUnsigned>(x: T, y: T, out: T) {
        assert_eq!(x.average(y), out);
        assert_eq!(y.average(x), out);
        let mut mut_x = x;
        mut_x.average_assign(y);
        assert_eq!(mut_x, out);
    }
    test_u::<u8>(0, 0, 0);
    test_u::<u16>(4, 6, 5);
    // 4.5 rounds to the even neighbor, 4
    test_u::<u32>(4, 5, 4);
    // 5.5 rounds to the even neighbor, 6
    test_u::<u64>(5, 6, 6);
    test_u::<usize>(123, 456, 290);
    // no overflow near the top of the range
    test_u::<u8>(u8::MAX, u8::MAX, u8::MAX);
    test_u::<u8>(u8::MAX, u8::MAX - 2, u8::MAX - 1);
    test_u::<u128>(u128::MAX, u128::MAX - 2, u128::MAX - 1);

    fn test_s<T: PrimitiveSigned>(x: T, y: T, out: T) {
        assert_eq!(x.average(y), out);
        assert_eq!(y.average(x), out);
        let mut mut_x = x;
        mut_x.average_assign(y);
        assert_eq!(mut_x, out);
    }
    test_s::<i8>(0, 0, 0);
    test_s::<i16>(4, 6, 5);
    test_s::<i32>(-4, -6, -5);
    // -4.5 rounds to the even neighbor, -4
    test_s::<i64>(-4, -5, -4);
    // -5.5 rounds to the even neighbor, -6
    test_s::<i128>(-5, -6, -6);
    test_s::<isize>(-123, 456, 166);
    // -0.5 rounds to the even neighbor, 0
    test_s::<i8>(i8::MIN, i8::MAX, 0);
    test_s::<i8>(i8::MIN, i8::MIN, i8::MIN);
    test_s::<i8>(i8::MIN, i8::MIN + 2, i8::MIN + 1);
}

#[test]
fn test_average_round() {
    fn test_u<T: PrimitiveUnsigned>(x: T, y: T, rm: RoundingMode, out: T, o: Ordering) {
        assert_eq!(x.average_round(y, rm), (out, o));
        assert_eq!(y.average_round(x, rm), (out, o));
        let mut mut_x = x;
        assert_eq!(mut_x.average_round_assign(y, rm), o);
        assert_eq!(mut_x, out);
    }
    // - exact averages are unaffected by the rounding mode
    test_u::<u8>(4, 6, Floor, 5, Equal);
    test_u::<u8>(4, 6, Ceiling, 5, Equal);
    test_u::<u8>(4, 6, Down, 5, Equal);
    test_u::<u8>(4, 6, Up, 5, Equal);
    test_u::<u8>(4, 6, Nearest, 5, Equal);
    test_u::<u8>(4, 6, Exact, 5, Equal);
    // - the exact average is 5.5
    test_u::<u16>(4, 7, Floor, 5, Less);
    test_u::<u16>(4, 7, Ceiling, 6, Greater);
    // - for unsigned values Down and Up coincide with Floor and Ceiling
    test_u::<u32>(4, 7, Down, 5, Less);
    test_u::<u32>(4, 7, Up, 6, Greater);
    test_u::<u64>(4, 7, Nearest, 6, Greater);
    test_u::<u64>(4, 9, Nearest, 6, Less);
    // - the ceiling does not overflow
    test_u::<u8>(u8::MAX, u8::MAX - 1, Ceiling, u8::MAX, Greater);
    test_u::<u128>(u128::MAX, u128::MAX - 1, Ceiling, u128::MAX, Greater);

    fn test_s<T: PrimitiveSigned>(x: T, y: T, rm: RoundingMode, out: T, o: Ordering) {
        assert_eq!(x.average_round(y, rm), (out, o));
        assert_eq!(y.average_round(x, rm), (out, o));
        let mut mut_x = x;
        assert_eq!(mut_x.average_round_assign(y, rm), o);
        assert_eq!(mut_x, out);
    }
    test_s::<i8>(-4, -6, Exact, -5, Equal);
    // - the exact average is -5.5; Down rounds toward zero and Up away from it
    test_s::<i16>(-4, -7, Floor, -6, Less);
    test_s::<i16>(-4, -7, Ceiling, -5, Greater);
    test_s::<i32>(-4, -7, Down, -5, Greater);
    test_s::<i32>(-4, -7, Up, -6, Less);
    test_s::<i64>(-4, -7, Nearest, -6, Less);
    test_s::<i64>(-4, -9, Nearest, -6, Greater);
    // - the exact average of MIN and MAX is -0.5
    test_s::<i8>(i8::MIN, i8::MAX, Floor, -1, Less);
    test_s::<i8>(i8::MIN, i8::MAX, Ceiling, 0, Greater);
    test_s::<i8>(i8::MIN, i8::MAX, Down, 0, Greater);
    test_s::<i8>(i8::MIN, i8::MAX, Up, -1, Less);
    test_s::<i8>(i8::MIN, i8::MAX, Nearest, 0, Greater);
    // - the floor does not overflow at the bottom of the range
    test_s::<i8>(i8::MIN, i8::MIN + 1, Floor, i8::MIN, Less);
    test_s::<i128>(i128::MIN, i128::MIN + 1, Floor, i128::MIN, Less);
}

fn average_round_exact_fail_helper_unsigned<T: PrimitiveUnsigned>() {
    assert_panic!(T::exact_from(4).average_round(T::exact_from(7), Exact));
}

fn average_round_exact_fail_helper_signed<T: PrimitiveSigned>() {
    assert_panic!(T::exact_from(4).average_round(T::exact_from(7), Exact));
    assert_panic!(T::NEGATIVE_ONE.average_round(T::TWO, Exact));
}

#[test]
fn average_round_fail() {
    apply_fn_to_unsigneds!(average_round_exact_fail_helper_unsigned);
    apply_fn_to_signeds!(average_round_exact_fail_helper_signed);
}

fn average_round_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    u128: ExactFrom<T>,
{
    unsigned_unsigned_rounding_mode_triple_gen_var_9::<T>().test_properties(|(x, y, rm)| {
        let (avg, o) = x.average_round(y, rm);
        assert_eq!(y.average_round(x, rm), (avg, o));
        let mut mut_x = x;
        assert_eq!(mut_x.average_round_assign(y, rm), o);
        assert_eq!(mut_x, avg);

        assert!(avg >= core::cmp::min(x, y));
        assert!(avg <= core::cmp::max(x, y));
        let (floor, floor_o) = x.average_round(y, Floor);
        let (ceiling, ceiling_o) = x.average_round(y, Ceiling);
        assert!(avg >= floor);
        assert!(avg <= ceiling);
        // the floor and ceiling of the average sum to x + y
        assert_eq!(floor.wrapping_add(ceiling), x.wrapping_add(y));
        if (x ^ y).even() {
            assert_eq!(o, Equal);
            assert_eq!(floor, ceiling);
            assert_eq!(floor_o, Equal);
        } else {
            assert_eq!(ceiling, floor + T::ONE);
            assert_eq!(floor_o, Less);
            assert_eq!(ceiling_o, Greater);
            match o {
                Less => assert_eq!(avg, floor),
                Greater => assert_eq!(avg, ceiling),
                Equal => panic!("inexact average reported as exact"),
            }
        }

        // an independent computation through a wider type
        if T::WIDTH < u128::WIDTH {
            assert_eq!(
                (u128::exact_from(x) + u128::exact_from(y)).div_round(2, rm),
                (u128::exact_from(avg), o)
            );
        }
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        let avg = x.average(y);
        assert_eq!(y.average(x), avg);
        let (nearest_avg, nearest_o) = x.average_round(y, Nearest);
        assert_eq!(nearest_avg, avg);
        assert_eq!(nearest_o == Equal, (x ^ y).even());
        let mut mut_x = x;
        mut_x.average_assign(y);
        assert_eq!(mut_x, avg);
        // a two-way tie rounds to the even neighbor
        if (x ^ y).odd() {
            assert!(avg.even());
        }
        assert_eq!(x.average(x), x);
    });
}

fn average_round_properties_helper_signed<T: PrimitiveSigned>()
where
    i128: ExactFrom<T>,
{
    signed_signed_rounding_mode_triple_gen_var_5::<T>().test_properties(|(x, y, rm)| {
        let (avg, o) = x.average_round(y, rm);
        assert_eq!(y.average_round(x, rm), (avg, o));
        let mut mut_x = x;
        assert_eq!(mut_x.average_round_assign(y, rm), o);
        assert_eq!(mut_x, avg);

        assert!(avg >= core::cmp::min(x, y));
        assert!(avg <= core::cmp::max(x, y));
        let (floor, floor_o) = x.average_round(y, Floor);
        let (ceiling, ceiling_o) = x.average_round(y, Ceiling);
        assert!(avg >= floor);
        assert!(avg <= ceiling);
        assert_eq!(floor.wrapping_add(ceiling), x.wrapping_add(y));
        if (x ^ y).even() {
            assert_eq!(o, Equal);
            assert_eq!(floor, ceiling);
            assert_eq!(floor_o, Equal);
        } else {
            assert_eq!(ceiling, floor + T::ONE);
            assert_eq!(floor_o, Less);
            assert_eq!(ceiling_o, Greater);
            match o {
                Less => assert_eq!(avg, floor),
                Greater => assert_eq!(avg, ceiling),
                Equal => panic!("inexact average reported as exact"),
            }
        }
        // averaging the negatives negates the average with the opposite rounding
        if x != T::MIN && y != T::MIN {
            assert_eq!((-x).average_round(-y, -rm), (-avg, o.reverse()));
        }

        if T::WIDTH < i128::WIDTH {
            assert_eq!(
                (i128::exact_from(x) + i128::exact_from(y)).div_round(2, rm),
                (i128::exact_from(avg), o)
            );
        }
    });

    signed_pair_gen::<T>().test_properties(|(x, y)| {
        let avg = x.average(y);
        assert_eq!(y.average(x), avg);
        assert_eq!(x.average_round(y, Nearest).0, avg);
        let mut mut_x = x;
        mut_x.average_assign(y);
        assert_eq!(mut_x, avg);
        if (x ^ y).odd() {
            assert!(avg.even());
        }
        assert_eq!(x.average(x), x);
    });
}

#[test]
fn average_round_properties() {
    apply_fn_to_unsigneds!(average_round_properties_helper_unsigned);
    apply_fn_to_signeds!(average_round_properties_helper_signed);
}

#[test]
fn test_average_primitive_float() {
    fn test<T: PrimitiveFloat>(x: T, y: T, out: T) {
        assert_eq!(NiceFloat(x.average(y)), NiceFloat(out));
        assert_eq!(NiceFloat(y.average(x)), NiceFloat(out));
        let mut mut_x = x;
        mut_x.average_assign(y);
        assert_eq!(NiceFloat(mut_x), NiceFloat(out));
    }
    test::<f32>(0.0, 0.0, 0.0);
    test::<f32>(1.0, 2.0, 1.5);
    test::<f64>(-1.0, -2.0, -1.5);
    test::<f64>(1.0, -1.0, 0.0);
    test::<f64>(-0.0, -0.0, -0.0);
    test::<f64>(0.1, 0.3, (0.1 + 0.3) / 2.0);
    // - extreme values whose naive sum would overflow to infinity
    test::<f32>(f32::MAX, f32::MAX, f32::MAX);
    test::<f64>(f64::MAX, f64::MAX, f64::MAX);
    test::<f64>(f64::MIN, f64::MIN, f64::MIN);
    test::<f64>(f64::MAX, f64::MIN, 0.0);
    test::<f64>(f64::MAX, 0.0, f64::MAX / 2.0);
    test::<f64>(f64::MAX, f64::MIN_POSITIVE_SUBNORMAL, f64::MAX / 2.0);
    // - subnormal averages; a two-way tie in the rounding goes to the even mantissa
    test::<f64>(f64::MIN_POSITIVE_SUBNORMAL, 0.0, 0.0);
    test::<f64>(
        f64::MIN_POSITIVE_SUBNORMAL * 3.0,
        0.0,
        f64::MIN_POSITIVE_SUBNORMAL * 2.0,
    );
    test::<f64>(
        f64::MIN_POSITIVE_SUBNORMAL,
        f64::MIN_POSITIVE_SUBNORMAL * 2.0,
        f64::MIN_POSITIVE_SUBNORMAL * 2.0,
    );
    // - special values behave exactly like (x + y) / 2.0
    test::<f64>(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    test::<f64>(f64::INFINITY, 1.0, f64::INFINITY);
    test::<f64>(f64::NEG_INFINITY, f64::MAX, f64::NEG_INFINITY);
    test::<f64>(f64::INFINITY, f64::NEG_INFINITY, f64::NAN);
    test::<f64>(f64::NAN, 1.0, f64::NAN);
    test::<f32>(f32::NAN, f32::INFINITY, f32::NAN);
}

fn average_primitive_float_properties_helper<T: PrimitiveFloat>() {
    primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        let avg = x.average(y);
        assert_eq!(NiceFloat(y.average(x)), NiceFloat(avg));
        let mut mut_x = x;
        mut_x.average_assign(y);
        assert_eq!(NiceFloat(mut_x), NiceFloat(avg));

        assert_eq!(NiceFloat(x.average(x)), NiceFloat(x));
        if x.is_finite() && y.is_finite() {
            // an average of finite values cannot overflow
            assert!(avg.is_finite());
            let (lo, hi) = if x <= y { (x, y) } else { (y, x) };
            assert!(avg >= lo);
            assert!(avg <= hi);
            // when the naive expression is safe, the two agree
            let half_max = T::MAX_FINITE / T::TWO;
            if x.abs() <= half_max && y.abs() <= half_max {
                assert_eq!(NiceFloat(avg), NiceFloat((x + y) / T::TWO));
            }
            let neg_avg = (-x).average(-y);
            if avg == T::ZERO {
                // exact cancellation yields +0.0 for either sign order, as in (x + y) / 2.0
                assert_eq!(neg_avg, T::ZERO);
            } else {
                assert_eq!(NiceFloat(neg_avg), NiceFloat(-avg));
            }
        } else {
            // for special values the function is defined as the naive expression
            assert_eq!(NiceFloat(avg), NiceFloat((x + y) / T::TWO));
        }
    });
}

#[test]
fn average_primitive_float_properties() {
    apply_fn_to_primitive_floats!(average_primitive_float_properties_helper);

    // f64 arithmetic is exact on f32 inputs, so rounding its average to f32 is an independent,
    // correctly rounded oracle, valid for the special values as well
    primitive_float_pair_gen::<f32>().test_properties(|(x, y)| {
        assert_eq!(
            NiceFloat(x.average(y)),
            NiceFloat(((f64::from(x) + f64::from(y)) / 2.0) as f32)
        );
    });
}
