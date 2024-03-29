use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_20, unsigned_pair_gen_var_32,
};
use malachite_float::test_util::common::{rug_round_try_from_rounding_mode, to_hex_string};
use malachite_float::test_util::generators::*;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;
use rug::Assign;
use std::cmp::{max, Ordering};
use std::panic::catch_unwind;

#[test]
fn test_from_primitive_int() {
    fn test_helper<T: PrimitiveInt>(u: T, out: &str, out_hex: &str)
    where
        Float: From<T>,
        rug::Float: Assign<T>,
    {
        let x = Float::from(u);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);

        let rug_x = rug::Float::with_val(max(1, u32::exact_from(u.significant_bits())), u);
        let x = Float::exact_from(&rug_x);
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
    }
    fn test_helper_ui<T: PrimitiveInt>()
    where
        Float: From<T>,
        rug::Float: Assign<T>,
    {
        test_helper(T::ZERO, "0.0", "0x0.0");
        test_helper(T::ONE, "1.0", "0x1.0#1");
        test_helper(T::exact_from(123u8), "123.0", "0x7b.0#7");
    }
    apply_fn_to_primitive_ints!(test_helper_ui);
    test_helper(1000000000000u64, "1000000000000.0", "0xe8d4a51000.0#40");

    fn test_helper_i<T: PrimitiveSigned>()
    where
        Float: From<T>,
        rug::Float: Assign<T>,
    {
        test_helper(T::NEGATIVE_ONE, "-1.0", "-0x1.0#1");
        test_helper(T::from(-123i8), "-123.0", "-0x7b.0#7");
    }
    apply_fn_to_signeds!(test_helper_i);
    test_helper(-1000000000000i64, "-1000000000000.0", "-0xe8d4a51000.0#40");
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
        test_helper_u(T::ZERO, 1, "0.0", "0x0.0", Ordering::Equal);
        test_helper_u(T::ZERO, 10, "0.0", "0x0.0", Ordering::Equal);
        test_helper_u(T::ZERO, 20, "0.0", "0x0.0", Ordering::Equal);

        test_helper_u(T::ONE, 1, "1.0", "0x1.0#1", Ordering::Equal);
        test_helper_u(T::ONE, 10, "1.0", "0x1.000#10", Ordering::Equal);
        test_helper_u(T::ONE, 20, "1.0", "0x1.00000#20", Ordering::Equal);

        test_helper_u(T::from(123u8), 1, "1.0e2", "0x8.0E+1#1", Ordering::Greater);
        test_helper_u(T::from(123u8), 10, "123.0", "0x7b.0#10", Ordering::Equal);
        test_helper_u(T::from(123u8), 20, "123.0", "0x7b.0000#20", Ordering::Equal);
    }
    apply_fn_to_unsigneds!(test_helper_u2);
    test_helper_u(
        1000000000000u64,
        1,
        "1.0e12",
        "0x1.0E+10#1",
        Ordering::Greater,
    );
    test_helper_u(
        1000000000000u64,
        10,
        "9.997e11",
        "0xe.8cE+9#10",
        Ordering::Less,
    );
    test_helper_u(
        1000000000000u64,
        20,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Ordering::Less,
    );

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
        test_helper_i(T::ZERO, 1, "0.0", "0x0.0", Ordering::Equal);
        test_helper_i(T::ZERO, 10, "0.0", "0x0.0", Ordering::Equal);
        test_helper_i(T::ZERO, 20, "0.0", "0x0.0", Ordering::Equal);

        test_helper_i(T::ONE, 1, "1.0", "0x1.0#1", Ordering::Equal);
        test_helper_i(T::ONE, 10, "1.0", "0x1.000#10", Ordering::Equal);
        test_helper_i(T::ONE, 20, "1.0", "0x1.00000#20", Ordering::Equal);

        test_helper_i(T::from(123i8), 1, "1.0e2", "0x8.0E+1#1", Ordering::Greater);
        test_helper_i(T::from(123i8), 10, "123.0", "0x7b.0#10", Ordering::Equal);
        test_helper_i(T::from(123i8), 20, "123.0", "0x7b.0000#20", Ordering::Equal);

        test_helper_i(T::NEGATIVE_ONE, 1, "-1.0", "-0x1.0#1", Ordering::Equal);
        test_helper_i(T::NEGATIVE_ONE, 10, "-1.0", "-0x1.000#10", Ordering::Equal);
        test_helper_i(
            T::NEGATIVE_ONE,
            20,
            "-1.0",
            "-0x1.00000#20",
            Ordering::Equal,
        );

        test_helper_i(T::from(-123i8), 1, "-1.0e2", "-0x8.0E+1#1", Ordering::Less);
        test_helper_i(T::from(-123i8), 10, "-123.0", "-0x7b.0#10", Ordering::Equal);
        test_helper_i(
            T::from(-123i8),
            20,
            "-123.0",
            "-0x7b.0000#20",
            Ordering::Equal,
        );
    }
    apply_fn_to_signeds!(test_helper_i2);
    test_helper_i(
        1000000000000i64,
        1,
        "1.0e12",
        "0x1.0E+10#1",
        Ordering::Greater,
    );
    test_helper_i(
        1000000000000i64,
        10,
        "9.997e11",
        "0xe.8cE+9#10",
        Ordering::Less,
    );
    test_helper_i(
        1000000000000i64,
        20,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Ordering::Less,
    );
    test_helper_i(
        -1000000000000i64,
        1,
        "-1.0e12",
        "-0x1.0E+10#1",
        Ordering::Less,
    );
    test_helper_i(
        -1000000000000i64,
        10,
        "-9.997e11",
        "-0xe.8cE+9#10",
        Ordering::Greater,
    );
    test_helper_i(
        -1000000000000i64,
        20,
        "-9.999997e11",
        "-0xe.8d4aE+9#20",
        Ordering::Greater,
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
        test_helper_u(
            T::ZERO,
            1,
            RoundingMode::Floor,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_u(
            T::ZERO,
            1,
            RoundingMode::Ceiling,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_u(
            T::ZERO,
            1,
            RoundingMode::Down,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_u(
            T::ZERO,
            1,
            RoundingMode::Up,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_u(
            T::ZERO,
            1,
            RoundingMode::Nearest,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_u(
            T::ZERO,
            1,
            RoundingMode::Exact,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );

        test_helper_u(
            T::ZERO,
            10,
            RoundingMode::Floor,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_u(
            T::ZERO,
            10,
            RoundingMode::Ceiling,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_u(
            T::ZERO,
            10,
            RoundingMode::Down,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_u(
            T::ZERO,
            10,
            RoundingMode::Up,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_u(
            T::ZERO,
            10,
            RoundingMode::Nearest,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_u(
            T::ZERO,
            10,
            RoundingMode::Exact,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );

        test_helper_u(
            T::ZERO,
            20,
            RoundingMode::Floor,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_u(
            T::ZERO,
            20,
            RoundingMode::Ceiling,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_u(
            T::ZERO,
            20,
            RoundingMode::Down,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_u(
            T::ZERO,
            20,
            RoundingMode::Up,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_u(
            T::ZERO,
            20,
            RoundingMode::Nearest,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_u(
            T::ZERO,
            20,
            RoundingMode::Exact,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );

        test_helper_u(
            T::ONE,
            1,
            RoundingMode::Floor,
            "1.0",
            "0x1.0#1",
            Ordering::Equal,
        );
        test_helper_u(
            T::ONE,
            1,
            RoundingMode::Ceiling,
            "1.0",
            "0x1.0#1",
            Ordering::Equal,
        );
        test_helper_u(
            T::ONE,
            1,
            RoundingMode::Down,
            "1.0",
            "0x1.0#1",
            Ordering::Equal,
        );
        test_helper_u(
            T::ONE,
            1,
            RoundingMode::Up,
            "1.0",
            "0x1.0#1",
            Ordering::Equal,
        );
        test_helper_u(
            T::ONE,
            1,
            RoundingMode::Nearest,
            "1.0",
            "0x1.0#1",
            Ordering::Equal,
        );
        test_helper_u(
            T::ONE,
            1,
            RoundingMode::Exact,
            "1.0",
            "0x1.0#1",
            Ordering::Equal,
        );

        test_helper_u(
            T::ONE,
            10,
            RoundingMode::Floor,
            "1.0",
            "0x1.000#10",
            Ordering::Equal,
        );
        test_helper_u(
            T::ONE,
            10,
            RoundingMode::Ceiling,
            "1.0",
            "0x1.000#10",
            Ordering::Equal,
        );
        test_helper_u(
            T::ONE,
            10,
            RoundingMode::Down,
            "1.0",
            "0x1.000#10",
            Ordering::Equal,
        );
        test_helper_u(
            T::ONE,
            10,
            RoundingMode::Up,
            "1.0",
            "0x1.000#10",
            Ordering::Equal,
        );
        test_helper_u(
            T::ONE,
            10,
            RoundingMode::Nearest,
            "1.0",
            "0x1.000#10",
            Ordering::Equal,
        );
        test_helper_u(
            T::ONE,
            10,
            RoundingMode::Exact,
            "1.0",
            "0x1.000#10",
            Ordering::Equal,
        );

        test_helper_u(
            T::ONE,
            20,
            RoundingMode::Floor,
            "1.0",
            "0x1.00000#20",
            Ordering::Equal,
        );
        test_helper_u(
            T::ONE,
            20,
            RoundingMode::Ceiling,
            "1.0",
            "0x1.00000#20",
            Ordering::Equal,
        );
        test_helper_u(
            T::ONE,
            20,
            RoundingMode::Down,
            "1.0",
            "0x1.00000#20",
            Ordering::Equal,
        );
        test_helper_u(
            T::ONE,
            20,
            RoundingMode::Up,
            "1.0",
            "0x1.00000#20",
            Ordering::Equal,
        );
        test_helper_u(
            T::ONE,
            20,
            RoundingMode::Nearest,
            "1.0",
            "0x1.00000#20",
            Ordering::Equal,
        );
        test_helper_u(
            T::ONE,
            20,
            RoundingMode::Exact,
            "1.0",
            "0x1.00000#20",
            Ordering::Equal,
        );

        test_helper_u(
            T::from(123u8),
            1,
            RoundingMode::Floor,
            "6.0e1",
            "0x4.0E+1#1",
            Ordering::Less,
        );
        test_helper_u(
            T::from(123u8),
            1,
            RoundingMode::Ceiling,
            "1.0e2",
            "0x8.0E+1#1",
            Ordering::Greater,
        );
        test_helper_u(
            T::from(123u8),
            1,
            RoundingMode::Down,
            "6.0e1",
            "0x4.0E+1#1",
            Ordering::Less,
        );
        test_helper_u(
            T::from(123u8),
            1,
            RoundingMode::Up,
            "1.0e2",
            "0x8.0E+1#1",
            Ordering::Greater,
        );
        test_helper_u(
            T::from(123u8),
            1,
            RoundingMode::Nearest,
            "1.0e2",
            "0x8.0E+1#1",
            Ordering::Greater,
        );

        test_helper_u(
            T::from(123u8),
            10,
            RoundingMode::Floor,
            "123.0",
            "0x7b.0#10",
            Ordering::Equal,
        );
        test_helper_u(
            T::from(123u8),
            10,
            RoundingMode::Ceiling,
            "123.0",
            "0x7b.0#10",
            Ordering::Equal,
        );
        test_helper_u(
            T::from(123u8),
            10,
            RoundingMode::Down,
            "123.0",
            "0x7b.0#10",
            Ordering::Equal,
        );
        test_helper_u(
            T::from(123u8),
            10,
            RoundingMode::Up,
            "123.0",
            "0x7b.0#10",
            Ordering::Equal,
        );
        test_helper_u(
            T::from(123u8),
            10,
            RoundingMode::Nearest,
            "123.0",
            "0x7b.0#10",
            Ordering::Equal,
        );
        test_helper_u(
            T::from(123u8),
            10,
            RoundingMode::Exact,
            "123.0",
            "0x7b.0#10",
            Ordering::Equal,
        );

        test_helper_u(
            T::from(123u8),
            20,
            RoundingMode::Floor,
            "123.0",
            "0x7b.0000#20",
            Ordering::Equal,
        );
        test_helper_u(
            T::from(123u8),
            20,
            RoundingMode::Ceiling,
            "123.0",
            "0x7b.0000#20",
            Ordering::Equal,
        );
        test_helper_u(
            T::from(123u8),
            20,
            RoundingMode::Down,
            "123.0",
            "0x7b.0000#20",
            Ordering::Equal,
        );
        test_helper_u(
            T::from(123u8),
            20,
            RoundingMode::Up,
            "123.0",
            "0x7b.0000#20",
            Ordering::Equal,
        );
        test_helper_u(
            T::from(123u8),
            20,
            RoundingMode::Nearest,
            "123.0",
            "0x7b.0000#20",
            Ordering::Equal,
        );
        test_helper_u(
            T::from(123u8),
            20,
            RoundingMode::Exact,
            "123.0",
            "0x7b.0000#20",
            Ordering::Equal,
        );
    }
    apply_fn_to_unsigneds!(test_helper_u2);
    test_helper_u(
        1000000000000u64,
        1,
        RoundingMode::Floor,
        "5.0e11",
        "0x8.0E+9#1",
        Ordering::Less,
    );
    test_helper_u(
        1000000000000u64,
        1,
        RoundingMode::Ceiling,
        "1.0e12",
        "0x1.0E+10#1",
        Ordering::Greater,
    );
    test_helper_u(
        1000000000000u64,
        1,
        RoundingMode::Down,
        "5.0e11",
        "0x8.0E+9#1",
        Ordering::Less,
    );
    test_helper_u(
        1000000000000u64,
        1,
        RoundingMode::Up,
        "1.0e12",
        "0x1.0E+10#1",
        Ordering::Greater,
    );
    test_helper_u(
        1000000000000u64,
        1,
        RoundingMode::Nearest,
        "1.0e12",
        "0x1.0E+10#1",
        Ordering::Greater,
    );

    test_helper_u(
        1000000000000u64,
        10,
        RoundingMode::Floor,
        "9.997e11",
        "0xe.8cE+9#10",
        Ordering::Less,
    );
    test_helper_u(
        1000000000000u64,
        10,
        RoundingMode::Ceiling,
        "1.001e12",
        "0xe.90E+9#10",
        Ordering::Greater,
    );
    test_helper_u(
        1000000000000u64,
        10,
        RoundingMode::Down,
        "9.997e11",
        "0xe.8cE+9#10",
        Ordering::Less,
    );
    test_helper_u(
        1000000000000u64,
        10,
        RoundingMode::Up,
        "1.001e12",
        "0xe.90E+9#10",
        Ordering::Greater,
    );
    test_helper_u(
        1000000000000u64,
        10,
        RoundingMode::Nearest,
        "9.997e11",
        "0xe.8cE+9#10",
        Ordering::Less,
    );

    test_helper_u(
        1000000000000u64,
        20,
        RoundingMode::Floor,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Ordering::Less,
    );
    test_helper_u(
        1000000000000u64,
        20,
        RoundingMode::Ceiling,
        "1.000001e12",
        "0xe.8d4bE+9#20",
        Ordering::Greater,
    );
    test_helper_u(
        1000000000000u64,
        20,
        RoundingMode::Down,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Ordering::Less,
    );
    test_helper_u(
        1000000000000u64,
        20,
        RoundingMode::Up,
        "1.000001e12",
        "0xe.8d4bE+9#20",
        Ordering::Greater,
    );
    test_helper_u(
        1000000000000u64,
        20,
        RoundingMode::Nearest,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Ordering::Less,
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
        test_helper_i(
            T::ZERO,
            1,
            RoundingMode::Floor,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_i(
            T::ZERO,
            1,
            RoundingMode::Ceiling,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_i(
            T::ZERO,
            1,
            RoundingMode::Down,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_i(
            T::ZERO,
            1,
            RoundingMode::Up,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_i(
            T::ZERO,
            1,
            RoundingMode::Nearest,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_i(
            T::ZERO,
            1,
            RoundingMode::Exact,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );

        test_helper_i(
            T::ZERO,
            10,
            RoundingMode::Floor,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_i(
            T::ZERO,
            10,
            RoundingMode::Ceiling,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_i(
            T::ZERO,
            10,
            RoundingMode::Down,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_i(
            T::ZERO,
            10,
            RoundingMode::Up,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_i(
            T::ZERO,
            10,
            RoundingMode::Nearest,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_i(
            T::ZERO,
            10,
            RoundingMode::Exact,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );

        test_helper_i(
            T::ZERO,
            20,
            RoundingMode::Floor,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_i(
            T::ZERO,
            20,
            RoundingMode::Ceiling,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_i(
            T::ZERO,
            20,
            RoundingMode::Down,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_i(
            T::ZERO,
            20,
            RoundingMode::Up,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_i(
            T::ZERO,
            20,
            RoundingMode::Nearest,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );
        test_helper_i(
            T::ZERO,
            20,
            RoundingMode::Exact,
            "0.0",
            "0x0.0",
            Ordering::Equal,
        );

        test_helper_i(
            T::ONE,
            1,
            RoundingMode::Floor,
            "1.0",
            "0x1.0#1",
            Ordering::Equal,
        );
        test_helper_i(
            T::ONE,
            1,
            RoundingMode::Ceiling,
            "1.0",
            "0x1.0#1",
            Ordering::Equal,
        );
        test_helper_i(
            T::ONE,
            1,
            RoundingMode::Down,
            "1.0",
            "0x1.0#1",
            Ordering::Equal,
        );
        test_helper_i(
            T::ONE,
            1,
            RoundingMode::Up,
            "1.0",
            "0x1.0#1",
            Ordering::Equal,
        );
        test_helper_i(
            T::ONE,
            1,
            RoundingMode::Nearest,
            "1.0",
            "0x1.0#1",
            Ordering::Equal,
        );
        test_helper_i(
            T::ONE,
            1,
            RoundingMode::Exact,
            "1.0",
            "0x1.0#1",
            Ordering::Equal,
        );

        test_helper_i(
            T::ONE,
            10,
            RoundingMode::Floor,
            "1.0",
            "0x1.000#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::ONE,
            10,
            RoundingMode::Ceiling,
            "1.0",
            "0x1.000#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::ONE,
            10,
            RoundingMode::Down,
            "1.0",
            "0x1.000#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::ONE,
            10,
            RoundingMode::Up,
            "1.0",
            "0x1.000#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::ONE,
            10,
            RoundingMode::Nearest,
            "1.0",
            "0x1.000#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::ONE,
            10,
            RoundingMode::Exact,
            "1.0",
            "0x1.000#10",
            Ordering::Equal,
        );

        test_helper_i(
            T::ONE,
            20,
            RoundingMode::Floor,
            "1.0",
            "0x1.00000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::ONE,
            20,
            RoundingMode::Ceiling,
            "1.0",
            "0x1.00000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::ONE,
            20,
            RoundingMode::Down,
            "1.0",
            "0x1.00000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::ONE,
            20,
            RoundingMode::Up,
            "1.0",
            "0x1.00000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::ONE,
            20,
            RoundingMode::Nearest,
            "1.0",
            "0x1.00000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::ONE,
            20,
            RoundingMode::Exact,
            "1.0",
            "0x1.00000#20",
            Ordering::Equal,
        );

        test_helper_i(
            T::from(123i8),
            1,
            RoundingMode::Floor,
            "6.0e1",
            "0x4.0E+1#1",
            Ordering::Less,
        );
        test_helper_i(
            T::from(123i8),
            1,
            RoundingMode::Ceiling,
            "1.0e2",
            "0x8.0E+1#1",
            Ordering::Greater,
        );
        test_helper_i(
            T::from(123i8),
            1,
            RoundingMode::Down,
            "6.0e1",
            "0x4.0E+1#1",
            Ordering::Less,
        );
        test_helper_i(
            T::from(123i8),
            1,
            RoundingMode::Up,
            "1.0e2",
            "0x8.0E+1#1",
            Ordering::Greater,
        );
        test_helper_i(
            T::from(123i8),
            1,
            RoundingMode::Nearest,
            "1.0e2",
            "0x8.0E+1#1",
            Ordering::Greater,
        );

        test_helper_i(
            T::from(123i8),
            10,
            RoundingMode::Floor,
            "123.0",
            "0x7b.0#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(123i8),
            10,
            RoundingMode::Ceiling,
            "123.0",
            "0x7b.0#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(123i8),
            10,
            RoundingMode::Down,
            "123.0",
            "0x7b.0#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(123i8),
            10,
            RoundingMode::Up,
            "123.0",
            "0x7b.0#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(123i8),
            10,
            RoundingMode::Nearest,
            "123.0",
            "0x7b.0#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(123i8),
            10,
            RoundingMode::Exact,
            "123.0",
            "0x7b.0#10",
            Ordering::Equal,
        );

        test_helper_i(
            T::from(123i8),
            20,
            RoundingMode::Floor,
            "123.0",
            "0x7b.0000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(123i8),
            20,
            RoundingMode::Ceiling,
            "123.0",
            "0x7b.0000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(123i8),
            20,
            RoundingMode::Down,
            "123.0",
            "0x7b.0000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(123i8),
            20,
            RoundingMode::Up,
            "123.0",
            "0x7b.0000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(123i8),
            20,
            RoundingMode::Nearest,
            "123.0",
            "0x7b.0000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(123i8),
            20,
            RoundingMode::Exact,
            "123.0",
            "0x7b.0000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::NEGATIVE_ONE,
            1,
            RoundingMode::Floor,
            "-1.0",
            "-0x1.0#1",
            Ordering::Equal,
        );
        test_helper_i(
            T::NEGATIVE_ONE,
            1,
            RoundingMode::Ceiling,
            "-1.0",
            "-0x1.0#1",
            Ordering::Equal,
        );
        test_helper_i(
            T::NEGATIVE_ONE,
            1,
            RoundingMode::Down,
            "-1.0",
            "-0x1.0#1",
            Ordering::Equal,
        );
        test_helper_i(
            T::NEGATIVE_ONE,
            1,
            RoundingMode::Up,
            "-1.0",
            "-0x1.0#1",
            Ordering::Equal,
        );
        test_helper_i(
            T::NEGATIVE_ONE,
            1,
            RoundingMode::Nearest,
            "-1.0",
            "-0x1.0#1",
            Ordering::Equal,
        );
        test_helper_i(
            T::NEGATIVE_ONE,
            1,
            RoundingMode::Exact,
            "-1.0",
            "-0x1.0#1",
            Ordering::Equal,
        );

        test_helper_i(
            T::NEGATIVE_ONE,
            10,
            RoundingMode::Floor,
            "-1.0",
            "-0x1.000#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::NEGATIVE_ONE,
            10,
            RoundingMode::Ceiling,
            "-1.0",
            "-0x1.000#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::NEGATIVE_ONE,
            10,
            RoundingMode::Down,
            "-1.0",
            "-0x1.000#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::NEGATIVE_ONE,
            10,
            RoundingMode::Up,
            "-1.0",
            "-0x1.000#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::NEGATIVE_ONE,
            10,
            RoundingMode::Nearest,
            "-1.0",
            "-0x1.000#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::NEGATIVE_ONE,
            10,
            RoundingMode::Exact,
            "-1.0",
            "-0x1.000#10",
            Ordering::Equal,
        );

        test_helper_i(
            T::NEGATIVE_ONE,
            20,
            RoundingMode::Floor,
            "-1.0",
            "-0x1.00000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::NEGATIVE_ONE,
            20,
            RoundingMode::Ceiling,
            "-1.0",
            "-0x1.00000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::NEGATIVE_ONE,
            20,
            RoundingMode::Down,
            "-1.0",
            "-0x1.00000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::NEGATIVE_ONE,
            20,
            RoundingMode::Up,
            "-1.0",
            "-0x1.00000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::NEGATIVE_ONE,
            20,
            RoundingMode::Nearest,
            "-1.0",
            "-0x1.00000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::NEGATIVE_ONE,
            20,
            RoundingMode::Exact,
            "-1.0",
            "-0x1.00000#20",
            Ordering::Equal,
        );

        test_helper_i(
            T::from(-123i8),
            1,
            RoundingMode::Floor,
            "-1.0e2",
            "-0x8.0E+1#1",
            Ordering::Less,
        );
        test_helper_i(
            T::from(-123i8),
            1,
            RoundingMode::Ceiling,
            "-6.0e1",
            "-0x4.0E+1#1",
            Ordering::Greater,
        );
        test_helper_i(
            T::from(-123i8),
            1,
            RoundingMode::Down,
            "-6.0e1",
            "-0x4.0E+1#1",
            Ordering::Greater,
        );
        test_helper_i(
            T::from(-123i8),
            1,
            RoundingMode::Up,
            "-1.0e2",
            "-0x8.0E+1#1",
            Ordering::Less,
        );
        test_helper_i(
            T::from(-123i8),
            1,
            RoundingMode::Nearest,
            "-1.0e2",
            "-0x8.0E+1#1",
            Ordering::Less,
        );

        test_helper_i(
            T::from(-123i8),
            10,
            RoundingMode::Floor,
            "-123.0",
            "-0x7b.0#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(-123i8),
            10,
            RoundingMode::Ceiling,
            "-123.0",
            "-0x7b.0#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(-123i8),
            10,
            RoundingMode::Down,
            "-123.0",
            "-0x7b.0#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(-123i8),
            10,
            RoundingMode::Up,
            "-123.0",
            "-0x7b.0#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(-123i8),
            10,
            RoundingMode::Nearest,
            "-123.0",
            "-0x7b.0#10",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(-123i8),
            10,
            RoundingMode::Exact,
            "-123.0",
            "-0x7b.0#10",
            Ordering::Equal,
        );

        test_helper_i(
            T::from(-123i8),
            20,
            RoundingMode::Floor,
            "-123.0",
            "-0x7b.0000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(-123i8),
            20,
            RoundingMode::Ceiling,
            "-123.0",
            "-0x7b.0000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(-123i8),
            20,
            RoundingMode::Down,
            "-123.0",
            "-0x7b.0000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(-123i8),
            20,
            RoundingMode::Up,
            "-123.0",
            "-0x7b.0000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(-123i8),
            20,
            RoundingMode::Nearest,
            "-123.0",
            "-0x7b.0000#20",
            Ordering::Equal,
        );
        test_helper_i(
            T::from(-123i8),
            20,
            RoundingMode::Exact,
            "-123.0",
            "-0x7b.0000#20",
            Ordering::Equal,
        );
    }
    apply_fn_to_signeds!(test_helper_i2);
    test_helper_i(
        1000000000000i64,
        1,
        RoundingMode::Floor,
        "5.0e11",
        "0x8.0E+9#1",
        Ordering::Less,
    );
    test_helper_i(
        1000000000000i64,
        1,
        RoundingMode::Ceiling,
        "1.0e12",
        "0x1.0E+10#1",
        Ordering::Greater,
    );
    test_helper_i(
        1000000000000i64,
        1,
        RoundingMode::Down,
        "5.0e11",
        "0x8.0E+9#1",
        Ordering::Less,
    );
    test_helper_i(
        1000000000000i64,
        1,
        RoundingMode::Up,
        "1.0e12",
        "0x1.0E+10#1",
        Ordering::Greater,
    );
    test_helper_i(
        1000000000000i64,
        1,
        RoundingMode::Nearest,
        "1.0e12",
        "0x1.0E+10#1",
        Ordering::Greater,
    );

    test_helper_i(
        1000000000000i64,
        10,
        RoundingMode::Floor,
        "9.997e11",
        "0xe.8cE+9#10",
        Ordering::Less,
    );
    test_helper_i(
        1000000000000i64,
        10,
        RoundingMode::Ceiling,
        "1.001e12",
        "0xe.90E+9#10",
        Ordering::Greater,
    );
    test_helper_i(
        1000000000000i64,
        10,
        RoundingMode::Down,
        "9.997e11",
        "0xe.8cE+9#10",
        Ordering::Less,
    );
    test_helper_i(
        1000000000000i64,
        10,
        RoundingMode::Up,
        "1.001e12",
        "0xe.90E+9#10",
        Ordering::Greater,
    );
    test_helper_i(
        1000000000000i64,
        10,
        RoundingMode::Nearest,
        "9.997e11",
        "0xe.8cE+9#10",
        Ordering::Less,
    );

    test_helper_i(
        1000000000000i64,
        20,
        RoundingMode::Floor,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Ordering::Less,
    );
    test_helper_i(
        1000000000000i64,
        20,
        RoundingMode::Ceiling,
        "1.000001e12",
        "0xe.8d4bE+9#20",
        Ordering::Greater,
    );
    test_helper_i(
        1000000000000i64,
        20,
        RoundingMode::Down,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Ordering::Less,
    );
    test_helper_i(
        1000000000000i64,
        20,
        RoundingMode::Up,
        "1.000001e12",
        "0xe.8d4bE+9#20",
        Ordering::Greater,
    );
    test_helper_i(
        1000000000000i64,
        20,
        RoundingMode::Nearest,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Ordering::Less,
    );
    test_helper_i(
        -1000000000000i64,
        1,
        RoundingMode::Floor,
        "-1.0e12",
        "-0x1.0E+10#1",
        Ordering::Less,
    );
    test_helper_i(
        -1000000000000i64,
        1,
        RoundingMode::Ceiling,
        "-5.0e11",
        "-0x8.0E+9#1",
        Ordering::Greater,
    );
    test_helper_i(
        -1000000000000i64,
        1,
        RoundingMode::Down,
        "-5.0e11",
        "-0x8.0E+9#1",
        Ordering::Greater,
    );
    test_helper_i(
        -1000000000000i64,
        1,
        RoundingMode::Up,
        "-1.0e12",
        "-0x1.0E+10#1",
        Ordering::Less,
    );
    test_helper_i(
        -1000000000000i64,
        1,
        RoundingMode::Nearest,
        "-1.0e12",
        "-0x1.0E+10#1",
        Ordering::Less,
    );

    test_helper_i(
        -1000000000000i64,
        10,
        RoundingMode::Floor,
        "-1.001e12",
        "-0xe.90E+9#10",
        Ordering::Less,
    );
    test_helper_i(
        -1000000000000i64,
        10,
        RoundingMode::Ceiling,
        "-9.997e11",
        "-0xe.8cE+9#10",
        Ordering::Greater,
    );
    test_helper_i(
        -1000000000000i64,
        10,
        RoundingMode::Down,
        "-9.997e11",
        "-0xe.8cE+9#10",
        Ordering::Greater,
    );
    test_helper_i(
        -1000000000000i64,
        10,
        RoundingMode::Up,
        "-1.001e12",
        "-0xe.90E+9#10",
        Ordering::Less,
    );
    test_helper_i(
        -1000000000000i64,
        10,
        RoundingMode::Nearest,
        "-9.997e11",
        "-0xe.8cE+9#10",
        Ordering::Greater,
    );

    test_helper_i(
        -1000000000000i64,
        20,
        RoundingMode::Floor,
        "-1.000001e12",
        "-0xe.8d4bE+9#20",
        Ordering::Less,
    );
    test_helper_i(
        -1000000000000i64,
        20,
        RoundingMode::Ceiling,
        "-9.999997e11",
        "-0xe.8d4aE+9#20",
        Ordering::Greater,
    );
    test_helper_i(
        -1000000000000i64,
        20,
        RoundingMode::Down,
        "-9.999997e11",
        "-0xe.8d4aE+9#20",
        Ordering::Greater,
    );
    test_helper_i(
        -1000000000000i64,
        20,
        RoundingMode::Up,
        "-1.000001e12",
        "-0xe.8d4bE+9#20",
        Ordering::Less,
    );
    test_helper_i(
        -1000000000000i64,
        20,
        RoundingMode::Nearest,
        "-9.999997e11",
        "-0xe.8d4aE+9#20",
        Ordering::Greater,
    );
}

fn from_unsigned_prec_round_fail_helper<T: PrimitiveUnsigned>()
where
    Natural: From<T>,
{
    assert_panic!(Float::from_unsigned_prec_round(
        T::ZERO,
        0,
        RoundingMode::Floor
    ));
    assert_panic!(Float::from_unsigned_prec_round(
        T::ONE,
        0,
        RoundingMode::Floor
    ));
    assert_panic!(Float::from_unsigned_prec_round(
        T::from(123u8),
        1,
        RoundingMode::Exact
    ));
}

fn from_signed_prec_round_fail_helper<T: PrimitiveSigned>()
where
    Integer: From<T>,
{
    assert_panic!(Float::from_signed_prec_round(
        T::ZERO,
        0,
        RoundingMode::Floor
    ));
    assert_panic!(Float::from_signed_prec_round(
        T::ONE,
        0,
        RoundingMode::Floor
    ));
    assert_panic!(Float::from_signed_prec_round(
        T::from(123i8),
        1,
        RoundingMode::Exact
    ));
    assert_panic!(Float::from_signed_prec_round(
        T::NEGATIVE_ONE,
        0,
        RoundingMode::Floor
    ));
    assert_panic!(Float::from_signed_prec_round(
        T::from(-123i8),
        1,
        RoundingMode::Exact
    ));
}

#[test]
fn from_primitive_int_prec_round_fail() {
    apply_fn_to_unsigneds!(from_unsigned_prec_round_fail_helper);
    apply_fn_to_signeds!(from_signed_prec_round_fail_helper);
}

#[allow(clippy::type_repetition_in_bounds)]
fn from_primitive_int_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    Float: From<T>,
    rug::Float: Assign<T>,
    Natural: From<T> + PartialEq<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    unsigned_gen::<T>().test_properties(|n| {
        let float_n = Float::from(n);
        assert!(float_n.is_valid());

        let rug_n = rug::Float::with_val(max(1, u32::exact_from(n.significant_bits())), n);
        assert_eq!(
            ComparableFloatRef(&float_n),
            ComparableFloatRef(&From::<&rug::Float>::from(&rug_n))
        );

        let n_alt: Float = From::from(Natural::from(n));
        assert_eq!(ComparableFloatRef(&n_alt), ComparableFloatRef(&float_n));

        assert_eq!(
            float_n.get_prec(),
            if n == T::ZERO {
                None
            } else {
                Some(n.significant_bits())
            }
        );
        assert_eq!(T::exact_from(&float_n), n);
        let bits = max(1, n.significant_bits());
        let (f, o) = Float::from_unsigned_prec(n, bits);
        assert_eq!(ComparableFloat(f), ComparableFloat(float_n));
        assert_eq!(o, Ordering::Equal);
    });
}

#[allow(clippy::type_repetition_in_bounds)]
fn from_primitive_int_properties_helper_signed<T: PrimitiveSigned>()
where
    Float: From<T>,
    rug::Float: Assign<T>,
    Integer: From<T> + PartialEq<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    signed_gen::<T>().test_properties(|n| {
        let float_n = Float::from(n);
        assert!(float_n.is_valid());

        let rug_n = rug::Float::with_val(max(1, u32::exact_from(n.significant_bits())), n);
        assert_eq!(
            ComparableFloatRef(&float_n),
            ComparableFloatRef(&From::<&rug::Float>::from(&rug_n))
        );

        let n_alt: Float = From::from(Integer::from(n));
        assert_eq!(ComparableFloatRef(&n_alt), ComparableFloatRef(&float_n));

        assert_eq!(
            float_n.get_prec(),
            if n == T::ZERO {
                None
            } else {
                Some(n.significant_bits())
            }
        );
        assert_eq!(T::exact_from(&float_n), n);
        let bits = max(1, n.significant_bits());
        let (f, o) = Float::from_signed_prec(n, bits);
        assert_eq!(ComparableFloat(f), ComparableFloat(float_n));
        assert_eq!(o, Ordering::Equal);
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
            RoundingMode::Floor | RoundingMode::Down => {
                assert_ne!(o, Ordering::Greater)
            }
            RoundingMode::Ceiling | RoundingMode::Up => {
                assert_ne!(o, Ordering::Less)
            }
            RoundingMode::Exact => assert_eq!(o, Ordering::Equal),
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
        let floor = Float::from_unsigned_prec_round(n, prec, RoundingMode::Floor);
        let r_floor = Rational::exact_from(&floor.0);
        assert!(r_floor <= n);
        if r_floor != T::ZERO {
            assert!(r_floor + Rational::exact_from(floor.0.ulp().unwrap()) > n);
        }
        let (floor_alt, floor_o_alt) = Float::from_unsigned_prec_round(n, prec, RoundingMode::Down);
        assert_eq!(ComparableFloatRef(&floor_alt), ComparableFloatRef(&floor.0));
        assert_eq!(floor_o_alt, floor.1);

        let ceiling = Float::from_unsigned_prec_round(n, prec, RoundingMode::Ceiling);
        let r_ceiling = Rational::exact_from(&ceiling.0);
        assert!(r_ceiling >= n);
        if r_ceiling != T::ZERO {
            assert!(r_ceiling - Rational::exact_from(ceiling.0.ulp().unwrap()) < n);
        }
        let (ceiling_alt, ceiling_o_alt) =
            Float::from_unsigned_prec_round(n, prec, RoundingMode::Up);
        assert_eq!(
            ComparableFloatRef(&ceiling_alt),
            ComparableFloatRef(&ceiling.0)
        );
        assert_eq!(ceiling_o_alt, ceiling.1);

        let nearest = Float::from_unsigned_prec_round(n, prec, RoundingMode::Nearest);
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
            (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
                assert_ne!(o, Ordering::Greater)
            }
            (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
                assert_ne!(o, Ordering::Less)
            }
            (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
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
        let floor = Float::from_signed_prec_round(n, prec, RoundingMode::Floor);
        let r_floor = Rational::exact_from(&floor.0);
        assert!(r_floor <= n);
        if r_floor != T::ZERO {
            assert!(r_floor + Rational::exact_from(floor.0.ulp().unwrap()) > n);
        }
        let (floor_n_alt, o_alt) = Float::from_signed_prec_round(
            n,
            prec,
            if n >= T::ZERO {
                RoundingMode::Down
            } else {
                RoundingMode::Up
            },
        );
        assert_eq!(
            ComparableFloatRef(&floor_n_alt),
            ComparableFloatRef(&floor.0)
        );
        assert_eq!(o_alt, floor.1);

        let ceiling = Float::from_signed_prec_round(n, prec, RoundingMode::Ceiling);
        let r_ceiling = Rational::exact_from(&ceiling.0);
        assert!(r_ceiling >= n);
        if r_ceiling != T::ZERO {
            assert!(r_ceiling - Rational::exact_from(ceiling.0.ulp().unwrap()) < n);
        }
        let (ceiling_n_alt, o_alt) = Float::from_signed_prec_round(
            n,
            prec,
            if n >= T::ZERO {
                RoundingMode::Up
            } else {
                RoundingMode::Down
            },
        );
        assert_eq!(
            ComparableFloatRef(&ceiling_n_alt),
            ComparableFloatRef(&ceiling.0)
        );
        assert_eq!(o_alt, ceiling.1);

        let nearest = Float::from_signed_prec_round(n, prec, RoundingMode::Nearest);
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
