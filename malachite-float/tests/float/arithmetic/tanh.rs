// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{PowerOf2, Tanh, TanhAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::tanh::{
    primitive_float_tanh, primitive_float_tanh_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::tanh::{
    rug_tanh, rug_tanh_prec, rug_tanh_prec_round, rug_tanh_rational_prec,
    rug_tanh_rational_prec_round, rug_tanh_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_36,
    rational_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_tanh_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (c, o) = x.clone().tanh_prec_round(prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.tanh_prec_round_ref(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.tanh_prec_round_assign(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_tanh_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test(
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        Equal,
    );
    test("0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Nearest, "-0.0", "-0x0.0", Equal);
    test(
        "Infinity",
        "Infinity",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    // - max(3, k + 1) exceeds half the working precision, so the loop retries
    test(
        "-Infinity",
        "-Infinity",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test("1.0", "0x1.0#1", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1.0", "0x1.0#1", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1.0", "0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1.0", "0x1.0#1", 10, Floor, "0.76074", "0x0.c2c#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "0.76172",
        "0x0.c30#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "0.76172",
        "0x0.c30#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Floor,
        "0.76159415595576488811945828260469",
        "0x0.c2f7d5a8a79ca2ac3195f149e#100",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Ceiling,
        "0.76159415595576488811945828260548",
        "0x0.c2f7d5a8a79ca2ac3195f149f#100",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Nearest,
        "0.76159415595576488811945828260469",
        "0x0.c2f7d5a8a79ca2ac3195f149e#100",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Floor,
        "-0.76172",
        "-0x0.c30#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Ceiling,
        "-0.76074",
        "-0x0.c2c#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-0.76172",
        "-0x0.c30#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Down,
        "-0.76074",
        "-0x0.c2c#10",
        Greater,
    );
    test("-1.0", "-0x1.0#1", 10, Up, "-0.76172", "-0x0.c30#10", Less);
    test(
        "-1.0",
        "-0x1.0#1",
        100,
        Floor,
        "-0.76159415595576488811945828260548",
        "-0x0.c2f7d5a8a79ca2ac3195f149f#100",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        100,
        Ceiling,
        "-0.76159415595576488811945828260469",
        "-0x0.c2f7d5a8a79ca2ac3195f149e#100",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        100,
        Nearest,
        "-0.76159415595576488811945828260469",
        "-0x0.c2f7d5a8a79ca2ac3195f149e#100",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        100,
        Down,
        "-0.76159415595576488811945828260469",
        "-0x0.c2f7d5a8a79ca2ac3195f149e#100",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        100,
        Up,
        "-0.76159415595576488811945828260548",
        "-0x0.c2f7d5a8a79ca2ac3195f149f#100",
        Less,
    );
    test("0.50", "0x0.8#1", 10, Floor, "0.46191", "0x0.764#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Ceiling,
        "0.46240",
        "0x0.766#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        10,
        Nearest,
        "0.46191",
        "0x0.764#10",
        Less,
    );
    test("0.50", "0x0.8#1", 10, Down, "0.46191", "0x0.764#10", Less);
    test("0.50", "0x0.8#1", 10, Up, "0.46240", "0x0.766#10", Greater);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Floor,
        "0.46211715726000975850231848364334",
        "0x0.764d4f5d5a2bcd944a3b887190#100",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        100,
        Ceiling,
        "0.46211715726000975850231848364373",
        "0x0.764d4f5d5a2bcd944a3b887198#100",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        100,
        Nearest,
        "0.46211715726000975850231848364373",
        "0x0.764d4f5d5a2bcd944a3b887198#100",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        100,
        Down,
        "0.46211715726000975850231848364334",
        "0x0.764d4f5d5a2bcd944a3b887190#100",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        100,
        Up,
        "0.46211715726000975850231848364373",
        "0x0.764d4f5d5a2bcd944a3b887198#100",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Floor,
        "0.32129",
        "0x0.524#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Ceiling,
        "0.32178",
        "0x0.526#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Nearest,
        "0.32129",
        "0x0.524#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Down,
        "0.32129",
        "0x0.524#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Up,
        "0.32178",
        "0x0.526#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        Floor,
        "0.32151273753163434471940622242516",
        "0x0.524ea8a4f220084cefa449a988#100",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        Ceiling,
        "0.32151273753163434471940622242555",
        "0x0.524ea8a4f220084cefa449a990#100",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        Nearest,
        "0.32151273753163434471940622242516",
        "0x0.524ea8a4f220084cefa449a988#100",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        Down,
        "0.32151273753163434471940622242516",
        "0x0.524ea8a4f220084cefa449a988#100",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        Up,
        "0.32151273753163434471940622242555",
        "0x0.524ea8a4f220084cefa449a990#100",
        Greater,
    );
    test("2.0", "0x2.0#2", 10, Floor, "0.96387", "0x0.f6c#10", Less);
    test(
        "2.0",
        "0x2.0#2",
        10,
        Ceiling,
        "0.96484",
        "0x0.f70#10",
        Greater,
    );
    test("2.0", "0x2.0#2", 10, Nearest, "0.96387", "0x0.f6c#10", Less);
    test("2.0", "0x2.0#2", 10, Down, "0.96387", "0x0.f6c#10", Less);
    test("2.0", "0x2.0#2", 10, Up, "0.96484", "0x0.f70#10", Greater);
    test(
        "2.0",
        "0x2.0#2",
        100,
        Floor,
        "0.96402758007581688394641372410087",
        "0x0.f6ca82f0de1e9e99e2197e1f4#100",
        Less,
    );
    test(
        "2.0",
        "0x2.0#2",
        100,
        Ceiling,
        "0.96402758007581688394641372410165",
        "0x0.f6ca82f0de1e9e99e2197e1f5#100",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#2",
        100,
        Nearest,
        "0.96402758007581688394641372410087",
        "0x0.f6ca82f0de1e9e99e2197e1f4#100",
        Less,
    );
    test(
        "2.0",
        "0x2.0#2",
        100,
        Down,
        "0.96402758007581688394641372410087",
        "0x0.f6ca82f0de1e9e99e2197e1f4#100",
        Less,
    );
    test(
        "2.0",
        "0x2.0#2",
        100,
        Up,
        "0.96402758007581688394641372410165",
        "0x0.f6ca82f0de1e9e99e2197e1f5#100",
        Greater,
    );
    // - the quotient rounds to 1: tanh(x) rounds from -1
    test(
        "-10.0",
        "-0xa.0#4",
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "-10.0",
        "-0xa.0#4",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
    );
    test(
        "-10.0",
        "-0xa.0#4",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "-10.0",
        "-0xa.0#4",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
    );
    test("-10.0", "-0xa.0#4", 10, Up, "-1.0000", "-0x1.000#10", Less);
    test(
        "-10.0",
        "-0xa.0#4",
        100,
        Floor,
        "-0.99999999587769276361959283713874",
        "-0x0.ffffffee4b79aaa94a2b6168a#100",
        Less,
    );
    test(
        "-10.0",
        "-0xa.0#4",
        100,
        Ceiling,
        "-0.99999999587769276361959283713795",
        "-0x0.ffffffee4b79aaa94a2b61689#100",
        Greater,
    );
    test(
        "-10.0",
        "-0xa.0#4",
        100,
        Nearest,
        "-0.99999999587769276361959283713795",
        "-0x0.ffffffee4b79aaa94a2b61689#100",
        Greater,
    );
    test(
        "-10.0",
        "-0xa.0#4",
        100,
        Down,
        "-0.99999999587769276361959283713795",
        "-0x0.ffffffee4b79aaa94a2b61689#100",
        Greater,
    );
    test(
        "-10.0",
        "-0xa.0#4",
        100,
        Up,
        "-0.99999999587769276361959283713874",
        "-0x0.ffffffee4b79aaa94a2b6168a#100",
        Less,
    );
    test("30.0", "0x1e.0#5", 10, Floor, "0.99902", "0x0.ffc#10", Less);
    test(
        "30.0",
        "0x1e.0#5",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "30.0",
        "0x1e.0#5",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test("30.0", "0x1e.0#5", 10, Down, "0.99902", "0x0.ffc#10", Less);
    test("30.0", "0x1e.0#5", 10, Up, "1.0000", "0x1.000#10", Greater);
    test(
        "30.0",
        "0x1e.0#5",
        100,
        Floor,
        "0.99999999999999999999999998248650",
        "0x0.fffffffffffffffffffffa947#100",
        Less,
    );
    test(
        "30.0",
        "0x1e.0#5",
        100,
        Ceiling,
        "0.99999999999999999999999998248729",
        "0x0.fffffffffffffffffffffa948#100",
        Greater,
    );
    test(
        "30.0",
        "0x1e.0#5",
        100,
        Nearest,
        "0.99999999999999999999999998248729",
        "0x0.fffffffffffffffffffffa948#100",
        Greater,
    );
    test(
        "30.0",
        "0x1e.0#5",
        100,
        Down,
        "0.99999999999999999999999998248650",
        "0x0.fffffffffffffffffffffa947#100",
        Less,
    );
    test(
        "30.0",
        "0x1e.0#5",
        100,
        Up,
        "0.99999999999999999999999998248729",
        "0x0.fffffffffffffffffffffa948#100",
        Greater,
    );
    test("45.0", "0x2d.0#6", 10, Floor, "0.99902", "0x0.ffc#10", Less);
    test(
        "45.0",
        "0x2d.0#6",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "45.0",
        "0x2d.0#6",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test("45.0", "0x2d.0#6", 10, Down, "0.99902", "0x0.ffc#10", Less);
    test("45.0", "0x2d.0#6", 10, Up, "1.0000", "0x1.000#10", Greater);
    // - the quotient rounds to 1, at a higher precision than with x = -10
    test(
        "45.0",
        "0x2d.0#6",
        100,
        Floor,
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        Less,
    );
    test(
        "45.0",
        "0x2d.0#6",
        100,
        Ceiling,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "45.0",
        "0x2d.0#6",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "45.0",
        "0x2d.0#6",
        100,
        Down,
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        Less,
    );
    test(
        "45.0",
        "0x2d.0#6",
        100,
        Up,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "-45.0",
        "-0x2d.0#6",
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "-45.0",
        "-0x2d.0#6",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
    );
    test(
        "-45.0",
        "-0x2d.0#6",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "-45.0",
        "-0x2d.0#6",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
    );
    test("-45.0", "-0x2d.0#6", 10, Up, "-1.0000", "-0x1.000#10", Less);
    test(
        "-45.0",
        "-0x2d.0#6",
        100,
        Floor,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Less,
    );
    test(
        "-45.0",
        "-0x2d.0#6",
        100,
        Ceiling,
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff#100",
        Greater,
    );
    test(
        "-45.0",
        "-0x2d.0#6",
        100,
        Nearest,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Less,
    );
    test(
        "-45.0",
        "-0x2d.0#6",
        100,
        Down,
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff#100",
        Greater,
    );
    test(
        "-45.0",
        "-0x2d.0#6",
        100,
        Up,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Less,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        10,
        Floor,
        "0.99609",
        "0x0.ff0#10",
        Less,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        10,
        Ceiling,
        "0.99707",
        "0x0.ff4#10",
        Greater,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        10,
        Nearest,
        "0.99609",
        "0x0.ff0#10",
        Less,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        10,
        Down,
        "0.99609",
        "0x0.ff0#10",
        Less,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        10,
        Up,
        "0.99707",
        "0x0.ff4#10",
        Greater,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        100,
        Floor,
        "0.99627207622074994426506390972887",
        "0x0.ff0bafd149407d67a50ebff02#100",
        Less,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        100,
        Ceiling,
        "0.99627207622074994426506390972966",
        "0x0.ff0bafd149407d67a50ebff03#100",
        Greater,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        100,
        Nearest,
        "0.99627207622074994426506390972887",
        "0x0.ff0bafd149407d67a50ebff02#100",
        Less,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        100,
        Down,
        "0.99627207622074994426506390972887",
        "0x0.ff0bafd149407d67a50ebff02#100",
        Less,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        100,
        Up,
        "0.99627207622074994426506390972966",
        "0x0.ff0bafd149407d67a50ebff03#100",
        Greater,
    );
    test(
        "1000.0",
        "0x3e8.0#10",
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "1000.0",
        "0x3e8.0#10",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1000.0",
        "0x3e8.0#10",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1000.0",
        "0x3e8.0#10",
        10,
        Down,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "1000.0",
        "0x3e8.0#10",
        10,
        Up,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1000.0",
        "0x3e8.0#10",
        100,
        Floor,
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        Less,
    );
    test(
        "1000.0",
        "0x3e8.0#10",
        100,
        Ceiling,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "1000.0",
        "0x3e8.0#10",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "1000.0",
        "0x3e8.0#10",
        100,
        Down,
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        Less,
    );
    test(
        "1000.0",
        "0x3e8.0#10",
        100,
        Up,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    // - exp(2x) overflows: tanh(x) rounds from 1
    test(
        "400000000.00",
        "0x17d78400.0#30",
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "400000000.00",
        "0x17d78400.0#30",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "400000000.00",
        "0x17d78400.0#30",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "400000000.00",
        "0x17d78400.0#30",
        10,
        Down,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "400000000.00",
        "0x17d78400.0#30",
        10,
        Up,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "400000000.00",
        "0x17d78400.0#30",
        100,
        Floor,
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        Less,
    );
    test(
        "400000000.00",
        "0x17d78400.0#30",
        100,
        Ceiling,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "400000000.00",
        "0x17d78400.0#30",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "400000000.00",
        "0x17d78400.0#30",
        100,
        Down,
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        Less,
    );
    test(
        "400000000.00",
        "0x17d78400.0#30",
        100,
        Up,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "-400000000.00",
        "-0x17d78400.0#30",
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "-400000000.00",
        "-0x17d78400.0#30",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
    );
    test(
        "-400000000.00",
        "-0x17d78400.0#30",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "-400000000.00",
        "-0x17d78400.0#30",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
    );
    test(
        "-400000000.00",
        "-0x17d78400.0#30",
        10,
        Up,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "-400000000.00",
        "-0x17d78400.0#30",
        100,
        Floor,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Less,
    );
    test(
        "-400000000.00",
        "-0x17d78400.0#30",
        100,
        Ceiling,
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff#100",
        Greater,
    );
    test(
        "-400000000.00",
        "-0x17d78400.0#30",
        100,
        Nearest,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Less,
    );
    test(
        "-400000000.00",
        "-0x17d78400.0#30",
        100,
        Down,
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff#100",
        Greater,
    );
    test(
        "-400000000.00",
        "-0x17d78400.0#30",
        100,
        Up,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Less,
    );
    // - x >= MAX_EXPONENT / 2: tanh(x) rounds from 1
    test(
        "600000000.00",
        "0x23c34600.0#30",
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "600000000.00",
        "0x23c34600.0#30",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "600000000.00",
        "0x23c34600.0#30",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "600000000.00",
        "0x23c34600.0#30",
        10,
        Down,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "600000000.00",
        "0x23c34600.0#30",
        10,
        Up,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "600000000.00",
        "0x23c34600.0#30",
        100,
        Floor,
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        Less,
    );
    test(
        "600000000.00",
        "0x23c34600.0#30",
        100,
        Ceiling,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "600000000.00",
        "0x23c34600.0#30",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "600000000.00",
        "0x23c34600.0#30",
        100,
        Down,
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        Less,
    );
    test(
        "600000000.00",
        "0x23c34600.0#30",
        100,
        Up,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Down,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Up,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        100,
        Floor,
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        100,
        Ceiling,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        100,
        Down,
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        100,
        Up,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "0.00098",
        "0x0.004#1",
        10,
        Floor,
        "0.00097561",
        "0x0.003ff#10",
        Less,
    );
    test(
        "-0.00098",
        "-0x0.004#1",
        10,
        Floor,
        "-0.00097656",
        "-0x0.00400#10",
        Less,
    );
    test(
        "0.00098",
        "0x0.004#1",
        10,
        Ceiling,
        "0.00097656",
        "0x0.00400#10",
        Greater,
    );
    test(
        "-0.00098",
        "-0x0.004#1",
        10,
        Ceiling,
        "-0.00097561",
        "-0x0.003ff#10",
        Greater,
    );
    test(
        "0.00098",
        "0x0.004#1",
        10,
        Nearest,
        "0.00097656",
        "0x0.00400#10",
        Greater,
    );
    test(
        "-0.00098",
        "-0x0.004#1",
        10,
        Nearest,
        "-0.00097656",
        "-0x0.00400#10",
        Less,
    );
    test(
        "0.00098",
        "0x0.004#1",
        10,
        Down,
        "0.00097561",
        "0x0.003ff#10",
        Less,
    );
    test(
        "-0.00098",
        "-0x0.004#1",
        10,
        Down,
        "-0.00097561",
        "-0x0.003ff#10",
        Greater,
    );
    test(
        "0.00098",
        "0x0.004#1",
        10,
        Up,
        "0.00097656",
        "0x0.00400#10",
        Greater,
    );
    test(
        "-0.00098",
        "-0x0.004#1",
        10,
        Up,
        "-0.00097656",
        "-0x0.00400#10",
        Less,
    );
    test(
        "0.00098",
        "0x0.004#1",
        100,
        Floor,
        "0.00097656218955926021858407527014620",
        "0x0.003ffffeaaaab33332fbefc0623c#100",
        Less,
    );
    test(
        "-0.00098",
        "-0x0.004#1",
        100,
        Floor,
        "-0.00097656218955926021858407527014697",
        "-0x0.003ffffeaaaab33332fbefc06240#100",
        Less,
    );
    test(
        "0.00098",
        "0x0.004#1",
        100,
        Ceiling,
        "0.00097656218955926021858407527014697",
        "0x0.003ffffeaaaab33332fbefc06240#100",
        Greater,
    );
    test(
        "-0.00098",
        "-0x0.004#1",
        100,
        Ceiling,
        "-0.00097656218955926021858407527014620",
        "-0x0.003ffffeaaaab33332fbefc0623c#100",
        Greater,
    );
    test(
        "0.00098",
        "0x0.004#1",
        100,
        Nearest,
        "0.00097656218955926021858407527014697",
        "0x0.003ffffeaaaab33332fbefc06240#100",
        Greater,
    );
    test(
        "-0.00098",
        "-0x0.004#1",
        100,
        Nearest,
        "-0.00097656218955926021858407527014697",
        "-0x0.003ffffeaaaab33332fbefc06240#100",
        Less,
    );
    test(
        "0.00098",
        "0x0.004#1",
        100,
        Down,
        "0.00097656218955926021858407527014620",
        "0x0.003ffffeaaaab33332fbefc0623c#100",
        Less,
    );
    test(
        "-0.00098",
        "-0x0.004#1",
        100,
        Down,
        "-0.00097656218955926021858407527014620",
        "-0x0.003ffffeaaaab33332fbefc0623c#100",
        Greater,
    );
    test(
        "0.00098",
        "0x0.004#1",
        100,
        Up,
        "0.00097656218955926021858407527014697",
        "0x0.003ffffeaaaab33332fbefc06240#100",
        Greater,
    );
    test(
        "-0.00098",
        "-0x0.004#1",
        100,
        Up,
        "-0.00097656218955926021858407527014697",
        "-0x0.003ffffeaaaab33332fbefc06240#100",
        Less,
    );
    test(
        "8.9e-16",
        "0x4.0E-13#1",
        100,
        Floor,
        "8.8817841970012523233890533447196e-16",
        "0x3.ffffffffffffffffffffffffcE-13#100",
        Less,
    );
    test(
        "-8.9e-16",
        "-0x4.0E-13#1",
        100,
        Floor,
        "-8.8817841970012523233890533447266e-16",
        "-0x4.0000000000000000000000000E-13#100",
        Less,
    );
    test(
        "8.9e-16",
        "0x4.0E-13#1",
        100,
        Ceiling,
        "8.8817841970012523233890533447266e-16",
        "0x4.0000000000000000000000000E-13#100",
        Greater,
    );
    test(
        "-8.9e-16",
        "-0x4.0E-13#1",
        100,
        Ceiling,
        "-8.8817841970012523233890533447196e-16",
        "-0x3.ffffffffffffffffffffffffcE-13#100",
        Greater,
    );
    test(
        "8.9e-16",
        "0x4.0E-13#1",
        100,
        Nearest,
        "8.8817841970012523233890533447266e-16",
        "0x4.0000000000000000000000000E-13#100",
        Greater,
    );
    test(
        "-8.9e-16",
        "-0x4.0E-13#1",
        100,
        Nearest,
        "-8.8817841970012523233890533447266e-16",
        "-0x4.0000000000000000000000000E-13#100",
        Less,
    );
    test(
        "8.9e-16",
        "0x4.0E-13#1",
        100,
        Down,
        "8.8817841970012523233890533447196e-16",
        "0x3.ffffffffffffffffffffffffcE-13#100",
        Less,
    );
    test(
        "-8.9e-16",
        "-0x4.0E-13#1",
        100,
        Down,
        "-8.8817841970012523233890533447196e-16",
        "-0x3.ffffffffffffffffffffffffcE-13#100",
        Greater,
    );
    test(
        "8.9e-16",
        "0x4.0E-13#1",
        100,
        Up,
        "8.8817841970012523233890533447266e-16",
        "0x4.0000000000000000000000000E-13#100",
        Greater,
    );
    test(
        "-8.9e-16",
        "-0x4.0E-13#1",
        100,
        Up,
        "-8.8817841970012523233890533447266e-16",
        "-0x4.0000000000000000000000000E-13#100",
        Less,
    );
    // - small_input_shortcut returns
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        10,
        Floor,
        "7.8809e-31",
        "0xf.fcE-26#10",
        Less,
    );
    test(
        "-7.9e-31",
        "-0x1.0E-25#1",
        10,
        Floor,
        "-7.8886e-31",
        "-0x1.000E-25#10",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        10,
        Ceiling,
        "7.8886e-31",
        "0x1.000E-25#10",
        Greater,
    );
    test(
        "-7.9e-31",
        "-0x1.0E-25#1",
        10,
        Ceiling,
        "-7.8809e-31",
        "-0xf.fcE-26#10",
        Greater,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        10,
        Nearest,
        "7.8886e-31",
        "0x1.000E-25#10",
        Greater,
    );
    test(
        "-7.9e-31",
        "-0x1.0E-25#1",
        10,
        Nearest,
        "-7.8886e-31",
        "-0x1.000E-25#10",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        10,
        Down,
        "7.8809e-31",
        "0xf.fcE-26#10",
        Less,
    );
    test(
        "-7.9e-31",
        "-0x1.0E-25#1",
        10,
        Down,
        "-7.8809e-31",
        "-0xf.fcE-26#10",
        Greater,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        10,
        Up,
        "7.8886e-31",
        "0x1.000E-25#10",
        Greater,
    );
    test(
        "-7.9e-31",
        "-0x1.0E-25#1",
        10,
        Up,
        "-7.8886e-31",
        "-0x1.000E-25#10",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        300,
        Floor,
        "7.888609052210118054117285652827862296732064351090230047702787670276136567424482301409348\
        3353e-31",
        "0xf.fffffffffffffffffffffffffffffffffffffffffffffffffaaaaaaaaaaaaaaaaaaaaaaaaaE-26#300",
        Less,
    );
    test(
        "-7.9e-31",
        "-0x1.0E-25#1",
        300,
        Floor,
        "-7.88860905221011805411728565282786229673206435109023004770278767027613656742448230140934\
        83392e-31",
        "-0xf.fffffffffffffffffffffffffffffffffffffffffffffffffaaaaaaaaaaaaaaaaaaaaaaaabE-26#300",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        300,
        Ceiling,
        "7.888609052210118054117285652827862296732064351090230047702787670276136567424482301409348\
        3392e-31",
        "0xf.fffffffffffffffffffffffffffffffffffffffffffffffffaaaaaaaaaaaaaaaaaaaaaaaabE-26#300",
        Greater,
    );
    test(
        "-7.9e-31",
        "-0x1.0E-25#1",
        300,
        Ceiling,
        "-7.88860905221011805411728565282786229673206435109023004770278767027613656742448230140934\
        83353e-31",
        "-0xf.fffffffffffffffffffffffffffffffffffffffffffffffffaaaaaaaaaaaaaaaaaaaaaaaaaE-26#300",
        Greater,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        300,
        Nearest,
        "7.888609052210118054117285652827862296732064351090230047702787670276136567424482301409348\
        3392e-31",
        "0xf.fffffffffffffffffffffffffffffffffffffffffffffffffaaaaaaaaaaaaaaaaaaaaaaaabE-26#300",
        Greater,
    );
    test(
        "-7.9e-31",
        "-0x1.0E-25#1",
        300,
        Nearest,
        "-7.88860905221011805411728565282786229673206435109023004770278767027613656742448230140934\
        83392e-31",
        "-0xf.fffffffffffffffffffffffffffffffffffffffffffffffffaaaaaaaaaaaaaaaaaaaaaaaabE-26#300",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        300,
        Down,
        "7.888609052210118054117285652827862296732064351090230047702787670276136567424482301409348\
        3353e-31",
        "0xf.fffffffffffffffffffffffffffffffffffffffffffffffffaaaaaaaaaaaaaaaaaaaaaaaaaE-26#300",
        Less,
    );
    test(
        "-7.9e-31",
        "-0x1.0E-25#1",
        300,
        Down,
        "-7.88860905221011805411728565282786229673206435109023004770278767027613656742448230140934\
        83353e-31",
        "-0xf.fffffffffffffffffffffffffffffffffffffffffffffffffaaaaaaaaaaaaaaaaaaaaaaaaaE-26#300",
        Greater,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        300,
        Up,
        "7.888609052210118054117285652827862296732064351090230047702787670276136567424482301409348\
        3392e-31",
        "0xf.fffffffffffffffffffffffffffffffffffffffffffffffffaaaaaaaaaaaaaaaaaaaaaaaabE-26#300",
        Greater,
    );
    test(
        "-7.9e-31",
        "-0x1.0E-25#1",
        300,
        Up,
        "-7.88860905221011805411728565282786229673206435109023004770278767027613656742448230140934\
        83392e-31",
        "-0xf.fffffffffffffffffffffffffffffffffffffffffffffffffaaaaaaaaaaaaaaaaaaaaaaaabE-26#300",
        Less,
    );
    test(
        "9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Floor,
        "9.0861e-13",
        "0xf.fcE-11#10",
        Less,
    );
    test(
        "-9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "-0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Floor,
        "-9.0949e-13",
        "-0x1.000E-10#10",
        Less,
    );
    test(
        "9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Ceiling,
        "9.0949e-13",
        "0x1.000E-10#10",
        Greater,
    );
    test(
        "-9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "-0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Ceiling,
        "-9.0861e-13",
        "-0xf.fcE-11#10",
        Greater,
    );
    test(
        "9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Nearest,
        "9.0949e-13",
        "0x1.000E-10#10",
        Greater,
    );
    test(
        "-9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "-0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Nearest,
        "-9.0949e-13",
        "-0x1.000E-10#10",
        Less,
    );
    test(
        "9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Down,
        "9.0861e-13",
        "0xf.fcE-11#10",
        Less,
    );
    test(
        "-9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "-0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Down,
        "-9.0861e-13",
        "-0xf.fcE-11#10",
        Greater,
    );
    test(
        "9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Up,
        "9.0949e-13",
        "0x1.000E-10#10",
        Greater,
    );
    test(
        "-9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "-0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Up,
        "-9.0949e-13",
        "-0x1.000E-10#10",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        64,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        64,
        Floor,
        "-2.38256490488795107322e-323228497",
        "-0x1.0000000000000000E-268435456#64",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        64,
        Ceiling,
        "2.38256490488795107322e-323228497",
        "0x1.0000000000000000E-268435456#64",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        64,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        64,
        Nearest,
        "2.38256490488795107322e-323228497",
        "0x1.0000000000000000E-268435456#64",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        64,
        Nearest,
        "-2.38256490488795107322e-323228497",
        "-0x1.0000000000000000E-268435456#64",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        64,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        64,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        64,
        Up,
        "2.38256490488795107322e-323228497",
        "0x1.0000000000000000E-268435456#64",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        64,
        Up,
        "-2.38256490488795107322e-323228497",
        "-0x1.0000000000000000E-268435456#64",
        Less,
    );
    // - the first Ziv iteration cannot round, so the loop retries
    test("0.25", "0x0.4#1", 2, Floor, "0.19", "0x0.3#2", Less);
    test("0.25", "0x0.4#1", 2, Ceiling, "0.25", "0x0.4#2", Greater);
    test("0.25", "0x0.4#1", 2, Nearest, "0.25", "0x0.4#2", Greater);
    test("0.25", "0x0.4#1", 2, Up, "0.25", "0x0.4#2", Greater);
}

#[test]
#[should_panic]
fn tanh_prec_round_fail() {
    Float::ONE.tanh_prec_round(0, Nearest);
}

#[test]
#[should_panic]
fn tanh_prec_round_exact_fail() {
    Float::ONE.tanh_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn tanh_prec_fail() {
    Float::ONE.tanh_prec(0);
}

#[test]
#[should_panic]
fn tanh_round_fail() {
    Float::ONE.tanh_round(Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn tanh_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (c, o) = x.clone().tanh_prec_round(prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = x.tanh_prec_round_ref(prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.tanh_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_tanh_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    // tanh is odd
    let (c_neg, o_neg) = (-&x).tanh_prec_round(prec, -rm);
    assert_eq!(ComparableFloatRef(&c_neg), ComparableFloatRef(&-&c));
    assert_eq!(o_neg, o.reverse());

    // |tanh x| <= 1, |tanh x| <= |x|, and tanh x has the sign of x
    if !x.is_nan() {
        assert!(c.le_abs(&1u32));
        assert!(c.le_abs(&x) || c.get_prec() < x.get_prec());
        assert_eq!(c.is_sign_positive(), x.is_sign_positive());
    }
    if c.is_normal() {
        assert_eq!(c.get_prec(), Some(prec));
    }

    if o == Equal {
        // tanh is exact only for x = ±0 (and NaN, and ±inf): the result is
        // rounding-mode-invariant
        for rm2 in exhaustive_rounding_modes() {
            let (c2, o2) = x.tanh_prec_round_ref(prec, rm2);
            assert_eq!(ComparableFloat(c2), ComparableFloat(c.clone()));
            assert_eq!(o2, Equal);
        }
    } else {
        assert_panic!(x.tanh_prec_round_ref(prec, Exact));
    }
}

#[test]
fn tanh_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        tanh_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (c, o) = Float::NAN.tanh_prec_round(prec, rm);
        assert!(c.is_nan());
        assert_eq!(o, Equal);

        let (c, o) = Float::INFINITY.tanh_prec_round(prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o, Equal);

        let (c, o) = Float::NEGATIVE_INFINITY.tanh_prec_round(prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(-Float::one_prec(prec)));
        assert_eq!(o, Equal);

        let (c, o) = Float::ZERO.tanh_prec_round(prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (c, o) = Float::NEGATIVE_ZERO.tanh_prec_round(prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
    });
}

#[test]
fn tanh_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        let (c, o) = x.clone().tanh_round(rm);
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&c, rm, o);
        let (c_alt, o_alt) = x.tanh_round_ref(rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.tanh_round_assign(rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (c_alt, o_alt) = x.tanh_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_tanh_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    });
}

#[test]
fn tanh_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (c, o) = x.clone().tanh_prec(prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.tanh_prec_ref(prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.tanh_prec_assign(prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (c_alt, o_alt) = x.tanh_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (rug_c, rug_o) = rug_tanh_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn tanh_properties() {
    float_gen().test_properties(|x| {
        let c = x.clone().tanh();
        assert!(c.is_valid());
        let c_alt = (&x).tanh();
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

        let mut x_alt = x.clone();
        x_alt.tanh_assign();
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));

        let c_alt = x.tanh_prec_round_ref(x.significant_bits(), Nearest).0;
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_tanh(&rug::Float::exact_from(&x)))),
            ComparableFloatRef(&c)
        );

        assert_eq!(ComparableFloatRef(&(-&x).tanh()), ComparableFloatRef(&-&c));
        if !x.is_nan() {
            assert_eq!(c.is_sign_positive(), x.is_sign_positive());
        }
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_tanh() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_tanh(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, 1.0);
    test::<f32>(f32::NEGATIVE_INFINITY, -1.0);
    test::<f32>(0.0, 0.0);
    test::<f32>(-0.0, -0.0);
    test::<f32>(1.0, 0.7615942);
    test::<f32>(-1.0, -0.7615942);
    test::<f32>(0.5, 0.46211717);
    test::<f32>(2.0, 0.9640276);
    test::<f32>(9.0, 0.99999994);
    test::<f32>(10.0, 1.0);
    test::<f32>(20.0, 1.0);
    test::<f32>(1.0e-20, 1.0e-20);
    test::<f32>(1.0e-45, 1.0e-45);
    test::<f32>(3.4028235e38, 1.0);

    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, 1.0);
    test::<f64>(f64::NEGATIVE_INFINITY, -1.0);
    test::<f64>(0.0, 0.0);
    test::<f64>(-0.0, -0.0);
    test::<f64>(1.0, 0.7615941559557649);
    test::<f64>(-1.0, -0.7615941559557649);
    test::<f64>(0.5, 0.46211715726000974);
    test::<f64>(2.0, 0.9640275800758169);
    test::<f64>(19.0, 0.9999999999999999);
    test::<f64>(20.0, 1.0);
    test::<f64>(40.0, 1.0);
    test::<f64>(1.0e-200, 1.0e-200);
    test::<f64>(5.0e-324, 5.0e-324);
    test::<f64>(1.7976931348623157e308, 1.0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_tanh_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let c = primitive_float_tanh(x);
        // tanh is NaN exactly for NaN inputs, and otherwise has the sign of x, |tanh x| <= 1, and
        // |tanh x| <= |x|.
        assert_eq!(c.is_nan(), x.is_nan());
        if !x.is_nan() {
            assert_eq!(c.is_sign_positive(), x.is_sign_positive());
            assert!(c.abs() <= T::ONE);
            assert!(c.abs() <= x.abs());
            // tanh is odd
            assert_eq!(NiceFloat(primitive_float_tanh(-x)), NiceFloat(-c));
        }
        if x.is_finite() {
            // the result is the correctly rounded hyperbolic cosine, as computed by MPFR at the
            // same precision. MPFR's exponent range is wider than T's, so a result beyond T's
            // largest finite value overflows to infinity.
            let rug_x = rug::Float::with_val(
                u32::exact_from(T::MANTISSA_WIDTH + 1),
                &rug::Float::exact_from(&Float::from(x)),
            );
            let rug_c = <Float as From<&rug::Float>>::from(&rug_x.tanh());
            let expected = if rug_c > T::MAX_FINITE {
                T::INFINITY
            } else if rug_c < -T::MAX_FINITE {
                T::NEGATIVE_INFINITY
            } else {
                T::exact_from(&rug_c)
            };
            assert_eq!(NiceFloat(expected), NiceFloat(c));
        }
    });
}

#[test]
fn primitive_float_tanh_properties() {
    apply_fn_to_primitive_floats!(primitive_float_tanh_properties_helper);
}

// x = 2000 at precision 5750: the quotient rounds to 1, but 1 - tanh(x) < 2^(1 - err), with err =
// floor(2x) + 7 floor(floor(2x) / 16) = 5750, does not certify that it is below half an ulp, so
// tanh(x) is computed from expm1. (The result has about 1700 decimal digits, so instead of a unit
// row, every rounding mode is checked against MPFR.)
#[test]
fn test_tanh_near_one_fallback() {
    let x = Float::from(2000u32);
    for rm in [Floor, Ceiling, Down, Up, Nearest] {
        let (c, o) = x.tanh_prec_round_ref(5750, rm);
        assert!(c.is_valid());
        assert_eq!(c.get_prec(), Some(5750));
        let (rug_c, rug_o) = rug_tanh_prec_round(
            &rug::Float::exact_from(&x),
            5750,
            rug_round_try_from_rounding_mode(rm).unwrap(),
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
        let (c_neg, o_neg) = (-&x).tanh_prec_round_ref(5750, -rm);
        assert_eq!(ComparableFloatRef(&c_neg), ComparableFloatRef(&-c));
        assert_eq!(o_neg, o.reverse());
    }
}

#[test]
fn test_tanh_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::tanh_rational_prec_round(x.clone(), prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);

        let (c, o) = Float::tanh_rational_prec_round_ref(&x, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_tanh_rational_prec_round(&x, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 1, Down, "0.0", "0x0.0", Equal);
    test("0", 10, Down, "0.0", "0x0.0", Equal);
    test("0", 1, Up, "0.0", "0x0.0", Equal);
    test("0", 10, Up, "0.0", "0x0.0", Equal);
    test("0", 1, Floor, "0.0", "0x0.0", Equal);
    test("0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0", 1, Exact, "0.0", "0x0.0", Equal);
    // - the first bracket of x rounds the same way at both ends
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
    test("3/5", 1, Floor, "0.50", "0x0.8#1", Less);
    test("3/5", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("3/5", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("3/5", 10, Floor, "0.53613", "0x0.894#10", Less);
    test("3/5", 10, Ceiling, "0.53711", "0x0.898#10", Greater);
    test("3/5", 10, Nearest, "0.53711", "0x0.898#10", Greater);
    test(
        "3/5",
        100,
        Floor,
        "0.53704956699803528586182530492643",
        "0x0.897c14969667df084085e1470#100",
        Less,
    );
    test(
        "3/5",
        100,
        Ceiling,
        "0.53704956699803528586182530492722",
        "0x0.897c14969667df084085e1471#100",
        Greater,
    );
    test(
        "3/5",
        100,
        Nearest,
        "0.53704956699803528586182530492722",
        "0x0.897c14969667df084085e1471#100",
        Greater,
    );
    test("-3/5", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-3/5", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("-3/5", 1, Nearest, "-0.50", "-0x0.8#1", Greater);
    test("-3/5", 10, Floor, "-0.53711", "-0x0.898#10", Less);
    test("-3/5", 10, Ceiling, "-0.53613", "-0x0.894#10", Greater);
    test("-3/5", 10, Nearest, "-0.53711", "-0x0.898#10", Less);
    test(
        "-3/5",
        100,
        Floor,
        "-0.53704956699803528586182530492722",
        "-0x0.897c14969667df084085e1471#100",
        Less,
    );
    test(
        "-3/5",
        100,
        Ceiling,
        "-0.53704956699803528586182530492643",
        "-0x0.897c14969667df084085e1470#100",
        Greater,
    );
    test(
        "-3/5",
        100,
        Nearest,
        "-0.53704956699803528586182530492722",
        "-0x0.897c14969667df084085e1471#100",
        Less,
    );
    test("1/3", 1, Floor, "0.25", "0x0.4#1", Less);
    test("1/3", 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("1/3", 1, Nearest, "0.25", "0x0.4#1", Less);
    test("1/3", 10, Floor, "0.32129", "0x0.524#10", Less);
    test("1/3", 10, Ceiling, "0.32178", "0x0.526#10", Greater);
    test("1/3", 10, Nearest, "0.32129", "0x0.524#10", Less);
    test(
        "1/3",
        100,
        Floor,
        "0.32151273753163434471940622242516",
        "0x0.524ea8a4f220084cefa449a988#100",
        Less,
    );
    test(
        "1/3",
        100,
        Ceiling,
        "0.32151273753163434471940622242555",
        "0x0.524ea8a4f220084cefa449a990#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "0.32151273753163434471940622242516",
        "0x0.524ea8a4f220084cefa449a988#100",
        Less,
    );
    test("22/7", 1, Floor, "0.50", "0x0.8#1", Less);
    test("22/7", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("22/7", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("22/7", 10, Floor, "0.99609", "0x0.ff0#10", Less);
    test("22/7", 10, Ceiling, "0.99707", "0x0.ff4#10", Greater);
    test("22/7", 10, Nearest, "0.99609", "0x0.ff0#10", Less);
    test(
        "22/7",
        100,
        Floor,
        "0.99628147464192520836125220785407",
        "0x0.ff0c4d7f329dea0cf862f9f9c#100",
        Less,
    );
    test(
        "22/7",
        100,
        Ceiling,
        "0.99628147464192520836125220785486",
        "0x0.ff0c4d7f329dea0cf862f9f9d#100",
        Greater,
    );
    test(
        "22/7",
        100,
        Nearest,
        "0.99628147464192520836125220785407",
        "0x0.ff0c4d7f329dea0cf862f9f9c#100",
        Less,
    );
    test("-22/7", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-22/7", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("-22/7", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("-22/7", 10, Floor, "-0.99707", "-0x0.ff4#10", Less);
    test("-22/7", 10, Ceiling, "-0.99609", "-0x0.ff0#10", Greater);
    test("-22/7", 10, Nearest, "-0.99609", "-0x0.ff0#10", Greater);
    test(
        "-22/7",
        100,
        Floor,
        "-0.99628147464192520836125220785486",
        "-0x0.ff0c4d7f329dea0cf862f9f9d#100",
        Less,
    );
    test(
        "-22/7",
        100,
        Ceiling,
        "-0.99628147464192520836125220785407",
        "-0x0.ff0c4d7f329dea0cf862f9f9c#100",
        Greater,
    );
    // - x is exactly representable at the working precision
    test(
        "-22/7",
        100,
        Nearest,
        "-0.99628147464192520836125220785407",
        "-0x0.ff0c4d7f329dea0cf862f9f9c#100",
        Greater,
    );
    test("100", 1, Floor, "0.50", "0x0.8#1", Less);
    test("100", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("100", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("100", 10, Floor, "0.99902", "0x0.ffc#10", Less);
    test("100", 10, Ceiling, "1.0000", "0x1.000#10", Greater);
    test("100", 10, Nearest, "1.0000", "0x1.000#10", Greater);
    test(
        "100",
        100,
        Floor,
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        Less,
    );
    test(
        "100",
        100,
        Ceiling,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "100",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test("-100", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-100", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("-100", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("-100", 10, Floor, "-1.0000", "-0x1.000#10", Less);
    test("-100", 10, Ceiling, "-0.99902", "-0x0.ffc#10", Greater);
    test("-100", 10, Nearest, "-1.0000", "-0x1.000#10", Less);
    test(
        "-100",
        100,
        Floor,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Less,
    );
    test(
        "-100",
        100,
        Ceiling,
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff#100",
        Greater,
    );
    // - small x: bracketed by the series of sinh and cosh
    test(
        "-100",
        100,
        Nearest,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Less,
    );
    test("1/1000", 1, Floor, "0.00098", "0x0.004#1", Less);
    test("1/1000", 1, Ceiling, "0.0020", "0x0.008#1", Greater);
    test("1/1000", 1, Nearest, "0.00098", "0x0.004#1", Less);
    test("1/1000", 10, Floor, "0.00099945", "0x0.00418#10", Less);
    test("1/1000", 10, Ceiling, "0.0010014", "0x0.0041a#10", Greater);
    test("1/1000", 10, Nearest, "0.00099945", "0x0.00418#10", Less);
    test(
        "1/1000",
        100,
        Floor,
        "0.00099999966666679999994603176789982",
        "0x0.00418935dd45b387f1c1d2a28948#100",
        Less,
    );
    test(
        "1/1000",
        100,
        Ceiling,
        "0.00099999966666679999994603176790136",
        "0x0.00418935dd45b387f1c1d2a28950#100",
        Greater,
    );
    test(
        "1/1000",
        100,
        Nearest,
        "0.00099999966666679999994603176790136",
        "0x0.00418935dd45b387f1c1d2a28950#100",
        Greater,
    );
    test("-1/1000000", 1, Floor, "-1.9e-6", "-0x0.00002#1", Less);
    test("-1/1000000", 1, Ceiling, "-9.5e-7", "-0x0.00001#1", Greater);
    test("-1/1000000", 1, Nearest, "-9.5e-7", "-0x0.00001#1", Greater);
    test(
        "-1/1000000",
        10,
        Floor,
        "-1.0002e-6",
        "-0x0.000010c8#10",
        Less,
    );
    test(
        "-1/1000000",
        10,
        Ceiling,
        "-9.9838e-7",
        "-0x0.000010c0#10",
        Greater,
    );
    test(
        "-1/1000000",
        10,
        Nearest,
        "-1.0002e-6",
        "-0x0.000010c8#10",
        Less,
    );
    test(
        "-1/1000000",
        100,
        Floor,
        "-9.9999999999966666666666680000067e-7",
        "-0x0.000010c6f7a0b5e767176ed734c452#100",
        Less,
    );
    test(
        "-1/1000000",
        100,
        Ceiling,
        "-9.9999999999966666666666679999917e-7",
        "-0x0.000010c6f7a0b5e767176ed734c450#100",
        Greater,
    );
    test(
        "-1/1000000",
        100,
        Nearest,
        "-9.9999999999966666666666680000067e-7",
        "-0x0.000010c6f7a0b5e767176ed734c452#100",
        Less,
    );
    test(
        "1/100000000000000000000",
        1,
        Floor,
        "6.8e-21",
        "0x2.0E-17#1",
        Less,
    );
    test(
        "1/100000000000000000000",
        1,
        Ceiling,
        "1.4e-20",
        "0x4.0E-17#1",
        Greater,
    );
    test(
        "1/100000000000000000000",
        1,
        Nearest,
        "6.8e-21",
        "0x2.0E-17#1",
        Less,
    );
    test(
        "1/100000000000000000000",
        10,
        Floor,
        "9.9923e-21",
        "0x2.f3E-17#10",
        Less,
    );
    test(
        "1/100000000000000000000",
        10,
        Ceiling,
        "1.0006e-20",
        "0x2.f4E-17#10",
        Greater,
    );
    test(
        "1/100000000000000000000",
        10,
        Nearest,
        "1.0006e-20",
        "0x2.f4E-17#10",
        Greater,
    );
    test(
        "1/100000000000000000000",
        100,
        Floor,
        "9.9999999999999999999999999999955e-21",
        "0x2.f394219248446baa23d2ec728E-17#100",
        Less,
    );
    test(
        "1/100000000000000000000",
        100,
        Ceiling,
        "1.0000000000000000000000000000006e-20",
        "0x2.f394219248446baa23d2ec72cE-17#100",
        Greater,
    );
    // - the first series bracket straddles a rounding boundary, so the series is refined
    test(
        "1/100000000000000000000",
        100,
        Nearest,
        "9.9999999999999999999999999999955e-21",
        "0x2.f394219248446baa23d2ec728E-17#100",
        Less,
    );
    test(
        "-108086391056891904/45671926166590716193865151022383844364247891967",
        1,
        Floor,
        "-3.2e-30",
        "-0x4.0E-25#1",
        Less,
    );
    test(
        "-108086391056891904/45671926166590716193865151022383844364247891967",
        1,
        Ceiling,
        "-1.6e-30",
        "-0x2.0E-25#1",
        Greater,
    );
    test(
        "-108086391056891904/45671926166590716193865151022383844364247891967",
        1,
        Nearest,
        "-3.2e-30",
        "-0x4.0E-25#1",
        Less,
    );
    test(
        "-108086391056891904/45671926166590716193865151022383844364247891967",
        10,
        Floor,
        "-2.3697e-30",
        "-0x3.01E-25#10",
        Less,
    );
    test(
        "-108086391056891904/45671926166590716193865151022383844364247891967",
        10,
        Ceiling,
        "-2.3666e-30",
        "-0x3.00E-25#10",
        Greater,
    );
    test(
        "-108086391056891904/45671926166590716193865151022383844364247891967",
        10,
        Nearest,
        "-2.3666e-30",
        "-0x3.00E-25#10",
        Greater,
    );
    test(
        "-108086391056891904/45671926166590716193865151022383844364247891967",
        100,
        Floor,
        "-2.3665827156630354162351856958508e-30",
        "-0x3.0000000000000000000000004E-25#100",
        Less,
    );
    test(
        "-108086391056891904/45671926166590716193865151022383844364247891967",
        100,
        Ceiling,
        "-2.3665827156630354162351856958484e-30",
        "-0x3.0000000000000000000000000E-25#100",
        Greater,
    );
    test(
        "-108086391056891904/45671926166590716193865151022383844364247891967",
        100,
        Nearest,
        "-2.3665827156630354162351856958484e-30",
        "-0x3.0000000000000000000000000E-25#100",
        Greater,
    );
    test("1488522235/2", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1488522235/2", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1488522235/2", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1488522235/2", 10, Floor, "0.99902", "0x0.ffc#10", Less);
    test("1488522235/2", 10, Ceiling, "1.0000", "0x1.000#10", Greater);
    test("1488522235/2", 10, Nearest, "1.0000", "0x1.000#10", Greater);
    test(
        "1488522235/2",
        100,
        Floor,
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        Less,
    );
    test(
        "1488522235/2",
        100,
        Ceiling,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "1488522235/2",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test("7442611177/10", 1, Floor, "0.50", "0x0.8#1", Less);
    test("7442611177/10", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("7442611177/10", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("7442611177/10", 10, Floor, "0.99902", "0x0.ffc#10", Less);
    test(
        "7442611177/10",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "7442611177/10",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "7442611177/10",
        100,
        Floor,
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        Less,
    );
    test(
        "7442611177/10",
        100,
        Ceiling,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "7442611177/10",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test("-10000000000", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-10000000000", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("-10000000000", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("-10000000000", 10, Floor, "-1.0000", "-0x1.000#10", Less);
    test(
        "-10000000000",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
    );
    test("-10000000000", 10, Nearest, "-1.0000", "-0x1.000#10", Less);
    test(
        "-10000000000",
        100,
        Floor,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Less,
    );
    test(
        "-10000000000",
        100,
        Ceiling,
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff#100",
        Greater,
    );
    test(
        "-10000000000",
        100,
        Nearest,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Less,
    );
    test(
        "1/1000",
        100,
        Down,
        "0.00099999966666679999994603176789982",
        "0x0.00418935dd45b387f1c1d2a28948#100",
        Less,
    );
    test(
        "-108086391056891904/45671926166590716193865151022383844364247891967",
        100,
        Down,
        "-2.3665827156630354162351856958484e-30",
        "-0x3.0000000000000000000000000E-25#100",
        Greater,
    );
    test(
        "1/1000",
        100,
        Up,
        "0.00099999966666679999994603176790136",
        "0x0.00418935dd45b387f1c1d2a28950#100",
        Greater,
    );
    test(
        "-108086391056891904/45671926166590716193865151022383844364247891967",
        100,
        Up,
        "-2.3665827156630354162351856958508e-30",
        "-0x3.0000000000000000000000004E-25#100",
        Less,
    );
    // - the bracket ends round differently, so the working precision grows
    test("11/15", 2, Floor, "0.50", "0x0.8#2", Less);
    test("11/15", 2, Ceiling, "0.75", "0x0.c#2", Greater);
    test("11/15", 2, Nearest, "0.75", "0x0.c#2", Greater);
}

#[test]
fn test_tanh_rational_extreme() {
    // |x| = 2^(-2^30 - 10) is too small to be a `Float`, and tanh(x) underflows
    let x = Rational::power_of_2(-(1i64 << 30) - 10);
    for (rm, out, o_out) in [
        (Floor, Float::ZERO, Less),
        (Down, Float::ZERO, Less),
        (Nearest, Float::ZERO, Less),
        (Ceiling, Float::min_positive_value_prec(10), Greater),
        (Up, Float::min_positive_value_prec(10), Greater),
    ] {
        let (c, o) = Float::tanh_rational_prec_round_ref(&x, 10, rm);
        assert_eq!(ComparableFloat(c.clone()), ComparableFloat(out));
        assert_eq!(o, o_out);
        let (c_neg, o_neg) = Float::tanh_rational_prec_round(-&x, 10, -rm);
        assert_eq!(ComparableFloat(c_neg), ComparableFloat(-c));
        assert_eq!(o_neg, o.reverse());
    }

    // |x| = 2^(2^30) is too large to be a `Float`, and tanh(x) rounds from 1
    let x = Rational::power_of_2(1i64 << 30);
    let (c, o) = Float::tanh_rational_prec_round_ref(&x, 10, Nearest);
    assert_eq!(c.to_string(), "1.0000");
    assert_eq!(o, Greater);
    let (c, o) = Float::tanh_rational_prec_round(-x, 10, Ceiling);
    assert_eq!(c.to_string(), "-0.99902");
    assert_eq!(o, Greater);
}

#[test]
#[should_panic]
fn tanh_rational_prec_fail() {
    Float::tanh_rational_prec(Rational::ONE, 0);
}

#[test]
#[should_panic]
fn tanh_rational_prec_ref_fail() {
    Float::tanh_rational_prec_ref(&Rational::ONE, 0);
}

#[test]
#[should_panic]
fn tanh_rational_prec_round_fail_1() {
    Float::tanh_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn tanh_rational_prec_round_fail_2() {
    Float::tanh_rational_prec_round(Rational::ONE, 10, Exact);
}

#[test]
#[should_panic]
fn tanh_rational_prec_round_ref_fail() {
    Float::tanh_rational_prec_round_ref(&Rational::ONE, 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn tanh_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (c, o) = Float::tanh_rational_prec_round(x.clone(), prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = Float::tanh_rational_prec_round_ref(&x, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    // tanh is odd (a `Rational` has no negative zero, so x = 0 is excluded)
    if x != 0u32 {
        let (c_neg, o_neg) = Float::tanh_rational_prec_round(-&x, prec, -rm);
        assert_eq!(ComparableFloatRef(&c_neg), ComparableFloatRef(&-&c));
        assert_eq!(o_neg, o.reverse());
    }

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_tanh_rational_prec_round(&x, prec, rrm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    if c.is_normal() {
        assert_eq!(c.get_prec(), Some(prec));
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = Float::tanh_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::tanh_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn tanh_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        tanh_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (c, o) = Float::tanh_rational_prec_round(Rational::ZERO, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn tanh_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (c, o) = Float::tanh_rational_prec(x.clone(), prec);
    assert!(c.is_valid());

    let (c_alt, o_alt) = Float::tanh_rational_prec_ref(&x, prec);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let (c_alt, o_alt) = Float::tanh_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let (rug_c, rug_o) = rug_tanh_rational_prec(&x, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_c)),
        ComparableFloatRef(&c)
    );
    assert_eq!(rug_o, o);

    // the hyperbolic tangent of an exactly representable rational is the Float hyperbolic tangent
    if let Ok(f) = Float::try_from(&x) {
        let (c_alt, o_alt) = f.tanh_prec(prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    }
}

#[test]
fn tanh_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        tanh_rational_prec_properties_helper(x, prec);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_tanh_rational() {
    fn test<T: PrimitiveFloat>(x: &Rational, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_tanh_rational::<T>(x)),
            NiceFloat(out)
        );
    }
    let test_s = |s: &str| Rational::from_str(s).unwrap();
    test::<f32>(&test_s("0"), 0.0);
    test::<f32>(&test_s("1"), 0.7615942);
    test::<f32>(&test_s("-1"), -0.7615942);
    test::<f32>(&test_s("1/3"), 0.32151273);
    test::<f32>(&test_s("22/7"), 0.99628145);
    test::<f32>(&test_s("20"), 1.0);
    test::<f32>(&test_s("-20"), -1.0);
    test::<f32>(&test_s("10000"), 1.0);
    test::<f32>(&test_s("1/100000000000000000000"), 1.0e-20);
    test::<f32>(
        &test_s("1/10000000000000000000000000000000000000000"),
        1.0e-40,
    );
    test::<f32>(
        &test_s("-1/100000000000000000000000000000000000000000000000000"),
        -0.0,
    );

    test::<f64>(&test_s("0"), 0.0);
    test::<f64>(&test_s("1"), 0.7615941559557649);
    test::<f64>(&test_s("-1"), -0.7615941559557649);
    test::<f64>(&test_s("1/3"), 0.32151273753163434);
    test::<f64>(&test_s("22/7"), 0.9962814746419252);
    test::<f64>(&test_s("20"), 1.0);
    test::<f64>(&test_s("-20"), -1.0);
    test::<f64>(&test_s("10000"), 1.0);
    test::<f64>(&test_s("1/100000000000000000000"), 1.0e-20);
    test::<f64>(
        &test_s("1/10000000000000000000000000000000000000000"),
        1.0e-40,
    );
    test::<f64>(
        &test_s("-1/100000000000000000000000000000000000000000000000000"),
        -1.0e-50,
    );

    // a subnormal f64 result, and results that underflow
    let x = Rational::power_of_2(-1070i64);
    test::<f32>(&x, 0.0);
    test::<f64>(&x, 8.0e-323);
    test::<f32>(&-&x, -0.0);
    test::<f64>(&-x, -8.0e-323);
    let x = Rational::from_unsigneds(1u32, 3) >> 1100u32;
    test::<f32>(&x, 0.0);
    test::<f64>(&x, 0.0);
    test::<f32>(&-&x, -0.0);
    test::<f64>(&-x, -0.0);
    // a subnormal f32 result
    let x = Rational::from_unsigneds(1u32, 3) >> 140u32;
    test::<f32>(&x, 2.4e-43);
    test::<f64>(&x, 2.3915493791143543e-43);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_tanh_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let c = primitive_float_tanh_rational::<T>(&x);
        // the hyperbolic tangent of a rational is never NaN
        assert!(!c.is_nan());
        // tanh is odd (a `Rational` has no negative zero, so x = 0 is excluded)
        if x != 0u32 {
            assert_eq!(
                NiceFloat(primitive_float_tanh_rational::<T>(&-&x)),
                NiceFloat(-c)
            );
        }
        // the result is the correctly rounded hyperbolic tangent, as computed by MPFR with 64 bits
        // to spare, so that a subnormal result is rounded once by the conversion rather than twice
        let rug_c = rug_tanh_rational_prec(&x, T::MANTISSA_WIDTH + 64).0;
        let rug_c: T = T::rounding_from(&<Float as From<&rug::Float>>::from(&rug_c), Nearest).0;
        assert_eq!(NiceFloat(rug_c), NiceFloat(c));
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // The hyperbolic tangent of a finite nonzero primitive float, taken through the `Rational`
        // path, matches the direct primitive-float hyperbolic tangent (a `Rational` cannot carry
        // the sign of a zero).
        if x.is_finite() && x != T::ZERO {
            assert_eq!(
                NiceFloat(primitive_float_tanh_rational::<T>(&Rational::exact_from(x))),
                NiceFloat(primitive_float_tanh(x))
            );
        }
    });
}

#[test]
fn primitive_float_tanh_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_tanh_rational_properties_helper);
}
