// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use gmp_mpfr_sys::mpfr::{self, rnd_t};
use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::{NaN, NegativeInfinity};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_float::float::arithmetic::fractional_part::{
    primitive_float_fractional_part, primitive_float_integer_and_fractional_parts,
};
use malachite_float::test_util::common::{parse_hex_string, to_hex_string};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::natural::Natural;
use std::panic::catch_unwind;

const fn mpfr_rnd(rm: RoundingMode) -> rnd_t {
    match rm {
        Floor => rnd_t::RNDD,
        Ceiling => rnd_t::RNDU,
        Down => rnd_t::RNDZ,
        Up => rnd_t::RNDA,
        Nearest => rnd_t::RNDN,
        Exact => panic!(),
    }
}

const fn ordering_of(t: i32) -> Ordering {
    if t < 0 {
        Less
    } else if t == 0 {
        Equal
    } else {
        Greater
    }
}

fn sweep_values() -> Vec<Float> {
    let mut xs = Vec::new();
    for prec_x in [1u64, 2, 5, 10, 64, 65, 100] {
        let mut sigs = vec![Natural::power_of_2(prec_x - 1), Natural::low_mask(prec_x)];
        for t in [1, 2, prec_x / 2, prec_x.saturating_sub(2)] {
            if t >= prec_x {
                continue;
            }
            sigs.push(Natural::power_of_2(prec_x - 1) + Natural::power_of_2(t));
            if t > 1 {
                sigs.push(
                    Natural::power_of_2(prec_x - 1)
                        + Natural::power_of_2(t)
                        + Natural::power_of_2(0u64),
                );
            }
        }
        sigs.sort_unstable();
        sigs.dedup();
        for sig in sigs {
            for exp in [
                -2i64,
                0,
                1,
                2,
                i64::exact_from(prec_x / 2 + 1),
                i64::exact_from(prec_x),
                i64::exact_from(prec_x) + 10,
            ] {
                let x = Float::from_natural_prec(sig.clone(), prec_x).0
                    << (exp - i64::exact_from(prec_x));
                if x != 0u32 {
                    xs.push(x.clone());
                    xs.push(-x);
                }
            }
        }
    }
    xs
}

#[test]
fn test_fractional_part_vs_mpfr() {
    for x in sweep_values() {
        let b = rug::Float::exact_from(&x);
        for prec in [1u64, 2, 3, 10, 64, 100] {
            for rm in [Floor, Ceiling, Down, Up, Nearest] {
                let (ours, o) = x.fractional_part_prec_round_ref(prec, rm);
                let mut r = rug::Float::new(u32::exact_from(prec));
                let t = unsafe { mpfr::frac(r.as_raw_mut(), b.as_raw(), mpfr_rnd(rm)) };
                assert_eq!(
                    ComparableFloat(Float::from(&r)),
                    ComparableFloat(ours),
                    "{x} {prec} {rm}"
                );
                assert_eq!(ordering_of(t), o, "ternary {x} {prec} {rm}");
            }
        }
    }
}

#[test]
fn test_integer_and_fractional_parts_vs_mpfr() {
    for x in sweep_values() {
        let b = rug::Float::exact_from(&x);
        for (iprec, fprec) in [(1u64, 1u64), (2, 3), (10, 10), (64, 10), (10, 64), (100, 100)] {
            for rm in [Floor, Ceiling, Down, Up, Nearest] {
                let ((i_ours, i_o), (f_ours, f_o)) =
                    x.integer_and_fractional_parts_prec_round_ref(iprec, fprec, rm);
                let mut ir = rug::Float::new(u32::exact_from(iprec));
                let mut fr = rug::Float::new(u32::exact_from(fprec));
                let t = unsafe {
                    mpfr::modf(ir.as_raw_mut(), fr.as_raw_mut(), b.as_raw(), mpfr_rnd(rm))
                };
                assert_eq!(
                    ComparableFloat(Float::from(&ir)),
                    ComparableFloat(i_ours),
                    "int {x} {iprec} {fprec} {rm}"
                );
                assert_eq!(
                    ComparableFloat(Float::from(&fr)),
                    ComparableFloat(f_ours),
                    "frac {x} {iprec} {fprec} {rm}"
                );
                // decode MPFR's packed pair of ternaries: INEXPOS(i) | INEXPOS(f) << 2, where
                // INEXPOS is 0 for exact, 1 for positive, 2 for negative
                let decode = |v: i32| match v {
                    0 => Equal,
                    1 => Greater,
                    2 => Less,
                    _ => unreachable!(),
                };
                assert_eq!(decode(t & 3), i_o, "int ternary {x} {iprec} {fprec} {rm}");
                assert_eq!(
                    decode(t >> 2 & 3),
                    f_o,
                    "frac ternary {x} {iprec} {fprec} {rm}"
                );
            }
        }
    }
}

