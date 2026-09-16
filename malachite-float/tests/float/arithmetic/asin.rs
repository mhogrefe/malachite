// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Asin, AsinAssign};
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
use malachite_float::float::arithmetic::asin::primitive_float_asin;
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::asin::{rug_asin, rug_asin_prec_round};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_36,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::panic::catch_unwind;

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
