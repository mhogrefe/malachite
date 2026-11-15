// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::NaN;
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::generators::*;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

fn hex(s: &str) -> BigExponentFloat {
    BigExponentFloat::from_string_base(16, s).unwrap()
}

fn sin_cos_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, BigExponentFloat, Ordering, Ordering) {
    let (sin, o_s) = x.sin_prec_round_ref(prec, rm);
    let (cos, o_c) = x.cos_prec_round_ref(prec, rm);
    for r in [x.clone().sin_cos_prec_round(prec, rm), x.sin_cos_prec_round_ref(prec, rm)] {
        assert!(same(&r.0, &sin) && same(&r.1, &cos) && r.2 == o_s && r.3 == o_c);
    }
    let mut z = x.clone();
    let mut c = BigExponentFloat::NAN;
    let os = z.sin_cos_prec_round_assign(&mut c, prec, rm);
    assert!(same(&z, &sin) && same(&c, &cos) && os == (o_s, o_c));
    if rm == Nearest {
        for r in [x.clone().sin_cos_prec(prec), x.sin_cos_prec_ref(prec)] {
            assert!(same(&r.0, &sin) && same(&r.1, &cos) && r.2 == o_s && r.3 == o_c);
        }
        let mut z = x.clone();
        let mut c = BigExponentFloat::NAN;
        let os = z.sin_cos_prec_assign(&mut c, prec);
        assert!(same(&z, &sin) && same(&c, &cos) && os == (o_s, o_c));
    }
    (sin, cos, o_s, o_c)
}

fn sin_cos_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, BigExponentFloat, Ordering, Ordering) {
    let (sin, o_s) = BigExponentFloat::sin_rational_prec_round_ref(x, prec, rm);
    let (cos, o_c) = BigExponentFloat::cos_rational_prec_round_ref(x, prec, rm);
    for r in [
        BigExponentFloat::sin_cos_rational_prec_round(x.clone(), prec, rm),
        BigExponentFloat::sin_cos_rational_prec_round_ref(x, prec, rm),
    ] {
        assert!(same(&r.0, &sin) && same(&r.1, &cos) && r.2 == o_s && r.3 == o_c);
    }
    if rm == Nearest {
        for r in [
            BigExponentFloat::sin_cos_rational_prec(x.clone(), prec),
            BigExponentFloat::sin_cos_rational_prec_ref(x, prec),
        ] {
            assert!(same(&r.0, &sin) && same(&r.1, &cos) && r.2 == o_s && r.3 == o_c);
        }
    }
    (sin, cos, o_s, o_c)
}

fn sin_cos_with_period_prec_round_all(
    x: &BigExponentFloat,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, BigExponentFloat, Ordering, Ordering) {
    let (sin, o_s) = x.sin_with_period_prec_round_ref(u, prec, rm);
    let (cos, o_c) = x.cos_with_period_prec_round_ref(u, prec, rm);
    for r in [
        x.clone().sin_cos_with_period_prec_round(u, prec, rm),
        x.sin_cos_with_period_prec_round_ref(u, prec, rm),
    ] {
        assert!(same(&r.0, &sin) && same(&r.1, &cos) && r.2 == o_s && r.3 == o_c);
    }
    let mut z = x.clone();
    let mut c = BigExponentFloat::NAN;
    let os = z.sin_cos_with_period_prec_round_assign(&mut c, u, prec, rm);
    assert!(same(&z, &sin) && same(&c, &cos) && os == (o_s, o_c));
    if rm == Nearest {
        for r in
            [x.clone().sin_cos_with_period_prec(u, prec), x.sin_cos_with_period_prec_ref(u, prec)]
        {
            assert!(same(&r.0, &sin) && same(&r.1, &cos) && r.2 == o_s && r.3 == o_c);
        }
        let mut z = x.clone();
        let mut c = BigExponentFloat::NAN;
        let os = z.sin_cos_with_period_prec_assign(&mut c, u, prec);
        assert!(same(&z, &sin) && same(&c, &cos) && os == (o_s, o_c));
    }
    (sin, cos, o_s, o_c)
}

fn sin_cos_with_period_rational_prec_round_all(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, BigExponentFloat, Ordering, Ordering) {
    let (sin, o_s) = BigExponentFloat::sin_with_period_rational_prec_round_ref(x, u, prec, rm);
    let (cos, o_c) = BigExponentFloat::cos_with_period_rational_prec_round_ref(x, u, prec, rm);
    for r in [
        BigExponentFloat::sin_cos_with_period_rational_prec_round(x.clone(), u, prec, rm),
        BigExponentFloat::sin_cos_with_period_rational_prec_round_ref(x, u, prec, rm),
    ] {
        assert!(same(&r.0, &sin) && same(&r.1, &cos) && r.2 == o_s && r.3 == o_c);
    }
    if rm == Nearest {
        for r in [
            BigExponentFloat::sin_cos_with_period_rational_prec(x.clone(), u, prec),
            BigExponentFloat::sin_cos_with_period_rational_prec_ref(x, u, prec),
        ] {
            assert!(same(&r.0, &sin) && same(&r.1, &cos) && r.2 == o_s && r.3 == o_c);
        }
    }
    (sin, cos, o_s, o_c)
}