#[test]
fn fractional_part_special() {
    let (r, o) = Float::NAN.fractional_part_ref();
    assert!(r.is_nan());
    assert_eq!(o, Equal);
    // the fractional part of an infinity is a zero with the same sign
    let (r, o) = Float::NEGATIVE_INFINITY.fractional_part_ref();
    assert_eq!(ComparableFloat(r), ComparableFloat(-Float::from(0u32)));
    assert_eq!(o, Equal);
    // the integral part of an infinity is itself, and its fractional part is a signed zero
    let ((i, io), (f, fo)) = Float::NEGATIVE_INFINITY.integer_and_fractional_parts_ref();
    assert_eq!(i, Float::NEGATIVE_INFINITY);
    assert_eq!(ComparableFloat(f), ComparableFloat(-Float::from(0u32)));
    assert_eq!((io, fo), (Equal, Equal));
    // variants agree
    let x = Float::from(2.5f64);
    let a = x.fractional_part_prec_round_ref(3, Nearest);
    let b = x.clone().fractional_part_prec_round(3, Nearest);
    assert_eq!(ComparableFloat(a.0.clone()), ComparableFloat(b.0));
    assert_eq!(a.1, b.1);
    let ((i1, io1), (f1, fo1)) = x.integer_and_fractional_parts_ref();
    let ((i2, io2), (f2, fo2)) = x.clone().integer_and_fractional_parts();
    assert_eq!(ComparableFloat(i1), ComparableFloat(i2));
    assert_eq!(ComparableFloat(f1), ComparableFloat(f2));
    assert_eq!((io1, fo1), (io2, fo2));
}

#[test]
#[should_panic]
fn fractional_part_fail() {
    Float::from(3u32).fractional_part_prec_round_ref(0, Nearest);
}

#[test]
#[should_panic]
fn integer_and_fractional_parts_fail() {
    Float::from(3u32).integer_and_fractional_parts_prec_round_ref(5, 0, Nearest);
}

#[test]
fn test_fractional_part() {
    let test = |s, s_hex, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (f, o) = x.clone().fractional_part();
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, o_out);

        let (f_alt, o_alt) = x.fractional_part_ref();
        assert!(f_alt.is_valid());
        assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
        assert_eq!(o_alt, o);
    };
    // - NaN stays NaN; the fractional part of an infinity is a zero of the same sign (as in
    //   mpfr_frac)
    test("NaN", "NaN", "NaN", "NaN", Equal);
    test("Infinity", "Infinity", "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0", Equal);
    // - an integer's fractional part is a zero with the integer's sign
    test("2.0", "0x2.0#1", "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0#1", "-0.0", "-0x0.0", Equal);
    test("1.3e30", "0x1.0E+25#1", "0.0", "0x0.0", Equal);
    // - a value below 1 in magnitude is its own fractional part
    test("0.75", "0x0.c#3", "0.75", "0x0.c#3", Equal);
    test("-0.75", "-0x0.c#3", "-0.75", "-0x0.c#3", Equal);
    // - the general extraction, both signs
    test("10.5", "0xa.8#6", "0.500", "0x0.80#6", Equal);
    test("-10.5", "-0xa.8#6", "-0.500", "-0x0.80#6", Equal);
    test("10.31", "0xa.50#9", "0.3125", "0x0.500#9", Equal);
}

