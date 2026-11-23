// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::DivExact;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeOne, One, Zero};
use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::{FromStringBase, ToStringBase};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::{Mod, ModPowerOf2};
use malachite_base::strings::{ToLowerHexString, ToUpperHexString};
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::exhaustive::*;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_power_of_2, big_exponent_float_to_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_1,
};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_gen;
use std::str::FromStr;

#[test]
fn test_to_hex_string() {
    let test = |x: BigExponentFloat, lower: &str, upper: &str| {
        assert_eq!(x.to_lower_hex_string(), lower);
        assert_eq!(x.to_upper_hex_string(), upper);
        let alt = if x.is_finite() {
            format!("0x{lower}").replace("0x-", "-0x")
        } else {
            lower.to_string()
        };
        assert_eq!(format!("{x:#x}"), alt);
        assert_eq!(format!("{x:?}"), format!("{x:#x}"));
    };
    test(BigExponentFloat::NAN, "NaN", "NaN");
    test(BigExponentFloat::INFINITY, "Infinity", "Infinity");
    test(
        BigExponentFloat::NEGATIVE_INFINITY,
        "-Infinity",
        "-Infinity",
    );
    test(BigExponentFloat::ZERO, "0.0", "0.0");
    test(BigExponentFloat::ONE, "1.0", "1.0");
    test(BigExponentFloat::NEGATIVE_ONE, "-1.0", "-1.0");
    test(parse_big_exponent_float_hex("0xff.0#8"), "ff.0", "FF.0");
    test(
        parse_big_exponent_float_hex("0x3.243f6a8885a30#53"),
        "3.243f6a8885a3",
        "3.243F6A8885A3",
    );
    test(
        parse_big_exponent_float_hex("0x4.0E+268435455#1"),
        "4.0E+268435455",
        "4.0E+268435455",
    );
    // - exponents outside `Float`'s range
    let test_power = |e: &str, out: &str| {
        let x = big_exponent_float_power_of_2(Integer::from_str(e).unwrap());
        assert_eq!(x.to_lower_hex_string(), out);
    };
    test_power("1073741824", "1.0E+268435456");
    test_power("1073741825", "2.0E+268435456");
    test_power("1073741827", "8.0E+268435456");
    test_power("-1073741825", "8.0E-268435457");
    test_power("100000000000000000000", "1.0E+25000000000000000000");
    test_power("-100000000000000000001", "8.0E-25000000000000000001");
}

#[test]
fn to_hex_string_properties() {
    // - agreement with `Float`
    big_exponent_float_gen_var_1().test_properties(|x| {
        let f = big_exponent_float_to_float(&x).unwrap();
        assert_eq!(format!("{x:x}"), format!("{f:x}"));
        assert_eq!(format!("{x:#x}"), format!("{f:#x}"));
        assert_eq!(format!("{x:X}"), format!("{f:X}"));
        assert_eq!(format!("{x:#X}"), format!("{f:#X}"));
    });

    integer_gen().test_properties(|s| {
        if s.unsigned_abs_ref().significant_bits() <= 31 {
            return;
        }
        // - 2^s is 2^(s mod 4) * 16^floor(s / 4)
        let x = big_exponent_float_power_of_2(s.clone());
        let digit = 1u32 << u32::try_from(&(&s).mod_power_of_2(2)).unwrap();
        let k: Integer = &s >> 2u32;
        let plus = if k >= 0u32 { "+" } else { "" };
        assert_eq!(x.to_lower_hex_string(), format!("{digit}.0E{plus}{k}"));
        // - shifting the sci-exponent by a multiple of 4 only shifts the printed exponent; the
        //   first 64 values have up to 7 significant bits
        let r = Integer::from((&s).mod_power_of_2(2));
        let near = if s >= 0u32 {
            r + Integer::from(1u32 << 29)
        } else {
            r - Integer::from(1u32 << 29)
        };
        let shift: Integer = (&s - &near) >> 2u32;
        for (y, z) in exhaustive_positive_big_exponent_floats_with_sci_exponent(s.clone())
            .zip(exhaustive_positive_big_exponent_floats_with_sci_exponent(
                near,
            ))
            .take(64)
        {
            let y = y.to_lower_hex_string();
            let z = z.to_lower_hex_string();
            let (y_digits, y_e) = y.split_once('E').unwrap();
            let (z_digits, z_e) = z.split_once('E').unwrap();
            assert_eq!(y_digits, z_digits);
            assert_eq!(
                Integer::from_str(y_e.trim_start_matches('+')).unwrap(),
                Integer::from_str(z_e.trim_start_matches('+')).unwrap() + &shift
            );
        }
    });
}

