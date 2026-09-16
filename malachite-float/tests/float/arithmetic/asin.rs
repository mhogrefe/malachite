// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Asin, AsinAssign, PowerOf2};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, OneHalf, Two, Zero,
};
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_gen, primitive_float_gen_var_1, primitive_float_unsigned_pair_gen_var_1,
    unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::asin::{
    primitive_float_asin, primitive_float_asin_pi, primitive_float_asin_pi_rational,
    primitive_float_asin_rational, primitive_float_asin_with_period,
    primitive_float_asin_with_period_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::asin::{
    rug_asin, rug_asin_prec_round, rug_asin_rational_prec_round, rug_asin_with_period_prec_round,
    rug_asin_with_period_rational_prec_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_2, float_unsigned_rounding_mode_triple_gen_var_36,
    float_unsigned_rounding_mode_triple_gen_var_37, float_unsigned_rounding_mode_triple_gen_var_41,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_21,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_22,
    float_unsigned_unsigned_triple_gen_var_1, rational_unsigned_rounding_mode_triple_gen_var_10,
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_7,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::{
    rational_gen, rational_unsigned_pair_gen_var_1, rational_unsigned_pair_gen_var_3,
};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_asin_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (t, o) = x.clone().asin_prec_round(prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = x.asin_prec_round_ref(prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        let mut t_alt = x.clone();
        let o_alt = t_alt.asin_prec_round_assign(prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = x.asin_prec_ref(prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_t, rug_o) = rug_asin_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_t)),
                ComparableFloatRef(&t)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", 53, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", 2, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Exact, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Ceiling, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Down, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Up, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 53, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 2, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Exact, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Down, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Up, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 53, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 2, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Exact, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 53, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 2, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Exact, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Down, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Up, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 53, Nearest, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Nearest, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 2, Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Exact, "-0.0", "-0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "1.5703", "0x1.920#10", Less);
    test("1.0", "0x1.0#1", 10, Floor, "1.5703", "0x1.920#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test("1.0", "0x1.0#1", 10, Down, "1.5703", "0x1.920#10", Less);
    test("1.0", "0x1.0#1", 10, Up, "1.5723", "0x1.928#10", Greater);
    test(
        "1.0",
        "0x1.0#1",
        53,
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test("1.0", "0x1.0#1", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", 2, Floor, "1.5", "0x1.8#2", Less);
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-1.5703",
        "-0x1.920#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Floor,
        "-1.5723",
        "-0x1.928#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Ceiling,
        "-1.5703",
        "-0x1.920#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Down,
        "-1.5703",
        "-0x1.920#10",
        Greater,
    );
    test("-1.0", "-0x1.0#1", 10, Up, "-1.5723", "-0x1.928#10", Less);
    test(
        "-1.0",
        "-0x1.0#1",
        53,
        Nearest,
        "-1.5707963267948966",
        "-0x1.921fb54442d18#53",
        Greater,
    );
    test("-1.0", "-0x1.0#1", 1, Nearest, "-2.0", "-0x2.0#1", Less);
    test("-1.0", "-0x1.0#1", 2, Floor, "-2.0", "-0x2.0#2", Less);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Nearest,
        "0.52344",
        "0x0.860#10",
        Less,
    );
    test("0.50", "0x0.8#1", 10, Floor, "0.52344", "0x0.860#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Ceiling,
        "0.52441",
        "0x0.864#10",
        Greater,
    );
    test("0.50", "0x0.8#1", 10, Down, "0.52344", "0x0.860#10", Less);
    test("0.50", "0x0.8#1", 10, Up, "0.52441", "0x0.864#10", Greater);
    test(
        "0.50",
        "0x0.8#1",
        53,
        Nearest,
        "0.52359877559829893",
        "0x0.860a91c16b9b30#53",
        Greater,
    );
    test("0.50", "0x0.8#1", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("0.50", "0x0.8#1", 2, Floor, "0.50", "0x0.8#2", Less);
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Nearest,
        "-0.52344",
        "-0x0.860#10",
        Greater,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Floor,
        "-0.52441",
        "-0x0.864#10",
        Less,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Ceiling,
        "-0.52344",
        "-0x0.860#10",
        Greater,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Down,
        "-0.52344",
        "-0x0.860#10",
        Greater,
    );
    test("-0.50", "-0x0.8#1", 10, Up, "-0.52441", "-0x0.864#10", Less);
    test(
        "-0.50",
        "-0x0.8#1",
        53,
        Nearest,
        "-0.52359877559829893",
        "-0x0.860a91c16b9b30#53",
        Less,
    );
    test(
        "-0.50", "-0x0.8#1", 1, Nearest, "-0.50", "-0x0.8#1", Greater,
    );
    test("-0.50", "-0x0.8#1", 2, Floor, "-0.75", "-0x0.c#2", Less);
    test(
        "0.25",
        "0x0.4#1",
        10,
        Nearest,
        "0.25244",
        "0x0.40a#10",
        Less,
    );
    test("0.25", "0x0.4#1", 10, Floor, "0.25244", "0x0.40a#10", Less);
    test(
        "0.25",
        "0x0.4#1",
        10,
        Ceiling,
        "0.25293",
        "0x0.40c#10",
        Greater,
    );
    test("0.25", "0x0.4#1", 10, Down, "0.25244", "0x0.40a#10", Less);
    test("0.25", "0x0.4#1", 10, Up, "0.25293", "0x0.40c#10", Greater);
    test(
        "0.25",
        "0x0.4#1",
        53,
        Nearest,
        "0.25268025514207865",
        "0x0.40afa7382e1f34#53",
        Less,
    );
    test("0.25", "0x0.4#1", 1, Nearest, "0.25", "0x0.4#1", Less);
    test("0.25", "0x0.4#1", 2, Floor, "0.25", "0x0.4#2", Less);
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Nearest,
        "0.10022",
        "0x0.19a8#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Floor,
        "0.10010",
        "0x0.19a0#10",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Ceiling,
        "0.10022",
        "0x0.19a8#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Down,
        "0.10010",
        "0x0.19a0#10",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Up,
        "0.10022",
        "0x0.19a8#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        53,
        Nearest,
        "0.10016742116155980",
        "0x0.19a49276037884#53",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        1,
        Nearest,
        "0.12",
        "0x0.2#1",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        2,
        Floor,
        "0.094",
        "0x0.18#2",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Nearest,
        "-0.10022",
        "-0x0.19a8#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Floor,
        "-0.10022",
        "-0x0.19a8#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Ceiling,
        "-0.10010",
        "-0x0.19a0#10",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Down,
        "-0.10010",
        "-0x0.19a0#10",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Up,
        "-0.10022",
        "-0x0.19a8#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        53,
        Nearest,
        "-0.10016742116155980",
        "-0x0.19a49276037884#53",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        1,
        Nearest,
        "-0.12",
        "-0x0.2#1",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        2,
        Floor,
        "-0.12",
        "-0x0.2#2",
        Less,
    );
    test(
        "0.75",
        "0x0.c#2",
        10,
        Nearest,
        "0.84766",
        "0x0.d90#10",
        Less,
    );
    test("0.75", "0x0.c#2", 10, Floor, "0.84766", "0x0.d90#10", Less);
    test(
        "0.75",
        "0x0.c#2",
        10,
        Ceiling,
        "0.84863",
        "0x0.d94#10",
        Greater,
    );
    test("0.75", "0x0.c#2", 10, Down, "0.84766", "0x0.d90#10", Less);
    test("0.75", "0x0.c#2", 10, Up, "0.84863", "0x0.d94#10", Greater);
    test(
        "0.75",
        "0x0.c#2",
        53,
        Nearest,
        "0.84806207898148100",
        "0x0.d91a98ae3406e0#53",
        Less,
    );
    test("0.75", "0x0.c#2", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("0.75", "0x0.c#2", 2, Floor, "0.75", "0x0.c#2", Less);
    test(
        "0.90000000000000002",
        "0x0.e6666666666668#53",
        10,
        Nearest,
        "1.1191",
        "0x1.1e8#10",
        Less,
    );
    test(
        "0.90000000000000002",
        "0x0.e6666666666668#53",
        10,
        Floor,
        "1.1191",
        "0x1.1e8#10",
        Less,
    );
    test(
        "0.90000000000000002",
        "0x0.e6666666666668#53",
        10,
        Ceiling,
        "1.1211",
        "0x1.1f0#10",
        Greater,
    );
    test(
        "0.90000000000000002",
        "0x0.e6666666666668#53",
        10,
        Down,
        "1.1191",
        "0x1.1e8#10",
        Less,
    );
    test(
        "0.90000000000000002",
        "0x0.e6666666666668#53",
        10,
        Up,
        "1.1211",
        "0x1.1f0#10",
        Greater,
    );
    test(
        "0.90000000000000002",
        "0x0.e6666666666668#53",
        53,
        Nearest,
        "1.1197695149986342",
        "0x1.1ea93705fa172#53",
        Less,
    );
    test(
        "0.90000000000000002",
        "0x0.e6666666666668#53",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "0.90000000000000002",
        "0x0.e6666666666668#53",
        2,
        Floor,
        "1.0",
        "0x1.0#2",
        Less,
    );
    test(
        "0.98999999999999999",
        "0x0.fd70a3d70a3d7#52",
        10,
        Nearest,
        "1.4297",
        "0x1.6e0#10",
        Greater,
    );
    test(
        "0.98999999999999999",
        "0x0.fd70a3d70a3d7#52",
        10,
        Floor,
        "1.4277",
        "0x1.6d8#10",
        Less,
    );
    test(
        "0.98999999999999999",
        "0x0.fd70a3d70a3d7#52",
        10,
        Ceiling,
        "1.4297",
        "0x1.6e0#10",
        Greater,
    );
    test(
        "0.98999999999999999",
        "0x0.fd70a3d70a3d7#52",
        10,
        Down,
        "1.4277",
        "0x1.6d8#10",
        Less,
    );
    test(
        "0.98999999999999999",
        "0x0.fd70a3d70a3d7#52",
        10,
        Up,
        "1.4297",
        "0x1.6e0#10",
        Greater,
    );
    test(
        "0.98999999999999999",
        "0x0.fd70a3d70a3d7#52",
        53,
        Nearest,
        "1.4292568534704693",
        "0x1.6de3c6f33d51d#53",
        Less,
    );
    test(
        "0.98999999999999999",
        "0x0.fd70a3d70a3d7#52",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "0.98999999999999999",
        "0x0.fd70a3d70a3d7#52",
        2,
        Floor,
        "1.0",
        "0x1.0#2",
        Less,
    );
    test("1.5", "0x1.8#2", 10, Nearest, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 10, Floor, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 10, Ceiling, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 10, Down, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 10, Up, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 53, Nearest, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 1, Nearest, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 2, Floor, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 10, Exact, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 10, Nearest, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 10, Floor, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 10, Down, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 10, Up, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 53, Nearest, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 1, Nearest, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 2, Floor, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 10, Exact, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Floor, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Down, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Up, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 53, Nearest, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 1, Nearest, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 2, Floor, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Exact, "NaN", "NaN", Equal);
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        10,
        Nearest,
        "0.33984",
        "0x0.570#10",
        Greater,
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        10,
        Floor,
        "0.33936",
        "0x0.56e#10",
        Less,
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        10,
        Ceiling,
        "0.33984",
        "0x0.570#10",
        Greater,
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        10,
        Down,
        "0.33936",
        "0x0.56e#10",
        Less,
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        10,
        Up,
        "0.33984",
        "0x0.570#10",
        Greater,
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        53,
        Nearest,
        "0.33983690945412193",
        "0x0.56ff8d3c144448#53",
        Greater,
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        1,
        Nearest,
        "0.25",
        "0x0.4#1",
        Less,
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        2,
        Floor,
        "0.25",
        "0x0.4#2",
        Less,
    );
    test(
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        10,
        Nearest,
        "0.78516",
        "0x0.c90#10",
        Less,
    );
    test(
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        10,
        Floor,
        "0.78516",
        "0x0.c90#10",
        Less,
    );
    test(
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        10,
        Ceiling,
        "0.78613",
        "0x0.c94#10",
        Greater,
    );
    test(
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        10,
        Down,
        "0.78516",
        "0x0.c90#10",
        Less,
    );
    test(
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        10,
        Up,
        "0.78613",
        "0x0.c94#10",
        Greater,
    );
    test(
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        53,
        Nearest,
        "0.78539816339744839",
        "0x0.c90fdaa22168c8#53",
        Greater,
    );
    test(
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        2,
        Floor,
        "0.75",
        "0x0.c#2",
        Less,
    );
    test(
        "-0.70710678118654757",
        "-0x0.b504f333f9de68#53",
        10,
        Nearest,
        "-0.78516",
        "-0x0.c90#10",
        Greater,
    );
    test(
        "-0.70710678118654757",
        "-0x0.b504f333f9de68#53",
        10,
        Floor,
        "-0.78613",
        "-0x0.c94#10",
        Less,
    );
    test(
        "-0.70710678118654757",
        "-0x0.b504f333f9de68#53",
        10,
        Ceiling,
        "-0.78516",
        "-0x0.c90#10",
        Greater,
    );
    test(
        "-0.70710678118654757",
        "-0x0.b504f333f9de68#53",
        10,
        Down,
        "-0.78516",
        "-0x0.c90#10",
        Greater,
    );
    test(
        "-0.70710678118654757",
        "-0x0.b504f333f9de68#53",
        10,
        Up,
        "-0.78613",
        "-0x0.c94#10",
        Less,
    );
    test(
        "-0.70710678118654757",
        "-0x0.b504f333f9de68#53",
        53,
        Nearest,
        "-0.78539816339744839",
        "-0x0.c90fdaa22168c8#53",
        Less,
    );
    test(
        "-0.70710678118654757",
        "-0x0.b504f333f9de68#53",
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "-0.70710678118654757",
        "-0x0.b504f333f9de68#53",
        2,
        Floor,
        "-1.0",
        "-0x1.0#2",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Nearest,
        "1.0004e-10",
        "0x6.e0E-9#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Floor,
        "9.9931e-11",
        "0x6.deE-9#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Ceiling,
        "1.0004e-10",
        "0x6.e0E-9#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Down,
        "9.9931e-11",
        "0x6.deE-9#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Up,
        "1.0004e-10",
        "0x6.e0E-9#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        53,
        Nearest,
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1,
        Nearest,
        "1.2e-10",
        "0x8.0E-9#1",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        2,
        Floor,
        "8.7e-11",
        "0x6.0E-9#2",
        Less,
    );
    test(
        "0.99999999999999999994579",
        "0x0.ffffffffffffffff00#70",
        10,
        Nearest,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "0.99999999999999999994579",
        "0x0.ffffffffffffffff00#70",
        10,
        Floor,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "0.99999999999999999994579",
        "0x0.ffffffffffffffff00#70",
        10,
        Ceiling,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "0.99999999999999999994579",
        "0x0.ffffffffffffffff00#70",
        10,
        Down,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "0.99999999999999999994579",
        "0x0.ffffffffffffffff00#70",
        10,
        Up,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "0.99999999999999999994579",
        "0x0.ffffffffffffffff00#70",
        53,
        Nearest,
        "1.5707963264656244",
        "0x1.921fb542d8c7a#53",
        Greater,
    );
    test(
        "0.99999999999999999994579",
        "0x0.ffffffffffffffff00#70",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "0.99999999999999999994579",
        "0x0.ffffffffffffffff00#70",
        2,
        Floor,
        "1.5",
        "0x1.8#2",
        Less,
    );
    test(
        "0.99902343750000000000000",
        "0x0.ffc000000000000000#70",
        10,
        Nearest,
        "1.5273",
        "0x1.870#10",
        Greater,
    );
    test(
        "0.99902343750000000000000",
        "0x0.ffc000000000000000#70",
        10,
        Floor,
        "1.5254",
        "0x1.868#10",
        Less,
    );
    test(
        "0.99902343750000000000000",
        "0x0.ffc000000000000000#70",
        10,
        Ceiling,
        "1.5273",
        "0x1.870#10",
        Greater,
    );
    test(
        "0.99902343750000000000000",
        "0x0.ffc000000000000000#70",
        10,
        Down,
        "1.5254",
        "0x1.868#10",
        Less,
    );
    test(
        "0.99902343750000000000000",
        "0x0.ffc000000000000000#70",
        10,
        Up,
        "1.5273",
        "0x1.870#10",
        Greater,
    );
    test(
        "0.99902343750000000000000",
        "0x0.ffc000000000000000#70",
        53,
        Nearest,
        "1.5265985556491812",
        "0x1.86cf29b6a2526#53",
        Less,
    );
    test(
        "0.99902343750000000000000",
        "0x0.ffc000000000000000#70",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "0.99902343750000000000000",
        "0x0.ffc000000000000000#70",
        2,
        Floor,
        "1.5",
        "0x1.8#2",
        Less,
    );
    test(
        "0.75000000000000000000000",
        "0x0.c00000000000000000#70",
        10,
        Nearest,
        "0.84766",
        "0x0.d90#10",
        Less,
    );
    test(
        "0.75000000000000000000000",
        "0x0.c00000000000000000#70",
        10,
        Floor,
        "0.84766",
        "0x0.d90#10",
        Less,
    );
    test(
        "0.75000000000000000000000",
        "0x0.c00000000000000000#70",
        10,
        Ceiling,
        "0.84863",
        "0x0.d94#10",
        Greater,
    );
    test(
        "0.75000000000000000000000",
        "0x0.c00000000000000000#70",
        10,
        Down,
        "0.84766",
        "0x0.d90#10",
        Less,
    );
    test(
        "0.75000000000000000000000",
        "0x0.c00000000000000000#70",
        10,
        Up,
        "0.84863",
        "0x0.d94#10",
        Greater,
    );
    test(
        "0.75000000000000000000000",
        "0x0.c00000000000000000#70",
        53,
        Nearest,
        "0.84806207898148100",
        "0x0.d91a98ae3406e0#53",
        Less,
    );
    test(
        "0.75000000000000000000000",
        "0x0.c00000000000000000#70",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "0.75000000000000000000000",
        "0x0.c00000000000000000#70",
        2,
        Floor,
        "0.75",
        "0x0.c#2",
        Less,
    );
}

#[test]
#[should_panic]
fn asin_prec_round_fail_1() {
    Float::ONE.asin_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn asin_prec_round_fail_2() {
    // the arcsine of a finite nonzero input is never exactly representable
    Float::from(0.5f64).asin_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn asin_prec_round_fail_3() {
    // and neither is +-pi/2
    Float::ONE.asin_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn asin_prec_fail() {
    Float::ONE.asin_prec(0);
}

#[test]
#[should_panic]
fn asin_round_fail() {
    Float::from(0.5f64).asin_round(Exact);
}

// Whether asin(x) is exactly representable: only at a zero, and where the result is NaN.
fn asin_exact(x: &Float) -> bool {
    x.is_nan() || *x == 0u32 || !x.is_finite() || PartialOrdAbs::gt_abs(x, &1u32)
}

#[allow(clippy::needless_pass_by_value)]
fn asin_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    if rm == Exact && !asin_exact(&x) {
        assert_panic!(x.asin_prec_round_ref(prec, Exact));
        return;
    }
    let (t, o) = x.clone().asin_prec_round(prec, rm);
    assert!(t.is_valid());
    assert_rounding_ordering_consistent(&t, rm, o);

    let (t_alt, o_alt) = x.asin_prec_round_ref(prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    let mut t_alt = x.clone();
    let o_alt = t_alt.asin_prec_round_assign(prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_t, rug_o) = rug_asin_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_t)),
            ComparableFloatRef(&t)
        );
        assert_eq!(rug_o, o, "x = {x} prec = {prec} rm = {rm:?}");
    }

    // NaN exactly for a NaN input and for one outside [-1, 1]
    assert_eq!(
        t.is_nan(),
        x.is_nan() || !x.is_finite() || PartialOrdAbs::gt_abs(&x, &1u32)
    );
    if !t.is_nan() {
        // |asin x| <= pi/2, so the result never overflows
        assert!(t.is_finite());
        assert!(PartialOrdAbs::le_abs(
            &t,
            &Float::from_unsigned_prec(2u32, 1).0
        ));
        if t.is_normal() {
            assert_eq!(t.get_prec(), Some(prec));
        }
        // asin is odd
        let (t_neg, o_neg) = (-&x).asin_prec_round_ref(prec, -rm);
        assert_eq!(ComparableFloatRef(&t_neg), ComparableFloatRef(&-&t));
        assert_eq!(o_neg, o.reverse());
        // and it never underflows: |asin x| > |x| for a nonzero x in range
        if x != 0u32 {
            assert!(t != 0u32);
        }
    }

    if o == Equal {
        assert!(asin_exact(&x));
        for rm in exhaustive_rounding_modes() {
            let (t2, oo) = x.asin_prec_round_ref(prec, rm);
            assert_eq!(ComparableFloatRef(&t2), ComparableFloatRef(&t));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.asin_prec_round_ref(prec, Exact));
    }
}

#[test]
fn asin_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        asin_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // asin is NaN at both infinities and everywhere outside [-1, 1], exactly
        for x in [
            Float::NAN,
            Float::INFINITY,
            Float::NEGATIVE_INFINITY,
            Float::from(2.0f64),
            Float::from(-2.0f64),
        ] {
            let (t, o) = x.asin_prec_round_ref(prec, rm);
            assert!(t.is_nan());
            assert_eq!(o, Equal);
        }
        // asin(+-0.0) = +-0.0, exactly
        let (t, o) = Float::ZERO.asin_prec_round(prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        let (t, o) = Float::NEGATIVE_ZERO.asin_prec_round(prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
    });
}

