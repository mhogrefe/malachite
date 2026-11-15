// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::strings::typst::ToTypst;
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::big_exponent_float_to_float;
use malachite_float::test_util::generators::big_exponent_float_gen;

#[test]
fn test_to_typst_string() {
    let test = |s: &str, s_hex: &str, out: &str| {
        let x = BigExponentFloat::from_string_base(16, s_hex).unwrap();
        assert_eq!(x.to_string(), s);
        assert_eq!(x.to_typst_string(), out);
        if let Some(f) = big_exponent_float_to_float(&x) {
            assert_eq!(f.to_typst_string(), out);
        }
    };
    test("1.5", "0x1.8", "1.5");
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3.1415926535897931",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        "0.10000000000000001",
    );
    test("-1000.0", "-0x3e8.0", "-1000.0");
    test("9.1e-13", "0x1.0E-10", "9.1 times 10^(-13)");
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        "4.13159956 times 10^(121)",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "2.4 times 10^(30102999566398119521)",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "4.2 times 10^(-30102999566398119522)",
    );
    test(
        "1.3185758957267685e323228497",
        "0x3.243f6a8885a30E+268435456",
        "1.3185758957267685 times 10^(323228497)",
    );
    test(
        "-5.5e-323228502",
        "-0x1.8E-268435460",
        "-5.5 times 10^(-323228502)",
    );
    test(
        "4.2e323228496",
        "0x1.0E+268435456",
        "4.2 times 10^(323228496)",
    );
    test(
        "-2.10064313e120411998265592478085",
        "-0xa.bcdefE+99999999999999999999",
        "-2.10064313 times 10^(120411998265592478085)",
    );
    test("0.0", "0x0.0", "0.0");
    test("NaN", "NaN", "\"NaN\"");
    test("Infinity", "Infinity", "infinity");
    test("-Infinity", "-Infinity", "-infinity");
}

#[test]
fn to_typst_string_properties() {
    big_exponent_float_gen().test_properties(|x| {
        let s = x.to_typst_string();
        // - agreement with `Float`
        if let Some(f) = big_exponent_float_to_float(&x) {
            assert_eq!(f.to_typst_string(), s);
        }
        // - a power of ten appears exactly when `Display` writes an exponent
        if x.is_finite() {
            assert_eq!(s.contains("times 10^("), x.to_string().contains('e'));
        }
    });
}