#[test]
fn power_of_2_base_properties() {
    // - agreement with `Float`
    big_exponent_float_gen_var_1().test_properties(|x| {
        let f = big_exponent_float_to_float(&x).unwrap();
        assert_eq!(format!("{x:b}"), format!("{f:b}"));
        assert_eq!(format!("{x:#b}"), format!("{f:#b}"));
        assert_eq!(format!("{x:o}"), format!("{f:o}"));
        assert_eq!(format!("{x:#o}"), format!("{f:#o}"));
        for base in [2, 4, 8, 16, 32] {
            assert_eq!(x.to_string_base(base), f.to_string_base(base));
            assert_eq!(x.to_string_base_upper(base), f.to_string_base_upper(base));
        }
    });

    // - in base 2^k, 2^s is 2^(s mod k) * (2^k)^floor(s / k), for exponents beyond `Float`'s range
    integer_gen().test_properties(|s| {
        if s.unsigned_abs_ref().significant_bits() <= 31 {
            return;
        }
        let x = big_exponent_float_power_of_2(s.clone());
        for (base, k) in [(2u8, 1u32), (4, 2), (8, 3), (32, 5)] {
            let kk = Integer::from(k);
            let r = u32::try_from(&(&s).mod_op(&kk)).unwrap();
            let q = (&s - Integer::from(r)).div_exact(&kk);
            let digit = std::char::from_digit(1 << r, u32::from(base)).unwrap();
            // - only base 32, where `E` is also a digit, writes the sign of a positive exponent
            let plus = if base == 32 && q >= 0u32 { "+" } else { "" };
            assert_eq!(x.to_string_base(base), format!("{digit}.0E{plus}{q}"));
        }
    });
}

#[test]
fn test_to_string() {
    // Rows outside `Float`'s exponent range were checked independently with exact decimal
    // arithmetic; the others agree with `Float`.
    let test = |s_hex: &str, out: &str| {
        let x = BigExponentFloat::from_string_base(16, s_hex).unwrap();
        assert_eq!(x.to_string(), out);
        if let Some(f) = big_exponent_float_to_float(&x) {
            assert_eq!(f.to_string(), out);
        }
    };
    test("0x1.8", "1.5");
    test("0x3.243f6a8885a30", "3.1415926535897931");
    test("0x0.1999999999999a", "0.10000000000000001");
    test("-0x3e8.0", "-1000.0");
    test("0x1.0E-10", "9.1e-13");
    test("0xf.fffffE+100", "4.13159956e121");
    test("0x1.0E+25000000000000000000", "2.4e30102999566398119521");
    test("0x1.0E-25000000000000000000", "4.2e-30102999566398119522");
    test(
        "0x3.243f6a8885a30E+268435456",
        "1.3185758957267685e323228497",
    );
    test("-0x1.8E-268435460", "-5.5e-323228502");
    test("0x1.0E+268435456", "4.2e323228496");
    test(
        "-0xa.bcdefE+99999999999999999999",
        "-2.10064313e120411998265592478085",
    );
    test("0x0.0", "0.0");
    test("NaN", "NaN");
    test("Infinity", "Infinity");
    test("-Infinity", "-Infinity");
}

