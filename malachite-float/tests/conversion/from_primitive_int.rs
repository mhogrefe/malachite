// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    signed_gen, signed_gen_var_5, signed_pair_gen_var_2, signed_unsigned_pair_gen_var_20,
    unsigned_gen, unsigned_pair_gen_var_32, unsigned_signed_pair_gen_var_1,
};
use malachite_float::test_util::common::{rug_round_try_from_rounding_mode, to_hex_string};
use malachite_float::test_util::generators::*;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;
use rug::Assign;
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

#[test]
fn test_from_primitive_int() {
    fn test_helper<T: PrimitiveInt>(u: T, out: &str, out_hex: &str)
    where
        Float: From<T>,
        rug::Float: Assign<T>,
        Limb: WrappingFrom<T>,
        SignedLimb: WrappingFrom<T>,
    {
        let x = Float::from(u);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);

        let rug_x = rug::Float::with_val(
            if u == T::ZERO {
                1
            } else {
                u32::exact_from(u.significant_bits() - TrailingZeros::trailing_zeros(u))
            },
            u,
        );
        let x = Float::exact_from(&rug_x);
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);

        if T::NAME == Limb::NAME {
            let x_alt = Float::const_from_unsigned(Limb::wrapping_from(u));
            assert!(x_alt.is_valid());
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
        }

        if T::NAME == SignedLimb::NAME {
            let x_alt = Float::const_from_signed(SignedLimb::wrapping_from(u));
            assert!(x_alt.is_valid());
            assert_eq!(ComparableFloat(x_alt), ComparableFloat(x));
        }
    }
    fn test_helper_ui<T: PrimitiveInt>()
    where
        Float: From<T>,
        rug::Float: Assign<T>,
        Limb: WrappingFrom<T>,
        SignedLimb: WrappingFrom<T>,
    {
        test_helper(T::ZERO, "0.0", "0x0.0");
        test_helper(T::ONE, "1.0", "0x1.0#1");
        test_helper(T::exact_from(123u8), "123.0", "0x7b.0#7");
    }
    apply_fn_to_primitive_ints!(test_helper_ui);
    test_helper(1000000000000u64, "1.0e12", "0xe.8d4a51E+9#28");

    fn test_helper_i<T: PrimitiveSigned>()
    where
        Float: From<T>,
        rug::Float: Assign<T>,
        Limb: WrappingFrom<T>,
        SignedLimb: WrappingFrom<T>,
    {
        test_helper(T::NEGATIVE_ONE, "-1.0", "-0x1.0#1");
        test_helper(T::from(-123i8), "-123.0", "-0x7b.0#7");
    }
    apply_fn_to_signeds!(test_helper_i);
    test_helper(-1000000000000i64, "-1.0e12", "-0xe.8d4a51E+9#28");
}