#[test]
fn asin_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (t, o) = x.clone().asin_prec(prec);
        assert!(t.is_valid());
        for (t_alt, o_alt) in [x.asin_prec_ref(prec), x.asin_prec_round_ref(prec, Nearest)] {
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
        let mut t_alt = x.clone();
        let o_alt = t_alt.asin_prec_assign(prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn asin_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        if rm == Exact && !asin_exact(&x) {
            assert_panic!(x.asin_round_ref(Exact));
            return;
        }
        let (t, o) = x.clone().asin_round(rm);
        assert!(t.is_valid());
        assert_rounding_ordering_consistent(&t, rm, o);
        for (t_alt, o_alt) in
            [x.asin_round_ref(rm), x.asin_prec_round_ref(x.significant_bits(), rm)]
        {
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
        let mut t_alt = x.clone();
        let o_alt = t_alt.asin_round_assign(rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn asin_properties() {
    float_gen().test_properties(|x| {
        let t = x.clone().asin();
        assert!(t.is_valid());
        let t_alt = (&x).asin();
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let mut t_alt = x.clone();
        t_alt.asin_assign();
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let (t_alt, _) = x.asin_prec_round_ref(x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        // asin is odd
        assert_eq!(ComparableFloatRef(&(-&x).asin()), ComparableFloatRef(&-&t));
        let rug_t = rug_asin(&rug::Float::exact_from(&x));
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_t)),
            ComparableFloatRef(&t)
        );
    });
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_asin_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let t = primitive_float_asin(x);
        // NaN exactly outside [-1, 1], and at NaN
        assert_eq!(t.is_nan(), x.is_nan() || !(-T::ONE..=T::ONE).contains(&x));
        if !t.is_nan() {
            assert!(t.is_finite());
            // odd
            assert_eq!(NiceFloat(primitive_float_asin(-x)), NiceFloat(-t));
            // the same as the `Float` version taken with 64 bits to spare and rounded once
            let (t_float, _) = Float::from(x).asin_prec(T::MANTISSA_WIDTH + 64);
            assert_eq!(
                NiceFloat(T::rounding_from(&t_float, Nearest).0),
                NiceFloat(t)
            );
        }
    });
}