#[test]
fn test_to_string_base_non_power_of_2() {
    let test = |s: &str, s_hex: &str, base: u8, out: &str, out_upper: &str| {
        let x = BigExponentFloat::from_string_base(16, s_hex).unwrap();
        assert_eq!(x.to_string(), s);
        assert_eq!(x.to_string_base(base), out);
        assert_eq!(x.to_string_base_upper(base), out_upper);
        if let Some(f) = big_exponent_float_to_float(&x) {
            assert_eq!(f.to_string_base(base), out);
            assert_eq!(f.to_string_base_upper(base), out_upper);
        }
    };
    test("1.5", "0x1.8", 3, "1.11", "1.11");
    test("1.5", "0x1.8", 7, "1.4", "1.4");
    test("1.5", "0x1.8", 10, "1.5", "1.5");
    test("1.5", "0x1.8", 20, "1.a", "1.A");
    test("1.5", "0x1.8", 36, "1.i", "1.I");
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        3,
        "10.0102110122220102110021111102220",
        "10.0102110122220102110021111102220",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        7,
        "3.066365143203613411",
        "3.066365143203613411",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        "3.1415926535897931",
        "3.1415926535897931",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        20,
        "3.2gceg9gbhj9d",
        "3.2GCEG9GBHJ9D",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        36,
        "3.53i5ab8p5f",
        "3.53I5AB8P5F",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        3,
        "0.002200220022002200220022002200220100",
        "0.002200220022002200220022002200220100",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        7,
        "0.046204620462046204624",
        "0.046204620462046204624",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        "0.10000000000000001",
        "0.10000000000000001",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        20,
        "0.20000000000009",
        "0.20000000000009",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        36,
        "0.3lllllllllmc",
        "0.3LLLLLLLLLMC",
    );
    test("-1000.0", "-0x3e8.0", 3, "-1.10100e6", "-1.10100E6");
    test("-1000.0", "-0x3e8.0", 7, "-2626.0", "-2626.0");
    test("-1000.0", "-0x3e8.0", 10, "-1000.0", "-1000.0");
    test("-1000.0", "-0x3e8.0", 20, "-2a0.0", "-2A0.0");
    test("-1000.0", "-0x3e8.0", 36, "-rs.0", "-RS.0");
    test("9.1e-13", "0x1.0E-10", 3, "2.1e-26", "2.1E-26");
    test("9.1e-13", "0x1.0E-10", 7, "4.2e-15", "4.2E-15");
    test("9.1e-13", "0x1.0E-10", 10, "9.1e-13", "9.1E-13");
    test("9.1e-13", "0x1.0E-10", 20, "9.6e-10", "9.6E-10");
    test("9.1e-13", "0x1.0E-10", 36, "2.ke-8", "2.KE-8");
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        3,
        "2.2000200011112011e254",
        "2.2000200011112011E254",
    );
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        7,
        "5.564166514e143",
        "5.564166514E143",
    );
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        10,
        "4.13159956e121",
        "4.13159956E121",
    );
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        20,
        "4.38efi4e+93",
        "4.38EFI4E+93",
    );
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        36,
        "1.odcbue+78",
        "1.ODCBUE+78",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        3,
        "1.0e63092975357145743710",
        "1.0E63092975357145743710",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        7,
        "2.2e35620718710802217651",
        "2.2E35620718710802217651",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        10,
        "2.4e30102999566398119521",
        "2.4E30102999566398119521",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        20,
        "3.1e+23137821315975917426",
        "3.1E+23137821315975917426",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        36,
        "6.ae+19342640361727079343",
        "6.AE+19342640361727079343",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        3,
        "1.0e-63092975357145743710",
        "1.0E-63092975357145743710",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        7,
        "3.1e-35620718710802217652",
        "3.1E-35620718710802217652",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        10,
        "4.2e-30102999566398119522",
        "4.2E-30102999566398119522",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        20,
        "6.ce-23137821315975917427",
        "6.CE-23137821315975917427",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        36,
        "5.re-19342640361727079344",
        "5.RE-19342640361727079344",
    );
    test(
        "1.3185758957267685e323228497",
        "0x3.243f6a8885a30E+268435456",
        3,
        "1.12212202112222200011110210021102e677455665",
        "1.12212202112222200011110210021102E677455665",
    );
    test(
        "1.3185758957267685e323228497",
        "0x3.243f6a8885a30E+268435456",
        7,
        "2.105404140251163105e382474555",
        "2.105404140251163105E382474555",
    );
    test(
        "1.3185758957267685e323228497",
        "0x3.243f6a8885a30E+268435456",
        10,
        "1.3185758957267685e323228497",
        "1.3185758957267685E323228497",
    );
    test(
        "1.3185758957267685e323228497",
        "0x3.243f6a8885a30E+268435456",
        20,
        "1.0h63b4ea3444e+248440465",
        "1.0H63B4EA3444E+248440465",
    );
    test(
        "1.3185758957267685e323228497",
        "0x3.243f6a8885a30E+268435456",
        36,
        "e.nlhkh618aie+207690019",
        "E.NLHKH618AIE+207690019",
    );
    test(
        "-5.5e-323228502",
        "-0x1.8E-268435460",
        3,
        "-2.12e-677455675",
        "-2.12E-677455675",
    );
    test(
        "-5.5e-323228502",
        "-0x1.8E-268435460",
        7,
        "-3.6e-382474561",
        "-3.6E-382474561",
    );
    test(
        "-5.5e-323228502",
        "-0x1.8E-268435460",
        10,
        "-5.5e-323228502",
        "-5.5E-323228502",
    );
    test(
        "-5.5e-323228502",
        "-0x1.8E-268435460",
        20,
        "-b.1e-248440469",
        "-B.1E-248440469",
    );
    test(
        "-5.5e-323228502",
        "-0x1.8E-268435460",
        36,
        "-8.9e-207690023",
        "-8.9E-207690023",
    );
    test(
        "4.2e323228496",
        "0x1.0E+268435456",
        3,
        "1.2e677455664",
        "1.2E677455664",
    );
    test(
        "4.2e323228496",
        "0x1.0E+268435456",
        7,
        "4.6e382474554",
        "4.6E382474554",
    );
    test(
        "4.2e323228496",
        "0x1.0E+268435456",
        10,
        "4.2e323228496",
        "4.2E323228496",
    );
    test(
        "4.2e323228496",
        "0x1.0E+268435456",
        20,
        "6.de+248440464",
        "6.DE+248440464",
    );
    test(
        "4.2e323228496",
        "0x1.0E+268435456",
        36,
        "4.oe+207690019",
        "4.OE+207690019",
    );
    test(
        "-2.10064313e120411998265592478085",
        "-0xa.bcdefE+99999999999999999999",
        3,
        "-1.1220110222002210e252371901428582974839",
        "-1.1220110222002210E252371901428582974839",
    );
    test(
        "-2.10064313e120411998265592478085",
        "-0xa.bcdefE+99999999999999999999",
        7,
        "-2.322123233e142482874843208870605",
        "-2.322123233E142482874843208870605",
    );
    test(
        "-2.10064313e120411998265592478085",
        "-0xa.bcdefE+99999999999999999999",
        10,
        "-2.10064313e120411998265592478085",
        "-2.10064313E120411998265592478085",
    );
    test(
        "-2.10064313e120411998265592478085",
        "-0xa.bcdefE+99999999999999999999",
        20,
        "-2.g7bh4ce+92551285263903669705",
        "-2.G7BH4CE+92551285263903669705",
    );
    test(
        "-2.10064313e120411998265592478085",
        "-0xa.bcdefE+99999999999999999999",
        36,
        "-s.tk3r3e+77370561446908317373",
        "-S.TK3R3E+77370561446908317373",
    );
    test("0.0", "0x0.0", 3, "0.0", "0.0");
    test("0.0", "0x0.0", 7, "0.0", "0.0");
    test("0.0", "0x0.0", 10, "0.0", "0.0");
    test("0.0", "0x0.0", 20, "0.0", "0.0");
    test("0.0", "0x0.0", 36, "0.0", "0.0");
    test("NaN", "NaN", 3, "NaN", "NaN");
    test("NaN", "NaN", 7, "NaN", "NaN");
    test("NaN", "NaN", 10, "NaN", "NaN");
    test("NaN", "NaN", 20, "NaN", "NaN");
    test("NaN", "NaN", 36, "NaN", "NaN");
    test("Infinity", "Infinity", 3, "Infinity", "Infinity");
    test("Infinity", "Infinity", 7, "Infinity", "Infinity");
    test("Infinity", "Infinity", 10, "Infinity", "Infinity");
    test("Infinity", "Infinity", 20, "Infinity", "Infinity");
    test("Infinity", "Infinity", 36, "Infinity", "Infinity");
    test("-Infinity", "-Infinity", 3, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", 7, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", 10, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", 20, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", 36, "-Infinity", "-Infinity");
}

