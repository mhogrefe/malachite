// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_power_of_2, big_exponent_float_to_float, float_to_big_exponent_float,
    parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_1,
    big_exponent_float_unsigned_pair_gen_var_1,
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_1,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_significand_exponent_and_prec() {
    // `odd` is the significand with its trailing zeros removed, which does not depend on the limb
    // width
    let test = |s, odd: &str, exponent: Option<&str>, prec: Option<u64>| {
        let x = parse_big_exponent_float_hex(s);
        let odd_part = |n: &Natural| (n >> n.trailing_zeros().unwrap()).to_string();
        let significand = x.to_significand();
        assert_eq!(x.significand_ref(), significand.as_ref());
        assert_eq!(x.clone().into_significand(), significand);
        assert_eq!(significand.as_ref().map_or(String::new(), odd_part), odd);
        assert_eq!(
            x.get_exponent().map(ToString::to_string).as_deref(),
            exponent
        );
        assert_eq!(x.get_prec(), prec);
    };
    test("NaN", "", None, None);
    test("Infinity", "", None, None);
    test("0x0.0", "", None, None);
    test("0x1.0#1", "1", Some("1"), Some(1));
    test("0x1.0000000000000000000000000#100", "1", Some("1"), Some(1));
    test("-0x1.8#2", "3", Some("1"), Some(2));
    test("0x0.8#1", "1", Some("0"), Some(1));
    test(
        "0x3.243f6a8885a30#53",
        "884279719003555",
        Some("2"),
        Some(50),
    );
    // - a trailing zero limb of the `Float` is dropped
    test(
        "0x1.0000000000000000000000002#100",
        "633825300114114700748351602689",
        Some("1"),
        Some(100),
    );
    let x = big_exponent_float_power_of_2(Integer::from_str("1000000000000000000000").unwrap());
    assert_eq!(
        x.get_exponent().unwrap().to_string(),
        "1000000000000000000001"
    );
    assert_eq!(x.get_prec(), Some(1));
}

#[test]
fn significand_exponent_and_prec_properties() {
    big_exponent_float_gen().test_properties(|x| {
        let significand = x.to_significand();
        assert_eq!(x.significand_ref(), significand.as_ref());
        assert_eq!(x.clone().into_significand(), significand);
        assert_eq!(significand.is_some(), x.is_normal());
        assert_eq!(x.get_exponent().is_some(), x.is_normal());
        assert_eq!(x.get_prec().is_some(), x.is_normal());
        if let Some(significand) = significand {
            // - the significand fills its limbs, with a nonzero lowest limb
            assert!(significand.is_valid());
            assert_ne!(significand, 0u32);
            let p = x.get_prec().unwrap();
            assert_ne!(p, 0);
            assert_eq!(
                significand.significant_bits() - significand.trailing_zeros().unwrap(),
                p
            );
        }
    });

    big_exponent_float_gen_var_1().test_properties(|x| {
        let f = big_exponent_float_to_float(&x).unwrap();
        assert_eq!(
            x.get_exponent().map(|e| i32::try_from(e).unwrap()),
            f.get_exponent()
        );
        assert_eq!(x.get_prec(), f.get_min_prec());
        assert_eq!(x.significand_ref(), f.significand_ref());
    });
}

#[test]
fn test_round_to_prec_round() {
    let test = |s, prec, rm, out: &str, o: &str| {
        let x = parse_big_exponent_float_hex(s);
        let (y, o_out) = x.round_to_prec_round_ref(prec, rm);
        assert!(y.is_valid());
        assert_eq!(format!("{y:#x}"), out);
        assert_eq!(format!("{o_out:?}"), o);
        let (y_alt, o_alt) = x.clone().round_to_prec_round(prec, rm);
        assert_eq!(format!("{y_alt:#x}"), out);
        assert_eq!(o_alt, o_out);
        let mut x_alt = x;
        assert_eq!(x_alt.round_to_prec_round_assign(prec, rm), o_out);
        assert_eq!(format!("{x_alt:#x}"), out);
    };
    test("NaN", 1, Floor, "NaN", "Equal");
    test("-Infinity", 1, Floor, "-Infinity", "Equal");
    test("0x0.0", 1, Exact, "0x0.0", "Equal");
    test("0x1.8#2", 2, Exact, "0x1.8", "Equal");
    test("0x1.8#2", 1, Floor, "0x1.0", "Less");
    test("0x1.8#2", 1, Ceiling, "0x2.0", "Greater");
    test("0x1.8#2", 1, Nearest, "0x2.0", "Greater");
    test("-0x1.8#2", 1, Floor, "-0x2.0", "Less");
    test("-0x1.8#2", 1, Down, "-0x1.0", "Greater");
    test("0x3.243f6a8885a30#53", 10, Nearest, "0x3.24", "Less");
    test("0x3.243f6a8885a30#53", 10, Ceiling, "0x3.25", "Greater");
    // - rounding up carries into the next power of 2
    test(
        "0x1.ffffffffffffffffffffffffe#100",
        10,
        Up,
        "0x2.0",
        "Greater",
    );
    // - a carry that would overflow a `Float` just increments the exponent
    test("0x7.f0E+268435455#10", 2, Up, "0x8.0E+268435455", "Greater");
}