#[test]
fn primitive_float_asin_properties() {
    apply_fn_to_primitive_floats!(primitive_float_asin_properties_helper);
}

#[test]
fn test_asin_rational_prec_round() {
    let test =
        |xs: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
            let x = Rational::from_str(xs).unwrap();

            let (t, o) = Float::asin_rational_prec_round(x.clone(), prec, rm);
            assert!(t.is_valid());
            assert_eq!(t.to_string(), out);
            assert_eq!(to_hex_string(&t), out_hex);
            assert_eq!(o, o_out);

            let (t_alt, o_alt) = Float::asin_rational_prec_round_ref(&x, prec, rm);
            assert!(t_alt.is_valid());
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);

            if rm == Nearest {
                let (t_alt, o_alt) = Float::asin_rational_prec(x.clone(), prec);
                assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
                assert_eq!(o_alt, o);
                let (t_alt, o_alt) = Float::asin_rational_prec_ref(&x, prec);
                assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
                assert_eq!(o_alt, o);
            }

            if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_t, rug_o) = rug_asin_rational_prec_round(&x, prec, rug_rm);
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_t)),
                    ComparableFloatRef(&t)
                );
                assert_eq!(rug_o, o);
            }
        };
    test("0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 10, Down, "0.0", "0x0.0", Equal);
    test("0", 10, Up, "0.0", "0x0.0", Equal);
    test("0", 53, Nearest, "0.0", "0x0.0", Equal);
    test("0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0", 2, Floor, "0.0", "0x0.0", Equal);
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 10, Nearest, "1.5703", "0x1.920#10", Less);
    test("1", 10, Floor, "1.5703", "0x1.920#10", Less);
    test("1", 10, Ceiling, "1.5723", "0x1.928#10", Greater);
    test("1", 10, Down, "1.5703", "0x1.920#10", Less);
    test("1", 10, Up, "1.5723", "0x1.928#10", Greater);
    test(
        "1",
        53,
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test("1", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("1", 2, Floor, "1.5", "0x1.8#2", Less);
    test("-1", 10, Nearest, "-1.5703", "-0x1.920#10", Greater);
    test("-1", 10, Floor, "-1.5723", "-0x1.928#10", Less);
    test("-1", 10, Ceiling, "-1.5703", "-0x1.920#10", Greater);
    test("-1", 10, Down, "-1.5703", "-0x1.920#10", Greater);
    test("-1", 10, Up, "-1.5723", "-0x1.928#10", Less);
    test(
        "-1",
        53,
        Nearest,
        "-1.5707963267948966",
        "-0x1.921fb54442d18#53",
        Greater,
    );
    test("-1", 1, Nearest, "-2.0", "-0x2.0#1", Less);
    test("-1", 2, Floor, "-2.0", "-0x2.0#2", Less);
    test("3/5", 10, Nearest, "0.64355", "0x0.a4c#10", Greater);
    test("3/5", 10, Floor, "0.64258", "0x0.a48#10", Less);
    test("3/5", 10, Ceiling, "0.64355", "0x0.a4c#10", Greater);
    test("3/5", 10, Down, "0.64258", "0x0.a48#10", Less);
    test("3/5", 10, Up, "0.64355", "0x0.a4c#10", Greater);
    test(
        "3/5",
        53,
        Nearest,
        "0.64350110879328437",
        "0x0.a4bc7d1934f708#53",
        Less,
    );
    test("3/5", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("3/5", 2, Floor, "0.50", "0x0.8#2", Less);
    test("-3/5", 10, Nearest, "-0.64355", "-0x0.a4c#10", Less);
    test("-3/5", 10, Floor, "-0.64355", "-0x0.a4c#10", Less);
    test("-3/5", 10, Ceiling, "-0.64258", "-0x0.a48#10", Greater);
    test("-3/5", 10, Down, "-0.64258", "-0x0.a48#10", Greater);
    test("-3/5", 10, Up, "-0.64355", "-0x0.a4c#10", Less);
    test(
        "-3/5",
        53,
        Nearest,
        "-0.64350110879328437",
        "-0x0.a4bc7d1934f708#53",
        Greater,
    );
    test("-3/5", 1, Nearest, "-0.50", "-0x0.8#1", Greater);
    test("-3/5", 2, Floor, "-0.75", "-0x0.c#2", Less);
    test("1/3", 10, Nearest, "0.33984", "0x0.570#10", Greater);
    test("1/3", 10, Floor, "0.33936", "0x0.56e#10", Less);
    test("1/3", 10, Ceiling, "0.33984", "0x0.570#10", Greater);
    test("1/3", 10, Down, "0.33936", "0x0.56e#10", Less);
    test("1/3", 10, Up, "0.33984", "0x0.570#10", Greater);
    test(
        "1/3",
        53,
        Nearest,
        "0.33983690945412193",
        "0x0.56ff8d3c144448#53",
        Less,
    );
    test("1/3", 1, Nearest, "0.25", "0x0.4#1", Less);
    test("1/3", 2, Floor, "0.25", "0x0.4#2", Less);
    test("1/2", 10, Nearest, "0.52344", "0x0.860#10", Less);
    test("1/2", 10, Floor, "0.52344", "0x0.860#10", Less);
    test("1/2", 10, Ceiling, "0.52441", "0x0.864#10", Greater);
    test("1/2", 10, Down, "0.52344", "0x0.860#10", Less);
    test("1/2", 10, Up, "0.52441", "0x0.864#10", Greater);
    test(
        "1/2",
        53,
        Nearest,
        "0.52359877559829893",
        "0x0.860a91c16b9b30#53",
        Greater,
    );
    test("1/2", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("1/2", 2, Floor, "0.50", "0x0.8#2", Less);
    test("-1/2", 10, Nearest, "-0.52344", "-0x0.860#10", Greater);
    test("-1/2", 10, Floor, "-0.52441", "-0x0.864#10", Less);
    test("-1/2", 10, Ceiling, "-0.52344", "-0x0.860#10", Greater);
    test("-1/2", 10, Down, "-0.52344", "-0x0.860#10", Greater);
    test("-1/2", 10, Up, "-0.52441", "-0x0.864#10", Less);
    test(
        "-1/2",
        53,
        Nearest,
        "-0.52359877559829893",
        "-0x0.860a91c16b9b30#53",
        Less,
    );
    test("-1/2", 1, Nearest, "-0.50", "-0x0.8#1", Greater);
    test("-1/2", 2, Floor, "-0.75", "-0x0.c#2", Less);
    test("7/10", 10, Nearest, "0.77539", "0x0.c68#10", Less);
    test("7/10", 10, Floor, "0.77539", "0x0.c68#10", Less);
    test("7/10", 10, Ceiling, "0.77637", "0x0.c6c#10", Greater);
    test("7/10", 10, Down, "0.77539", "0x0.c68#10", Less);
    test("7/10", 10, Up, "0.77637", "0x0.c6c#10", Greater);
    test(
        "7/10",
        53,
        Nearest,
        "0.77539749661075308",
        "0x0.c680734957ecb0#53",
        Greater,
    );
    test("7/10", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("7/10", 2, Floor, "0.75", "0x0.c#2", Less);
    test("99/100", 10, Nearest, "1.4297", "0x1.6e0#10", Greater);
    test("99/100", 10, Floor, "1.4277", "0x1.6d8#10", Less);
    test("99/100", 10, Ceiling, "1.4297", "0x1.6e0#10", Greater);
    test("99/100", 10, Down, "1.4277", "0x1.6d8#10", Less);
    test("99/100", 10, Up, "1.4297", "0x1.6e0#10", Greater);
    test(
        "99/100",
        53,
        Nearest,
        "1.4292568534704695",
        "0x1.6de3c6f33d51e#53",
        Greater,
    );
    test("99/100", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("99/100", 2, Floor, "1.0", "0x1.0#2", Less);
    test("999/1000", 10, Nearest, "1.5254", "0x1.868#10", Less);
    test("999/1000", 10, Floor, "1.5254", "0x1.868#10", Less);
    test("999/1000", 10, Ceiling, "1.5273", "0x1.870#10", Greater);
    test("999/1000", 10, Down, "1.5254", "0x1.868#10", Less);
    test("999/1000", 10, Up, "1.5273", "0x1.870#10", Greater);
    test(
        "999/1000",
        53,
        Nearest,
        "1.5260712396261631",
        "0x1.86ac9ad18f803#53",
        Less,
    );
    test("999/1000", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("999/1000", 2, Floor, "1.5", "0x1.8#2", Less);
    test("22/7", 10, Nearest, "NaN", "NaN", Equal);
    test("22/7", 10, Floor, "NaN", "NaN", Equal);
    test("22/7", 10, Ceiling, "NaN", "NaN", Equal);
    test("22/7", 10, Down, "NaN", "NaN", Equal);
    test("22/7", 10, Up, "NaN", "NaN", Equal);
    test("22/7", 53, Nearest, "NaN", "NaN", Equal);
    test("22/7", 1, Nearest, "NaN", "NaN", Equal);
    test("22/7", 2, Floor, "NaN", "NaN", Equal);
    test("22/7", 10, Exact, "NaN", "NaN", Equal);
    test("-22/7", 10, Nearest, "NaN", "NaN", Equal);
    test("-22/7", 10, Floor, "NaN", "NaN", Equal);
    test("-22/7", 10, Ceiling, "NaN", "NaN", Equal);
    test("-22/7", 10, Down, "NaN", "NaN", Equal);
    test("-22/7", 10, Up, "NaN", "NaN", Equal);
    test("-22/7", 53, Nearest, "NaN", "NaN", Equal);
    test("-22/7", 1, Nearest, "NaN", "NaN", Equal);
    test("-22/7", 2, Floor, "NaN", "NaN", Equal);
    test("-22/7", 10, Exact, "NaN", "NaN", Equal);
    test("3/2", 10, Nearest, "NaN", "NaN", Equal);
    test("3/2", 10, Floor, "NaN", "NaN", Equal);
    test("3/2", 10, Ceiling, "NaN", "NaN", Equal);
    test("3/2", 10, Down, "NaN", "NaN", Equal);
    test("3/2", 10, Up, "NaN", "NaN", Equal);
    test("3/2", 53, Nearest, "NaN", "NaN", Equal);
    test("3/2", 1, Nearest, "NaN", "NaN", Equal);
    test("3/2", 2, Floor, "NaN", "NaN", Equal);
    test("3/2", 10, Exact, "NaN", "NaN", Equal);
    test("2", 10, Nearest, "NaN", "NaN", Equal);
    test("2", 10, Floor, "NaN", "NaN", Equal);
    test("2", 10, Ceiling, "NaN", "NaN", Equal);
    test("2", 10, Down, "NaN", "NaN", Equal);
    test("2", 10, Up, "NaN", "NaN", Equal);
    test("2", 53, Nearest, "NaN", "NaN", Equal);
    test("2", 1, Nearest, "NaN", "NaN", Equal);
    test("2", 2, Floor, "NaN", "NaN", Equal);
    test("2", 10, Exact, "NaN", "NaN", Equal);
    test("-2", 10, Nearest, "NaN", "NaN", Equal);
    test("-2", 10, Floor, "NaN", "NaN", Equal);
    test("-2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-2", 10, Down, "NaN", "NaN", Equal);
    test("-2", 10, Up, "NaN", "NaN", Equal);
    test("-2", 53, Nearest, "NaN", "NaN", Equal);
    test("-2", 1, Nearest, "NaN", "NaN", Equal);
    test("-2", 2, Floor, "NaN", "NaN", Equal);
    test("-2", 10, Exact, "NaN", "NaN", Equal);
    test(
        "1/1000000",
        10,
        Nearest,
        "1.0002e-6",
        "0x0.000010c8#10",
        Greater,
    );
    test("1/1000000", 10, Floor, "9.9838e-7", "0x0.000010c0#10", Less);
    test(
        "1/1000000",
        10,
        Ceiling,
        "1.0002e-6",
        "0x0.000010c8#10",
        Greater,
    );
    test("1/1000000", 10, Down, "9.9838e-7", "0x0.000010c0#10", Less);
    test("1/1000000", 10, Up, "1.0002e-6", "0x0.000010c8#10", Greater);
    test(
        "1/1000000",
        53,
        Nearest,
        "1.0000000000001666e-6",
        "0x0.000010c6f7a0b5f0a0#53",
        Less,
    );
    test("1/1000000", 1, Nearest, "9.5e-7", "0x0.00001#1", Less);
    test("1/1000000", 2, Floor, "9.5e-7", "0x0.000010#2", Less);
    test(
        "123456789/1000000000",
        10,
        Nearest,
        "0.12378",
        "0x0.1fb0#10",
        Greater,
    );
    test(
        "123456789/1000000000",
        10,
        Floor,
        "0.12366",
        "0x0.1fa8#10",
        Less,
    );
    test(
        "123456789/1000000000",
        10,
        Ceiling,
        "0.12378",
        "0x0.1fb0#10",
        Greater,
    );
    test(
        "123456789/1000000000",
        10,
        Down,
        "0.12366",
        "0x0.1fa8#10",
        Less,
    );
    test(
        "123456789/1000000000",
        10,
        Up,
        "0.12378",
        "0x0.1fb0#10",
        Greater,
    );
    test(
        "123456789/1000000000",
        53,
        Nearest,
        "0.12377257242671708",
        "0x0.1faf8f2eb6ec2c#53",
        Less,
    );
    test(
        "123456789/1000000000",
        1,
        Nearest,
        "0.12",
        "0x0.2#1",
        Greater,
    );
    test("123456789/1000000000", 2, Floor, "0.094", "0x0.18#2", Less);
}

#[test]
#[should_panic]
fn asin_rational_prec_round_fail_1() {
    Float::asin_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn asin_rational_prec_round_fail_2() {
    // the arcsine of a nonzero rational in range is never exactly representable
    Float::asin_rational_prec_round(Rational::from_unsigneds(3u8, 5), 10, Exact);
}

#[test]
#[should_panic]
fn asin_rational_prec_fail() {
    Float::asin_rational_prec(Rational::ONE, 0);
}

// Whether asin(x) is exactly representable for a `Rational` x: only at zero, and where the result
// is NaN.
fn asin_rational_exact(x: &Rational) -> bool {
    *x == 0u32 || PartialOrdAbs::gt_abs(x, &1u32)
}

#[allow(clippy::needless_pass_by_value)]
fn asin_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    if rm == Exact && !asin_rational_exact(&x) {
        assert_panic!(Float::asin_rational_prec_round_ref(&x, prec, Exact));
        return;
    }
    let (t, o) = Float::asin_rational_prec_round(x.clone(), prec, rm);
    assert!(t.is_valid());
    assert_rounding_ordering_consistent(&t, rm, o);

    let (t_alt, o_alt) = Float::asin_rational_prec_round_ref(&x, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_t, rug_o) = rug_asin_rational_prec_round(&x, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_t)),
            ComparableFloatRef(&t)
        );
        assert_eq!(rug_o, o, "x = {x} prec = {prec} rm = {rm:?}");
    }

    // NaN exactly outside [-1, 1]
    assert_eq!(t.is_nan(), PartialOrdAbs::gt_abs(&x, &1u32));
    if !t.is_nan() {
        // |asin x| <= pi/2, so the result never overflows
        assert!(t.is_finite());
        assert!(PartialOrdAbs::le_abs(
            &t,
            &Float::from_unsigned_prec(2u32, 1).0
        ));
        if t.is_normal() {
            assert_eq!(t.get_prec(), Some(prec));
        }
        // asin is odd (a `Rational` has no negative zero, so x = 0 is excluded)
        if x != 0u32 {
            let (t_neg, o_neg) = Float::asin_rational_prec_round_ref(&-&x, prec, -rm);
            assert_eq!(ComparableFloatRef(&t_neg), ComparableFloatRef(&-&t));
            assert_eq!(o_neg, o.reverse());
        }
        // a `Float` input agrees with the `Float` version
        if let Ok(f) = Float::try_from(&x) {
            let (t_alt, o_alt) = f.asin_prec_round_ref(prec, rm);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    }

    if o == Equal {
        assert!(asin_rational_exact(&x));
    } else {
        assert_panic!(Float::asin_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn asin_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        asin_rational_prec_round_properties_helper(x, prec, rm);
    });
}

#[test]
fn asin_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (t, o) = Float::asin_rational_prec(x.clone(), prec);
        assert!(t.is_valid());
        assert_rounding_ordering_consistent(&t, Nearest, o);
        for (t_alt, o_alt) in [
            Float::asin_rational_prec_ref(&x, prec),
            Float::asin_rational_prec_round_ref(&x, prec, Nearest),
        ] {
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    });
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_asin_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let t = primitive_float_asin_rational::<T>(&x);
        assert_eq!(t.is_nan(), PartialOrdAbs::gt_abs(&x, &1u32));
        if !t.is_nan() {
            assert!(t.is_finite());
            if x != 0u32 {
                assert_eq!(
                    NiceFloat(primitive_float_asin_rational::<T>(&-&x)),
                    NiceFloat(-t)
                );
            }
            let (t_float, _) = Float::asin_rational_prec_ref(&x, T::MANTISSA_WIDTH + 64);
            assert_eq!(
                NiceFloat(T::rounding_from(&t_float, Nearest).0),
                NiceFloat(t)
            );
        }
    });

    primitive_float_gen_var_1::<T>().test_properties(|x| {
        // A `Rational` has no signed zeros, so -0.0 loses its sign on the way through and comes
        // back as +0.0, where the primitive-float arcsine keeps it.
        if x == T::ZERO {
            return;
        }
        // a finite primitive float, taken through the `Rational` path, matches the direct one
        assert_eq!(
            NiceFloat(primitive_float_asin_rational::<T>(&Rational::exact_from(x))),
            NiceFloat(primitive_float_asin(x))
        );
    });
}