fn sin_cos_pi_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, BigExponentFloat, Ordering, Ordering) {
    let (sin, o_s) = x.sin_pi_prec_round_ref(prec, rm);
    let (cos, o_c) = x.cos_pi_prec_round_ref(prec, rm);
    for r in [x.clone().sin_cos_pi_prec_round(prec, rm), x.sin_cos_pi_prec_round_ref(prec, rm)] {
        assert!(same(&r.0, &sin) && same(&r.1, &cos) && r.2 == o_s && r.3 == o_c);
    }
    let mut z = x.clone();
    let mut c = BigExponentFloat::NAN;
    let os = z.sin_cos_pi_prec_round_assign(&mut c, prec, rm);
    assert!(same(&z, &sin) && same(&c, &cos) && os == (o_s, o_c));
    if rm == Nearest {
        for r in [x.clone().sin_cos_pi_prec(prec), x.sin_cos_pi_prec_ref(prec)] {
            assert!(same(&r.0, &sin) && same(&r.1, &cos) && r.2 == o_s && r.3 == o_c);
        }
        let mut z = x.clone();
        let mut c = BigExponentFloat::NAN;
        let os = z.sin_cos_pi_prec_assign(&mut c, prec);
        assert!(same(&z, &sin) && same(&c, &cos) && os == (o_s, o_c));
    }
    (sin, cos, o_s, o_c)
}

fn sin_cos_pi_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, BigExponentFloat, Ordering, Ordering) {
    let (sin, o_s) = BigExponentFloat::sin_pi_rational_prec_round_ref(x, prec, rm);
    let (cos, o_c) = BigExponentFloat::cos_pi_rational_prec_round_ref(x, prec, rm);
    for r in [
        BigExponentFloat::sin_cos_pi_rational_prec_round(x.clone(), prec, rm),
        BigExponentFloat::sin_cos_pi_rational_prec_round_ref(x, prec, rm),
    ] {
        assert!(same(&r.0, &sin) && same(&r.1, &cos) && r.2 == o_s && r.3 == o_c);
    }
    if rm == Nearest {
        for r in [
            BigExponentFloat::sin_cos_pi_rational_prec(x.clone(), prec),
            BigExponentFloat::sin_cos_pi_rational_prec_ref(x, prec),
        ] {
            assert!(same(&r.0, &sin) && same(&r.1, &cos) && r.2 == o_s && r.3 == o_c);
        }
    }
    (sin, cos, o_s, o_c)
}