#[test]
fn test_from_primitive_int_prec() {
    fn test_helper_u<T: PrimitiveUnsigned>(
        u: T,
        prec: u64,
        out: &str,
        out_hex: &str,
        out_o: Ordering,
    ) where
        Natural: From<T>,
        rug::Float: Assign<T>,
    {
        let (x, o) = Float::from_unsigned_prec(u, prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let rug_x = rug::Float::with_val(u32::exact_from(prec), u);
        let x = Float::exact_from(&rug_x);
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
    }
    fn test_helper_u2<T: PrimitiveUnsigned>()
    where
        Natural: From<T>,
        rug::Float: Assign<T>,
    {
        test_helper_u(T::ZERO, 1, "0.0", "0x0.0", Equal);
        test_helper_u(T::ZERO, 10, "0.0", "0x0.0", Equal);
        test_helper_u(T::ZERO, 20, "0.0", "0x0.0", Equal);

        test_helper_u(T::ONE, 1, "1.0", "0x1.0#1", Equal);
        test_helper_u(T::ONE, 10, "1.0", "0x1.000#10", Equal);
        test_helper_u(T::ONE, 20, "1.0", "0x1.00000#20", Equal);

        test_helper_u(T::from(123u8), 1, "1.0e2", "0x8.0E+1#1", Greater);
        test_helper_u(T::from(123u8), 10, "123.0", "0x7b.0#10", Equal);
        test_helper_u(T::from(123u8), 20, "123.0", "0x7b.0000#20", Equal);
    }
    apply_fn_to_unsigneds!(test_helper_u2);
    test_helper_u(1000000000000u64, 1, "1.0e12", "0x1.0E+10#1", Greater);
    test_helper_u(1000000000000u64, 10, "9.997e11", "0xe.8cE+9#10", Less);
    test_helper_u(1000000000000u64, 20, "9.999997e11", "0xe.8d4aE+9#20", Less);

    fn test_helper_i<T: PrimitiveSigned>(u: T, prec: u64, out: &str, out_hex: &str, out_o: Ordering)
    where
        Integer: From<T>,
        rug::Float: Assign<T>,
    {
        let (x, o) = Float::from_signed_prec(u, prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let rug_x = rug::Float::with_val(u32::exact_from(prec), u);
        let x = Float::exact_from(&rug_x);
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
    }
    fn test_helper_i2<T: PrimitiveSigned>()
    where
        Integer: From<T>,
        rug::Float: Assign<T>,
    {
        test_helper_i(T::ZERO, 1, "0.0", "0x0.0", Equal);
        test_helper_i(T::ZERO, 10, "0.0", "0x0.0", Equal);
        test_helper_i(T::ZERO, 20, "0.0", "0x0.0", Equal);

        test_helper_i(T::ONE, 1, "1.0", "0x1.0#1", Equal);
        test_helper_i(T::ONE, 10, "1.0", "0x1.000#10", Equal);
        test_helper_i(T::ONE, 20, "1.0", "0x1.00000#20", Equal);

        test_helper_i(T::from(123i8), 1, "1.0e2", "0x8.0E+1#1", Greater);
        test_helper_i(T::from(123i8), 10, "123.0", "0x7b.0#10", Equal);
        test_helper_i(T::from(123i8), 20, "123.0", "0x7b.0000#20", Equal);

        test_helper_i(T::NEGATIVE_ONE, 1, "-1.0", "-0x1.0#1", Equal);
        test_helper_i(T::NEGATIVE_ONE, 10, "-1.0", "-0x1.000#10", Equal);
        test_helper_i(T::NEGATIVE_ONE, 20, "-1.0", "-0x1.00000#20", Equal);

        test_helper_i(T::from(-123i8), 1, "-1.0e2", "-0x8.0E+1#1", Less);
        test_helper_i(T::from(-123i8), 10, "-123.0", "-0x7b.0#10", Equal);
        test_helper_i(T::from(-123i8), 20, "-123.0", "-0x7b.0000#20", Equal);
    }
    apply_fn_to_signeds!(test_helper_i2);
    test_helper_i(1000000000000i64, 1, "1.0e12", "0x1.0E+10#1", Greater);
    test_helper_i(1000000000000i64, 10, "9.997e11", "0xe.8cE+9#10", Less);
    test_helper_i(1000000000000i64, 20, "9.999997e11", "0xe.8d4aE+9#20", Less);
    test_helper_i(-1000000000000i64, 1, "-1.0e12", "-0x1.0E+10#1", Less);
    test_helper_i(-1000000000000i64, 10, "-9.997e11", "-0xe.8cE+9#10", Greater);
    test_helper_i(
        -1000000000000i64,
        20,
        "-9.999997e11",
        "-0xe.8d4aE+9#20",
        Greater,
    );
}

fn from_unsigned_prec_fail_helper<T: PrimitiveUnsigned>()
where
    Natural: From<T>,
{
    assert_panic!(Float::from_unsigned_prec(T::ZERO, 0));
    assert_panic!(Float::from_unsigned_prec(T::ONE, 0));
}

fn from_signed_prec_fail_helper<T: PrimitiveSigned>()
where
    Integer: From<T>,
{
    assert_panic!(Float::from_signed_prec(T::ZERO, 0));
    assert_panic!(Float::from_signed_prec(T::ONE, 0));
    assert_panic!(Float::from_signed_prec(T::NEGATIVE_ONE, 0));
}

#[test]
fn from_primitive_int_prec_fail() {
    apply_fn_to_unsigneds!(from_unsigned_prec_fail_helper);
    apply_fn_to_signeds!(from_signed_prec_fail_helper);
}

#[test]
fn test_from_primitive_int_prec_round() {
    fn test_helper_u<T: PrimitiveUnsigned>(
        u: T,
        prec: u64,
        rm: RoundingMode,
        out: &str,
        out_hex: &str,
        out_o: Ordering,
    ) where
        Natural: From<T>,
        rug::Float: AssignRound<T, Round = Round, Ordering = Ordering>,
    {
        let (x, o) = Float::from_unsigned_prec_round(u, prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_x, rug_o) = rug::Float::with_val_round(u32::exact_from(prec), u, rm);
            let x = Float::exact_from(&rug_x);
            assert_eq!(x.to_string(), out);
            assert_eq!(to_hex_string(&x), out_hex);
            assert_eq!(rug_o, out_o);
        }
    }
    fn test_helper_u2<T: PrimitiveUnsigned>()
    where
        Natural: From<T>,
        rug::Float: AssignRound<T, Round = Round, Ordering = Ordering>,
    {
        test_helper_u(T::ZERO, 1, Floor, "0.0", "0x0.0", Equal);
        test_helper_u(T::ZERO, 1, Ceiling, "0.0", "0x0.0", Equal);
        test_helper_u(T::ZERO, 1, Down, "0.0", "0x0.0", Equal);
        test_helper_u(T::ZERO, 1, Up, "0.0", "0x0.0", Equal);
        test_helper_u(T::ZERO, 1, Nearest, "0.0", "0x0.0", Equal);
        test_helper_u(T::ZERO, 1, Exact, "0.0", "0x0.0", Equal);

        test_helper_u(T::ZERO, 10, Floor, "0.0", "0x0.0", Equal);
        test_helper_u(T::ZERO, 10, Ceiling, "0.0", "0x0.0", Equal);
        test_helper_u(T::ZERO, 10, Down, "0.0", "0x0.0", Equal);
        test_helper_u(T::ZERO, 10, Up, "0.0", "0x0.0", Equal);
        test_helper_u(T::ZERO, 10, Nearest, "0.0", "0x0.0", Equal);
        test_helper_u(T::ZERO, 10, Exact, "0.0", "0x0.0", Equal);

        test_helper_u(T::ZERO, 20, Floor, "0.0", "0x0.0", Equal);
        test_helper_u(T::ZERO, 20, Ceiling, "0.0", "0x0.0", Equal);
        test_helper_u(T::ZERO, 20, Down, "0.0", "0x0.0", Equal);
        test_helper_u(T::ZERO, 20, Up, "0.0", "0x0.0", Equal);
        test_helper_u(T::ZERO, 20, Nearest, "0.0", "0x0.0", Equal);
        test_helper_u(T::ZERO, 20, Exact, "0.0", "0x0.0", Equal);

        test_helper_u(T::ONE, 1, Floor, "1.0", "0x1.0#1", Equal);
        test_helper_u(T::ONE, 1, Ceiling, "1.0", "0x1.0#1", Equal);
        test_helper_u(T::ONE, 1, Down, "1.0", "0x1.0#1", Equal);
        test_helper_u(T::ONE, 1, Up, "1.0", "0x1.0#1", Equal);
        test_helper_u(T::ONE, 1, Nearest, "1.0", "0x1.0#1", Equal);
        test_helper_u(T::ONE, 1, Exact, "1.0", "0x1.0#1", Equal);

        test_helper_u(T::ONE, 10, Floor, "1.0", "0x1.000#10", Equal);
        test_helper_u(T::ONE, 10, Ceiling, "1.0", "0x1.000#10", Equal);
        test_helper_u(T::ONE, 10, Down, "1.0", "0x1.000#10", Equal);
        test_helper_u(T::ONE, 10, Up, "1.0", "0x1.000#10", Equal);
        test_helper_u(T::ONE, 10, Nearest, "1.0", "0x1.000#10", Equal);
        test_helper_u(T::ONE, 10, Exact, "1.0", "0x1.000#10", Equal);

        test_helper_u(T::ONE, 20, Floor, "1.0", "0x1.00000#20", Equal);
        test_helper_u(T::ONE, 20, Ceiling, "1.0", "0x1.00000#20", Equal);
        test_helper_u(T::ONE, 20, Down, "1.0", "0x1.00000#20", Equal);
        test_helper_u(T::ONE, 20, Up, "1.0", "0x1.00000#20", Equal);
        test_helper_u(T::ONE, 20, Nearest, "1.0", "0x1.00000#20", Equal);
        test_helper_u(T::ONE, 20, Exact, "1.0", "0x1.00000#20", Equal);

        test_helper_u(T::from(123u8), 1, Floor, "6.0e1", "0x4.0E+1#1", Less);
        test_helper_u(T::from(123u8), 1, Ceiling, "1.0e2", "0x8.0E+1#1", Greater);
        test_helper_u(T::from(123u8), 1, Down, "6.0e1", "0x4.0E+1#1", Less);
        test_helper_u(T::from(123u8), 1, Up, "1.0e2", "0x8.0E+1#1", Greater);
        test_helper_u(T::from(123u8), 1, Nearest, "1.0e2", "0x8.0E+1#1", Greater);

        test_helper_u(T::from(123u8), 10, Floor, "123.0", "0x7b.0#10", Equal);
        test_helper_u(T::from(123u8), 10, Ceiling, "123.0", "0x7b.0#10", Equal);
        test_helper_u(T::from(123u8), 10, Down, "123.0", "0x7b.0#10", Equal);
        test_helper_u(T::from(123u8), 10, Up, "123.0", "0x7b.0#10", Equal);
        test_helper_u(T::from(123u8), 10, Nearest, "123.0", "0x7b.0#10", Equal);
        test_helper_u(T::from(123u8), 10, Exact, "123.0", "0x7b.0#10", Equal);

        test_helper_u(T::from(123u8), 20, Floor, "123.0", "0x7b.0000#20", Equal);
        test_helper_u(T::from(123u8), 20, Ceiling, "123.0", "0x7b.0000#20", Equal);
        test_helper_u(T::from(123u8), 20, Down, "123.0", "0x7b.0000#20", Equal);
        test_helper_u(T::from(123u8), 20, Up, "123.0", "0x7b.0000#20", Equal);
        test_helper_u(T::from(123u8), 20, Nearest, "123.0", "0x7b.0000#20", Equal);
        test_helper_u(T::from(123u8), 20, Exact, "123.0", "0x7b.0000#20", Equal);
    }
    apply_fn_to_unsigneds!(test_helper_u2);
    test_helper_u(1000000000000u64, 1, Floor, "5.0e11", "0x8.0E+9#1", Less);
    test_helper_u(
        1000000000000u64,
        1,
        Ceiling,
        "1.0e12",
        "0x1.0E+10#1",
        Greater,
    );
    test_helper_u(1000000000000u64, 1, Down, "5.0e11", "0x8.0E+9#1", Less);
    test_helper_u(1000000000000u64, 1, Up, "1.0e12", "0x1.0E+10#1", Greater);
    test_helper_u(
        1000000000000u64,
        1,
        Nearest,
        "1.0e12",
        "0x1.0E+10#1",
        Greater,
    );

    test_helper_u(
        1000000000000u64,
        10,
        Floor,
        "9.997e11",
        "0xe.8cE+9#10",
        Less,
    );
    test_helper_u(
        1000000000000u64,
        10,
        Ceiling,
        "1.001e12",
        "0xe.90E+9#10",
        Greater,
    );
    test_helper_u(1000000000000u64, 10, Down, "9.997e11", "0xe.8cE+9#10", Less);
    test_helper_u(
        1000000000000u64,
        10,
        Up,
        "1.001e12",
        "0xe.90E+9#10",
        Greater,
    );
    test_helper_u(
        1000000000000u64,
        10,
        Nearest,
        "9.997e11",
        "0xe.8cE+9#10",
        Less,
    );

    test_helper_u(
        1000000000000u64,
        20,
        Floor,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Less,
    );
    test_helper_u(
        1000000000000u64,
        20,
        Ceiling,
        "1.000001e12",
        "0xe.8d4bE+9#20",
        Greater,
    );
    test_helper_u(
        1000000000000u64,
        20,
        Down,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Less,
    );
    test_helper_u(
        1000000000000u64,
        20,
        Up,
        "1.000001e12",
        "0xe.8d4bE+9#20",
        Greater,
    );
    test_helper_u(
        1000000000000u64,
        20,
        Nearest,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Less,
    );

    fn test_helper_i<T: PrimitiveSigned>(
        u: T,
        prec: u64,
        rm: RoundingMode,
        out: &str,
        out_hex: &str,
        out_o: Ordering,
    ) where
        Integer: From<T>,
        rug::Float: AssignRound<T, Round = Round, Ordering = Ordering>,
    {
        let (x, o) = Float::from_signed_prec_round(u, prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_x, rug_o) = rug::Float::with_val_round(u32::exact_from(prec), u, rm);
            let x = Float::exact_from(&rug_x);
            assert_eq!(x.to_string(), out);
            assert_eq!(to_hex_string(&x), out_hex);
            assert_eq!(rug_o, out_o);
        }
    }
    fn test_helper_i2<T: PrimitiveSigned>()
    where
        Integer: From<T>,
        rug::Float: AssignRound<T, Round = Round, Ordering = Ordering>,
    {
        test_helper_i(T::ZERO, 1, Floor, "0.0", "0x0.0", Equal);
        test_helper_i(T::ZERO, 1, Ceiling, "0.0", "0x0.0", Equal);
        test_helper_i(T::ZERO, 1, Down, "0.0", "0x0.0", Equal);
        test_helper_i(T::ZERO, 1, Up, "0.0", "0x0.0", Equal);
        test_helper_i(T::ZERO, 1, Nearest, "0.0", "0x0.0", Equal);
        test_helper_i(T::ZERO, 1, Exact, "0.0", "0x0.0", Equal);

        test_helper_i(T::ZERO, 10, Floor, "0.0", "0x0.0", Equal);
        test_helper_i(T::ZERO, 10, Ceiling, "0.0", "0x0.0", Equal);
        test_helper_i(T::ZERO, 10, Down, "0.0", "0x0.0", Equal);
        test_helper_i(T::ZERO, 10, Up, "0.0", "0x0.0", Equal);
        test_helper_i(T::ZERO, 10, Nearest, "0.0", "0x0.0", Equal);
        test_helper_i(T::ZERO, 10, Exact, "0.0", "0x0.0", Equal);

        test_helper_i(T::ZERO, 20, Floor, "0.0", "0x0.0", Equal);
        test_helper_i(T::ZERO, 20, Ceiling, "0.0", "0x0.0", Equal);
        test_helper_i(T::ZERO, 20, Down, "0.0", "0x0.0", Equal);
        test_helper_i(T::ZERO, 20, Up, "0.0", "0x0.0", Equal);
        test_helper_i(T::ZERO, 20, Nearest, "0.0", "0x0.0", Equal);
        test_helper_i(T::ZERO, 20, Exact, "0.0", "0x0.0", Equal);

        test_helper_i(T::ONE, 1, Floor, "1.0", "0x1.0#1", Equal);
        test_helper_i(T::ONE, 1, Ceiling, "1.0", "0x1.0#1", Equal);
        test_helper_i(T::ONE, 1, Down, "1.0", "0x1.0#1", Equal);
        test_helper_i(T::ONE, 1, Up, "1.0", "0x1.0#1", Equal);
        test_helper_i(T::ONE, 1, Nearest, "1.0", "0x1.0#1", Equal);
        test_helper_i(T::ONE, 1, Exact, "1.0", "0x1.0#1", Equal);

        test_helper_i(T::ONE, 10, Floor, "1.0", "0x1.000#10", Equal);
        test_helper_i(T::ONE, 10, Ceiling, "1.0", "0x1.000#10", Equal);
        test_helper_i(T::ONE, 10, Down, "1.0", "0x1.000#10", Equal);
        test_helper_i(T::ONE, 10, Up, "1.0", "0x1.000#10", Equal);
        test_helper_i(T::ONE, 10, Nearest, "1.0", "0x1.000#10", Equal);
        test_helper_i(T::ONE, 10, Exact, "1.0", "0x1.000#10", Equal);

        test_helper_i(T::ONE, 20, Floor, "1.0", "0x1.00000#20", Equal);
        test_helper_i(T::ONE, 20, Ceiling, "1.0", "0x1.00000#20", Equal);
        test_helper_i(T::ONE, 20, Down, "1.0", "0x1.00000#20", Equal);
        test_helper_i(T::ONE, 20, Up, "1.0", "0x1.00000#20", Equal);
        test_helper_i(T::ONE, 20, Nearest, "1.0", "0x1.00000#20", Equal);
        test_helper_i(T::ONE, 20, Exact, "1.0", "0x1.00000#20", Equal);

        test_helper_i(T::from(123i8), 1, Floor, "6.0e1", "0x4.0E+1#1", Less);
        test_helper_i(T::from(123i8), 1, Ceiling, "1.0e2", "0x8.0E+1#1", Greater);
        test_helper_i(T::from(123i8), 1, Down, "6.0e1", "0x4.0E+1#1", Less);
        test_helper_i(T::from(123i8), 1, Up, "1.0e2", "0x8.0E+1#1", Greater);
        test_helper_i(T::from(123i8), 1, Nearest, "1.0e2", "0x8.0E+1#1", Greater);

        test_helper_i(T::from(123i8), 10, Floor, "123.0", "0x7b.0#10", Equal);
        test_helper_i(T::from(123i8), 10, Ceiling, "123.0", "0x7b.0#10", Equal);
        test_helper_i(T::from(123i8), 10, Down, "123.0", "0x7b.0#10", Equal);
        test_helper_i(T::from(123i8), 10, Up, "123.0", "0x7b.0#10", Equal);
        test_helper_i(T::from(123i8), 10, Nearest, "123.0", "0x7b.0#10", Equal);
        test_helper_i(T::from(123i8), 10, Exact, "123.0", "0x7b.0#10", Equal);

        test_helper_i(T::from(123i8), 20, Floor, "123.0", "0x7b.0000#20", Equal);
        test_helper_i(T::from(123i8), 20, Ceiling, "123.0", "0x7b.0000#20", Equal);
        test_helper_i(T::from(123i8), 20, Down, "123.0", "0x7b.0000#20", Equal);
        test_helper_i(T::from(123i8), 20, Up, "123.0", "0x7b.0000#20", Equal);
        test_helper_i(T::from(123i8), 20, Nearest, "123.0", "0x7b.0000#20", Equal);
        test_helper_i(T::from(123i8), 20, Exact, "123.0", "0x7b.0000#20", Equal);
        test_helper_i(T::NEGATIVE_ONE, 1, Floor, "-1.0", "-0x1.0#1", Equal);
        test_helper_i(T::NEGATIVE_ONE, 1, Ceiling, "-1.0", "-0x1.0#1", Equal);
        test_helper_i(T::NEGATIVE_ONE, 1, Down, "-1.0", "-0x1.0#1", Equal);
        test_helper_i(T::NEGATIVE_ONE, 1, Up, "-1.0", "-0x1.0#1", Equal);
        test_helper_i(T::NEGATIVE_ONE, 1, Nearest, "-1.0", "-0x1.0#1", Equal);
        test_helper_i(T::NEGATIVE_ONE, 1, Exact, "-1.0", "-0x1.0#1", Equal);

        test_helper_i(T::NEGATIVE_ONE, 10, Floor, "-1.0", "-0x1.000#10", Equal);
        test_helper_i(T::NEGATIVE_ONE, 10, Ceiling, "-1.0", "-0x1.000#10", Equal);
        test_helper_i(T::NEGATIVE_ONE, 10, Down, "-1.0", "-0x1.000#10", Equal);
        test_helper_i(T::NEGATIVE_ONE, 10, Up, "-1.0", "-0x1.000#10", Equal);
        test_helper_i(T::NEGATIVE_ONE, 10, Nearest, "-1.0", "-0x1.000#10", Equal);
        test_helper_i(T::NEGATIVE_ONE, 10, Exact, "-1.0", "-0x1.000#10", Equal);

        test_helper_i(T::NEGATIVE_ONE, 20, Floor, "-1.0", "-0x1.00000#20", Equal);
        test_helper_i(T::NEGATIVE_ONE, 20, Ceiling, "-1.0", "-0x1.00000#20", Equal);
        test_helper_i(T::NEGATIVE_ONE, 20, Down, "-1.0", "-0x1.00000#20", Equal);
        test_helper_i(T::NEGATIVE_ONE, 20, Up, "-1.0", "-0x1.00000#20", Equal);
        test_helper_i(T::NEGATIVE_ONE, 20, Nearest, "-1.0", "-0x1.00000#20", Equal);
        test_helper_i(T::NEGATIVE_ONE, 20, Exact, "-1.0", "-0x1.00000#20", Equal);

        test_helper_i(T::from(-123i8), 1, Floor, "-1.0e2", "-0x8.0E+1#1", Less);
        test_helper_i(
            T::from(-123i8),
            1,
            Ceiling,
            "-6.0e1",
            "-0x4.0E+1#1",
            Greater,
        );
        test_helper_i(T::from(-123i8), 1, Down, "-6.0e1", "-0x4.0E+1#1", Greater);
        test_helper_i(T::from(-123i8), 1, Up, "-1.0e2", "-0x8.0E+1#1", Less);
        test_helper_i(T::from(-123i8), 1, Nearest, "-1.0e2", "-0x8.0E+1#1", Less);

        test_helper_i(T::from(-123i8), 10, Floor, "-123.0", "-0x7b.0#10", Equal);
        test_helper_i(T::from(-123i8), 10, Ceiling, "-123.0", "-0x7b.0#10", Equal);
        test_helper_i(T::from(-123i8), 10, Down, "-123.0", "-0x7b.0#10", Equal);
        test_helper_i(T::from(-123i8), 10, Up, "-123.0", "-0x7b.0#10", Equal);
        test_helper_i(T::from(-123i8), 10, Nearest, "-123.0", "-0x7b.0#10", Equal);
        test_helper_i(T::from(-123i8), 10, Exact, "-123.0", "-0x7b.0#10", Equal);

        test_helper_i(T::from(-123i8), 20, Floor, "-123.0", "-0x7b.0000#20", Equal);
        test_helper_i(
            T::from(-123i8),
            20,
            Ceiling,
            "-123.0",
            "-0x7b.0000#20",
            Equal,
        );
        test_helper_i(T::from(-123i8), 20, Down, "-123.0", "-0x7b.0000#20", Equal);
        test_helper_i(T::from(-123i8), 20, Up, "-123.0", "-0x7b.0000#20", Equal);
        test_helper_i(
            T::from(-123i8),
            20,
            Nearest,
            "-123.0",
            "-0x7b.0000#20",
            Equal,
        );
        test_helper_i(T::from(-123i8), 20, Exact, "-123.0", "-0x7b.0000#20", Equal);
    }
    apply_fn_to_signeds!(test_helper_i2);
    test_helper_i(1000000000000i64, 1, Floor, "5.0e11", "0x8.0E+9#1", Less);
    test_helper_i(
        1000000000000i64,
        1,
        Ceiling,
        "1.0e12",
        "0x1.0E+10#1",
        Greater,
    );
    test_helper_i(1000000000000i64, 1, Down, "5.0e11", "0x8.0E+9#1", Less);
    test_helper_i(1000000000000i64, 1, Up, "1.0e12", "0x1.0E+10#1", Greater);
    test_helper_i(
        1000000000000i64,
        1,
        Nearest,
        "1.0e12",
        "0x1.0E+10#1",
        Greater,
    );

    test_helper_i(
        1000000000000i64,
        10,
        Floor,
        "9.997e11",
        "0xe.8cE+9#10",
        Less,
    );
    test_helper_i(
        1000000000000i64,
        10,
        Ceiling,
        "1.001e12",
        "0xe.90E+9#10",
        Greater,
    );
    test_helper_i(1000000000000i64, 10, Down, "9.997e11", "0xe.8cE+9#10", Less);
    test_helper_i(
        1000000000000i64,
        10,
        Up,
        "1.001e12",
        "0xe.90E+9#10",
        Greater,
    );
    test_helper_i(
        1000000000000i64,
        10,
        Nearest,
        "9.997e11",
        "0xe.8cE+9#10",
        Less,
    );

    test_helper_i(
        1000000000000i64,
        20,
        Floor,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Less,
    );
    test_helper_i(
        1000000000000i64,
        20,
        Ceiling,
        "1.000001e12",
        "0xe.8d4bE+9#20",
        Greater,
    );
    test_helper_i(
        1000000000000i64,
        20,
        Down,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Less,
    );
    test_helper_i(
        1000000000000i64,
        20,
        Up,
        "1.000001e12",
        "0xe.8d4bE+9#20",
        Greater,
    );
    test_helper_i(
        1000000000000i64,
        20,
        Nearest,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Less,
    );
    test_helper_i(-1000000000000i64, 1, Floor, "-1.0e12", "-0x1.0E+10#1", Less);
    test_helper_i(
        -1000000000000i64,
        1,
        Ceiling,
        "-5.0e11",
        "-0x8.0E+9#1",
        Greater,
    );
    test_helper_i(
        -1000000000000i64,
        1,
        Down,
        "-5.0e11",
        "-0x8.0E+9#1",
        Greater,
    );
    test_helper_i(-1000000000000i64, 1, Up, "-1.0e12", "-0x1.0E+10#1", Less);
    test_helper_i(
        -1000000000000i64,
        1,
        Nearest,
        "-1.0e12",
        "-0x1.0E+10#1",
        Less,
    );

    test_helper_i(
        -1000000000000i64,
        10,
        Floor,
        "-1.001e12",
        "-0xe.90E+9#10",
        Less,
    );
    test_helper_i(
        -1000000000000i64,
        10,
        Ceiling,
        "-9.997e11",
        "-0xe.8cE+9#10",
        Greater,
    );
    test_helper_i(
        -1000000000000i64,
        10,
        Down,
        "-9.997e11",
        "-0xe.8cE+9#10",
        Greater,
    );
    test_helper_i(
        -1000000000000i64,
        10,
        Up,
        "-1.001e12",
        "-0xe.90E+9#10",
        Less,
    );
    test_helper_i(
        -1000000000000i64,
        10,
        Nearest,
        "-9.997e11",
        "-0xe.8cE+9#10",
        Greater,
    );

    test_helper_i(
        -1000000000000i64,
        20,
        Floor,
        "-1.000001e12",
        "-0xe.8d4bE+9#20",
        Less,
    );
    test_helper_i(
        -1000000000000i64,
        20,
        Ceiling,
        "-9.999997e11",
        "-0xe.8d4aE+9#20",
        Greater,
    );
    test_helper_i(
        -1000000000000i64,
        20,
        Down,
        "-9.999997e11",
        "-0xe.8d4aE+9#20",
        Greater,
    );
    test_helper_i(
        -1000000000000i64,
        20,
        Up,
        "-1.000001e12",
        "-0xe.8d4bE+9#20",
        Less,
    );
    test_helper_i(
        -1000000000000i64,
        20,
        Nearest,
        "-9.999997e11",
        "-0xe.8d4aE+9#20",
        Greater,
    );
}