#[test]
fn to_string_properties() {
    // - agreement with `Float`, in every base
    big_exponent_float_gen_var_1().test_properties(|x| {
        let f = big_exponent_float_to_float(&x).unwrap();
        assert_eq!(x.to_string(), f.to_string());
        for base in 3..=36 {
            if !u8::is_power_of_two(base) {
                assert_eq!(x.to_string_base(base), f.to_string_base(base));
                assert_eq!(x.to_string_base_upper(base), f.to_string_base_upper(base));
            }
        }
    });

    big_exponent_float_gen().test_properties(|x| {
        let s = x.to_string();
        assert_eq!(s, x.to_string_base(10));
        assert_eq!(
            x.to_string_base_upper(10),
            s.to_uppercase()
                .replace("NAN", "NaN")
                .replace("INFINITY", "Infinity")
        );
        if !x.is_finite() || x == 0u32 {
            return;
        }
        // - negation
        assert_eq!(
            (&x * BigExponentFloat::NEGATIVE_ONE).to_string(),
            s.strip_prefix('-')
                .map_or_else(|| format!("-{s}"), ToString::to_string)
        );
        // - the digit count round-trips: reading the string back at `x`'s precision recovers `x`
        let prec = x.get_prec().unwrap();
        for base in [3, 10, 20, 36] {
            let mut options = FromSciStringOptions::default();
            options.set_base(base);
            let y = BigExponentFloat::from_sci_string_with_options_prec(
                &x.to_string_base(base),
                options,
                prec,
            )
            .unwrap()
            .0;
            assert_eq!(
                ComparableBigExponentFloatRef(&y),
                ComparableBigExponentFloatRef(&x)
            );
        }
    });
}

#[test]
#[should_panic]
fn to_string_base_fail_1() {
    BigExponentFloat::ONE.to_string_base(1);
}

#[test]
#[should_panic]
fn to_string_base_fail_2() {
    BigExponentFloat::ONE.to_string_base(37);
}

#[test]
#[should_panic]
fn to_string_base_upper_fail() {
    BigExponentFloat::ONE.to_string_base_upper(37);
}