#[test]
fn test_fractional_part_prec_round() {
    let test = |s, s_hex, prec, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (f, o) = x.clone().fractional_part_prec_round(prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, o_out);

        let (f_alt, o_alt) = x.fractional_part_prec_round_ref(prec, rm);
        assert!(f_alt.is_valid());
        assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
        assert_eq!(o_alt, o);
    };
    // - rounding the extracted fraction 0.3125 at precision 1
    test("10.31", "0xa.50#9", 1, Floor, "0.25", "0x0.4#1", Less);
    test("10.31", "0xa.50#9", 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("10.31", "0xa.50#9", 1, Nearest, "0.25", "0x0.4#1", Less);
    // - a negative fraction: Floor rounds away from zero
    test("-10.31", "-0xa.50#9", 1, Floor, "-0.50", "-0x0.8#1", Less);
    test(
        "-10.31",
        "-0xa.50#9",
        1,
        Ceiling,
        "-0.25",
        "-0x0.4#1",
        Greater,
    );
    // - Exact succeeds when the fraction is exactly representable
    test("10.31", "0xa.50#9", 4, Exact, "0.312", "0x0.50#4", Equal);
    test(
        "10.5",
        "0xa.8#6",
        10,
        Nearest,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    // - specials and integers ignore the precision request (zeros and NaN have none)
    test("2.0", "0x2.0#1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Nearest, "0.0", "0x0.0", Equal);
}

#[test]
fn test_integer_and_fractional_parts() {
    let test = |s,
                s_hex,
                i_out: &str,
                i_out_hex: &str,
                io_out: Ordering,
                f_out: &str,
                f_out_hex: &str,
                fo_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let ((i, io), (f, fo)) = x.clone().integer_and_fractional_parts();
        assert!(i.is_valid());
        assert!(f.is_valid());
        assert_eq!(i.to_string(), i_out);
        assert_eq!(to_hex_string(&i), i_out_hex);
        assert_eq!(io, io_out);
        assert_eq!(f.to_string(), f_out);
        assert_eq!(to_hex_string(&f), f_out_hex);
        assert_eq!(fo, fo_out);

        let ((i_alt, io_alt), (f_alt, fo_alt)) = x.integer_and_fractional_parts_ref();
        assert!(i_alt.is_valid());
        assert!(f_alt.is_valid());
        assert_eq!(ComparableFloatRef(&i_alt), ComparableFloatRef(&i));
        assert_eq!(io_alt, io);
        assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
        assert_eq!(fo_alt, fo);
    };
    // - NaN splits into two NaNs; an infinity keeps its integer part and has a zero fraction (as in
    //   mpfr_modf)
    test("NaN", "NaN", "NaN", "NaN", Equal, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Equal, "0.0", "0x0.0", Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Equal,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test("0.0", "0x0.0", "0.0", "0x0.0", Equal, "0.0", "0x0.0", Equal);
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Equal, "-0.0", "-0x0.0", Equal,
    );
    // - integers: the fraction is a signed zero
    test(
        "2.0", "0x2.0#1", "2.0", "0x2.0#1", Equal, "0.0", "0x0.0", Equal,
    );
    test(
        "-2.0", "-0x2.0#1", "-2.0", "-0x2.0#1", Equal, "-0.0", "-0x0.0", Equal,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        "1.3e30",
        "0x1.0E+25#1",
        Equal,
        "0.0",
        "0x0.0",
        Equal,
    );
    // - values below 1: the integer part is a signed zero
    test(
        "0.75", "0x0.c#3", "0.0", "0x0.0", Equal, "0.75", "0x0.c#3", Equal,
    );
    test(
        "-0.75", "-0x0.c#3", "-0.0", "-0x0.0", Equal, "-0.75", "-0x0.c#3", Equal,
    );
    // - the general split, both signs
    test(
        "10.5", "0xa.8#6", "10.0", "0xa.0#6", Equal, "0.500", "0x0.80#6", Equal,
    );
    test(
        "-10.5",
        "-0xa.8#6",
        "-10.0",
        "-0xa.0#6",
        Equal,
        "-0.500",
        "-0x0.80#6",
        Equal,
    );
    test(
        "10.31",
        "0xa.50#9",
        "10.00",
        "0xa.00#9",
        Equal,
        "0.3125",
        "0x0.500#9",
        Equal,
    );
}

#[test]
fn test_integer_and_fractional_parts_prec_round() {
    let test = |s,
                s_hex,
                iprec,
                fprec,
                rm: RoundingMode,
                i_out: &str,
                i_out_hex: &str,
                io_out: Ordering,
                f_out: &str,
                f_out_hex: &str,
                fo_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let ((i, io), (f, fo)) = x
            .clone()
            .integer_and_fractional_parts_prec_round(iprec, fprec, rm);
        assert!(i.is_valid());
        assert!(f.is_valid());
        assert_eq!(i.to_string(), i_out);
        assert_eq!(to_hex_string(&i), i_out_hex);
        assert_eq!(io, io_out);
        assert_eq!(f.to_string(), f_out);
        assert_eq!(to_hex_string(&f), f_out_hex);
        assert_eq!(fo, fo_out);

        let ((i_alt, io_alt), (f_alt, fo_alt)) =
            x.integer_and_fractional_parts_prec_round_ref(iprec, fprec, rm);
        assert!(i_alt.is_valid());
        assert!(f_alt.is_valid());
        assert_eq!(ComparableFloatRef(&i_alt), ComparableFloatRef(&i));
        assert_eq!(io_alt, io);
        assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
        assert_eq!(fo_alt, fo);
    };
    // - independent precisions for the two parts, and rounding both (10 -> 8 at precision 2, 0.3125
    //   -> 0.25 at precision 1)
    test(
        "10.31", "0xa.50#9", 2, 1, Floor, "8.0", "0x8.0#2", Less, "0.25", "0x0.4#1", Less,
    );
    test(
        "10.31", "0xa.50#9", 2, 1, Nearest, "8.0", "0x8.0#2", Less, "0.25", "0x0.4#1", Less,
    );
    // - a negative value: Floor rounds both parts down (away from zero)
    test(
        "-10.31",
        "-0xa.50#9",
        2,
        1,
        Floor,
        "-12.0",
        "-0xc.0#2",
        Less,
        "-0.50",
        "-0x0.8#1",
        Less,
    );
    // - exactly representable at the requested precisions
    test(
        "10.5",
        "0xa.8#6",
        10,
        10,
        Nearest,
        "10.000",
        "0xa.00#10",
        Equal,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "0.75", "0x0.c#3", 5, 5, Nearest, "0.0", "0x0.0", Equal, "0.750", "0x0.c0#5", Equal,
    );
    test(
        "2.0", "0x2.0#1", 5, 5, Nearest, "2.00", "0x2.0#5", Equal, "0.0", "0x0.0", Equal,
    );
}