#[test]
#[should_panic]
fn round_to_prec_round_fail_1() {
    BigExponentFloat::ONE.round_to_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn round_to_prec_round_fail_2() {
    parse_big_exponent_float_hex("0x1.8#2").round_to_prec_round(1, Exact);
}

#[test]
fn round_to_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_1().test_properties(
        |(x, prec, rm)| {
            let (y, o) = x.round_to_prec_round_ref(prec, rm);
            assert!(y.is_valid());
            let (y_alt, o_alt) = x.clone().round_to_prec_round(prec, rm);
            assert_eq!(format!("{y_alt:#x}"), format!("{y:#x}"));
            assert_eq!(o_alt, o);
            let mut x_alt = x.clone();
            assert_eq!(x_alt.round_to_prec_round_assign(prec, rm), o);
            assert_eq!(format!("{x_alt:#x}"), format!("{y:#x}"));
            assert_eq!(y.classify(), x.classify());
            if let Some(p) = y.get_prec() {
                assert!(p <= prec);
            }
            // - exactness
            assert_eq!(o == Equal, format!("{y:#x}") == format!("{x:#x}"));
            assert_eq!(o == Equal, x.get_prec().is_none_or(|p| p <= prec));
            // - rounding is idempotent
            let (z, o_z) = y.round_to_prec_round_ref(prec, Exact);
            assert_eq!(o_z, Equal);
            assert_eq!(format!("{z:#x}"), format!("{y:#x}"));
            match rm {
                Floor => assert_ne!(o, Greater),
                Ceiling => assert_ne!(o, Less),
                Down => assert!(if x.is_sign_positive() {
                    o != Greater
                } else {
                    o != Less
                }),
                Up => assert!(if x.is_sign_positive() {
                    o != Less
                } else {
                    o != Greater
                }),
                Exact => assert_eq!(o, Equal),
                Nearest => {}
            }
            // - agreement with `Float`, when the `Float` doesn't overflow
            if let Some(mut f) = big_exponent_float_to_float(&x) {
                let o_f = f.set_prec_round(prec, rm);
                if !f.is_infinite() || x.is_infinite() {
                    assert_eq!(o_f, o);
                    assert_eq!(
                        format!("{:#x}", float_to_big_exponent_float(&f)),
                        format!("{y:#x}")
                    );
                }
            }
        },
    );

    big_exponent_float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (y, o) = x.round_to_prec_ref(prec);
        let (y_alt, o_alt) = x.round_to_prec_round_ref(prec, Nearest);
        assert_eq!(format!("{y:#x}"), format!("{y_alt:#x}"));
        assert_eq!(o, o_alt);
        let (y_alt, o_alt) = x.clone().round_to_prec(prec);
        assert_eq!(format!("{y:#x}"), format!("{y_alt:#x}"));
        assert_eq!(o, o_alt);
        let mut x_alt = x.clone();
        assert_eq!(x_alt.round_to_prec_assign(prec), o);
        assert_eq!(format!("{x_alt:#x}"), format!("{y:#x}"));
        // - Floor and Ceiling bracket the value
        let rms: [RoundingMode; 2] = [Floor, Ceiling];
        let [(lo, o_lo), (hi, o_hi)] = rms.map(|rm| x.round_to_prec_round_ref(prec, rm));
        if o_lo == Equal {
            assert_eq!(o_hi, Equal);
            assert_eq!(o, Equal);
        } else {
            assert_eq!((o_lo, o_hi), (Less, Greater));
            let y = format!("{y:#x}");
            assert!(y == format!("{lo:#x}") || y == format!("{hi:#x}"));
        }
    });
}