fn from_unsigned_prec_round_fail_helper<T: PrimitiveUnsigned>()
where
    Natural: From<T>,
{
    assert_panic!(Float::from_unsigned_prec_round(T::ZERO, 0, Floor));
    assert_panic!(Float::from_unsigned_prec_round(T::ONE, 0, Floor));
    assert_panic!(Float::from_unsigned_prec_round(T::from(123u8), 1, Exact));
}

fn from_signed_prec_round_fail_helper<T: PrimitiveSigned>()
where
    Integer: From<T>,
{
    assert_panic!(Float::from_signed_prec_round(T::ZERO, 0, Floor));
    assert_panic!(Float::from_signed_prec_round(T::ONE, 0, Floor));
    assert_panic!(Float::from_signed_prec_round(T::from(123i8), 1, Exact));
    assert_panic!(Float::from_signed_prec_round(T::NEGATIVE_ONE, 0, Floor));
    assert_panic!(Float::from_signed_prec_round(T::from(-123i8), 1, Exact));
}

#[test]
fn from_primitive_int_prec_round_fail() {
    apply_fn_to_unsigneds!(from_unsigned_prec_round_fail_helper);
    apply_fn_to_signeds!(from_signed_prec_round_fail_helper);
}

#[test]
fn test_const_from_unsigned_times_power_of_2() {
    fn test_helper(u: Limb, pow: i32, out: &str, out_hex: &str) {
        let x = Float::const_from_unsigned_times_power_of_2(u, pow);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
    }
    test_helper(0, 0, "0.0", "0x0.0");
    test_helper(0, 10, "0.0", "0x0.0");
    test_helper(0, -10, "0.0", "0x0.0");
    test_helper(1, 0, "1.0", "0x1.0#1");
    test_helper(1, 10, "1.0e3", "0x4.0E+2#1");
    test_helper(1, -10, "0.001", "0x0.004#1");
    test_helper(1, 1073741822, "too_big", "0x4.0E+268435455#1");
    test_helper(1, -1073741824, "too_small", "0x1.0E-268435456#1");
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test_helper(
            884279719003555,
            -48,
            "3.141592653589793",
            "0x3.243f6a8885a3#50",
        );
    }
}

