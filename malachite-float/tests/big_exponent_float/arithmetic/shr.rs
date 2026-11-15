// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_pair_gen, float_signed_pair_gen,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_q::Rational;

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

#[test]
fn test_shr() {
    let test = |s: &str, s_hex: &str, k: i64, t: &str, t_hex: &str| {
        let x = &parse_big_exponent_float_hex(s_hex);
        assert_eq!(x.to_string(), s);
        let out = &parse_big_exponent_float_hex(t_hex);
        assert_eq!(out.to_string(), t);
        let r = x.clone() >> k;
        assert!(r.is_valid());
        assert!(same(&r, out));
        assert!(same(&(x >> k), out));
        let mut m = x.clone();
        m >>= k;
        assert!(same(&m, out));
    };
    test("NaN", "NaN", 10, "NaN", "NaN");
    test("Infinity", "Infinity", -10, "Infinity", "Infinity");
    test("0.0", "0x0.0", 10, "0.0", "0x0.0");
    test("1.0", "0x1.0", 10, "0.00098", "0x0.004");
    test("1.0", "0x1.0", -10, "1.0e3", "0x4.0E+2");
    test("-12.0", "-0xc.0", 3, "-1.5", "-0x1.8");
    test(
        "1.0",
        "0x1.0",
        i64::MAX,
        "1.4e-2776511644261678566",
        "0x2.0E-2305843009213693952",
    );
    test(
        "1.0",
        "0x1.0",
        i64::MIN,
        "1.4e2776511644261678566",
        "0x1.0E+2305843009213693952",
    );
}

#[test]
fn shr_properties() {
    big_exponent_float_unsigned_pair_gen::<u8>().test_properties(|(x, k)| {
        let r = &x >> k;
        assert!(r.is_valid());
        assert!(same(&(&r << k), &x));
        assert!(same(&(x.clone() >> k), &r));
        assert!(same(&(&x << -i16::from(k)), &r));
        assert_eq!(r.get_prec(), x.get_prec());
        if x.is_finite() && big_exponent_float_exponent_is_small(&x) {
            assert_eq!(Rational::exact_from(&r), Rational::exact_from(&x) >> k);
        }
    });

    float_signed_pair_gen::<i8>().test_properties(|(x, k)| {
        let f = &x >> k;
        if f.is_normal()
            && f.get_exponent().unwrap() < Float::MAX_EXPONENT
            && f.get_exponent().unwrap() > Float::MIN_EXPONENT
        {
            assert!(same(
                &(float_to_big_exponent_float(&x) >> k),
                &float_to_big_exponent_float(&f)
            ));
        }
    });
}
