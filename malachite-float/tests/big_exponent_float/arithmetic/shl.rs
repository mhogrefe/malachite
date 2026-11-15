// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_power_of_2,
    float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_signed_pair_gen, float_signed_pair_gen,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use malachite_q::Rational;

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

#[test]
fn test_shl() {
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let test = |s: &str, s_hex: &str, k: i64, t: &str, t_hex: &str| {
        let x = &parse_big_exponent_float_hex(s_hex);
        assert_eq!(x.to_string(), s);
        let out = &parse_big_exponent_float_hex(t_hex);
        assert_eq!(out.to_string(), t);
        let l = x.clone() << k;
        assert!(l.is_valid());
        assert!(same(&l, out));
        assert!(same(&(x << k), out));
        let mut m = x.clone();
        m <<= k;
        assert!(same(&m, out));
        let neg_k = -i128::from(k);
        assert!(same(&(x.clone() >> neg_k), out));
        assert!(same(&(x >> neg_k), out));
        let mut m = x.clone();
        m >>= neg_k;
        assert!(same(&m, out));
    };
    test("NaN", "NaN", 10, "NaN", "NaN");
    test("Infinity", "Infinity", -10, "Infinity", "Infinity");
    test("0.0", "0x0.0", 10, "0.0", "0x0.0");
    test("1.0", "0x1.0", 0, "1.0", "0x1.0");
    test("1.0", "0x1.0", 10, "1.0e3", "0x4.0E+2");
    test("1.0", "0x1.0", -10, "0.00098", "0x0.004");
    test("-1.5", "-0x1.8", 3, "-12.0", "-0xc.0");
    // Far beyond Float's exponent range, exactly.
    test(
        "1.0",
        "0x1.0",
        i64::MAX,
        "6.9e2776511644261678565",
        "0x8.0E+2305843009213693951",
    );
    test(
        "1.0",
        "0x1.0",
        i64::MIN,
        "7.2e-2776511644261678567",
        "0x1.0E-2305843009213693952",
    );
    assert!(same(
        &(&huge << u128::MAX),
        &big_exponent_float_power_of_2(big + Integer::from(u128::MAX))
    ));
}

#[test]
fn shl_properties() {
    big_exponent_float_signed_pair_gen::<i64>().test_properties(|(x, k)| {
        let l = &x << k;
        assert!(l.is_valid());
        assert!(same(&(x.clone() << k), &l));
        assert!(same(&(&x >> -i128::from(k)), &l));
        assert!(same(&(&l >> k), &x));
        assert_eq!(l.get_prec(), x.get_prec());
        if x.is_finite() && !x.is_zero() {
            assert_eq!(
                l.get_exponent().unwrap(),
                &(x.get_exponent().unwrap() + Integer::from(k))
            );
            if big_exponent_float_exponent_is_small(&x) && k.unsigned_abs() < 1 << 16 {
                assert_eq!(Rational::exact_from(&l), Rational::exact_from(&x) << k);
            }
        }
    });

    float_signed_pair_gen::<i8>().test_properties(|(x, k)| {
        let f = &x << k;
        if f.is_normal()
            && f.get_exponent().unwrap() < Float::MAX_EXPONENT
            && f.get_exponent().unwrap() > Float::MIN_EXPONENT
        {
            assert!(same(
                &(float_to_big_exponent_float(&x) << k),
                &float_to_big_exponent_float(&f)
            ));
        }
    });
}