#[test]
fn const_from_unsigned_times_power_of_2_fail() {
    assert_panic!(Float::const_from_unsigned_times_power_of_2(1, 1073741823));
    assert_panic!(Float::const_from_unsigned_times_power_of_2(1, -1073741825));
}

#[test]
fn test_const_from_signed_times_power_of_2() {
    fn test_helper(u: SignedLimb, pow: i32, out: &str, out_hex: &str) {
        let x = Float::const_from_signed_times_power_of_2(u, pow);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
    }
    test_helper(0, 0, "0.0", "0x0.0");
    test_helper(0, 10, "0.0", "0x0.0");
    test_helper(0, -10, "0.0", "0x0.0");
    test_helper(1, 0, "1.0", "0x1.0#1");
    test_helper(1, 10, "1.0e3", "0x4.0E+2#1");
    test_helper(1, -10, "0.001", "0x0.004#1");
    test_helper(-1, 0, "-1.0", "-0x1.0#1");
    test_helper(-1, 10, "-1.0e3", "-0x4.0E+2#1");
    test_helper(-1, -10, "-0.001", "-0x0.004#1");
    test_helper(1, 1073741822, "too_big", "0x4.0E+268435455#1");
    test_helper(1, -1073741824, "too_small", "0x1.0E-268435456#1");
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test_helper(
            884279719003555,
            -48,
            "3.141592653589793",
            "0x3.243f6a8885a3#50",
        );
        test_helper(
            -884279719003555,
            -48,
            "-3.141592653589793",
            "-0x3.243f6a8885a3#50",
        );
    }
}

