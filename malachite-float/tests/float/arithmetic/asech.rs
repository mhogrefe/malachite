// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Asech, AsechAssign, PowerOf2, Reciprocal};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, OneHalf, Two, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::asech::{
    primitive_float_asech, primitive_float_asech_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, round_once_to_primitive,
    rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::arithmetic::asech::{
    rug_asech, rug_asech_prec, rug_asech_prec_round, rug_asech_rational_prec,
    rug_asech_rational_prec_round, rug_asech_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_rounding_mode_triple_gen_var_36,
    float_unsigned_rounding_mode_triple_gen_var_54,
    rational_unsigned_rounding_mode_triple_gen_var_17,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_asech_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (c, o) = x.clone().asech_prec_round(prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.asech_prec_round_ref(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.asech_prec_round_assign(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_asech_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, Ceiling, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Ceiling, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Floor, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 1, Ceiling, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 1, Nearest, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 10, Floor, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 10, Nearest, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 1, Floor, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 1, Ceiling, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 1, Nearest, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 10, Floor, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 10, Nearest, "Infinity", "Infinity", Equal);
    test("1.0", "0x1.0#1", 1, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 1, Floor, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 1, Ceiling, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 1, Nearest, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Floor, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 1, Floor, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 1, Ceiling, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 1, Nearest, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Floor, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 1, Floor, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 1, Ceiling, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 1, Nearest, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 10, Floor, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 10, Ceiling, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 10, Nearest, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 1, Floor, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 1, Ceiling, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 1, Nearest, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, Floor, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, Nearest, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 1, Floor, "1.0", "0x1.0#1", Less);
    test("0.50", "0x0.8#1", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("0.50", "0x0.8#1", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("0.50", "0x0.8#1", 10, Floor, "1.3164", "0x1.510#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Ceiling,
        "1.3184",
        "0x1.518#10",
        Greater,
    );
    test("0.50", "0x0.8#1", 10, Nearest, "1.3164", "0x1.510#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Floor,
        "1.3169578969248167086250463473073",
        "0x1.51242719804349be684bd0188#100",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        100,
        Ceiling,
        "1.3169578969248167086250463473089",
        "0x1.51242719804349be684bd018a#100",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        100,
        Nearest,
        "1.3169578969248167086250463473073",
        "0x1.51242719804349be684bd0188#100",
        Less,
    );
    test("0.25", "0x0.4#1", 1, Floor, "2.0", "0x2.0#1", Less);
    test("0.25", "0x0.4#1", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("0.25", "0x0.4#1", 1, Nearest, "2.0", "0x2.0#1", Less);
    test("0.25", "0x0.4#1", 10, Floor, "2.0625", "0x2.10#10", Less);
    test(
        "0.25",
        "0x0.4#1",
        10,
        Ceiling,
        "2.0664",
        "0x2.11#10",
        Greater,
    );
    test("0.25", "0x0.4#1", 10, Nearest, "2.0625", "0x2.10#10", Less);
    test(
        "0.25",
        "0x0.4#1",
        100,
        Floor,
        "2.0634370688955605467272811726174",
        "0x2.103d696842b22f5e1a6f5e2fc#100",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        100,
        Ceiling,
        "2.0634370688955605467272811726205",
        "0x2.103d696842b22f5e1a6f5e300#100",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        100,
        Nearest,
        "2.0634370688955605467272811726205",
        "0x2.103d696842b22f5e1a6f5e300#100",
        Greater,
    );
    test("0.75", "0x0.c#2", 1, Floor, "0.50", "0x0.8#1", Less);
    test("0.75", "0x0.c#2", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("0.75", "0x0.c#2", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("0.75", "0x0.c#2", 10, Floor, "0.79492", "0x0.cb8#10", Less);
    test(
        "0.75",
        "0x0.c#2",
        10,
        Ceiling,
        "0.79590",
        "0x0.cbc#10",
        Greater,
    );
    test(
        "0.75",
        "0x0.c#2",
        10,
        Nearest,
        "0.79492",
        "0x0.cb8#10",
        Less,
    );
    test(
        "0.75",
        "0x0.c#2",
        100,
        Floor,
        "0.79536546122390563052789093314737",
        "0x0.cb9d1224531b01542c78186b6#100",
        Less,
    );
    test(
        "0.75",
        "0x0.c#2",
        100,
        Ceiling,
        "0.79536546122390563052789093314815",
        "0x0.cb9d1224531b01542c78186b7#100",
        Greater,
    );
    test(
        "0.75",
        "0x0.c#2",
        100,
        Nearest,
        "0.79536546122390563052789093314815",
        "0x0.cb9d1224531b01542c78186b7#100",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Floor,
        "1.7617",
        "0x1.c30#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Ceiling,
        "1.7637",
        "0x1.c38#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Nearest,
        "1.7637",
        "0x1.c38#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        Floor,
        "1.7627471740390860504652186499578",
        "0x1.c34366179d426cc1b1f33d1b8#100",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        Ceiling,
        "1.7627471740390860504652186499594",
        "0x1.c34366179d426cc1b1f33d1ba#100",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        Nearest,
        "1.7627471740390860504652186499594",
        "0x1.c34366179d426cc1b1f33d1ba#100",
        Greater,
    );
    test(
        "0.89999999999999999999999999999968",
        "0x0.e666666666666666666666666#100",
        1,
        Floor,
        "0.25",
        "0x0.4#1",
        Less,
    );
    test(
        "0.89999999999999999999999999999968",
        "0x0.e666666666666666666666666#100",
        1,
        Ceiling,
        "0.50",
        "0x0.8#1",
        Greater,
    );
    test(
        "0.89999999999999999999999999999968",
        "0x0.e666666666666666666666666#100",
        1,
        Nearest,
        "0.50",
        "0x0.8#1",
        Greater,
    );
    test(
        "0.89999999999999999999999999999968",
        "0x0.e666666666666666666666666#100",
        10,
        Floor,
        "0.46680",
        "0x0.778#10",
        Less,
    );
    test(
        "0.89999999999999999999999999999968",
        "0x0.e666666666666666666666666#100",
        10,
        Ceiling,
        "0.46729",
        "0x0.77a#10",
        Greater,
    );
    test(
        "0.89999999999999999999999999999968",
        "0x0.e666666666666666666666666#100",
        10,
        Nearest,
        "0.46729",
        "0x0.77a#10",
        Greater,
    );
    test(
        "0.89999999999999999999999999999968",
        "0x0.e666666666666666666666666#100",
        100,
        Floor,
        "0.46714530810326201812838149166636",
        "0x0.7796d5bcc889e5ed25b0b01e28#100",
        Less,
    );
    test(
        "0.89999999999999999999999999999968",
        "0x0.e666666666666666666666666#100",
        100,
        Ceiling,
        "0.46714530810326201812838149166676",
        "0x0.7796d5bcc889e5ed25b0b01e30#100",
        Greater,
    );
    test(
        "0.89999999999999999999999999999968",
        "0x0.e666666666666666666666666#100",
        100,
        Nearest,
        "0.46714530810326201812838149166676",
        "0x0.7796d5bcc889e5ed25b0b01e30#100",
        Greater,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        1,
        Floor,
        "0.12",
        "0x0.2#1",
        Less,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        1,
        Ceiling,
        "0.25",
        "0x0.4#1",
        Greater,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        1,
        Nearest,
        "0.12",
        "0x0.2#1",
        Less,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        10,
        Floor,
        "0.14185",
        "0x0.245#10",
        Less,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        10,
        Ceiling,
        "0.14209",
        "0x0.246#10",
        Greater,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        10,
        Nearest,
        "0.14209",
        "0x0.246#10",
        Greater,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        100,
        Floor,
        "0.14201444074607706874579679430085",
        "0x0.245b0ef2906ae121d0802cb098#100",
        Less,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        100,
        Ceiling,
        "0.14201444074607706874579679430105",
        "0x0.245b0ef2906ae121d0802cb09c#100",
        Greater,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        100,
        Nearest,
        "0.14201444074607706874579679430105",
        "0x0.245b0ef2906ae121d0802cb09c#100",
        Greater,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        1,
        Floor,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        1,
        Ceiling,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        10,
        Floor,
        "2.9922",
        "0x2.fe#10",
        Less,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        10,
        Ceiling,
        "2.9961",
        "0x2.ff#10",
        Greater,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        10,
        Nearest,
        "2.9922",
        "0x2.fe#10",
        Less,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        100,
        Floor,
        "2.9932228461263808979126677137722",
        "0x2.fe43da39c0ba87f9072bef0f4#100",
        Less,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        100,
        Ceiling,
        "2.9932228461263808979126677137753",
        "0x2.fe43da39c0ba87f9072bef0f8#100",
        Greater,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        100,
        Nearest,
        "2.9932228461263808979126677137753",
        "0x2.fe43da39c0ba87f9072bef0f8#100",
        Greater,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        1,
        Floor,
        "8.9e-16",
        "0x4.0E-13#1",
        Less,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        1,
        Ceiling,
        "1.8e-15",
        "0x8.0E-13#1",
        Greater,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        1,
        Nearest,
        "8.9e-16",
        "0x4.0E-13#1",
        Less,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        10,
        Floor,
        "1.2559e-15",
        "0x5.a8E-13#10",
        Less,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        10,
        Ceiling,
        "1.2577e-15",
        "0x5.aaE-13#10",
        Greater,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        10,
        Nearest,
        "1.2559e-15",
        "0x5.a8E-13#10",
        Less,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        100,
        Floor,
        "1.2560739669470200475147058975704e-15",
        "0x5.a827999fcef32422cbec4d9b8E-13#100",
        Less,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        100,
        Ceiling,
        "1.2560739669470200475147058975718e-15",
        "0x5.a827999fcef32422cbec4d9c0E-13#100",
        Greater,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        100,
        Nearest,
        "1.2560739669470200475147058975718e-15",
        "0x5.a827999fcef32422cbec4d9c0E-13#100",
        Greater,
    );
    test(
        "1.6363644884325755176985906516627e-91",
        "0x5.5555555555555555555555558E-76#100",
        1,
        Floor,
        "1.3e2",
        "0x8.0E+1#1",
        Less,
    );
    test(
        "1.6363644884325755176985906516627e-91",
        "0x5.5555555555555555555555558E-76#100",
        1,
        Ceiling,
        "2.6e2",
        "0x1.0E+2#1",
        Greater,
    );
    test(
        "1.6363644884325755176985906516627e-91",
        "0x5.5555555555555555555555558E-76#100",
        1,
        Nearest,
        "2.6e2",
        "0x1.0E+2#1",
        Greater,
    );
    test(
        "1.6363644884325755176985906516627e-91",
        "0x5.5555555555555555555555558E-76#100",
        10,
        Floor,
        "209.50",
        "0xd1.8#10",
        Less,
    );
    test(
        "1.6363644884325755176985906516627e-91",
        "0x5.5555555555555555555555558E-76#100",
        10,
        Ceiling,
        "209.75",
        "0xd1.c#10",
        Greater,
    );
    test(
        "1.6363644884325755176985906516627e-91",
        "0x5.5555555555555555555555558E-76#100",
        10,
        Nearest,
        "209.75",
        "0xd1.c#10",
        Greater,
    );
    test(
        "1.6363644884325755176985906516627e-91",
        "0x5.5555555555555555555555558E-76#100",
        100,
        Floor,
        "209.73591363721164782598211379581",
        "0xd1.bc64d60c8122b87304d3b71#100",
        Less,
    );
    test(
        "1.6363644884325755176985906516627e-91",
        "0x5.5555555555555555555555558E-76#100",
        100,
        Ceiling,
        "209.73591363721164782598211379601",
        "0xd1.bc64d60c8122b87304d3b72#100",
        Greater,
    );
    test(
        "1.6363644884325755176985906516627e-91",
        "0x5.5555555555555555555555558E-76#100",
        100,
        Nearest,
        "209.73591363721164782598211379581",
        "0xd1.bc64d60c8122b87304d3b71#100",
        Less,
    );
    test("0.00098", "0x0.004#1", 1, Floor, "4.0", "0x4.0#1", Less);
    test(
        "0.00098",
        "0x0.004#1",
        1,
        Ceiling,
        "8.0",
        "0x8.0#1",
        Greater,
    );
    test(
        "0.00098",
        "0x0.004#1",
        1,
        Nearest,
        "8.0",
        "0x8.0#1",
        Greater,
    );
    test(
        "0.00098",
        "0x0.004#1",
        10,
        Floor,
        "7.6172",
        "0x7.9e#10",
        Less,
    );
    test(
        "0.00098",
        "0x0.004#1",
        10,
        Ceiling,
        "7.6250",
        "0x7.a0#10",
        Greater,
    );
    test(
        "0.00098",
        "0x0.004#1",
        10,
        Nearest,
        "7.6250",
        "0x7.a0#10",
        Greater,
    );
    test(
        "0.00098",
        "0x0.004#1",
        100,
        Floor,
        "7.6246187477407340368535870052204",
        "0x7.9fe703a603d23a60d77359728#100",
        Less,
    );
    test(
        "0.00098",
        "0x0.004#1",
        100,
        Ceiling,
        "7.6246187477407340368535870052267",
        "0x7.9fe703a603d23a60d77359730#100",
        Greater,
    );
    test(
        "0.00098",
        "0x0.004#1",
        100,
        Nearest,
        "7.6246187477407340368535870052267",
        "0x7.9fe703a603d23a60d77359730#100",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Floor,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Ceiling,
        "1.1e9",
        "0x4.0E+7#1",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Nearest,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        10,
        Floor,
        "7.4344e8",
        "0x2.c5E+7#10",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        10,
        Ceiling,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        10,
        Nearest,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        100,
        Floor,
        "744261118.64804019843384850454251",
        "0x2c5c85fe.a5e5f662c4486691c8#100",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        100,
        Ceiling,
        "744261118.64804019843384850454336",
        "0x2c5c85fe.a5e5f662c4486691cc#100",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        100,
        Nearest,
        "744261118.64804019843384850454336",
        "0x2c5c85fe.a5e5f662c4486691cc#100",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Floor,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Ceiling,
        "1.1e9",
        "0x4.0E+7#1",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Nearest,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Floor,
        "7.4344e8",
        "0x2.c5E+7#10",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Ceiling,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Nearest,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        100,
        Floor,
        "744261118.64804019843384850454251",
        "0x2c5c85fe.a5e5f662c4486691c8#100",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        100,
        Ceiling,
        "744261118.64804019843384850454336",
        "0x2c5c85fe.a5e5f662c4486691cc#100",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        100,
        Nearest,
        "744261118.64804019843384850454336",
        "0x2c5c85fe.a5e5f662c4486691cc#100",
        Greater,
    );
    // - x^2 < 2^(1 - wp), so ln 2 - ln(x) is used, and its first approximation does not allow
    //   rounding, so the loop retries
    test("0.000671", "0x0.002c#4", 4, Up, "8.00", "0x8.0#4", Greater);
    // - x < 1/2 but not tiny, and the first approximation of ln(1 + sqrt(1 - x^2)) - ln(x) does not
    //   allow rounding, so the loop retries
    test(
        "0.03174",
        "0x0.082#7",
        30,
        Down,
        "4.1431269646",
        "0x4.24a3f80#30",
        Less,
    );
    // - 1/2 <= x < 1, and the first approximation of ln(1 + u) does not allow rounding, so the loop
    //   retries
    test("0.9922", "0x0.fe#7", 1, Down, "0.12", "0x0.2#1", Less);
}

#[test]
#[should_panic]
fn asech_prec_round_fail() {
    Float::ONE_HALF.asech_prec_round(0, Nearest);
}

#[test]
#[should_panic]
fn asech_prec_round_exact_fail() {
    Float::ONE_HALF.asech_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn asech_prec_fail() {
    Float::ONE_HALF.asech_prec(0);
}

#[test]
#[should_panic]
fn asech_round_fail() {
    Float::ONE_HALF.asech_round(Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn asech_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (c, o) = x.clone().asech_prec_round(prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = x.asech_prec_round_ref(prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.asech_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_asech_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    // asech is NaN for NaN, both infinities, and every x outside [0, 1], +inf at +/-0, and
    // otherwise nonnegative
    if x.is_nan() || x.is_infinite() || (x != 0u32 && !(0u32..=1u32).contains(&x)) {
        assert!(c.is_nan());
    } else if x == 0u32 {
        assert_eq!(ComparableFloat(c.clone()), ComparableFloat(Float::INFINITY));
    } else {
        assert!(c.is_sign_positive());
        // asech(x) = acosh(1/x), and the reciprocal of a positive Float is an exact Rational, so
        // the independent Rational inverse hyperbolic cosine must agree
        if rm != Exact {
            let (c_alt, o_alt) =
                Float::acosh_rational_prec_round(Rational::exact_from(&x).reciprocal(), prec, rm);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }
    }
    if c.is_normal() {
        assert_eq!(c.get_prec(), Some(prec));
    }

    if o == Equal {
        // asech is exact only for x = 1 (and for the inputs whose result is NaN or inf): the result
        // is rounding-mode-invariant
        for rm2 in exhaustive_rounding_modes() {
            let (c2, o2) = x.asech_prec_round_ref(prec, rm2);
            assert_eq!(ComparableFloat(c2), ComparableFloat(c.clone()));
            assert_eq!(o2, Equal);
        }
    } else {
        assert_panic!(x.asech_prec_round_ref(prec, Exact));
    }
}

#[test]
fn asech_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        asech_prec_round_properties_helper(x, prec, rm);
    });

    float_unsigned_rounding_mode_triple_gen_var_54().test_properties(|(x, prec, rm)| {
        asech_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        for x in [Float::NAN, Float::INFINITY, Float::NEGATIVE_INFINITY, Float::TWO, -Float::ONE] {
            let (c, o) = x.asech_prec_round(prec, rm);
            assert!(c.is_nan());
            assert_eq!(o, Equal);
        }
        for (x, out) in [
            (Float::ZERO, Float::INFINITY),
            (Float::NEGATIVE_ZERO, Float::INFINITY),
            (Float::ONE, Float::ZERO),
        ] {
            let (c, o) = x.asech_prec_round(prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(out));
            assert_eq!(o, Equal);
        }
    });
}

#[test]
fn asech_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        let (c, o) = x.clone().asech_round(rm);
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&c, rm, o);
        let (c_alt, o_alt) = x.asech_round_ref(rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.asech_round_assign(rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (c_alt, o_alt) = x.asech_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_asech_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    });
}

#[test]
fn asech_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (c, o) = x.clone().asech_prec(prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.asech_prec_ref(prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.asech_prec_assign(prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (c_alt, o_alt) = x.asech_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (rug_c, rug_o) = rug_asech_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn asech_properties_helper(x: Float) {
    let c = x.clone().asech();
    assert!(c.is_valid());
    let c_alt = (&x).asech();
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

    let mut x_alt = x.clone();
    x_alt.asech_assign();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));

    let c_alt = x.asech_prec_round_ref(x.significant_bits(), Nearest).0;
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

    assert_eq!(
        ComparableFloatRef(&<Float as From<&rug::Float>>::from(&rug_asech(
            &rug::Float::exact_from(&x)
        ))),
        ComparableFloatRef(&c)
    );

    assert_eq!(
        c.is_nan(),
        x.is_nan() || x.is_infinite() || (x != 0u32 && !(0u32..=1u32).contains(&x))
    );
}

#[test]
fn asech_properties() {
    float_gen().test_properties(|x| {
        asech_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        asech_properties_helper(x);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_asech() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_asech(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, f32::INFINITY);
    test::<f32>(-0.0, f32::INFINITY);
    test::<f32>(1.0, 0.0);
    test::<f32>(-1.0, f32::NAN);
    test::<f32>(2.0, f32::NAN);
    test::<f32>(0.5, 1.316958);
    test::<f32>(0.1, 2.9932227);
    test::<f32>(0.99999994, 0.00034526698);
    test::<f32>(1.0e-10, 23.718998);
    test::<f32>(1.0e-45, 103.97208);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, f64::INFINITY);
    test::<f64>(-0.0, f64::INFINITY);
    test::<f64>(1.0, 0.0);
    test::<f64>(-1.0, f64::NAN);
    test::<f64>(2.0, f64::NAN);
    test::<f64>(0.5, 1.3169578969248168);
    test::<f64>(0.1, 2.993222846126381);
    test::<f64>(0.9999999999999999, 1.4901161193847656e-8);
    test::<f64>(1.0e-100, 230.95165647996453);
    test::<f64>(5.0e-324, 745.1332191019412);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_asech_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let c = primitive_float_asech(x);
        // asech is NaN for NaN and every x outside [0, 1], +inf at +/-0, and otherwise nonnegative
        assert_eq!(
            c.is_nan(),
            x.is_nan() || (x != T::ZERO && !(T::ZERO..=T::ONE).contains(&x))
        );
        if x > T::ZERO && x <= T::ONE {
            assert!(c >= T::ZERO);
            // the result is the correctly rounded inverse hyperbolic secant, as given by the oracle
            // with 64 bits to spare
            let rug_c: T = round_once_to_primitive(|p| {
                <Float as From<&rug::Float>>::from(
                    &rug_asech_prec(&rug::Float::exact_from(&Float::from(x)), p).0,
                )
            });
            assert_eq!(NiceFloat(rug_c), NiceFloat(c));
        }
    });
}

#[test]
fn primitive_float_asech_properties() {
    apply_fn_to_primitive_floats!(primitive_float_asech_properties_helper);
}

#[test]
fn test_asech_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::asech_rational_prec_round(x.clone(), prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);

        let (c, o) = Float::asech_rational_prec_round_ref(&x, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_asech_rational_prec_round(&x, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 1, Floor, "Infinity", "Infinity", Equal);
    test("0", 1, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 1, Down, "Infinity", "Infinity", Equal);
    test("0", 1, Up, "Infinity", "Infinity", Equal);
    test("0", 1, Nearest, "Infinity", "Infinity", Equal);
    test("0", 1, Exact, "Infinity", "Infinity", Equal);
    test("0", 10, Floor, "Infinity", "Infinity", Equal);
    test("0", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 10, Down, "Infinity", "Infinity", Equal);
    test("0", 10, Up, "Infinity", "Infinity", Equal);
    test("0", 10, Nearest, "Infinity", "Infinity", Equal);
    test("0", 10, Exact, "Infinity", "Infinity", Equal);
    test("1", 1, Floor, "0.0", "0x0.0", Equal);
    test("1", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("1", 1, Down, "0.0", "0x0.0", Equal);
    test("1", 1, Up, "0.0", "0x0.0", Equal);
    test("1", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1", 1, Exact, "0.0", "0x0.0", Equal);
    test("1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1", 10, Down, "0.0", "0x0.0", Equal);
    test("1", 10, Up, "0.0", "0x0.0", Equal);
    test("1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1", 10, Exact, "0.0", "0x0.0", Equal);
    test("-1", 1, Floor, "NaN", "NaN", Equal);
    test("-1", 1, Ceiling, "NaN", "NaN", Equal);
    test("-1", 1, Down, "NaN", "NaN", Equal);
    test("-1", 1, Up, "NaN", "NaN", Equal);
    test("-1", 1, Nearest, "NaN", "NaN", Equal);
    test("-1", 1, Exact, "NaN", "NaN", Equal);
    test("-1", 10, Floor, "NaN", "NaN", Equal);
    test("-1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1", 10, Down, "NaN", "NaN", Equal);
    test("-1", 10, Up, "NaN", "NaN", Equal);
    test("-1", 10, Nearest, "NaN", "NaN", Equal);
    test("-1", 10, Exact, "NaN", "NaN", Equal);
    test("2", 1, Floor, "NaN", "NaN", Equal);
    test("2", 1, Ceiling, "NaN", "NaN", Equal);
    test("2", 1, Down, "NaN", "NaN", Equal);
    test("2", 1, Up, "NaN", "NaN", Equal);
    test("2", 1, Nearest, "NaN", "NaN", Equal);
    test("2", 1, Exact, "NaN", "NaN", Equal);
    test("2", 10, Floor, "NaN", "NaN", Equal);
    test("2", 10, Ceiling, "NaN", "NaN", Equal);
    test("2", 10, Down, "NaN", "NaN", Equal);
    test("2", 10, Up, "NaN", "NaN", Equal);
    test("2", 10, Nearest, "NaN", "NaN", Equal);
    test("2", 10, Exact, "NaN", "NaN", Equal);
    test("-1/2", 1, Floor, "NaN", "NaN", Equal);
    test("-1/2", 1, Ceiling, "NaN", "NaN", Equal);
    test("-1/2", 1, Down, "NaN", "NaN", Equal);
    test("-1/2", 1, Up, "NaN", "NaN", Equal);
    test("-1/2", 1, Nearest, "NaN", "NaN", Equal);
    test("-1/2", 1, Exact, "NaN", "NaN", Equal);
    test("-1/2", 10, Floor, "NaN", "NaN", Equal);
    test("-1/2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1/2", 10, Down, "NaN", "NaN", Equal);
    test("-1/2", 10, Up, "NaN", "NaN", Equal);
    test("-1/2", 10, Nearest, "NaN", "NaN", Equal);
    test("-1/2", 10, Exact, "NaN", "NaN", Equal);
    test("1/2", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1/2", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1/2", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1/2", 10, Floor, "1.3164", "0x1.510#10", Less);
    test("1/2", 10, Ceiling, "1.3184", "0x1.518#10", Greater);
    test("1/2", 10, Nearest, "1.3164", "0x1.510#10", Less);
    test(
        "1/2",
        100,
        Floor,
        "1.3169578969248167086250463473073",
        "0x1.51242719804349be684bd0188#100",
        Less,
    );
    test(
        "1/2",
        100,
        Ceiling,
        "1.3169578969248167086250463473089",
        "0x1.51242719804349be684bd018a#100",
        Greater,
    );
    test(
        "1/2",
        100,
        Nearest,
        "1.3169578969248167086250463473073",
        "0x1.51242719804349be684bd0188#100",
        Less,
    );
    test("1/3", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1/3", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1/3", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("1/3", 10, Floor, "1.7617", "0x1.c30#10", Less);
    test("1/3", 10, Ceiling, "1.7637", "0x1.c38#10", Greater);
    test("1/3", 10, Nearest, "1.7637", "0x1.c38#10", Greater);
    test(
        "1/3",
        100,
        Floor,
        "1.7627471740390860504652186499594",
        "0x1.c34366179d426cc1b1f33d1ba#100",
        Less,
    );
    test(
        "1/3",
        100,
        Ceiling,
        "1.7627471740390860504652186499609",
        "0x1.c34366179d426cc1b1f33d1bc#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "1.7627471740390860504652186499594",
        "0x1.c34366179d426cc1b1f33d1ba#100",
        Less,
    );
    test("3/4", 1, Floor, "0.50", "0x0.8#1", Less);
    test("3/4", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("3/4", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("3/4", 10, Floor, "0.79492", "0x0.cb8#10", Less);
    test("3/4", 10, Ceiling, "0.79590", "0x0.cbc#10", Greater);
    test("3/4", 10, Nearest, "0.79492", "0x0.cb8#10", Less);
    test(
        "3/4",
        100,
        Floor,
        "0.79536546122390563052789093314737",
        "0x0.cb9d1224531b01542c78186b6#100",
        Less,
    );
    test(
        "3/4",
        100,
        Ceiling,
        "0.79536546122390563052789093314815",
        "0x0.cb9d1224531b01542c78186b7#100",
        Greater,
    );
    test(
        "3/4",
        100,
        Nearest,
        "0.79536546122390563052789093314815",
        "0x0.cb9d1224531b01542c78186b7#100",
        Greater,
    );
    test("22/23", 1, Floor, "0.25", "0x0.4#1", Less);
    test("22/23", 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("22/23", 1, Nearest, "0.25", "0x0.4#1", Less);
    test("22/23", 10, Floor, "0.30029", "0x0.4ce#10", Less);
    test("22/23", 10, Ceiling, "0.30078", "0x0.4d0#10", Greater);
    test("22/23", 10, Nearest, "0.30029", "0x0.4ce#10", Less);
    test(
        "22/23",
        100,
        Floor,
        "0.30038078099025220704452751426386",
        "0x0.4ce5c13eb337e687a40a4f2210#100",
        Less,
    );
    test(
        "22/23",
        100,
        Ceiling,
        "0.30038078099025220704452751426426",
        "0x0.4ce5c13eb337e687a40a4f2218#100",
        Greater,
    );
    test(
        "22/23",
        100,
        Nearest,
        "0.30038078099025220704452751426386",
        "0x0.4ce5c13eb337e687a40a4f2210#100",
        Less,
    );
    test("99/100", 1, Floor, "0.12", "0x0.2#1", Less);
    test("99/100", 1, Ceiling, "0.25", "0x0.4#1", Greater);
    test("99/100", 1, Nearest, "0.12", "0x0.2#1", Less);
    test("99/100", 10, Floor, "0.14185", "0x0.245#10", Less);
    test("99/100", 10, Ceiling, "0.14209", "0x0.246#10", Greater);
    test("99/100", 10, Nearest, "0.14209", "0x0.246#10", Greater);
    test(
        "99/100",
        100,
        Floor,
        "0.14201444074607706874579679429947",
        "0x0.245b0ef2906ae121d0802cb07c#100",
        Less,
    );
    test(
        "99/100",
        100,
        Ceiling,
        "0.14201444074607706874579679429967",
        "0x0.245b0ef2906ae121d0802cb080#100",
        Greater,
    );
    test(
        "99/100",
        100,
        Nearest,
        "0.14201444074607706874579679429967",
        "0x0.245b0ef2906ae121d0802cb080#100",
        Greater,
    );
    test("1/10", 1, Floor, "2.0", "0x2.0#1", Less);
    test("1/10", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("1/10", 1, Nearest, "2.0", "0x2.0#1", Less);
    test("1/10", 10, Floor, "2.9922", "0x2.fe#10", Less);
    test("1/10", 10, Ceiling, "2.9961", "0x2.ff#10", Greater);
    test("1/10", 10, Nearest, "2.9922", "0x2.fe#10", Less);
    test(
        "1/10",
        100,
        Floor,
        "2.9932228461263808979126677137722",
        "0x2.fe43da39c0ba87f9072bef0f4#100",
        Less,
    );
    test(
        "1/10",
        100,
        Ceiling,
        "2.9932228461263808979126677137753",
        "0x2.fe43da39c0ba87f9072bef0f8#100",
        Greater,
    );
    test(
        "1/10",
        100,
        Nearest,
        "2.9932228461263808979126677137753",
        "0x2.fe43da39c0ba87f9072bef0f8#100",
        Greater,
    );
    test("1/1000000", 1, Floor, "8.0", "0x8.0#1", Less);
    test("1/1000000", 1, Ceiling, "16.0", "0x1.0E+1#1", Greater);
    test("1/1000000", 1, Nearest, "16.0", "0x1.0E+1#1", Greater);
    test("1/1000000", 10, Floor, "14.500", "0xe.80#10", Less);
    test("1/1000000", 10, Ceiling, "14.516", "0xe.84#10", Greater);
    test("1/1000000", 10, Nearest, "14.516", "0xe.84#10", Greater);
    test(
        "1/1000000",
        100,
        Floor,
        "14.508657738523969413525180755811",
        "0xe.823764bfd1593d6bbd500264#100",
        Less,
    );
    test(
        "1/1000000",
        100,
        Ceiling,
        "14.508657738523969413525180755824",
        "0xe.823764bfd1593d6bbd500265#100",
        Greater,
    );
    test(
        "1/1000000",
        100,
        Nearest,
        "14.508657738523969413525180755811",
        "0xe.823764bfd1593d6bbd500264#100",
        Less,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        1,
        Floor,
        "8.9e-16",
        "0x4.0E-13#1",
        Less,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        1,
        Ceiling,
        "1.8e-15",
        "0x8.0E-13#1",
        Greater,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        1,
        Nearest,
        "8.9e-16",
        "0x4.0E-13#1",
        Less,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        10,
        Floor,
        "1.2559e-15",
        "0x5.a8E-13#10",
        Less,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        10,
        Ceiling,
        "1.2577e-15",
        "0x5.aaE-13#10",
        Greater,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        10,
        Nearest,
        "1.2559e-15",
        "0x5.a8E-13#10",
        Less,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        100,
        Floor,
        "1.2560739669470200475147058975704e-15",
        "0x5.a827999fcef32422cbec4d9b8E-13#100",
        Less,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        100,
        Ceiling,
        "1.2560739669470200475147058975718e-15",
        "0x5.a827999fcef32422cbec4d9c0E-13#100",
        Greater,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        100,
        Nearest,
        "1.2560739669470200475147058975718e-15",
        "0x5.a827999fcef32422cbec4d9c0E-13#100",
        Greater,
    );
}

#[test]
fn test_asech_rational_extreme() {
    // x = 2^(-2^30) is below the smallest positive `Float`; asech(x) = acosh(2^(2^30)) lies within
    // 2^(-2^31) of ln 2^(2^30 + 1), far below an ulp of the result
    let x = Rational::power_of_2(-(1i64 << 30));
    let two_over_x = Rational::power_of_2((1i64 << 30) + 1);
    for rm in [Floor, Ceiling, Down, Up, Nearest] {
        let (c, o) = Float::asech_rational_prec_round_ref(&x, 100, rm);
        let (l, o_l) = Float::ln_rational_prec_round_ref(&two_over_x, 100, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(l));
        assert_eq!(o, o_l);
    }
}

#[test]
#[should_panic]
fn asech_rational_prec_fail() {
    Float::asech_rational_prec(Rational::ONE_HALF, 0);
}

#[test]
#[should_panic]
fn asech_rational_prec_ref_fail() {
    Float::asech_rational_prec_ref(&Rational::ONE_HALF, 0);
}

#[test]
#[should_panic]
fn asech_rational_prec_round_fail_1() {
    Float::asech_rational_prec_round(Rational::ONE_HALF, 0, Floor);
}

#[test]
#[should_panic]
fn asech_rational_prec_round_fail_2() {
    Float::asech_rational_prec_round(Rational::ONE_HALF, 10, Exact);
}

#[test]
#[should_panic]
fn asech_rational_prec_round_ref_fail() {
    Float::asech_rational_prec_round_ref(&Rational::ONE_HALF, 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn asech_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (c, o) = Float::asech_rational_prec_round(x.clone(), prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = Float::asech_rational_prec_round_ref(&x, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    // asech is NaN outside [0, 1], +inf at 0, and otherwise nonnegative
    if !(0u32..=1u32).contains(&x) {
        assert!(c.is_nan());
    } else if x == 0u32 {
        assert_eq!(ComparableFloat(c.clone()), ComparableFloat(Float::INFINITY));
    } else {
        assert!(c >= 0u32);
    }

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_asech_rational_prec_round(&x, prec, rrm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    // the inverse hyperbolic secant of an exactly representable rational is the Float inverse
    // hyperbolic secant, computed by an independent algorithm
    if let Ok(f) = Float::try_from(&x)
        && (rm != Exact || o == Equal)
    {
        let (c_alt, o_alt) = f.asech_prec_round(prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    }

    if c.is_normal() {
        assert_eq!(c.get_prec(), Some(prec));
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = Float::asech_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::asech_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn asech_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_17().test_properties(|(x, prec, rm)| {
        asech_rational_prec_round_properties_helper(x, prec, rm);
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        if x <= 0u32 || x > 1u32 {
            for rm in exhaustive_rounding_modes() {
                asech_rational_prec_round_properties_helper(x.clone(), prec, rm);
            }
        }
    });
}

#[allow(clippy::needless_pass_by_value)]
fn asech_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (c, o) = Float::asech_rational_prec(x.clone(), prec);
    assert!(c.is_valid());

    let (c_alt, o_alt) = Float::asech_rational_prec_ref(&x, prec);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let (c_alt, o_alt) = Float::asech_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let (rug_c, rug_o) = rug_asech_rational_prec(&x, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_c)),
        ComparableFloatRef(&c)
    );
    assert_eq!(rug_o, o);
}

#[test]
fn asech_rational_prec_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_17().test_properties(|(x, prec, _)| {
        asech_rational_prec_properties_helper(x, prec);
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        asech_rational_prec_properties_helper(x, prec);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_asech_rational() {
    fn test<T: PrimitiveFloat>(x: &Rational, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_asech_rational::<T>(x)),
            NiceFloat(out)
        );
    }
    let test_s = |s: &str| Rational::from_str(s).unwrap();
    test::<f32>(&test_s("0"), f32::INFINITY);
    test::<f64>(&test_s("0"), f64::INFINITY);
    test::<f32>(&test_s("1"), 0.0);
    test::<f64>(&test_s("1"), 0.0);
    test::<f32>(&test_s("-1"), f32::NAN);
    test::<f64>(&test_s("-1"), f64::NAN);
    test::<f32>(&test_s("2"), f32::NAN);
    test::<f64>(&test_s("2"), f64::NAN);
    test::<f32>(&test_s("1/2"), 1.316958);
    test::<f64>(&test_s("1/2"), 1.3169578969248168);
    test::<f32>(&test_s("1/3"), 1.7627472);
    test::<f64>(&test_s("1/3"), 1.762747174039086);
    test::<f32>(&test_s("99/100"), 0.14201444);
    test::<f64>(&test_s("99/100"), 0.14201444074607708);
    test::<f32>(&test_s("1/100000000000000000000"), 46.74485);
    test::<f64>(&test_s("1/100000000000000000000"), 46.74484904044086);
    test::<f32>(
        &test_s("1/1000000000000000000000000000000000000000000000000000000000000"),
        138.84825,
    );
    test::<f64>(
        &test_s("1/1000000000000000000000000000000000000000000000000000000000000"),
        138.84825276020268,
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_asech_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let c = primitive_float_asech_rational::<T>(&x);
        // the inverse hyperbolic secant of a rational is NaN exactly outside [0, 1]
        assert_eq!(c.is_nan(), !(0u32..=1u32).contains(&x));
        if x > 0u32 && x <= 1u32 {
            let rug_c: T = round_once_to_primitive(|p| {
                <Float as From<&rug::Float>>::from(&rug_asech_rational_prec(&x, p).0)
            });
            assert_eq!(NiceFloat(rug_c), NiceFloat(c));
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // The inverse hyperbolic secant of a finite primitive float, taken through the `Rational`
        // path, matches the direct primitive-float inverse hyperbolic secant.
        if x.is_finite() {
            assert_eq!(
                NiceFloat(primitive_float_asech_rational::<T>(&Rational::exact_from(
                    x
                ))),
                NiceFloat(primitive_float_asech(x))
            );
        }
    });
}

#[test]
fn primitive_float_asech_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_asech_rational_properties_helper);
}