#[test]
fn test_sin_cos_prec_round() {
    let r = sin_cos_prec_round_all(&hex("0x1"), 10, Floor);
    assert_eq!(format!("{:?}", r.0), "0x0.d74");
    assert_eq!(r.0.to_string(), "0.84082");
    assert_eq!(format!("{:?}", r.1), "0x0.8a4");
    assert_eq!(r.1.to_string(), "0.54004");
    assert_eq!((r.2, r.3), (Less, Less));
    let r = sin_cos_prec_round_all(&hex("0x0"), 10, Exact);
    assert_eq!(format!("{:?}", r.0), "0x0.0");
    assert_eq!(r.0.to_string(), "0.0");
    assert_eq!(format!("{:?}", r.1), "0x1.0");
    assert_eq!(r.1.to_string(), "1.0");
    assert_eq!((r.2, r.3), (Equal, Equal));
    let r = sin_cos_prec_round_all(&hex("NaN"), 10, Floor);
    assert_eq!(format!("{:?}", r.0), "NaN");
    assert_eq!(r.0.to_string(), "NaN");
    assert_eq!(format!("{:?}", r.1), "NaN");
    assert_eq!(r.1.to_string(), "NaN");
    assert_eq!((r.2, r.3), (Equal, Equal));
    let r = sin_cos_prec_round_all(&hex("0x1.0E+25"), 100, Nearest);
    assert_eq!(format!("{:?}", r.0), "-0x0.df476cbd60fac5f54ce60f245");
    assert_eq!(r.0.to_string(), "-0.87218360541826730978071977821350");
    assert_eq!(format!("{:?}", r.1), "0x0.7d3acffd9b8db34b71d86c7ff");
    assert_eq!(r.1.to_string(), "0.4891786569747214499057893087514");
    assert_eq!((r.2, r.3), (Less, Greater));
    let r = sin_cos_prec_round_all(&hex("0x3.0E-10000000000"), 10, Ceiling);
    assert_eq!(format!("{:?}", r.0), "0x3.0E-10000000000");
    assert_eq!(r.0.to_string(), "8.3e-12041199827");
    assert_eq!(format!("{:?}", r.1), "0x1.0");
    assert_eq!(r.1.to_string(), "1.0");
    assert_eq!((r.2, r.3), (Greater, Greater));
    let r = sin_cos_rational_prec_round_all(&Rational::from_str("1/3").unwrap(), 20, Nearest);
    assert_eq!(format!("{:?}", r.0), "0x0.53c308");
    assert_eq!(r.0.to_string(), "0.32719469");
    assert_eq!(format!("{:?}", r.1), "0x0.f1e8b");
    assert_eq!(r.1.to_string(), "0.94495678");
    assert_eq!((r.2, r.3), (Less, Less));
    let r = sin_cos_rational_prec_round_all(&Rational::from_str("0").unwrap(), 10, Exact);
    assert_eq!(format!("{:?}", r.0), "0x0.0");
    assert_eq!(r.0.to_string(), "0.0");
    assert_eq!(format!("{:?}", r.1), "0x1.0");
    assert_eq!(r.1.to_string(), "1.0");
    assert_eq!((r.2, r.3), (Equal, Equal));
    let r = sin_cos_with_period_prec_round_all(&hex("0x1"), 7, 20, Nearest);
    assert_eq!(format!("{:?}", r.0), "0x0.c8262");
    assert_eq!(r.0.to_string(), "0.7818317");
    assert_eq!(format!("{:?}", r.1), "0x0.9f9d");
    assert_eq!(r.1.to_string(), "0.623489");
    assert_eq!((r.2, r.3), (Greater, Less));
    let r = sin_cos_with_period_prec_round_all(&hex("0x2"), 24, 20, Floor);
    assert_eq!(format!("{:?}", r.0), "0x0.8");
    assert_eq!(r.0.to_string(), "0.50");
    assert_eq!(format!("{:?}", r.1), "0x0.ddb3d");
    assert_eq!(r.1.to_string(), "0.86602497");
    assert_eq!((r.2, r.3), (Equal, Less));
    let r = sin_cos_with_period_prec_round_all(&hex("0x1.0E+10000000000"), 7, 20, Nearest);
    assert_eq!(format!("{:?}", r.0), "0x0.f994e");
    assert_eq!(r.0.to_string(), "0.9749279");
    assert_eq!(format!("{:?}", r.1), "-0x0.38f72");
    assert_eq!(r.1.to_string(), "-0.2225208");
    assert_eq!((r.2, r.3), (Less, Greater));
    let r = sin_cos_with_period_rational_prec_round_all(
        &Rational::from_str("1/3").unwrap(),
        7,
        20,
        Nearest,
    );
    assert_eq!(format!("{:?}", r.0), "0x0.4b751");
    assert_eq!(r.0.to_string(), "0.2947550");
    assert_eq!(format!("{:?}", r.1), "0x0.f4a07");
    assert_eq!(r.1.to_string(), "0.95557308");
    assert_eq!((r.2, r.3), (Less, Greater));
    let r = sin_cos_pi_prec_round_all(&hex("0x0.8"), 10, Floor);
    assert_eq!(format!("{:?}", r.0), "0x1.0");
    assert_eq!(r.0.to_string(), "1.0");
    assert_eq!(format!("{:?}", r.1), "0x0.0");
    assert_eq!(r.1.to_string(), "0.0");
    assert_eq!((r.2, r.3), (Equal, Equal));
    let r = sin_cos_pi_prec_round_all(&hex("0x0.6"), 20, Nearest);
    assert_eq!(format!("{:?}", r.0), "0x0.ec836");
    assert_eq!(r.0.to_string(), "0.9238796");
    assert_eq!(format!("{:?}", r.1), "0x0.61f788");
    assert_eq!(r.1.to_string(), "0.38268328");
    assert_eq!((r.2, r.3), (Greater, Less));
    let r = sin_cos_pi_rational_prec_round_all(&Rational::from_str("1/5").unwrap(), 20, Nearest);
    assert_eq!(format!("{:?}", r.0), "0x0.96792");
    assert_eq!(r.0.to_string(), "0.5877857");
    assert_eq!(format!("{:?}", r.1), "0x0.cf1bc");
    assert_eq!(r.1.to_string(), "0.8090172");
    assert_eq!((r.2, r.3), (Greater, Greater));
}

#[test]
fn sin_cos_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_21().test_properties(
        |(x, prec, rm)| {
            sin_cos_prec_round_all(&x, prec, rm);
        },
    );
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        sin_cos_rational_prec_round_all(&x, prec, rm);
    });
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_8().test_properties(
        |(x, u, prec, rm)| {
            sin_cos_with_period_prec_round_all(&x, u, prec, rm);
        },
    );
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_13().test_properties(
        |(x, u, prec, rm)| {
            sin_cos_with_period_rational_prec_round_all(&x, u, prec, rm);
        },
    );
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_22().test_properties(
        |(x, prec, rm)| {
            sin_cos_pi_prec_round_all(&x, prec, rm);
        },
    );
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        sin_cos_pi_rational_prec_round_all(&x, prec, rm);
    });
}