#[test]
fn const_from_signed_times_power_of_2_fail() {
    assert_panic!(Float::const_from_signed_times_power_of_2(1, 1073741823));
    assert_panic!(Float::const_from_signed_times_power_of_2(1, -1073741825));
}

#[allow(clippy::type_repetition_in_bounds)]
fn from_primitive_int_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    Float: From<T>,
    rug::Float: Assign<T>,
    Natural: From<T> + PartialEq<T>,
    for<'a> T: ExactFrom<&'a Float>,
    Limb: WrappingFrom<T>,
{
    unsigned_gen::<T>().test_properties(|n| {
        let float_n = Float::from(n);
        assert!(float_n.is_valid());

        if T::WIDTH == Limb::WIDTH {
            let n_alt = Float::const_from_unsigned(Limb::wrapping_from(n));
            assert!(n_alt.is_valid());
            assert_eq!(ComparableFloatRef(&n_alt), ComparableFloatRef(&float_n));

            let n_alt = Float::const_from_unsigned_times_power_of_2(Limb::wrapping_from(n), 0);
            assert!(n_alt.is_valid());
            assert_eq!(ComparableFloatRef(&n_alt), ComparableFloatRef(&float_n));
        }

        let expected_prec = if n == T::ZERO {
            None
        } else {
            Some(n.significant_bits() - TrailingZeros::trailing_zeros(n))
        };
        let rug_n = rug::Float::with_val(expected_prec.map_or(1, u32::exact_from), n);
        assert_eq!(
            ComparableFloatRef(&float_n),
            ComparableFloatRef(&From::<&rug::Float>::from(&rug_n))
        );

        let n_alt: Float = ExactFrom::exact_from(Natural::exact_from(n));
        assert_eq!(ComparableFloatRef(&n_alt), ComparableFloatRef(&float_n));

        assert_eq!(float_n.get_prec(), expected_prec);
        assert_eq!(T::exact_from(&float_n), n);
        let (f, o) = Float::from_unsigned_prec(n, expected_prec.unwrap_or(1));
        assert_eq!(ComparableFloat(f), ComparableFloat(float_n));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::type_repetition_in_bounds)]
fn from_primitive_int_properties_helper_signed<T: PrimitiveSigned>()
where
    Float: From<T>,
    rug::Float: Assign<T>,
    Integer: From<T> + PartialEq<T>,
    for<'a> T: ExactFrom<&'a Float>,
    SignedLimb: WrappingFrom<T>,
{
    signed_gen::<T>().test_properties(|n| {
        let float_n = Float::from(n);
        assert!(float_n.is_valid());

        if T::WIDTH == SignedLimb::WIDTH {
            let n_alt = Float::const_from_signed(SignedLimb::wrapping_from(n));
            assert!(n_alt.is_valid());
            assert_eq!(ComparableFloatRef(&n_alt), ComparableFloatRef(&float_n));

            let n_alt = Float::const_from_signed_times_power_of_2(SignedLimb::wrapping_from(n), 0);
            assert!(n_alt.is_valid());
            assert_eq!(ComparableFloatRef(&n_alt), ComparableFloatRef(&float_n));
        }

        let expected_prec = if n == T::ZERO {
            None
        } else {
            Some(n.significant_bits() - TrailingZeros::trailing_zeros(n))
        };
        let rug_n = rug::Float::with_val(expected_prec.map_or(1, u32::exact_from), n);
        assert_eq!(
            ComparableFloatRef(&float_n),
            ComparableFloatRef(&From::<&rug::Float>::from(&rug_n))
        );

        let n_alt: Float = ExactFrom::exact_from(Integer::from(n));
        assert_eq!(ComparableFloatRef(&n_alt), ComparableFloatRef(&float_n));

        assert_eq!(float_n.get_prec(), expected_prec);
        assert_eq!(T::exact_from(&float_n), n);
        let (f, o) = Float::from_signed_prec(n, expected_prec.unwrap_or(1));
        assert_eq!(ComparableFloat(f), ComparableFloat(float_n));
        assert_eq!(o, Equal);
    });
}

#[test]
fn from_primitive_int_properties() {
    apply_fn_to_unsigneds!(from_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(from_primitive_int_properties_helper_signed);
}

fn from_primitive_int_prec_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    Natural: From<T>,
    Float: PartialOrd<T>,
    rug::Float: Assign<T>,
{
    unsigned_pair_gen_var_32::<T, u64>().test_properties(|(n, prec)| {
        let (float_n, o) = Float::from_unsigned_prec(n, prec);
        assert!(float_n.is_valid());
        assert_eq!(float_n.partial_cmp(&n), Some(o));

        let rug_n = rug::Float::with_val(u32::exact_from(prec), n);
        assert_eq!(
            ComparableFloatRef(&float_n),
            ComparableFloatRef(&Float::from(&rug_n))
        );

        let (float_n_alt, o_alt) = Float::from_natural_prec(Natural::from(n), prec);
        assert_eq!(
            ComparableFloatRef(&float_n_alt),
            ComparableFloatRef(&float_n)
        );
        assert_eq!(o_alt, o);

        assert_eq!(
            float_n.get_prec(),
            if n == T::ZERO { None } else { Some(prec) }
        );
    });
}

fn from_primitive_int_prec_properties_helper_signed<T: PrimitiveSigned>()
where
    Integer: From<T>,
    Float: PartialOrd<T>,
    rug::Float: Assign<T>,
{
    signed_unsigned_pair_gen_var_20::<T, u64>().test_properties(|(n, prec)| {
        let (float_n, o) = Float::from_signed_prec(n, prec);
        assert!(float_n.is_valid());

        assert_eq!(float_n.partial_cmp(&n), Some(o));

        let rug_n = rug::Float::with_val(u32::exact_from(prec), n);
        assert_eq!(
            ComparableFloatRef(&float_n),
            ComparableFloatRef(&Float::from(&rug_n))
        );

        let (float_n_alt, o_alt) = Float::from_integer_prec(Integer::from(n), prec);
        assert_eq!(
            ComparableFloatRef(&float_n_alt),
            ComparableFloatRef(&float_n)
        );
        assert_eq!(o_alt, o);

        assert_eq!(
            float_n.get_prec(),
            if n == T::ZERO { None } else { Some(prec) }
        );
    });
}

#[test]
fn from_primitive_int_prec_properties() {
    apply_fn_to_unsigneds!(from_primitive_int_prec_properties_helper_unsigned);
    apply_fn_to_signeds!(from_primitive_int_prec_properties_helper_signed);
}

fn from_primitive_int_prec_round_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    Natural: From<T>,
    Float: PartialOrd<T>,
    Rational: From<T> + PartialOrd<T>,
    rug::Float: AssignRound<T, Round = Round, Ordering = Ordering>,
{
    unsigned_unsigned_rounding_mode_triple_gen_var_5::<T>().test_properties(|(n, prec, rm)| {
        let (float_n, o) = Float::from_unsigned_prec_round(n, prec, rm);
        assert!(float_n.is_valid());

        assert_eq!(float_n.partial_cmp(&n), Some(o));
        match rm {
            Floor | Down => {
                assert_ne!(o, Greater);
            }
            Ceiling | Up => {
                assert_ne!(o, Less);
            }
            Exact => assert_eq!(o, Equal),
            _ => {}
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_n, rug_o) = rug::Float::with_val_round(u32::exact_from(prec), n, rm);
            assert_eq!(
                ComparableFloatRef(&float_n),
                ComparableFloatRef(&Float::from(&rug_n))
            );
            assert_eq!(rug_o, o);
        }

        let (float_n_alt, o_alt) = Float::from_natural_prec_round(Natural::from(n), prec, rm);
        assert_eq!(
            ComparableFloatRef(&float_n_alt),
            ComparableFloatRef(&float_n)
        );
        assert_eq!(o_alt, o);

        assert_eq!(
            float_n.get_prec(),
            if n == T::ZERO { None } else { Some(prec) }
        );
    });

    unsigned_pair_gen_var_32::<T, u64>().test_properties(|(n, prec)| {
        let floor = Float::from_unsigned_prec_round(n, prec, Floor);
        let r_floor = Rational::exact_from(&floor.0);
        assert!(r_floor <= n);
        if r_floor != T::ZERO {
            assert!(r_floor + Rational::exact_from(floor.0.ulp().unwrap()) > n);
        }
        let (floor_alt, floor_o_alt) = Float::from_unsigned_prec_round(n, prec, Down);
        assert_eq!(ComparableFloatRef(&floor_alt), ComparableFloatRef(&floor.0));
        assert_eq!(floor_o_alt, floor.1);

        let ceiling = Float::from_unsigned_prec_round(n, prec, Ceiling);
        let r_ceiling = Rational::exact_from(&ceiling.0);
        assert!(r_ceiling >= n);
        if r_ceiling != T::ZERO {
            assert!(r_ceiling - Rational::exact_from(ceiling.0.ulp().unwrap()) < n);
        }
        let (ceiling_alt, ceiling_o_alt) = Float::from_unsigned_prec_round(n, prec, Up);
        assert_eq!(
            ComparableFloatRef(&ceiling_alt),
            ComparableFloatRef(&ceiling.0)
        );
        assert_eq!(ceiling_o_alt, ceiling.1);

        let nearest = Float::from_unsigned_prec_round(n, prec, Nearest);
        assert!(
            ComparableFloatRef(&nearest.0) == ComparableFloatRef(&floor.0) && nearest.1 == floor.1
                || ComparableFloatRef(&nearest.0) == ComparableFloatRef(&ceiling.0)
                    && nearest.1 == ceiling.1
        );
        let r_nearest = Rational::exact_from(&nearest.0);
        if r_nearest != T::ZERO {
            assert!((r_nearest - Rational::from(n))
                .le_abs(&(Rational::exact_from(nearest.0.ulp().unwrap()) >> 1)));
        }
    });
}

fn from_primitive_int_prec_round_properties_helper_signed<T: PrimitiveSigned>()
where
    Integer: From<T>,
    Float: PartialOrd<T>,
    Rational: From<T> + PartialOrd<T>,
    rug::Float: AssignRound<T, Round = Round, Ordering = Ordering>,
{
    signed_unsigned_rounding_mode_triple_gen_var_3::<T>().test_properties(|(n, prec, rm)| {
        let (float_n, o) = Float::from_signed_prec_round(n, prec, rm);
        assert!(float_n.is_valid());

        assert_eq!(float_n.partial_cmp(&n), Some(o));
        match (n >= T::ZERO, rm) {
            (_, Floor) | (true, Down) | (false, Up) => {
                assert_ne!(o, Greater);
            }
            (_, Ceiling) | (true, Up) | (false, Down) => {
                assert_ne!(o, Less);
            }
            (_, Exact) => assert_eq!(o, Equal),
            _ => {}
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_n, rug_o) = rug::Float::with_val_round(u32::exact_from(prec), n, rm);
            assert_eq!(
                ComparableFloatRef(&float_n),
                ComparableFloatRef(&Float::from(&rug_n))
            );
            assert_eq!(rug_o, o);
        }

        let (float_n_alt, o_alt) = Float::from_integer_prec_round(Integer::from(n), prec, rm);
        assert_eq!(
            ComparableFloatRef(&float_n_alt),
            ComparableFloatRef(&float_n)
        );
        assert_eq!(o_alt, o);

        assert_eq!(
            float_n.get_prec(),
            if n == T::ZERO { None } else { Some(prec) }
        );
    });

    signed_unsigned_pair_gen_var_20::<T, u64>().test_properties(|(n, prec)| {
        let floor = Float::from_signed_prec_round(n, prec, Floor);
        let r_floor = Rational::exact_from(&floor.0);
        assert!(r_floor <= n);
        if r_floor != T::ZERO {
            assert!(r_floor + Rational::exact_from(floor.0.ulp().unwrap()) > n);
        }
        let (floor_n_alt, o_alt) =
            Float::from_signed_prec_round(n, prec, if n >= T::ZERO { Down } else { Up });
        assert_eq!(
            ComparableFloatRef(&floor_n_alt),
            ComparableFloatRef(&floor.0)
        );
        assert_eq!(o_alt, floor.1);

        let ceiling = Float::from_signed_prec_round(n, prec, Ceiling);
        let r_ceiling = Rational::exact_from(&ceiling.0);
        assert!(r_ceiling >= n);
        if r_ceiling != T::ZERO {
            assert!(r_ceiling - Rational::exact_from(ceiling.0.ulp().unwrap()) < n);
        }
        let (ceiling_n_alt, o_alt) =
            Float::from_signed_prec_round(n, prec, if n >= T::ZERO { Up } else { Down });
        assert_eq!(
            ComparableFloatRef(&ceiling_n_alt),
            ComparableFloatRef(&ceiling.0)
        );
        assert_eq!(o_alt, ceiling.1);

        let nearest = Float::from_signed_prec_round(n, prec, Nearest);
        let r_nearest = Rational::exact_from(&nearest.0);
        assert!(
            ComparableFloatRef(&nearest.0) == ComparableFloatRef(&floor.0) && nearest.1 == floor.1
                || ComparableFloatRef(&nearest.0) == ComparableFloatRef(&ceiling.0)
                    && nearest.1 == ceiling.1
        );
        if r_nearest != T::ZERO {
            assert!((r_nearest - Rational::from(n))
                .le_abs(&(Rational::exact_from(nearest.0.ulp().unwrap()) >> 1)));
        }
    });
}