#[test]
fn fractional_part_prec_round_fail() {
    assert_panic!(Float::from(1u32).fractional_part_prec_round(0, Nearest));
    assert_panic!(Float::from(1u32).fractional_part_prec_round_ref(0, Nearest));
    // Exact with an inexact fraction
    assert_panic!(parse_hex_string("0xa.50#9").fractional_part_prec_round(1, Exact));
}

#[test]
fn integer_and_fractional_parts_prec_round_fail() {
    assert_panic!(Float::from(1u32).integer_and_fractional_parts_prec_round(0, 1, Nearest));
    assert_panic!(Float::from(1u32).integer_and_fractional_parts_prec_round(1, 0, Nearest));
    assert_panic!(
        parse_hex_string("0xa.50#9").integer_and_fractional_parts_prec_round(2, 1, Exact)
    );
}

// The emulated primitive-float fractional part agrees bit-for-bit with the standard library for
// finite values; infinities follow mpfr_frac (a same-signed zero) rather than `fract`'s NaN.
#[test]
fn primitive_float_fractional_part_properties() {
    primitive_float_gen::<f64>().test_properties(|x| {
        let f = primitive_float_fractional_part(x);
        if x.is_finite() {
            if x.fract() == 0.0 {
                // a zero fractional part takes the input's sign (as in mpfr_frac), while `fract`
                // returns a positive zero for negative integers
                assert_eq!(NiceFloat(f), NiceFloat(0.0f64.copysign(x)));
            } else {
                assert_eq!(NiceFloat(f), NiceFloat(x.fract()));
            }
        } else if x.is_infinite() {
            assert_eq!(NiceFloat(f), NiceFloat(if x > 0.0 { 0.0 } else { -0.0 }));
        } else {
            assert!(f.is_nan());
        }
        let (i, f2) = primitive_float_integer_and_fractional_parts(x);
        if x.is_finite() {
            assert_eq!(NiceFloat(i), NiceFloat(x.trunc()));
            if x.fract() == 0.0 {
                assert_eq!(NiceFloat(f2), NiceFloat(0.0f64.copysign(x)));
            } else {
                assert_eq!(NiceFloat(f2), NiceFloat(x.fract()));
            }
        }
    });

    primitive_float_gen::<f32>().test_properties(|x| {
        if x.is_finite() && x.fract() != 0.0 {
            assert_eq!(
                NiceFloat(primitive_float_fractional_part(x)),
                NiceFloat(x.fract())
            );
        }
    });
}