#[test]
fn primitive_float_asin_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_asin_rational_properties_helper);
}

// Whether MPFR and Malachite are expected to disagree on `asinu(x, u)`. For a zero period MPFR
// returns +0 even for a negative x, although its own zero-input case keeps the sign so that the
// function stays odd; Malachite keeps the sign in both cases, as `mpfr_atanu` does.
fn mpfr_signed_zero_divergence(x: &Float, u: u64) -> bool {
    u == 0 && *x < 0u32 && x.le_abs(&1u32)
}

#[test]
fn test_asin_with_period_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                u: u64,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (t, o) = x.clone().asin_with_period_prec_round(u, prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = x.asin_with_period_prec_round_ref(u, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        let mut t_alt = x.clone();
        let o_alt = t_alt.asin_with_period_prec_round_assign(u, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = x.asin_with_period_prec_ref(u, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
            && u <= u64::from(u32::MAX)
            && !mpfr_signed_zero_divergence(&x, u)
        {
            let (rug_t, rug_o) =
                rug_asin_with_period_prec_round(&rug::Float::exact_from(&x), u, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_t)),
                ComparableFloatRef(&t)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 360, 10, Nearest, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 360, 10, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        360,
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("0.0", "0x0.0", 360, 10, Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 360, 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", 0, 10, Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 0, 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("2.0", "0x2.0#1", 360, 10, Nearest, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 360, 10, Nearest, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 0, 10, Nearest, "NaN", "NaN", Equal);
    test(
        "1.0",
        "0x1.0#1",
        360,
        10,
        Exact,
        "90.000",
        "0x5a.0#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        360,
        10,
        Nearest,
        "90.000",
        "0x5a.0#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        360,
        10,
        Exact,
        "-90.000",
        "-0x5a.0#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        7,
        10,
        Exact,
        "1.7500",
        "0x1.c00#10",
        Equal,
    );
    test("1.0", "0x1.0#1", 7, 1, Floor, "1.0", "0x1.0#1", Less);
    test("1.0", "0x1.0#1", 7, 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", 7, 1, Nearest, "2.0", "0x2.0#1", Greater);
    test(
        "0.50",
        "0x0.8#1",
        360,
        10,
        Exact,
        "30.000",
        "0x1e.00#10",
        Equal,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        360,
        10,
        Exact,
        "-30.000",
        "-0x1e.00#10",
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        12,
        10,
        Exact,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        3,
        10,
        Exact,
        "0.25000",
        "0x0.400#10",
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        7,
        10,
        Floor,
        "0.58301",
        "0x0.954#10",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        7,
        10,
        Ceiling,
        "0.58398",
        "0x0.958#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        7,
        10,
        Nearest,
        "0.58301",
        "0x0.954#10",
        Less,
    );
    test("0.50", "0x0.8#1", 0, 10, Nearest, "0.0", "0x0.0", Equal);
    test("-0.50", "-0x0.8#1", 0, 10, Nearest, "-0.0", "-0x0.0", Equal);
    test(
        "0.25",
        "0x0.4#1",
        360,
        10,
        Floor,
        "14.469",
        "0xe.78#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        360,
        10,
        Ceiling,
        "14.484",
        "0xe.7c#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        360,
        10,
        Down,
        "14.469",
        "0xe.78#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        360,
        10,
        Up,
        "14.484",
        "0xe.7c#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        360,
        10,
        Nearest,
        "14.484",
        "0xe.7c#10",
        Greater,
    );
    test(
        "-0.25",
        "-0x0.4#1",
        360,
        10,
        Nearest,
        "-14.484",
        "-0xe.7c#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        360,
        100,
        Nearest,
        "14.477512185929923878771034799128",
        "0xe.7a3e3d1602afc2ef3f3d2390#100",
        Greater,
    );
    test(
        "0.75",
        "0x0.c#2",
        360,
        20,
        Nearest,
        "48.590393",
        "0x30.9724#20",
        Greater,
    );
    test(
        "0.88",
        "0x0.e#3",
        360,
        20,
        Nearest,
        "61.044983",
        "0x3d.0b84#20",
        Greater,
    );
    test(
        "0.99902",
        "0x0.ffc#10",
        360,
        20,
        Nearest,
        "87.467651",
        "0x57.77b8#20",
        Less,
    );
    test(
        "0.99902",
        "0x0.ffc#10",
        360,
        100,
        Nearest,
        "87.467654249465423350139499248930",
        "0x57.77b8305b4a17593f0f1f3240#100",
        Less,
    );
    test(
        "-0.99902",
        "-0x0.ffc#10",
        360,
        20,
        Nearest,
        "-87.467651",
        "-0x57.77b8#20",
        Greater,
    );
    test(
        "0.600000000000000000022",
        "0x0.999999999999999a#64",
        360,
        64,
        Nearest,
        "36.8698976458440212979",
        "0x24.deb19cb3c478604#64",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        360,
        20,
        Nearest,
        "4.5198398e-29",
        "0x3.94bb8E-24#20",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        1,
        20,
        Nearest,
        "1.2555107e-31",
        "0x2.8be60E-26#20",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        18446744073709551615,
        20,
        Nearest,
        "2.3160085e-12",
        "0x2.8be60E-10#20",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        18446744073709551615,
        20,
        Nearest,
        "7.4184159e17",
        "0xa.4b8dE+14#20",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        18446744073709551615,
        64,
        Exact,
        "4611686018427387903.75",
        "0x3fffffffffffffff.c#64",
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        18446744073709551613,
        64,
        Nearest,
        "1537228672809129301.12",
        "0x1555555555555555.2#64",
        Greater,
    );
}

#[test]
#[should_panic]
fn asin_with_period_prec_round_fail_1() {
    Float::ONE.asin_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn asin_with_period_prec_round_fail_2() {
    // asin(1/4) is not an exact number of sevenths of a turn
    Float::from(0.25).asin_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn asin_with_period_prec_round_fail_3() {
    // asin(1/2) is a twelfth of a turn, but 7 is not a multiple of 3
    Float::ONE_HALF.asin_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn asin_with_period_prec_round_fail_4() {
    // a quarter turn needs more than 2 bits when u = 7
    Float::ONE.asin_with_period_prec_round(7, 2, Exact);
}

#[test]
#[should_panic]
fn asin_with_period_prec_round_ref_fail() {
    Float::ONE.asin_with_period_prec_round_ref(7, 0, Floor);
}

#[test]
#[should_panic]
fn asin_with_period_prec_fail() {
    Float::ONE.asin_with_period_prec(7, 0);
}

#[test]
#[should_panic]
fn asin_with_period_round_fail() {
    Float::from(0.25).asin_with_period_round(7, Exact);
}

// A tiny input with a small period, where the result is about xu/(2 pi) and falls below the
// smallest positive `Float`. MPFR's wider exponent range never reaches this, so the quotient is
// formed with the numerator scaled up and the underflow decided by the rounding mode alone.
#[test]
fn test_asin_with_period_underflow() {
    let min_positive = Float::min_positive_value_prec(10);
    let tiny = Float::one_prec(10) >> (1u64 << 30);
    for (rm, expected, o_out) in [
        (Nearest, Float::ZERO, Less),
        (Floor, Float::ZERO, Less),
        (Down, Float::ZERO, Less),
        (Ceiling, min_positive.clone(), Greater),
        (Up, min_positive.clone(), Greater),
    ] {
        let (t, o) = tiny.asin_with_period_prec_round_ref(1, 10, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(expected));
        assert_eq!(o, o_out);
    }
    // the function is odd, so the negation mirrors
    let (t, o) = (-&tiny).asin_with_period_prec_round_ref(1, 10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
    // with u = 8 the result is representable after all
    let (t, o) = tiny.asin_with_period_prec_round_ref(8, 10, Nearest);
    assert!(t > 0u32);
    assert_eq!(o, Greater);
}

// Whether asinu(x, u) is exactly representable at `prec`: at NaN, at an input outside [-1, 1] or
// infinite (where the result is NaN), at zero, at u = 0, at |x| = 1, where the result is a quarter
// turn, and at |x| = 1/2 with u a multiple of 3, where it is a twelfth of a turn. The last two need
// a `prec` wide enough to hold the turn fraction.
fn asin_with_period_exact(x: &Float, u: u64, prec: u64) -> bool {
    x.is_nan()
        || !x.is_finite()
        || x.gt_abs(&1u32)
        || *x == 0u32
        || u == 0
        || (x.eq_abs(&1u32) && Float::from_unsigned_prec(u, prec).1 == Equal)
        || (x.eq_abs(&Float::ONE_HALF)
            && u.is_multiple_of(3)
            && Float::from_unsigned_prec(u / 3, prec).1 == Equal)
}

#[allow(clippy::needless_pass_by_value)]
fn asin_with_period_prec_round_properties_helper(x: Float, u: u64, prec: u64, rm: RoundingMode) {
    if rm == Exact && !asin_with_period_exact(&x, u, prec) {
        assert_panic!(x.asin_with_period_prec_round_ref(u, prec, Exact));
        return;
    }
    let (t, o) = x.clone().asin_with_period_prec_round(u, prec, rm);
    assert!(t.is_valid());
    assert_rounding_ordering_consistent(&t, rm, o);

    let (t_alt, o_alt) = x.asin_with_period_prec_round_ref(u, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    let mut t_alt = x.clone();
    let o_alt = t_alt.asin_with_period_prec_round_assign(u, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        && u <= u64::from(u32::MAX)
        && !mpfr_signed_zero_divergence(&x, u)
    {
        let (rug_t, rug_o) =
            rug_asin_with_period_prec_round(&rug::Float::exact_from(&x), u, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_t)),
            ComparableFloatRef(&t)
        );
        assert_eq!(rug_o, o);
    }

    // NaN exactly for a NaN input, an infinite input, or an input outside [-1, 1]
    assert_eq!(t.is_nan(), x.is_nan() || !x.is_finite() || x.gt_abs(&1u32));
    if !t.is_nan() {
        // |asinu(x, u)| <= u/4, a quarter turn, and the result never overflows
        assert!(t.is_finite());
        assert!(PartialOrdAbs::le_abs(
            &t,
            &Float::from_unsigned_prec(u, prec + 2).0
        ));
        if t.is_normal() {
            assert_eq!(t.get_prec(), Some(prec));
        }
        // asin_with_period is odd
        let (t_neg, o_neg) = (-&x).asin_with_period_prec_round(u, prec, -rm);
        assert_eq!(ComparableFloatRef(&t_neg), ComparableFloatRef(&-&t));
        assert_eq!(o_neg, o.reverse());
    }

    if o == Equal {
        assert!(asin_with_period_exact(&x, u, prec));
        for rm in exhaustive_rounding_modes() {
            let (t2, oo) = x.asin_with_period_prec_round_ref(u, prec, rm);
            assert_eq!(ComparableFloatRef(&t2), ComparableFloatRef(&t));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.asin_with_period_prec_round_ref(u, prec, Exact));
    }
}

#[test]
fn asin_with_period_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_21().test_properties(
        |(x, u, prec, rm)| {
            asin_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_22().test_properties(
        |(x, u, prec, rm)| {
            asin_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // asinu(NaN, u) = asinu(±infinity, u) = NaN
        for x in [Float::NAN, Float::INFINITY, Float::NEGATIVE_INFINITY] {
            let (t, o) = x.asin_with_period_prec_round(4, prec, rm);
            assert!(t.is_nan());
            assert_eq!(o, Equal);
        }
        // asinu(x, u) = NaN for |x| > 1, even for u = 0
        for u in [0, 4] {
            let (t, o) = Float::TWO.asin_with_period_prec_round(u, prec, rm);
            assert!(t.is_nan());
            assert_eq!(o, Equal);
        }
        // asinu(±0.0, u) = ±0.0, exactly
        let (t, o) = Float::ZERO.asin_with_period_prec_round(4, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        let (t, o) = Float::NEGATIVE_ZERO.asin_with_period_prec_round(4, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
        // asinu(x, 0) = ±0.0 with the sign of x, exactly, which keeps the function odd
        let (t, o) = Float::ONE.asin_with_period_prec_round(0, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        let (t, o) = (-Float::ONE).asin_with_period_prec_round(0, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
        // asinu(±1, u) = ±u/4, a quarter turn, exact when `prec` holds it
        let (q, o_q) = Float::from_unsigned_prec_round(8u32, prec, rm);
        let (t, o) = Float::ONE.asin_with_period_prec_round(8, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(q >> 2u32));
        assert_eq!(o, o_q);
        // asinu(±1/2, u) = ±u/12, a twelfth of a turn, when u is a multiple of 3
        let (q, o_q) = Float::from_unsigned_prec_round(4u32, prec, rm);
        let (t, o) = Float::ONE_HALF.asin_with_period_prec_round(12, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(q >> 2u32));
        assert_eq!(o, o_q);
    });
}

#[test]
fn asin_with_period_prec_properties() {
    float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().test_properties(|(x, u, prec)| {
        let (t, o) = x.clone().asin_with_period_prec(u, prec);
        assert!(t.is_valid());
        let (t_alt, o_alt) = x.asin_with_period_prec_ref(u, prec);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.asin_with_period_prec_round_ref(u, prec, Nearest);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.asin_with_period_prec_assign(u, prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn asin_with_period_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_41().test_properties(|(x, u, rm)| {
        if rm == Exact && !asin_with_period_exact(&x, u, x.significant_bits()) {
            assert_panic!(x.asin_with_period_round_ref(u, Exact));
            return;
        }
        let (t, o) = x.clone().asin_with_period_round(u, rm);
        assert!(t.is_valid());
        assert_rounding_ordering_consistent(&t, rm, o);
        let (t_alt, o_alt) = x.asin_with_period_round_ref(u, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.asin_with_period_prec_round_ref(u, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.asin_with_period_round_assign(u, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn asin_with_period_properties() {
    float_unsigned_pair_gen_var_2::<u64>().test_properties(|(x, u)| {
        let t = x.clone().asin_with_period(u);
        assert!(t.is_valid());
        let t_alt = x.asin_with_period_ref(u);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let mut t_alt = x.clone();
        t_alt.asin_with_period_assign(u);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let (t_alt, _) = x.asin_with_period_prec_round_ref(u, x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        // asin_with_period is odd
        assert_eq!(
            ComparableFloatRef(&(-&x).asin_with_period_ref(u)),
            ComparableFloatRef(&-&t)
        );
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_asin_with_period() {
    fn test<T: PrimitiveFloat>(x: T, u: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_asin_with_period(x, u)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, 360, f32::NAN);
    test::<f32>(f32::INFINITY, 360, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, 360, f32::NAN);
    test::<f32>(2.0, 360, f32::NAN);
    test::<f32>(-2.0, 360, f32::NAN);
    test::<f32>(2.0, 0, f32::NAN);
    test::<f32>(1.0, 0, 0.0);
    test::<f32>(-1.0, 0, -0.0);
    test::<f32>(0.0, 360, 0.0);
    test::<f32>(-0.0, 360, -0.0);
    test::<f32>(1.0, 360, 90.0);
    test::<f32>(-1.0, 360, -90.0);
    test::<f32>(0.5, 360, 30.0);
    test::<f32>(-0.5, 360, -30.0);
    test::<f32>(0.5, 12, 1.0);
    test::<f32>(0.5, 7, 0.5833333);
    test::<f32>(1.0, 7, 1.75);
    test::<f32>(0.25, 360, 14.477512);
    test::<f32>(-0.25, 360, -14.477512);
    test::<f32>(0.1, 360, 5.7391706);
    test::<f32>(0.75, 360, 48.590378);
    test::<f32>(0.9, 360, 64.158066);
    test::<f32>(0.99, 360, 81.89039);
    test::<f32>(0.999999, 360, 89.918434);
    test::<f32>(0.5, 1, 0.083333336);
    test::<f32>(0.25, 1, 0.040215313);
    test::<f32>(1.0e-30, 1, 1.5915494e-31);
    test::<f32>(1.0e-45, 1, 0.0);
    test::<f32>(1.0e-45, 360, 8.0e-44);
    test::<f32>(1.0, 18446744073709551615, 4.611686e18);
    test::<f32>(0.5, 18446744073709551615, 1.5372287e18);
    test::<f64>(f64::NAN, 360, f64::NAN);
    test::<f64>(f64::INFINITY, 360, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, 360, f64::NAN);
    test::<f64>(2.0, 360, f64::NAN);
    test::<f64>(-2.0, 360, f64::NAN);
    test::<f64>(2.0, 0, f64::NAN);
    test::<f64>(1.0, 0, 0.0);
    test::<f64>(-1.0, 0, -0.0);
    test::<f64>(0.0, 360, 0.0);
    test::<f64>(-0.0, 360, -0.0);
    test::<f64>(1.0, 360, 90.0);
    test::<f64>(-1.0, 360, -90.0);
    test::<f64>(0.5, 360, 30.0);
    test::<f64>(-0.5, 360, -30.0);
    test::<f64>(0.5, 12, 1.0);
    test::<f64>(0.5, 7, 0.5833333333333334);
    test::<f64>(1.0, 7, 1.75);
    test::<f64>(0.25, 360, 14.477512185929925);
    test::<f64>(-0.25, 360, -14.477512185929925);
    test::<f64>(0.1, 360, 5.739170477266787);
    test::<f64>(0.75, 360, 48.590377890729144);
    test::<f64>(0.9, 360, 64.15806723683288);
    test::<f64>(0.99, 360, 81.89038554400582);
    test::<f64>(0.999999, 360, 89.91897152479233);
    test::<f64>(0.5, 1, 0.08333333333333333);
    test::<f64>(0.25, 1, 0.04021531162758312);
    test::<f64>(1.0e-100, 1, 1.5915494309189535e-101);
    test::<f64>(5.0e-324, 1, 0.0);
    test::<f64>(5.0e-324, 360, 2.8e-322);
    test::<f64>(1.0, 18446744073709551615, 4.611686018427388e18);
    test::<f64>(0.5, 18446744073709551615, 1.5372286728091292e18);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_asin_with_period_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        let t = primitive_float_asin_with_period(x, u);
        // NaN exactly for a NaN input, an infinite input, or an input outside [-1, 1]
        assert_eq!(t.is_nan(), x.is_nan() || x.abs() > T::ONE);
        if !t.is_nan() {
            // odd
            assert_eq!(
                NiceFloat(primitive_float_asin_with_period(-x, u)),
                NiceFloat(-t)
            );
            // the same as the `Float` version taken with 64 bits to spare and rounded once
            let (t_float, _) =
                Float::asin_with_period_prec(Float::from(x), u, T::MANTISSA_WIDTH + 64);
            assert_eq!(
                NiceFloat(T::rounding_from(&t_float, Nearest).0),
                NiceFloat(t)
            );
            // the result never overflows, since it is at most a quarter turn
            assert!(t.is_finite());
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // u = 0 gives a zero with the sign of x, except that an input outside [-1, 1] is still NaN
        assert_eq!(
            primitive_float_asin_with_period(x, 0).is_nan(),
            x.is_nan() || x.abs() > T::ONE
        );
    });
}

#[test]
fn primitive_float_asin_with_period_properties() {
    apply_fn_to_primitive_floats!(primitive_float_asin_with_period_properties_helper);
}

// As `mpfr_signed_zero_divergence`, for a `Rational` input.
fn mpfr_rational_signed_zero_divergence(x: &Rational, u: u64) -> bool {
    u == 0 && *x < 0u32 && x.le_abs(&1u32)
}

#[test]
fn test_asin_with_period_rational_prec_round() {
    let test = |s: &str,
                u: u64,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (t, o) = Float::asin_with_period_rational_prec_round(x.clone(), u, prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = Float::asin_with_period_rational_prec_round_ref(&x, u, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = Float::asin_with_period_rational_prec(x.clone(), u, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) = Float::asin_with_period_rational_prec_ref(&x, u, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
            && u <= u64::from(u32::MAX)
            && !mpfr_rational_signed_zero_divergence(&x, u)
        {
            let (rug_t, rug_o) = rug_asin_with_period_rational_prec_round(&x, u, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_t)),
                ComparableFloatRef(&t)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 360, 10, Nearest, "0.0", "0x0.0", Equal);
    test("0", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("0", 18446744073709551615, 1, Exact, "0.0", "0x0.0", Equal);
    test("2", 360, 10, Nearest, "NaN", "NaN", Equal);
    test("-2", 360, 10, Nearest, "NaN", "NaN", Equal);
    test("2", 0, 10, Nearest, "NaN", "NaN", Equal);
    test("100/99", 360, 10, Nearest, "NaN", "NaN", Equal);
    test("1", 360, 10, Exact, "90.000", "0x5a.0#10", Equal);
    test("-1", 360, 10, Exact, "-90.000", "-0x5a.0#10", Equal);
    test("1", 7, 10, Exact, "1.7500", "0x1.c00#10", Equal);
    test("1", 7, 1, Floor, "1.0", "0x1.0#1", Less);
    test("1", 7, 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1", 0, 10, Nearest, "0.0", "0x0.0", Equal);
    test("-1", 0, 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("1/2", 360, 10, Exact, "30.000", "0x1e.00#10", Equal);
    test("-1/2", 360, 10, Exact, "-30.000", "-0x1e.00#10", Equal);
    test("1/2", 12, 10, Exact, "1.0000", "0x1.000#10", Equal);
    test("1/2", 3, 10, Exact, "0.25000", "0x0.400#10", Equal);
    test("1/2", 7, 10, Floor, "0.58301", "0x0.954#10", Less);
    test("1/2", 7, 10, Ceiling, "0.58398", "0x0.958#10", Greater);
    test("1/2", 7, 10, Nearest, "0.58301", "0x0.954#10", Less);
    test("-1/2", 0, 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("3/5", 360, 10, Floor, "36.812", "0x24.d#10", Less);
    test("3/5", 360, 10, Ceiling, "36.875", "0x24.e#10", Greater);
    test("3/5", 360, 10, Nearest, "36.875", "0x24.e#10", Greater);
    test(
        "3/5",
        360,
        100,
        Nearest,
        "36.869897645844021296855612559085",
        "0x24.deb19cb3c478602c9ec9b860#100",
        Less,
    );
    test("-3/5", 360, 10, Nearest, "-36.875", "-0x24.e#10", Less);
    test(
        "1/3",
        360,
        20,
        Nearest,
        "19.471222",
        "0x13.78a2#20",
        Greater,
    );
    test("2/3", 360, 20, Nearest, "41.810303", "0x29.cf70#20", Less);
    test(
        "999999/1000000",
        360,
        20,
        Nearest,
        "89.918945",
        "0x59.eb40#20",
        Less,
    );
    test(
        "-999999/1000000",
        360,
        20,
        Nearest,
        "-89.918945",
        "-0x59.eb40#20",
        Greater,
    );
    test(
        "1/1000000",
        360,
        20,
        Nearest,
        "0.000057295780",
        "0x0.0003c1438#20",
        Greater,
    );
    test(
        "1/1000000",
        1,
        20,
        Nearest,
        "1.5915498e-7",
        "0x2.ab90cE-6#20",
        Greater,
    );
    test(
        "1/3",
        1,
        53,
        Nearest,
        "0.054086723984696362",
        "0x0.0dd8a0a6a97c168#53",
        Less,
    );
    test(
        "1",
        18446744073709551615,
        64,
        Exact,
        "4611686018427387903.75",
        "0x3fffffffffffffff.c#64",
        Equal,
    );
    test(
        "1/2",
        18446744073709551613,
        64,
        Nearest,
        "1537228672809129301.12",
        "0x1555555555555555.2#64",
        Greater,
    );
    test(
        "1/2",
        18446744073709551615,
        64,
        Exact,
        "1537228672809129301.25",
        "0x1555555555555555.4#64",
        Equal,
    );
}

#[test]
#[should_panic]
fn asin_with_period_rational_prec_round_fail_1() {
    Float::asin_with_period_rational_prec_round(Rational::ONE, 7, 0, Floor);
}

#[test]
#[should_panic]
fn asin_with_period_rational_prec_round_fail_2() {
    // asin(1/4) is not an exact number of sevenths of a turn
    Float::asin_with_period_rational_prec_round(Rational::from_unsigneds(1u8, 4), 7, 10, Exact);
}

#[test]
#[should_panic]
fn asin_with_period_rational_prec_round_fail_3() {
    // asin(1/2) is a twelfth of a turn, but 7 is not a multiple of 3
    Float::asin_with_period_rational_prec_round(Rational::ONE_HALF, 7, 10, Exact);
}

#[test]
#[should_panic]
fn asin_with_period_rational_prec_round_fail_4() {
    // a quarter turn needs more than 2 bits when u = 7
    Float::asin_with_period_rational_prec_round(Rational::ONE, 7, 2, Exact);
}

#[test]
#[should_panic]
fn asin_with_period_rational_prec_round_ref_fail() {
    Float::asin_with_period_rational_prec_round_ref(&Rational::ONE, 7, 0, Floor);
}

#[test]
#[should_panic]
fn asin_with_period_rational_prec_fail() {
    Float::asin_with_period_rational_prec(Rational::ONE, 7, 0);
}

// A [`Rational`] may sit far below the bottom of the exponent range, where its arcsine is its own
// leading term. Scaling such an input by a power of 2 scales the result by the same power, until a
// small enough period pushes the quotient below the smallest positive `Float` and the rounding mode
// alone decides.
#[test]
fn test_asin_with_period_rational_underflow() {
    let k = u64::power_of_2(30) - 1000;
    for m in
        [Rational::ONE, Rational::from_unsigneds(3u8, 5), Rational::from_unsigneds(123456789u64, 7)]
    {
        let small = m >> 1000u32;
        let big = &small >> k;
        for (small, big) in [(small.clone(), big.clone()), (-small, -big)] {
            for u in [1u64 << 20, 1u64 << 40, u64::MAX] {
                for rm in exhaustive_rounding_modes() {
                    if rm == Exact {
                        continue;
                    }
                    let (t, o) = Float::asin_with_period_rational_prec_round_ref(&big, u, 53, rm);
                    let (t_alt, o_alt) =
                        Float::asin_with_period_rational_prec_round_ref(&small, u, 53, rm);
                    assert_eq!(
                        ComparableFloatRef(&t),
                        ComparableFloatRef(&(t_alt >> k)),
                        "u = {u} rm = {rm:?}"
                    );
                    assert_eq!(o, o_alt, "u = {u} rm = {rm:?}");
                }
            }
            // with a small u the quotient can fall below the bottom of the range, and the rounding
            // mode alone decides; the larger m keeps it inside, so round away first to see which
            // case this is
            let min_positive = Float::min_positive_value_prec(53);
            let positive = big > 0u32;
            let (away_t, _) = Float::asin_with_period_rational_prec_round_ref(&big, 1, 53, Up);
            if !away_t.eq_abs(&min_positive) {
                continue;
            }
            for rm in exhaustive_rounding_modes() {
                if rm == Exact {
                    continue;
                }
                let (t, o) = Float::asin_with_period_rational_prec_round_ref(&big, 1, 53, rm);
                let away = match rm {
                    Up => true,
                    Ceiling => positive,
                    Floor => !positive,
                    _ => false,
                };
                let expected = match (positive, away) {
                    (true, true) => min_positive.clone(),
                    (true, false) => Float::ZERO,
                    (false, true) => -min_positive.clone(),
                    (false, false) => Float::NEGATIVE_ZERO,
                };
                assert_eq!(
                    ComparableFloatRef(&t),
                    ComparableFloatRef(&expected),
                    "rm = {rm:?}"
                );
                assert_eq!(o, if away == positive { Greater } else { Less });
            }
        }
    }
}

// Whether asinu(x, u) is exactly representable at `prec` for a `Rational` x: at an input outside
// [-1, 1] (where the result is NaN), at zero, at u = 0, at |x| = 1, where the result is a quarter
// turn, and at |x| = 1/2 with u a multiple of 3, where it is a twelfth of a turn. The last two need
// a `prec` wide enough to hold the turn fraction.
fn asin_with_period_rational_exact(x: &Rational, u: u64, prec: u64) -> bool {
    x.gt_abs(&1u32)
        || *x == 0u32
        || u == 0
        || (x.eq_abs(&1u32) && Float::from_unsigned_prec(u, prec).1 == Equal)
        || (x.eq_abs(&Rational::ONE_HALF)
            && u.is_multiple_of(3)
            && Float::from_unsigned_prec(u / 3, prec).1 == Equal)
}

#[allow(clippy::needless_pass_by_value)]
fn asin_with_period_rational_prec_round_properties_helper(
    x: Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) {
    if rm == Exact && !asin_with_period_rational_exact(&x, u, prec) {
        assert_panic!(Float::asin_with_period_rational_prec_round_ref(
            &x, u, prec, Exact
        ));
        return;
    }
    let (t, o) = Float::asin_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(t.is_valid());
    assert_rounding_ordering_consistent(&t, rm, o);

    let (t_alt, o_alt) = Float::asin_with_period_rational_prec_round_ref(&x, u, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        && u <= u64::from(u32::MAX)
        && !mpfr_rational_signed_zero_divergence(&x, u)
    {
        let (rug_t, rug_o) = rug_asin_with_period_rational_prec_round(&x, u, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_t)),
            ComparableFloatRef(&t)
        );
        assert_eq!(rug_o, o, "x = {x} u = {u} prec = {prec} rm = {rm:?}");
    }

    // NaN exactly for an input outside [-1, 1]
    assert_eq!(t.is_nan(), x.gt_abs(&1u32));
    if !t.is_nan() {
        // |asinu(x, u)| <= u/4, a quarter turn, so the result never overflows
        assert!(t.is_finite());
        assert!(PartialOrdAbs::le_abs(
            &t,
            &Float::from_unsigned_prec(u, prec + 2).0
        ));
        if t.is_normal() {
            assert_eq!(t.get_prec(), Some(prec));
        }
        // asin_with_period is odd (a `Rational` has no negative zero, so x = 0 is excluded)
        if x != 0u32 {
            let (t_neg, o_neg) = Float::asin_with_period_rational_prec_round(-&x, u, prec, -rm);
            assert_eq!(ComparableFloatRef(&t_neg), ComparableFloatRef(&-&t));
            assert_eq!(o_neg, o.reverse());
        }
    }
    // a `Float` input agrees with the `Float` version
    if let Ok(f) = Float::try_from(&x) {
        let (t_alt, o_alt) = f.asin_with_period_prec_round(u, prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    }

    if o == Equal {
        assert!(asin_with_period_rational_exact(&x, u, prec));
        for rm in exhaustive_rounding_modes() {
            let (t2, oo) = Float::asin_with_period_rational_prec_round_ref(&x, u, prec, rm);
            assert_eq!(ComparableFloatRef(&t2), ComparableFloatRef(&t));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::asin_with_period_rational_prec_round_ref(
            &x, u, prec, Exact
        ));
    }
}

#[test]
fn asin_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_7().test_properties(
        |(x, u, prec, rm)| {
            asin_with_period_rational_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // asinu(x, u) = NaN for |x| > 1, even for u = 0
        for u in [0, 4] {
            let (t, o) = Float::asin_with_period_rational_prec_round(Rational::TWO, u, prec, rm);
            assert!(t.is_nan());
            assert_eq!(o, Equal);
        }
        // asinu(0, u) = 0, exactly (a `Rational` zero has no sign, so the result is positive)
        let (t, o) = Float::asin_with_period_rational_prec_round(Rational::ZERO, 360, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        // asinu(x, 0) = ±0.0 with the sign of x, exactly, which keeps the function odd
        let (t, o) = Float::asin_with_period_rational_prec_round(Rational::ONE, 0, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        let (t, o) = Float::asin_with_period_rational_prec_round(-Rational::ONE, 0, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
        // asinu(±1, u) = ±u/4, a quarter turn, and asinu(±1/2, u) = ±u/12, a twelfth, when 3 |
        // u; both are exact when `prec` holds them
        for (x, u, v, neg) in [
            (Rational::ONE, 8u64, 8u32, false),
            (-Rational::ONE, 8, 8, true),
            (Rational::ONE_HALF, 12, 4, false),
            (-Rational::ONE_HALF, 12, 4, true),
        ] {
            let (q, o_q) = Float::from_unsigned_prec_round(v, prec, if neg { -rm } else { rm });
            let (t, o) = Float::asin_with_period_rational_prec_round(x, u, prec, rm);
            let q = q >> 2u32;
            assert_eq!(
                ComparableFloat(t),
                ComparableFloat(if neg { -q } else { q })
            );
            assert_eq!(o, if neg { o_q.reverse() } else { o_q });
        }
    });
}

#[test]
fn asin_with_period_rational_prec_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_7().test_properties(
        |(x, u, prec, _)| {
            let (t, o) = Float::asin_with_period_rational_prec(x.clone(), u, prec);
            assert!(t.is_valid());
            let (t_alt, o_alt) = Float::asin_with_period_rational_prec_ref(&x, u, prec);
            assert!(t_alt.is_valid());
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) =
                Float::asin_with_period_rational_prec_round_ref(&x, u, prec, Nearest);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        },
    );
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_asin_with_period_rational() {
    fn test<T: PrimitiveFloat>(s: &str, u: u64, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_asin_with_period_rational::<T>(
                &Rational::from_str(s).unwrap(),
                u
            )),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 360, 0.0);
    test::<f32>("2", 360, f32::NAN);
    test::<f32>("-2", 360, f32::NAN);
    test::<f32>("2", 0, f32::NAN);
    test::<f32>("1", 0, 0.0);
    test::<f32>("-1", 0, -0.0);
    test::<f32>("1", 360, 90.0);
    test::<f32>("-1", 360, -90.0);
    test::<f32>("1/2", 360, 30.0);
    test::<f32>("-1/2", 360, -30.0);
    test::<f32>("1/2", 12, 1.0);
    test::<f32>("1/2", 7, 0.5833333);
    test::<f32>("1", 7, 1.75);
    test::<f32>("3/5", 360, 36.869896);
    test::<f32>("-3/5", 360, -36.869896);
    test::<f32>("1/3", 360, 19.47122);
    test::<f32>("999999/1000000", 360, 89.91897);
    test::<f32>("1/1000000", 360, 0.00005729578);
    test::<f32>("1/1000000", 1, 1.5915494e-7);
    test::<f32>(
        "1/100000000000000000000000000000000000000000000000000",
        1,
        0.0,
    );
    test::<f32>("1", 18446744073709551615, 4.611686e18);
    test::<f32>("1/2", 18446744073709551615, 1.5372287e18);
    test::<f64>("0", 360, 0.0);
    test::<f64>("2", 360, f64::NAN);
    test::<f64>("-2", 360, f64::NAN);
    test::<f64>("2", 0, f64::NAN);
    test::<f64>("1", 0, 0.0);
    test::<f64>("-1", 0, -0.0);
    test::<f64>("1", 360, 90.0);
    test::<f64>("-1", 360, -90.0);
    test::<f64>("1/2", 360, 30.0);
    test::<f64>("-1/2", 360, -30.0);
    test::<f64>("1/2", 12, 1.0);
    test::<f64>("1/2", 7, 0.5833333333333334);
    test::<f64>("1", 7, 1.75);
    test::<f64>("3/5", 360, 36.86989764584402);
    test::<f64>("-3/5", 360, -36.86989764584402);
    test::<f64>("1/3", 360, 19.47122063449069);
    test::<f64>("999999/1000000", 360, 89.91897152479349);
    test::<f64>("1/1000000", 360, 0.00005729577951309187);
    test::<f64>("1/1000000", 1, 1.5915494309192187e-7);
    test::<f64>(
        "1/100000000000000000000000000000000000000000000000000",
        1,
        1.5915494309189534e-51,
    );
    test::<f64>("1", 18446744073709551615, 4.611686018427388e18);
    test::<f64>("1/2", 18446744073709551615, 1.5372286728091292e18);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_asin_with_period_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_unsigned_pair_gen_var_1::<u64>().test_properties(|(x, u)| {
        let t = primitive_float_asin_with_period_rational::<T>(&x, u);
        // NaN exactly for an input outside [-1, 1], and never overflowing, since the result is at
        // most a quarter turn
        assert_eq!(t.is_nan(), x.gt_abs(&1u32));
        if !t.is_nan() {
            assert!(t.is_finite());
            // odd (a `Rational` has no negative zero, so x = 0 is excluded)
            if x != 0u32 {
                assert_eq!(
                    NiceFloat(primitive_float_asin_with_period_rational::<T>(&-&x, u)),
                    NiceFloat(-t)
                );
            }
        }
        // the same as the `Float` version taken with 64 bits to spare and rounded once
        let (t_float, _) = Float::asin_with_period_rational_prec_ref(&x, u, T::MANTISSA_WIDTH + 64);
        assert_eq!(
            NiceFloat(T::rounding_from(&t_float, Nearest).0),
            NiceFloat(t)
        );
    });

    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        // The arcsine of a finite primitive float, taken through the `Rational` path, matches the
        // direct primitive-float arcsine. A zero is excluded, since `Rational` has no signed zeros.
        if x != T::ZERO {
            assert_eq!(
                NiceFloat(primitive_float_asin_with_period_rational::<T>(
                    &Rational::exact_from(x),
                    u
                )),
                NiceFloat(primitive_float_asin_with_period(x, u))
            );
        }
    });
}

#[test]
fn primitive_float_asin_with_period_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_asin_with_period_rational_properties_helper);
}

#[test]
fn test_asin_pi_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (t, o) = x.clone().asin_pi_prec_round(prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = x.asin_pi_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        let mut t_alt = x.clone();
        let o_alt = t_alt.asin_pi_prec_round_assign(prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = x.asin_pi_prec_ref(prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }

        // asinu with u = 2, which MPFR's asin_u gives directly
        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_t, rug_o) =
                rug_asin_with_period_prec_round(&rug::Float::exact_from(&x), 2, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_t)),
                ComparableFloatRef(&t)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 10, Exact, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Exact, "-0.0", "-0x0.0", Equal);
    test("2.0", "0x2.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("1.0", "0x1.0#1", 10, Exact, "0.50000", "0x0.800#10", Equal);
    test("1.0", "0x1.0#1", 1, Exact, "0.50", "0x0.8#1", Equal);
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Exact,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test("0.50", "0x0.8#1", 10, Floor, "0.16650", "0x0.2aa#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Ceiling,
        "0.16675",
        "0x0.2ab#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        10,
        Nearest,
        "0.16675",
        "0x0.2ab#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        53,
        Nearest,
        "0.16666666666666666",
        "0x0.2aaaaaaaaaaaaa#53",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        10,
        Floor,
        "0.080322",
        "0x0.1490#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        10,
        Ceiling,
        "0.080444",
        "0x0.1498#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        10,
        Nearest,
        "0.080444",
        "0x0.1498#10",
        Greater,
    );
    test(
        "-0.25",
        "-0x0.4#1",
        10,
        Nearest,
        "-0.080444",
        "-0x0.1498#10",
        Less,
    );
    test(
        "0.99902",
        "0x0.ffc#10",
        20,
        Nearest,
        "0.48593140",
        "0x0.7c6600#20",
        Less,
    );
    test(
        "0.600000000000000000022",
        "0x0.999999999999999a#64",
        64,
        Nearest,
        "0.204832764699133451655",
        "0x0.346feb898833de66c#64",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        20,
        Nearest,
        "2.5110214e-31",
        "0x5.17cc0E-26#20",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        53,
        Down,
        "2.5110222495574232e-31",
        "0x5.17cc1b7272208E-26#53",
        Less,
    );
}

#[test]
fn test_asin_pi_rational_prec_round() {
    let test = |s: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (t, o) = Float::asin_pi_rational_prec_round(x.clone(), prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = Float::asin_pi_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = Float::asin_pi_rational_prec(x.clone(), prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) = Float::asin_pi_rational_prec_ref(&x, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_t, rug_o) = rug_asin_with_period_rational_prec_round(&x, 2, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_t)),
                ComparableFloatRef(&t)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
    test("2", 10, Nearest, "NaN", "NaN", Equal);
    test("-2", 10, Nearest, "NaN", "NaN", Equal);
    test("1", 10, Exact, "0.50000", "0x0.800#10", Equal);
    test("1", 1, Exact, "0.50", "0x0.8#1", Equal);
    test("-1", 10, Exact, "-0.50000", "-0x0.800#10", Equal);
    test("1/2", 10, Floor, "0.16650", "0x0.2aa#10", Less);
    test("1/2", 10, Ceiling, "0.16675", "0x0.2ab#10", Greater);
    test("1/2", 10, Nearest, "0.16675", "0x0.2ab#10", Greater);
    test("3/5", 10, Floor, "0.20459", "0x0.346#10", Less);
    test("3/5", 10, Ceiling, "0.20483", "0x0.347#10", Greater);
    test(
        "3/5",
        53,
        Nearest,
        "0.20483276469913345",
        "0x0.346feb898833de#53",
        Less,
    );
    test(
        "-3/5",
        53,
        Nearest,
        "-0.20483276469913345",
        "-0x0.346feb898833de#53",
        Greater,
    );
    test("1/3", 20, Nearest, "0.10817349", "0x0.1bb142#20", Greater);
    test(
        "999999/1000000",
        20,
        Nearest,
        "0.49954987",
        "0x0.7fe280#20",
        Greater,
    );
    test(
        "1/1000000",
        20,
        Nearest,
        "3.1830996e-7",
        "0x5.57218E-6#20",
        Greater,
    );
}

#[test]
#[should_panic]
fn asin_pi_prec_round_fail_1() {
    Float::ONE.asin_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn asin_pi_prec_round_fail_2() {
    // asin(1/4)/pi is not exactly representable
    Float::from(0.25).asin_pi_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn asin_pi_rational_prec_round_fail() {
    Float::asin_pi_rational_prec_round(Rational::from_unsigneds(1u8, 4), 10, Exact);
}

#[test]
fn asin_pi_properties() {
    // The borrowed generators admit `Exact` for inputs whose arcsine is not exact, so `Exact` is
    // checked against the exactness of the result.
    let exact_ok = |x: &Float, prec: u64, rm: RoundingMode| {
        rm != Exact || x.asin_with_period_prec_round_ref(2, prec, Nearest).1 == Equal
    };
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        if !exact_ok(&x, prec, rm) {
            assert_panic!(x.asin_pi_prec_round_ref(prec, Exact));
            return;
        }
        let (t, o) = x.clone().asin_pi_prec_round(prec, rm);
        assert!(t.is_valid());
        assert_rounding_ordering_consistent(&t, rm, o);
        let (t_alt, o_alt) = x.asin_with_period_prec_round_ref(2, prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.asin_pi_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.asin_pi_prec_round_assign(prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        // |asin(x)/pi| <= 1/2, so the result never overflows
        assert!(!t.is_infinite());
        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_t, rug_o) =
                rug_asin_with_period_prec_round(&rug::Float::exact_from(&x), 2, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_t)),
                ComparableFloatRef(&t)
            );
            assert_eq!(rug_o, o);
        }
    });

    float_unsigned_rounding_mode_triple_gen_var_37().test_properties(|(x, prec, rm)| {
        if !exact_ok(&x, prec, rm) {
            return;
        }
        let (t, o) = x.asin_pi_prec_round_ref(prec, rm);
        let (t_alt, o_alt) = x.asin_with_period_prec_round_ref(2, prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (t, o) = x.clone().asin_pi_prec(prec);
        let (t_alt, o_alt) = x.asin_with_period_prec_ref(2, prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.asin_pi_prec_ref(prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.asin_pi_prec_assign(prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });

    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        if !exact_ok(&x, x.significant_bits(), rm) {
            return;
        }
        let (t, o) = x.clone().asin_pi_round(rm);
        assert_rounding_ordering_consistent(&t, rm, o);
        let (t_alt, o_alt) = x.asin_with_period_round_ref(2, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.asin_pi_round_ref(rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.asin_pi_round_assign(rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });

    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        if rm == Exact && Float::asin_with_period_rational_prec_ref(&x, 2, prec).1 != Equal {
            assert_panic!(Float::asin_pi_rational_prec_round_ref(&x, prec, Exact));
            return;
        }
        let (t, o) = Float::asin_pi_rational_prec_round(x.clone(), prec, rm);
        assert!(t.is_valid());
        assert_rounding_ordering_consistent(&t, rm, o);
        let (t_alt, o_alt) = Float::asin_with_period_rational_prec_round_ref(&x, 2, prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = Float::asin_pi_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_t, rug_o) = rug_asin_with_period_rational_prec_round(&x, 2, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_t)),
                ComparableFloatRef(&t)
            );
            assert_eq!(rug_o, o);
        }
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (t, o) = Float::asin_pi_rational_prec(x.clone(), prec);
        let (t_alt, o_alt) = Float::asin_with_period_rational_prec_ref(&x, 2, prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = Float::asin_pi_rational_prec_ref(&x, prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });

    float_gen().test_properties(|x| {
        let t = x.clone().asin_pi();
        assert!(t.is_valid());
        let t_alt = x.asin_pi_ref();
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let mut t_alt = x.clone();
        t_alt.asin_pi_assign();
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        // the same as asin_with_period with a period of 2, and as rounding to the input's
        // precision, to nearest
        let t_alt = x.asin_with_period_ref(2);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let (t_alt, _) = x.asin_pi_prec_round_ref(x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_asin_pi() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_asin_pi(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(2.0, f32::NAN);
    test::<f32>(-2.0, f32::NAN);
    test::<f32>(0.0, 0.0);
    test::<f32>(-0.0, -0.0);
    test::<f32>(1.0, 0.5);
    test::<f32>(-1.0, -0.5);
    test::<f32>(0.5, 0.16666667);
    test::<f32>(0.25, 0.08043063);
    test::<f32>(0.1, 0.03188428);
    test::<f32>(0.99, 0.4549466);
    test::<f32>(1.0e-30, 3.1830988e-31);
    test::<f32>(1.0e-45, 0.0);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(2.0, f64::NAN);
    test::<f64>(-2.0, f64::NAN);
    test::<f64>(0.0, 0.0);
    test::<f64>(-0.0, -0.0);
    test::<f64>(1.0, 0.5);
    test::<f64>(-1.0, -0.5);
    test::<f64>(0.5, 0.16666666666666666);
    test::<f64>(0.25, 0.08043062325516624);
    test::<f64>(0.1, 0.03188428042925993);
    test::<f64>(0.99, 0.4549465863555879);
    test::<f64>(1.0e-100, 3.183098861837907e-101);
    test::<f64>(5.0e-324, 0.0);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_asin_pi_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_asin_pi_rational::<T>(
                &Rational::from_str(s).unwrap()
            )),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 0.0);
    test::<f32>("1", 0.5);
    test::<f32>("-1", -0.5);
    test::<f32>("2", f32::NAN);
    test::<f32>("1/2", 0.16666667);
    test::<f32>("3/5", 0.20483276);
    test::<f32>("1/3", 0.108173445);
    test::<f32>("1/1000000", 3.1830987e-7);
    test::<f32>("1/100000000000000000000000000000000000000000000000000", 0.0);
    test::<f64>("0", 0.0);
    test::<f64>("1", 0.5);
    test::<f64>("-1", -0.5);
    test::<f64>("2", f64::NAN);
    test::<f64>("1/2", 0.16666666666666666);
    test::<f64>("3/5", 0.20483276469913345);
    test::<f64>("1/3", 0.10817344796939272);
    test::<f64>("1/1000000", 3.1830988618384374e-7);
    test::<f64>(
        "1/100000000000000000000000000000000000000000000000000",
        3.183098861837907e-51,
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_asin_pi_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        // the same as the period-2 version
        assert_eq!(
            NiceFloat(primitive_float_asin_pi(x)),
            NiceFloat(primitive_float_asin_with_period(x, 2))
        );
    });

    rational_gen().test_properties(|x| {
        assert_eq!(
            NiceFloat(primitive_float_asin_pi_rational::<T>(&x)),
            NiceFloat(primitive_float_asin_with_period_rational::<T>(&x, 2))
        );
    });
}

#[test]
fn primitive_float_asin_pi_properties() {
    apply_fn_to_primitive_floats!(primitive_float_asin_pi_properties_helper);
}