#[test]
fn from_primitive_int_prec_round_properties() {
    apply_fn_to_unsigneds!(from_primitive_int_prec_round_properties_helper_unsigned);
    apply_fn_to_signeds!(from_primitive_int_prec_round_properties_helper_signed);
}

#[test]
fn const_from_unsigned_times_power_of_2_properties() {
    unsigned_signed_pair_gen_var_1().test_properties(|(n, pow)| {
        let float_n = Float::const_from_unsigned_times_power_of_2(n, pow);
        assert!(float_n.is_valid());
        assert!(float_n >= 0);
        assert_eq!(
            ComparableFloat(float_n),
            ComparableFloat(Float::from(n) << pow)
        );
    });

    signed_gen_var_5().test_properties(|pow| {
        assert_eq!(
            ComparableFloat(Float::const_from_unsigned_times_power_of_2(1, pow)),
            ComparableFloat(Float::power_of_2(i64::from(pow)))
        );
    });
}

#[test]
fn const_from_signed_times_power_of_2_properties() {
    signed_pair_gen_var_2().test_properties(|(n, pow)| {
        let float_n = Float::const_from_signed_times_power_of_2(n, pow);
        assert!(float_n.is_valid());
        assert_eq!(float_n >= 0, n >= 0);
        assert_eq!(
            ComparableFloat(float_n),
            ComparableFloat(Float::from(n) << pow)
        );
    });

    signed_gen_var_5().test_properties(|pow| {
        assert_eq!(
            ComparableFloat(Float::const_from_signed_times_power_of_2(1, pow)),
            ComparableFloat(Float::power_of_2(i64::from(pow)))
        );
    });
}
